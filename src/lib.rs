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
        Ok(XrplClient {
            url: url.into(),
            socket: XrplSocket::new(url, None).await?,
        })
    }

    pub async fn call(
        &self,
        request: impl Into<Value>,
    ) -> Result<String, XrplError> {
        let response = self.socket.request(request.into()).await?;

        // Parse response to check for XRPL API errors
        if let Ok(parsed) = serde_json::from_str::<Value>(&response) {
            if let Some(error) = parsed.get("error") {
                return Err(XrplError::ApiError {
                    error: error.as_str().unwrap_or("unknown").to_string(),
                    error_exception: parsed
                        .get("error_exception")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    error_message: parsed
                        .get("error_message")
                        .and_then(|v| v.as_str())
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
        request: T,
    ) -> Result<(T::Response, broadcast::Receiver<T::Message>), XrplError> {
        let response = self.request(request).await?;
        let receiver = self.socket.subscribe::<T>().await?;
        Ok((response, receiver))
    }
}
