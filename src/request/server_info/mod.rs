use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::{XrplRequest, XrplResponse};

#[derive(Default, Serialize)]
pub struct ServerInfoRequest;

impl From<ServerInfoRequest> for Value {
    fn from(_: ServerInfoRequest) -> Self {
        json!({
            "command": "server_info",
            "id": Uuid::new_v4().to_string()
        })
    }
}

impl XrplRequest for ServerInfoRequest {
    type Response = XrplResponse<ServerInfoResult>;
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerInfoResult {
    pub info: ServerInfo,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerInfo {
    pub amendment_blocked: Option<bool>,
    pub build_version: String,
    pub closed_ledger: Option<Value>,
    pub complete_ledgers: String,
    pub hostid: String,
    pub initial_sync_duration_us: String,
    pub io_latency_ms: u64,
    pub jq_trans_overflow: String,
    pub last_close: ServerInfoLastClose,
    pub load_factor: u64,
    pub network_id: u64,
    pub peer_disconnects: String,
    pub peer_disconnects_resources: String,
    pub peers: u64,
    pub pubkey_node: String,
    pub server_state: String,
    pub server_state_duration_us: String,
    pub state_accounting: ServerInfoStateAccounting,
    pub time: String,
    pub uptime: u64,
    pub validated_ledger: ServerInfoValidatedLedger,
    pub validation_quorum: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerInfoValidatedLedger {
    pub age: u64,
    pub base_fee_xrp: f64,
    pub hash: String,
    pub reserve_base_xrp: u32,
    pub reserve_inc_xrp: u32,
    pub seq: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerInfoLastClose {
    pub converge_time_s: u64,
    pub proposers: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerInfoStateAccounting {
    pub connected: ServerInfoStateAccount,
    pub disconnected: ServerInfoStateAccount,
    pub full: ServerInfoStateAccount,
    pub syncing: ServerInfoStateAccount,
    pub tracking: ServerInfoStateAccount,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerInfoStateAccount {
    pub duration_us: String,
    pub transitions: String,
}
