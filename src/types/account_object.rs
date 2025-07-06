use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(tag = "LedgerEntryType")]
pub enum AccountObject {
    Check(Check),
    DepositPreauth(DepositPreauth),
    Escrow(Escrow),
    MPToken(MPToken),
    NFTokenOffer(NFTokenOffer),
    NFTokenPage(NFTokenPage),
    Offer(Offer),
    PaymentChannel(PaymentChannel),
    RippleState(RippleState),
    SignerList(SignerList),
    Ticket(Ticket),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Check {
    pub account: String,
    pub destination: String,
    pub flags: i64,
    pub owner_node: String,
    pub send_max: Value,
    pub sequence: i64,
    pub destination_node: String,
    pub detination_tag: String,
    pub invoice_id: String,
    pub source_tag: String,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DepositPreauth {
    pub account: String,
    pub authorize: Option<String>,
    pub authorize_credentials: Option<Value>,
    pub owner_node: String,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Escrow {
    pub account: String,
    pub destination: String,
    pub amount: String,
    pub condition: Option<String>,
    pub cancel_after: Option<i64>,
    pub finish_after: Option<i64>,
    pub flags: i64,
    pub source_tag: Option<i64>,
    pub destination_tag: Option<i64>,
    pub owner_node: String,
    pub destionation_node: Option<String>,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MPToken {
    pub account: String,
    #[serde(rename = "MPTokenIssuanceID")]
    pub mpt_issuance_id: Value,
    pub mpt_amount: Value,
    pub flags: Option<u32>,
    pub owner_node: String,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NFTokenOffer {
    pub amount: Value,
    pub destination: Option<String>,
    pub expiration: Option<u32>,
    #[serde(rename = "NFTokenID")]
    pub nftoken_id: String,
    #[serde(rename = "NFTokenOfferNode")]
    pub nftoken_offer_node: String,
    pub owner: String,
    pub owner_node: String,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NFTokenPage {
    pub next_page_min: Option<String>,
    pub previous_page_min: Option<String>,
    #[serde(rename = "NFTokens")]
    pub nftokens: Value,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: Option<String>,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Offer {
    pub flags: i64,
    pub account: String,
    pub sequence: i64,
    pub taker_pays: Value,
    pub taker_gets: Value,
    pub book_directory: String,
    pub book_node: String,
    pub owner_node: String,
    pub expiration: Option<i64>,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentChannel {
    pub account: String,
    pub destination: String,
    pub amount: String,
    pub balance: String,
    pub public_key: String,
    pub settle_delay: i64,
    pub owner_node: String,
    pub flags: i64,
    pub expiration: Option<i64>,
    pub cancel_after: Option<i64>,
    pub source_tag: Option<i64>,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SignerList {
    pub owner_node: String,
    pub signer_entries: Vec<SignerEntry>,
    #[serde(rename = "SignerListID")]
    pub signer_list_id: u32,
    pub signer_quorum: u32,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SignerEntry {
    pub account: String,
    pub signer_weight: u16,
    pub wallet_locator: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ticket {
    pub account: String,
    pub flags: i64,
    pub owner_node: String,
    pub ticket_sequence: i64,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RippleState {
    pub flags: i64,
    pub balance: Value,
    pub low_limit: Value,
    pub high_limit: Value,
    pub low_node: String,
    pub high_node: String,
    pub low_quality_in: Option<i64>,
    pub low_quality_out: Option<i64>,
    pub high_quality_in: Option<i64>,
    pub high_quality_out: Option<i64>,
    #[serde(rename = "index")]
    pub index: String,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: i64,
}
