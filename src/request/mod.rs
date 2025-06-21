use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde_derive::Deserialize;
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

// pub use account_channels::AccountChannelsRequest;
// pub use account_currencies::AccountCurrenciesRequest;
// pub use account_info::AccountInfoRequest;
// pub use account_lines::AccountLinesRequest;
// pub use account_nfts::AccountNftsRequest;
// pub use account_objects::AccountObjectsRequest;
// pub use account_offers::AccountOffersRequest;
// pub use account_tx::AccountTxRequest;
// pub use server_info::ServerInfoRequest;
// pub use submit::SubmitRequest;
// pub use subscriptions::ledger::LedgerClosedSubscription;

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
        id: String,
        error: String,
        error_exception: String,
        #[serde(rename = "type")]
        kind: String,
        status: String,
    },
}

impl<T> XrplResponse<T> {
    pub fn result(self) -> Result<T, String> {
        match self {
            XrplResponse::Success { result, .. } => Ok(result),
            XrplResponse::Error { error, error_exception, .. } => {
                Err(format!("{}: {}", error, error_exception))
            }
        }
    }
}
