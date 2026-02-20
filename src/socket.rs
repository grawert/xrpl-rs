use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::sync::{broadcast, mpsc, oneshot, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::error::XrplError;

type PendingMap = Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>;
type SubscriptionList = Arc<Mutex<Vec<Value>>>;

static REQUEST_ID: AtomicU64 = AtomicU64::new(1);

fn next_id() -> u64 {
    REQUEST_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone)]
pub struct XrplSocket {
    outgoing: mpsc::Sender<String>,
    pending: PendingMap,
    events: broadcast::Sender<Value>,
    subscriptions: SubscriptionList,
}

impl XrplSocket {
    pub async fn connect(url: &str) -> Result<Self, XrplError> {
        let (outgoing_tx, outgoing_rx) = mpsc::channel(32);
        let (events_tx, _) = broadcast::channel(64);
        let pending: PendingMap = Arc::new(Mutex::new(HashMap::new()));
        let subscriptions: SubscriptionList = Arc::new(Mutex::new(Vec::new()));

        tokio::spawn(connection_loop(
            url.to_string(),
            outgoing_rx,
            pending.clone(),
            events_tx.clone(),
            subscriptions.clone(),
        ));

        Ok(Self {
            outgoing: outgoing_tx,
            pending,
            events: events_tx,
            subscriptions,
        })
    }

    pub async fn request(
        &self,
        mut payload: Value,
    ) -> Result<Value, XrplError> {
        let id = next_id();
        payload["id"] = id.into();

        let (tx, rx) = oneshot::channel();
        self.pending.lock().await.insert(id, tx);

        self.outgoing
            .send(payload.to_string())
            .await
            .map_err(|_| XrplError::Disconnected)?;

        tokio::time::timeout(Duration::from_secs(30), rx)
            .await
            .map_err(|_| XrplError::Timeout(30_000))?
            .map_err(|_| XrplError::Disconnected)
    }

    pub async fn track_subscription(&self, payload: Value) {
        self.subscriptions.lock().await.push(payload);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Value> {
        self.events.subscribe()
    }

    pub fn is_connected(&self) -> bool {
        !self.outgoing.is_closed()
    }
}

async fn connection_loop(
    url: String,
    mut outgoing_rx: mpsc::Receiver<String>,
    pending: PendingMap,
    events: broadcast::Sender<Value>,
    subscriptions: SubscriptionList,
) {
    let mut backoff = Duration::from_secs(1);
    const MAX_BACKOFF: Duration = Duration::from_secs(30);

    loop {
        match connect_async(&url).await {
            Ok((ws_stream, _)) => {
                backoff = Duration::from_secs(1);
                let (mut write, mut read) = ws_stream.split();

                // Re-subscribe to active subscriptions after reconnect
                for sub in subscriptions.lock().await.iter() {
                    let _ =
                        write.send(Message::Text(sub.to_string().into())).await;
                }

                loop {
                    tokio::select! {
                        msg = outgoing_rx.recv() => {
                            match msg {
                                Some(text) => {
                                    if write.send(Message::Text(text.into())).await.is_err() {
                                        break;
                                    }
                                }
                                None => return, // sender dropped, shut down
                            }
                        }
                        msg = read.next() => {
                            match msg {
                                Some(Ok(Message::Text(text))) => {
                                    let value: Value = match serde_json::from_str(text.as_str()) {
                                        Ok(v) => v,
                                        Err(_) => continue,
                                    };

                                    if let Some(id) = value["id"].as_u64() {
                                        if let Some(tx) = pending.lock().await.remove(&id) {
                                            let _ = tx.send(value);
                                        }
                                    } else {
                                        let _ = events.send(value);
                                    }
                                }
                                _ => break, // connection dropped, reconnect
                            }
                        }
                    }
                }

                // Clear pending requests — they won't get responses
                pending.lock().await.clear();
            }
            Err(e) => {
                eprintln!(
                    "Connection failed: {e}, retrying in {}s",
                    backoff.as_secs()
                );
            }
        }

        tokio::time::sleep(backoff).await;
        backoff = (backoff * 2).min(MAX_BACKOFF);
    }
}
