use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::types::Transaction;
use crate::request::{XrplRequest, XrplResponse, XrplSubscription};

#[derive(Serialize)]
pub struct AccountTransactionsSubscription {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub accounts: Vec<String>,
}

impl AccountTransactionsSubscription {
    pub fn new(accounts: Vec<String>) -> Self {
        Self { accounts, id: None }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

impl From<AccountTransactionsSubscription> for Value {
    fn from(mut val: AccountTransactionsSubscription) -> Self {
        let id = val.id.unwrap_or_else(|| Uuid::new_v4().to_string());
        val.id = Some(id.clone());

        json!({
            "id": id,
            "command": "subscribe",
            "accounts": val.accounts
        })
    }
}

impl XrplRequest for AccountTransactionsSubscription {
    type Response = XrplResponse<AccountSubscriptionResponse>;
}

#[derive(Debug, Deserialize)]
pub struct AccountSubscriptionResponse {}

impl XrplSubscription for AccountTransactionsSubscription {
    type Message = AccountTransactionMessage;
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountTransactionMessage {
    pub close_time_iso: Option<String>,
    pub engine_result: String,
    pub engine_result_code: i32,
    pub engine_result_message: String,
    pub ledger_hash: Option<String>,
    pub ledger_index: Option<i64>,
    pub meta: Option<TransactionMeta>,
    pub status: String,
    pub transaction: Transaction,
    #[serde(rename = "type")]
    pub kind: String,
    pub validated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(skip_serializing)]
    pub id: Option<String>,
}

impl AccountTransactionsUnsubscription {
    pub fn new(accounts: Vec<String>) -> Self {
        Self { accounts, id: None }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

impl From<AccountTransactionsUnsubscription> for Value {
    fn from(mut val: AccountTransactionsUnsubscription) -> Self {
        let id = val.id.unwrap_or_else(|| Uuid::new_v4().to_string());
        val.id = Some(id.clone());

        json!({
            "id": id,
            "command": "unsubscribe",
            "accounts": val.accounts
        })
    }
}

impl XrplRequest for AccountTransactionsUnsubscription {
    type Response = XrplResponse<UnsubscribeResponse>;
}

#[derive(Debug, Deserialize)]
pub struct UnsubscribeResponse {}

#[derive(Serialize)]
pub struct LedgerClosedUnsubscription {
    #[serde(skip_serializing)]
    pub id: Option<String>,
}

impl LedgerClosedUnsubscription {
    pub fn new() -> Self {
        Self { id: None }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

impl From<LedgerClosedUnsubscription> for Value {
    fn from(mut val: LedgerClosedUnsubscription) -> Self {
        let id = val.id.unwrap_or_else(|| Uuid::new_v4().to_string());
        val.id = Some(id.clone());

        json!({
            "id": id,
            "command": "unsubscribe",
            "streams": ["ledger"]
        })
    }
}

impl XrplRequest for LedgerClosedUnsubscription {
    type Response = XrplResponse<UnsubscribeResponse>;
}
