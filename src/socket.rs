use std::time::Duration;
use tracing::*;
use serde_json::Value;
use futures_util::{SinkExt, StreamExt};
use tokio::{
    sync::{broadcast, mpsc, oneshot},
    time,
};
use tokio_util::sync::CancellationToken;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::error::XrplSocketError;
use crate::request::XrplSubscription;

const PING_INTERVAL: Duration = Duration::from_secs(30);
const WEBSOCKET_RECEIVE_TIMEOUT: Duration = Duration::from_secs(40);

#[derive(Debug)]
pub struct XrplSocket {
    cancel: CancellationToken,
    sender: mpsc::Sender<String>,
    receiver: broadcast::Receiver<String>,
    timeout_dur: Option<i64>,
}

impl Clone for XrplSocket {
    fn clone(&self) -> Self {
        Self {
            receiver: self.receiver.resubscribe(),
            sender: self.sender.clone(),
            timeout_dur: self.timeout_dur,
            cancel: self.cancel.clone(),
        }
    }
}

impl XrplSocket {
    pub async fn new(
        url: &str,
        timeout_dur: Option<i64>,
    ) -> Result<XrplSocket, XrplSocketError> {
        let (receiver_out, receiver) = broadcast::channel(1000);
        let (sender, sender_in) = mpsc::channel(1000);

        let socket = XrplSocket {
            receiver,
            sender,
            timeout_dur,
            cancel: CancellationToken::new(),
        };

        socket.start_connection(url.to_string(), receiver_out, sender_in).await;

        Ok(socket)
    }

    async fn start_connection(
        &self,
        url: String,
        receiver_out: broadcast::Sender<String>,
        sender_in: mpsc::Receiver<String>,
    ) {
        let cancel = self.cancel.clone();

        tokio::spawn(async move {
            let mut sender_in = sender_in;
            let _ = Self::connect_and_run(
                url,
                receiver_out,
                &mut sender_in,
                cancel,
            )
            .await;
        });
    }

    async fn connect_and_run(
        url: String,
        receiver_out: broadcast::Sender<String>,
        sender_in: &mut mpsc::Receiver<String>,
        cancel: CancellationToken,
    ) -> Result<(), XrplSocketError> {
        let (stream, _) = connect_async(&url).await?;
        let (mut ws_sender, mut ws_receiver) = stream.split();

        let mut ping_interval = time::interval(PING_INTERVAL);
        let mut last_receive = time::Instant::now();

        loop {
            tokio::select! {
                msg = ws_receiver.next() => {
                    match msg {
                        Some(Ok(Message::Text(msg))) => {
                            if let Err(e) = receiver_out.send(msg) {
                                error!("Error sending websocket response over broadcast channel: {e:?}");
                            }
                            last_receive = time::Instant::now();
                        },
                        Some(Ok(Message::Ping(data))) => {
                            if let Err(e) = ws_sender.send(Message::Pong(data)).await {
                                warn!("Failed to send pong: {e:?}");
                                return Err(XrplSocketError::Disconnected);
                            }
                            last_receive = time::Instant::now();
                        },
                        Some(Ok(Message::Pong(_))) => {
                            last_receive = time::Instant::now();
                        },
                        Some(Ok(Message::Close(_))) => {
                            warn!("WebSocket connection closed by server");
                            return Err(XrplSocketError::Disconnected);
                        },
                        Some(Err(e)) => {
                            error!("WebSocket error: {e:?}");
                            return Err(XrplSocketError::Disconnected);
                        },
                        _ => {
                            warn!("WebSocket stream ended");
                            return Err(XrplSocketError::Disconnected);
                        },
                    }
                }

                msg = sender_in.recv() => {
                    match msg {
                        Some(msg) => {
                            if let Err(e) = ws_sender.send(Message::Text(msg)).await {
                                error!("Error sending request message: {e:?}");
                                return Err(XrplSocketError::Disconnected);
                            }
                        },
                        _ => {
                            return Ok(());
                        }
                    }
                }

                _ = ping_interval.tick() => {
                    if let Err(e) = ws_sender.send(Message::Ping(Vec::new())).await {
                        warn!("Failed to ping socket: {e}");
                        return Err(XrplSocketError::Disconnected);
                    }
                }

                _ = time::sleep(Duration::from_secs(1)) => {
                    if last_receive.elapsed() > WEBSOCKET_RECEIVE_TIMEOUT {
                        warn!("WebSocket receive timeout - connection may be dead");
                        return Err(XrplSocketError::Disconnected);
                    }
                }

                _ = cancel.cancelled() => {
                    warn!("Connection cancelled");
                    return Ok(());
                }
            }
        }
    }

    pub async fn request(
        &self,
        request: Value,
    ) -> Result<String, XrplSocketError> {
        if self.cancel.is_cancelled() {
            return Err(XrplSocketError::Disconnected);
        }

        let mut ws_receiver = self.receiver.resubscribe();
        let (out_sender, out_rec) = oneshot::channel::<String>();

        let req_obj = request.as_object().ok_or_else(|| {
            XrplSocketError::InvalidRequest {
                field: "request must be an object".to_string(),
            }
        })?;

        let req_id = req_obj.get("id").ok_or_else(|| {
            XrplSocketError::InvalidRequest { field: "id".to_string() }
        })?;

        self.sender
            .send(request.to_string())
            .await
            .map_err(|_| XrplSocketError::Disconnected)?;

        let cancel = self.cancel.clone();
        let req_id = req_id.clone();

        tokio::spawn(async move {
            loop {
                if cancel.is_cancelled() {
                    break;
                }

                match ws_receiver.recv().await {
                    Ok(msg) => match serde_json::from_str::<Value>(&msg) {
                        Ok(response) => {
                            if let Some(response_obj) = response.as_object() {
                                if let Some(response_id) =
                                    response_obj.get("id")
                                {
                                    if req_id == *response_id {
                                        let _ = out_sender
                                            .send(response.to_string());
                                        break;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to parse response JSON: {e}");
                        }
                    },
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        continue;
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
        });

        let timeout_ms = self.timeout_dur.unwrap_or(5000) as u64;

        tokio::select! {
            res = out_rec => {
                res.map_err(|_| XrplSocketError::Disconnected)
            },
            _ = tokio::time::sleep(Duration::from_millis(timeout_ms)) => {
                Err(XrplSocketError::RequestTimeout { timeout_ms })
            }
        }
    }

    pub async fn subscribe<T: XrplSubscription>(
        &self,
    ) -> Result<broadcast::Receiver<T::Message>, XrplSocketError> {
        if self.cancel.is_cancelled() {
            return Err(XrplSocketError::Disconnected);
        }

        let cancel = self.cancel.clone();
        let mut ws_receiver = self.receiver.resubscribe();
        let (sender, receiver) = broadcast::channel::<T::Message>(100);

        tokio::spawn(async move {
            loop {
                if cancel.is_cancelled() {
                    break;
                }

                match ws_receiver.recv().await {
                    Ok(msg) => {
                        match serde_json::from_str::<T::Message>(&msg) {
                            Ok(parsed) => {
                                if let Err(e) = sender.send(parsed) {
                                    warn!("Failed to send subscription message: {e}");
                                }
                            }
                            Err(_) => {
                                continue;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        continue;
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
        });

        Ok(receiver)
    }

    pub fn is_connected(&self) -> bool {
        !self.cancel.is_cancelled()
    }

    pub async fn close(&self) {
        self.cancel.cancel();
    }
}
