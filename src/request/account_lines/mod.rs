use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use super::{XrplRequest, XrplResponse};

#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct AccountLinesRequest {
    pub account: String,
    pub ingnore_default: Option<bool>,
    pub ledger_hash: Option<String>,
    pub ledger_index: Option<String>,
    pub limit: Option<i64>,
    pub marker: Option<Value>,
    pub peer: Option<String>,
}

impl From<AccountLinesRequest> for Value {
    fn from(val: AccountLinesRequest) -> Self {
        let value = serde_json::to_value(val);
        if let Err(e) = &value {
            dbg!(e);
        };
        let mut value = value.unwrap().as_object().unwrap().to_owned();
        value.insert("id".into(), Uuid::new_v4().to_string().into());
        value.insert("command".into(), "account_lines".into());
        value.insert("api_version".into(), 2.into());
        value.into()
    }
}

impl XrplRequest for AccountLinesRequest {
    type Response = XrplResponse<AccountLinesResult>;
}

#[derive(Debug, Deserialize)]
pub struct AccountLinesResult {
    pub account: String,
    pub lines: Vec<Trustline>,
    pub ledger_current_index: Option<i64>,
    pub ledger_index: Option<i64>,
    pub ledger_hash: Option<String>,
    pub marker: Option<Value>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Trustline {
    pub account: String,
    pub balance: String,
    pub currency: String,
    pub limit: String,
    pub limit_peer: String,
    pub quality_in: i64,
    pub quality_out: i64,
    pub no_ripple: Option<bool>,
    pub no_ripple_peer: Option<bool>,
    pub authorized: Option<bool>,
    pub peer_authorized: Option<bool>,
    pub freeze: Option<bool>,
    pub freeze_peer: Option<bool>,
}
