use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use super::{XrplRequest, XrplResponse};
use crate::types::transaction::Transaction;

#[derive(Default, Serialize)]
#[skip_serializing_none]
pub struct AccountTxRequest {
    pub account: String,
    pub ledger_index_min: Option<i64>,
    pub ledger_index_max: Option<i64>,
    pub ledger_hash: Option<String>,
    pub ledger_index: Option<i64>,
    pub binary: Option<bool>,
    pub forward: Option<bool>,
    pub limit: Option<i64>,
    pub marker: Option<Value>,
}

impl From<AccountTxRequest> for Value {
    fn from(val: AccountTxRequest) -> Self {
        let value = serde_json::to_value(val);
        if let Err(e) = &value {
            dbg!(e);
        };
        let mut value = value.unwrap().as_object().unwrap().to_owned();
        value.insert("id".into(), Uuid::new_v4().to_string().into());
        value.insert("command".into(), "account_tx".into());
        value.into()
    }
}

impl XrplRequest for AccountTxRequest {
    type Response = XrplResponse<AccountOffersResponse>;
}

#[derive(Debug, Deserialize)]
pub struct AccountOffersResponse {
    pub account: String,
    pub ledger_index_min: i64,
    pub ledger_index_max: i64,
    pub marker: Option<Value>,
    pub transactions: Vec<AccountTransaction>,
    pub validated: Option<bool>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct AccountTransaction {
    pub meta: Value,
    pub tx_json: Value,
    pub ledger_index: Option<i64>,
    pub hash: String,
    pub ledger_hash: String,
    pub close_time_iso: String,
    pub validated: bool,
}
