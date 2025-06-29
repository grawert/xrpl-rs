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
pub mod tx;

use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::error::XrplError;

pub trait XrplRequest: Into<Value> {
    type Response: Debug + DeserializeOwned;
}

pub trait XrplSubscription: XrplRequest + Serialize {
    type Message: Clone + Debug + Send + DeserializeOwned + 'static;
}

#[skip_serializing_none]
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
        id: Option<String>,
        error: String,
        error_code: Option<i32>,
        error_message: Option<String>,
        request: Option<serde_json::Value>,
        #[serde(rename = "type")]
        kind: String,
        status: String,
    },
}

impl<T> XrplResponse<T> {
    pub fn result(self) -> Result<T, XrplError> {
        match self {
            XrplResponse::Success { result, .. } => Ok(result),
            XrplResponse::Error { error, error_message, .. } => {
                Err(XrplError::ApiError { error, error_message })
            }
        }
    }

    pub fn id(&self) -> Option<&str> {
        match self {
            XrplResponse::Success { id, .. } => Some(id),
            XrplResponse::Error { id, .. } => id.as_deref(),
        }
    }
}
