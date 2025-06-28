use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use super::{XrplRequest, XrplResponse};
use crate::types::Amount;

#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct AccountOffersRequest {
    pub account: String,
    pub ledger_hash: Option<String>,
    pub ledger_index: i64,
    pub limit: Option<i64>,
    pub marker: Option<Value>,
}

impl From<AccountOffersRequest> for Value {
    fn from(val: AccountOffersRequest) -> Self {
        let value = serde_json::to_value(val);
        if let Err(e) = &value {
            dbg!(e);
        };
        let mut value = value.unwrap().as_object().unwrap().to_owned();
        value.insert("id".into(), Uuid::new_v4().to_string().into());
        value.insert("command".into(), "account_offers".into());
        value.into()
    }
}

impl XrplRequest for AccountOffersRequest {
    type Response = XrplResponse<AccountOffersResponse>;
}

#[derive(Debug, Deserialize)]
pub struct AccountOffersResponse {
    pub account: String,
    pub offers: Vec<AccountOffer>,
    pub ledger_current_index: Option<i64>,
    pub ledger_index: Option<i64>,
    pub ledger_hash: Option<String>,
    pub marker: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountOffer {
    pub flags: i64,
    pub seq: i64,
    pub taker_gets: Amount,
    pub taker_pays: Amount,
    pub quality: String,
    pub expiration: Option<i64>,
}
