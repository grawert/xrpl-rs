use std::fmt::Debug;

use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub mod account_channels;
pub mod account_currencies;
pub mod account_info;
pub mod account_lines;
pub mod account_nfts;
pub mod account_objects;
pub mod account_offers;
pub mod account_tx;
pub mod server_info;
pub mod submit;

pub trait XrplRequest: Into<Value> {
    type Response: Debug + DeserializeOwned;
}

pub trait XrplSubscription: XrplRequest {
    type Message: Clone + Debug + Send + DeserializeOwned + 'static;
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum XrplResponse<T> {
    Success {
        id: String,
        result: T,
        #[serde(rename = "type")]
        kind: String,
        status: String,
    },
    Error {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        error: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        error_exception: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error_code: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error_message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        request: Option<serde_json::Value>,
        #[serde(rename = "type")]
        kind: String,
        status: String,
    },
}

impl<T> XrplResponse<T> {
    pub fn result(self) -> Result<T, String> {
        match self {
            XrplResponse::Success { result, .. } => Ok(result),
            XrplResponse::Error {
                error,
                error_exception,
                error_message,
                ..
            } => {
                let parts: Vec<&str> = [
                    Some(error.as_str()),
                    error_exception.as_deref(),
                    error_message.as_deref(),
                ]
                .into_iter()
                .flatten()
                .collect();

                Err(parts.join(": "))
            }
        }
    }
}
