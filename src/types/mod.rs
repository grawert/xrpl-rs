pub mod account_object;
pub mod builders;
pub mod transaction;

pub use builders::*;
pub use account_object::*;
pub use transaction::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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
