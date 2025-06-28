use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use super::{XrplRequest, XrplResponse};

#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct AccountNftsRequest {
    pub account: String,
    pub ledger_hash: Option<String>,
    pub ledger_index: i64,
    pub limit: Option<u32>,
    pub marker: Option<Value>,
}

impl From<AccountNftsRequest> for Value {
    fn from(val: AccountNftsRequest) -> Self {
        let value = serde_json::to_value(val);
        if let Err(e) = &value {
            dbg!(e);
        };
        let mut value = value.unwrap().as_object().unwrap().to_owned();
        value.insert("id".into(), Uuid::new_v4().to_string().into());
        value.insert("command".into(), "account_nfts".into());
        value.into()
    }
}

impl XrplRequest for AccountNftsRequest {
    type Response = XrplResponse<AccountNftsResult>;
}

#[derive(Debug, Deserialize)]
pub struct AccountNftsResult {
    pub account: String,
    pub account_nfts: Vec<AccountNFToken>,
    pub ledger_hash: Option<String>,
    pub ledger_index: i64,
    pub ledger_current_index: i64,
    pub validated: bool,
    pub marker: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountNFToken {
    pub flags: u32,
    pub issuer: String,
    #[serde(rename = "NFTokenID")]
    pub nftoken_id: String,
    #[serde(rename = "NFTokenTaxon")]
    pub nftoken_taxon: i64,
    pub uri: Option<String>,
    #[serde(rename = "nft_serial")]
    pub nft_serial: i64,
}
