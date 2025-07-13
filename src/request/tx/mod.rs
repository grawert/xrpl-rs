use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use super::{XrplRequest, XrplResponse};
use crate::types::Transaction;

const API_VERSION: u32 = 2;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
pub struct TxRequest {
    pub ctid: Option<String>,
    pub transaction: Option<String>,
    pub binary: Option<bool>,
    pub min_ledger: Option<u32>,
    pub max_ledger: Option<u32>,
}

impl From<TxRequest> for Value {
    fn from(val: TxRequest) -> Self {
        let value = serde_json::to_value(val);
        if let Err(e) = &value {
            dbg!(e);
        };
        let mut value = value.unwrap().as_object().unwrap().to_owned();
        value.insert("id".into(), Uuid::new_v4().to_string().into());
        value.insert("command".into(), "tx".into());
        value.insert("api_version".into(), API_VERSION.into());
        value.into()
    }
}

impl XrplRequest for TxRequest {
    type Response = XrplResponse<TxResponse>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct TxResponse {
    pub close_time_iso: String,
    pub ctid: String,
    pub hash: String,
    pub ledger_hash: String,
    pub ledger_index: u32,
    pub meta: Value,
    pub tx_json: Transaction,
    pub validated: bool,
}
