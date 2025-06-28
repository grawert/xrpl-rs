pub mod account_object;
pub mod builders;
pub mod transaction;

pub use builders::*;
pub use transaction::*;
pub use account_object::*;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Amount {
    Xrpl(String),
    IssuedCurrency { value: String, currency: String, issuer: String },
}

impl Default for Amount {
    fn default() -> Self {
        Amount::Xrpl("0".into())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[skip_serializing_none]
pub struct PathStep {
    pub account: Option<String>,
    pub currency: Option<String>,
    pub isssuer: Option<String>,
}
