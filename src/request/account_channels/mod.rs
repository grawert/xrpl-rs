use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use super::{XrplRequest, XrplResponse};

#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct AccountChannelsRequest {
    pub account: String,
    pub destination_account: Option<String>,
    pub ledger_hash: Option<String>,
    pub ledger_index: Option<String>,
    pub limit: Option<u32>,
    pub marker: Option<Value>,
}

impl From<AccountChannelsRequest> for Value {
    fn from(val: AccountChannelsRequest) -> Self {
        let value = serde_json::to_value(val);
        if let Err(e) = &value {
            dbg!(e);
        };
        let mut value = value.unwrap().as_object().unwrap().to_owned();
        value.insert("id".into(), Uuid::new_v4().to_string().into());
        value.insert("command".into(), "account_channels".into());
        value.into()
    }
}

impl XrplRequest for AccountChannelsRequest {
    type Response = XrplResponse<AccountChannelsResult>;
}

#[derive(Debug, Deserialize)]
pub struct AccountChannelsResult {
    pub account: String,
    pub channels: Vec<AccountChannel>,
    pub ledger_hash: Option<String>,
    pub ledger_index: Option<String>,
    pub validated: Option<bool>,
    pub marker: Option<Value>,
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountChannel {
    pub account: String,
    pub amount: String,
    pub balance: String,
    pub channel_id: String,
    pub destination_account: String,
    pub settle_delay: i64,
    pub public_key: Option<String>,
    pub public_key_hex: Option<String>,
    pub expiration: Option<i64>,
    pub cancel_after: Option<i64>,
    pub source_tab: Option<i64>,
    pub destination_tag: Option<i64>,
}
