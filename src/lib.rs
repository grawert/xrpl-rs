pub mod error;
pub mod request;
pub mod socket;
pub mod subscriptions;
pub mod types;

use tokio::sync::broadcast;
use serde_json::Value;

use error::XrplError;
use socket::XrplSocket;
use request::{XrplRequest, XrplSubscription};

#[derive(Debug, Clone)]
pub struct XrplClient {
    pub url: String,
    socket: XrplSocket,
}

impl XrplClient {
    pub async fn new(url: &str) -> Result<XrplClient, XrplError> {
        let socket = XrplSocket::new(url, None).await?;
        Ok(XrplClient { url: url.into(), socket })
    }

    pub async fn call(
        &self,
        request: impl Into<Value>,
    ) -> Result<String, XrplError> {
        let request = request.into();
        let response = self.socket.request(request).await?;

        if let Ok(parsed) = serde_json::from_str::<Value>(&response) {
            if let Some(error) = parsed.get("error") {
                return Err(XrplError::ApiError {
                    error: error.as_str().unwrap_or("unknown").to_string(),
                    error_message: parsed
                        .get("error_message")
                        .and_then(|msg| msg.as_str())
                        .map(|s| s.to_string()),
                });
            }
        }
        Ok(response)
    }

    pub async fn request<T: XrplRequest>(
        &self,
        request: T,
    ) -> Result<T::Response, XrplError> {
        let response = self.call(request).await?;
        let parsed = serde_json::from_str::<T::Response>(&response)
            .map_err(|e| XrplError::ParseError(e.to_string()))?;
        Ok(parsed)
    }

    pub async fn subscribe<T: XrplSubscription>(
        &self,
        subscription: T,
    ) -> Result<(T::Response, broadcast::Receiver<T::Message>), XrplError> {
        let subscription_value: Value = subscription.into();

        let response = self.call(subscription_value).await?;
        let response = serde_json::from_str::<T::Response>(&response)
            .map_err(|e| XrplError::ParseError(e.to_string()))?;

        let receiver = self.socket.subscribe::<T>().await?;
        Ok((response, receiver))
    }

    pub fn is_connected(&self) -> bool {
        self.socket.is_connected()
    }

    pub async fn close(&self) {
        self.socket.close().await;
    }
}
