use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::request::{XrplRequest, XrplResponse, XrplSubscription};

#[derive(Serialize)]
pub struct AccountTransactionsSubscription {
    pub accounts: Vec<String>,
}

impl From<AccountTransactionsSubscription> for Value {
    fn from(val: AccountTransactionsSubscription) -> Self {
        json!({
            "id": Uuid::new_v4().to_string(),
            "command": "subscribe",
            "accounts": val.accounts
        })
    }
}

impl XrplRequest for AccountTransactionsSubscription {
    type Response = XrplResponse<AccountSubscriptionResponse>;
}

#[derive(Debug, Deserialize)]
pub struct AccountSubscriptionResponse {
    // Account subscription acknowledgments return empty result objects
    // The status is handled by the XrplResponse wrapper
}

impl XrplSubscription for AccountTransactionsSubscription {
    type Message = AccountTransactionMessage;
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountTransactionMessage {
    pub close_time_iso: String,
    pub engine_result: String,
    pub engine_result_code: i32,
    pub engine_result_message: String,
    pub ledger_hash: String,
    pub ledger_index: i64,
    pub meta: TransactionMeta,
    pub status: String,
    pub transaction: Transaction,
    #[serde(rename = "type")]
    pub kind: String,
    pub validated: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transaction {
    pub account: String,
    pub amount: Value,
    pub deliver_max: Option<String>,
    pub destination: Option<String>,
    pub destination_tag: Option<u32>,
    pub fee: String,
    pub flags: u32,
    pub last_ledger_sequence: Option<i64>,
    pub sequence: i64,
    pub signing_pub_key: String,
    pub transaction_type: String,
    pub txn_signature: String,
    pub date: Option<i64>,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(flatten)]
    pub other_fields: std::collections::HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionMeta {
    pub affected_nodes: Vec<Value>,
    pub transaction_index: i32,
    pub transaction_result: String,
    #[serde(rename = "delivered_amount")]
    pub delivered_amount: Option<Value>,
}

#[derive(Serialize)]
pub struct AccountTransactionsUnsubscription {
    pub accounts: Vec<String>,
}

impl From<AccountTransactionsUnsubscription> for Value {
    fn from(val: AccountTransactionsUnsubscription) -> Self {
        json!({
            "id": Uuid::new_v4().to_string(),
            "command": "unsubscribe",
            "accounts": val.accounts
        })
    }
}

impl XrplRequest for AccountTransactionsUnsubscription {
    type Response = XrplResponse<UnsubscribeResponse>;
}

#[derive(Debug, Deserialize)]
pub struct UnsubscribeResponse {
    // Unsubscribe responses are typically empty
}

#[derive(Serialize)]
pub struct LedgerClosedUnsubscription;

impl From<LedgerClosedUnsubscription> for Value {
    fn from(_: LedgerClosedUnsubscription) -> Self {
        json!({
            "id": Uuid::new_v4().to_string(),
            "command": "unsubscribe",
            "streams": ["ledger"]
        })
    }
}

impl XrplRequest for LedgerClosedUnsubscription {
    type Response = XrplResponse<UnsubscribeResponse>;
}
