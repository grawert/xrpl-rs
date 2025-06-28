use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::{
    sync::{broadcast, mpsc, oneshot},
    time,
};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio_util::sync::CancellationToken;

use crate::error::XrplSocketError;
use crate::request::XrplSubscription;

pub struct XrplSocket {
    receiver: broadcast::Receiver<String>,
    sender: mpsc::Sender<String>,
    timeout_dur: Option<i64>,
    cancel: CancellationToken,
}

impl XrplSocket {
    pub async fn new(
        url: &str,
        timeout_dur: Option<i64>,
    ) -> Result<XrplSocket, XrplSocketError> {
        let (receiver_out, receiver) = broadcast::channel(1000);
        let (sender, mut sender_in) = mpsc::channel(1000);

        let client = XrplSocket {
            receiver,
            sender,
            timeout_dur,
            cancel: CancellationToken::new(),
        };

        let (stream, _) = connect_async(url).await?;
        let (mut ws_sender, mut ws_receiver) = stream.split();

        // Split the WebSocket sender for sharing between tasks
        let (ws_sender_tx, mut ws_sender_rx) = mpsc::channel::<Message>(100);

        // Task to handle outbound WebSocket messages (including pings and pongs)
        let cancel_ws_out = client.cancel.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    msg = ws_sender_rx.recv() => {
                        match msg {
                            Some(msg) => {
                                if let Err(e) = ws_sender.send(msg).await {
                                    eprintln!("Error sending WebSocket message: {e:?}");
                                    cancel_ws_out.cancel();
                                    break;
                                }
                            },
                            None => break, // Channel closed
                        }
                    }
                    _ = cancel_ws_out.cancelled() => {
                        break;
                    }
                }
            }
        });

        // Receive messages from the ws receiver, and send them over broadcast sender
        let cancel = client.cancel.clone();
        let receiver_out_clone = receiver_out.clone();
        let ws_sender_for_pong = ws_sender_tx.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    msg = ws_receiver.next() => {
                        match msg {
                            Some(Ok(Message::Text(msg))) => {
                                if let Err(e) = receiver_out_clone.send(msg) {
                                    eprintln!("Error sending websocket response over broadcast channel: {e:?}");
                                    // Don't cancel here - might just be no receivers
                                }
                            },
                            Some(Ok(Message::Ping(data))) => {
                                // Handle ping by sending pong
                                if let Err(e) = ws_sender_for_pong.send(Message::Pong(data)).await {
                                    eprintln!("Failed to send pong: {e:?}");
                                    cancel.cancel();
                                }
                            },
                            Some(Ok(Message::Close(_))) => {
                                eprintln!("WebSocket connection closed by server");
                                cancel.cancel();
                                break;
                            },
                            Some(Err(e)) => {
                                eprintln!("WebSocket error: {e:?}");
                                cancel.cancel();
                                break;
                            },
                            None => {
                                eprintln!("WebSocket stream ended");
                                cancel.cancel();
                                break;
                            },
                            _ => {
                                // Handle other message types (Binary, Pong, etc.)
                                continue;
                            }
                        }
                    }
                    _ = time::sleep(Duration::from_secs(30)) => {
                        // Longer timeout for detecting dead connections
                        eprintln!("WebSocket receive timeout - connection may be dead");
                        cancel.cancel();
                        break;
                    }
                    _ = cancel.cancelled() => {
                        break;
                    }
                }
            }
        });

        // Receive message from the mpsc receiver, send them over ws sender, or ping periodically
        let cancel = client.cancel.clone();
        let ws_sender_for_requests = ws_sender_tx;
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    msg = sender_in.recv() => {
                        match msg {
                            Some(msg) => {
                                if let Err(e) = ws_sender_for_requests.send(Message::Text(msg)).await {
                                    eprintln!("Error sending request message: {e:?}");
                                    cancel.cancel();
                                    break;
                                }
                            },
                            None => {
                                // Sender channel closed
                                break;
                            }
                        }
                    }
                    _ = time::sleep(Duration::from_secs(30)) => {
                        // Send ping to keep connection alive
                        if let Err(e) = ws_sender_for_requests.send(Message::Ping(Vec::new())).await {
                            eprintln!("Failed to ping socket: {e}");
                            cancel.cancel();
                            break;
                        }
                    }
                    _ = cancel.cancelled() => {
                        break;
                    }
                }
            }
        });

        Ok(client)
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

        // Validate request has required fields
        let req_obj = request.as_object().ok_or_else(|| {
            XrplSocketError::InvalidRequest {
                field: "request must be an object".to_string(),
            }
        })?;

        let req_id = req_obj.get("id").ok_or_else(|| {
            XrplSocketError::InvalidRequest { field: "id".to_string() }
        })?;

        // Send the request
        self.sender
            .send(request.to_string())
            .await
            .map_err(|_| XrplSocketError::ChannelSendError)?;

        let cancel = self.cancel.clone();
        let req_id = req_id.clone();

        tokio::spawn(async move {
            loop {
                if cancel.is_cancelled() {
                    break;
                }

                match ws_receiver.recv().await {
                    Ok(msg) => {
                        match serde_json::from_str::<Value>(&msg) {
                            Ok(response) => {
                                if let Some(response_obj) = response.as_object()
                                {
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
                                // Continue listening for the right response
                            }
                            Err(e) => {
                                eprintln!("Failed to parse response JSON: {e}");
                                // Continue listening - might be a different message type
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        // We're lagging behind, continue listening
                        continue;
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        // Channel closed, connection is dead
                        break;
                    }
                }
            }
        });

        let timeout_ms = self.timeout_dur.unwrap_or(5000) as u64;

        tokio::select! {
            res = out_rec => {
                res.map_err(|_| XrplSocketError::ChannelReceiveError)
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
                                    eprintln!("Failed to send subscription message: {e}");
                                    // If no receivers, that's fine - continue
                                }
                            }
                            Err(_) => {
                                // Message doesn't match this subscription type - continue
                                continue;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        // We're lagging behind, continue
                        continue;
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        // Connection closed
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
