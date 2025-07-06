use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use super::{XrplRequest, XrplResponse};
use crate::types::Amount;
use crate::types::AccountObject;

#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct AccountObjectsRequest {
    pub account: String,
    pub amount: Option<Amount>,
    pub deletion_blockers_only: Option<bool>,
    pub ledger_hash: Option<String>,
    pub ledger_index: Option<String>,
    pub limit: Option<u32>,
    pub marker: Option<Value>,
    pub transfer_rate: Option<i64>,
    #[serde(rename = "type")]
    pub kind: Option<AccountObjectRequestType>,
}

#[derive(Serialize)]
pub enum AccountObjectRequestType {
    Bridge,
    Check,
    DepositPreauth,
    Escrow,
    MPToken,
    NFTokenOffer,
    NFTokenPage,
    Offer,
    PayChannel,
    RippleState,
    SignerList,
    Ticket,
}

impl From<AccountObjectsRequest> for Value {
    fn from(val: AccountObjectsRequest) -> Self {
        let value = serde_json::to_value(val);
        if let Err(e) = &value {
            dbg!(e);
        };
        let mut value = value.unwrap().as_object().unwrap().to_owned();
        value.insert("id".into(), Uuid::new_v4().to_string().into());
        value.insert("command".into(), "account_objects".into());
        value.into()
    }
}

impl XrplRequest for AccountObjectsRequest {
    type Response = XrplResponse<AccountObjectsResult>;
}

#[derive(Debug, Deserialize)]
pub struct AccountObjectsResult {
    pub account: String,
    pub account_objects: Vec<AccountObject>,
    pub ledger_hash: Option<String>,
    pub ledger_index: Option<i64>,
    pub ledger_current_index: Option<i64>,
    pub limit: Option<u32>,
    pub marker: Option<Value>,
    pub validated: bool,
}
