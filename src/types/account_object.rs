use serde::Deserialize;
use serde_json::Value;

use super::Amount;

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
    PayChannel(PayChannel),
    RippleState(RippleState),
    SignerList(SignerList),
    Ticket(Ticket),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Common {
    pub flags: u32,
    pub index: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Check {
    pub account: String,
    pub destination: String,
    pub destination_node: String,
    pub detination_tag: String,
    pub expiration: u32,
    #[serde(rename = "InvoiceID")]
    pub invoice_id: String,
    pub owner_node: String,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,
    pub send_max: Value,
    pub sequence: u32,
    pub source_tag: String,

    #[serde(flatten)]
    pub common: Common,
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

    #[serde(flatten)]
    pub common: Common,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Escrow {
    pub account: String,
    pub amount: String,
    pub cancel_after: Option<u32>,
    pub condition: Option<String>,
    pub destination: String,
    pub destionation_node: Option<String>,
    pub destination_tag: Option<u32>,
    pub finish_after: Option<u32>,
    pub owner_node: String,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,
    pub source_tag: Option<u32>,
    pub transfer_rate: Option<u32>,

    #[serde(flatten)]
    pub common: Common,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MPToken {
    pub account: String,
    #[serde(rename = "MPTokenIssuanceID")]
    pub mpt_issuance_id: Value,
    pub mpt_amount: Value,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,
    pub owner_node: String,

    #[serde(flatten)]
    pub common: Common,
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

    #[serde(flatten)]
    pub common: Common,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NFTokenPage {
    pub next_page_min: Option<String>,
    #[serde(rename = "NFTokens")]
    pub nftokens: Value,
    pub previous_page_min: Option<String>,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: Option<String>,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: Option<u32>,

    #[serde(flatten)]
    pub common: Common,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Offer {
    pub account: String,
    pub additional_books: Option<Value>,
    pub book_directory: String,
    pub book_node: String,
    #[serde(rename = "DomainID")]
    pub domain_id: String,
    pub expiration: Option<u32>,
    pub owner_node: String,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,
    pub sequence: u32,
    pub taker_pays: Amount,
    pub taker_gets: Amount,

    #[serde(flatten)]
    pub common: Common,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PayChannel {
    pub account: String,
    pub amount: Amount,
    pub balance: Amount,
    pub cancel_after: Option<u32>,
    pub destination: String,
    pub destination_tag: Option<u32>,
    pub destination_node: Option<u64>,
    pub expiration: Option<u32>,
    pub owner_node: String,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,
    pub public_key: String,
    pub settle_delay: u32,
    pub source_tag: Option<u32>,
    pub transfer_rate: Option<u32>,

    #[serde(flatten)]
    pub common: Common,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RippleState {
    pub balance: Amount,
    pub high_limit: Amount,
    pub high_node: String,
    pub high_quality_in: Option<u32>,
    pub high_quality_out: Option<u32>,
    pub lock_count: Option<Amount>,
    pub locked_balance: Option<Amount>,
    pub low_limit: Amount,
    pub low_node: String,
    pub low_quality_in: Option<u32>,
    pub low_quality_out: Option<u32>,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,

    #[serde(flatten)]
    pub common: Common,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SignerList {
    pub owner_node: String,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,
    pub signer_entries: Vec<SignerEntry>,
    #[serde(rename = "SignerListID")]
    pub signer_list_id: u32,
    pub signer_quorum: u32,

    #[serde(flatten)]
    pub common: Common,
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
    pub owner_node: String,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    #[serde(rename = "PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u32,
    pub ticket_sequence: u32,

    #[serde(flatten)]
    pub common: Common,
}
