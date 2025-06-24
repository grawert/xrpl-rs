use serde_json::Value;
use serde_derive::{Deserialize, Serialize};
use super::{Amount, PathStep};

pub fn to_json_skip_nulls<T: serde::Serialize>(value: &T) -> String {
    let mut json =
        serde_json::to_value(value).expect("Failed to serialize to JSON value");
    remove_nulls(&mut json);
    serde_json::to_string(&json)
        .expect("Failed to serialize JSON value to string")
}

pub fn to_value_skip_nulls<T: serde::Serialize>(value: &T) -> Value {
    let mut json =
        serde_json::to_value(value).expect("Failed to serialize to JSON value");
    remove_nulls(&mut json);
    json
}

fn remove_nulls(value: &mut Value) {
    if let Value::Object(map) = value {
        map.retain(|_, v| !v.is_null());
        map.values_mut().for_each(remove_nulls);
    } else if let Value::Array(arr) = value {
        arr.iter_mut().for_each(remove_nulls);
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transaction {
    pub account: String,
    #[serde(rename = "AccountTxnID")]
    pub account_txn_id: Option<String>,
    pub fee: Option<String>,
    pub flags: Option<u32>,
    #[serde(rename = "LastLedgerSequence")]
    pub last_ledger_sequence: Option<u32>,
    pub memos: Option<Vec<Memo>>,
    pub sequence: Option<u32>,
    pub signers: Option<Vec<Signer>>,
    #[serde(rename = "SourceTag")]
    pub source_tag: Option<u32>,
    #[serde(rename = "TicketSequence")]
    pub ticket_sequence: Option<u32>,
    #[serde(flatten)]
    pub transaction_type: TransactionType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "TransactionType")]
pub enum TransactionType {
    #[serde(rename_all = "PascalCase")]
    NFTokenAcceptOffer {
        #[serde(rename = "NFTokenSellOffer")]
        nftoken_sell_offer: Option<String>,
        #[serde(rename = "NFTokenBuyOffer")]
        nftoken_buy_offer: Option<String>,
        #[serde(rename = "NFTokenBrokerFee")]
        nftoken_broker_fee: Option<Amount>,
    },
    #[serde(rename_all = "PascalCase")]
    NFTokenBurn {
        #[serde(rename = "NFTokenID")]
        nftoken_id: String,
        owner: String,
    },
    #[serde(rename_all = "PascalCase")]
    NFTokenCancelOffer {
        #[serde(rename = "NFTokenOffers")]
        nftoken_offers: Vec<String>,
    },
    #[serde(rename_all = "PascalCase")]
    NFTokenCreateOffer {
        #[serde(rename = "NFTokenID")]
        nftoken_id: String,
        amount: Amount,
        owner: Option<String>,
        expiration: Option<i64>,
        destination: Option<String>,
    },
    #[serde(rename_all = "PascalCase")]
    NFTokenMint {
        #[serde(rename = "NFTokenTaxon")]
        nftoken_taxon: String,
        issuer: String,
        transfer_fee: Option<i64>,
        uri: Option<String>,
    },
    #[serde(rename_all = "PascalCase")]
    AccountSet {
        clear_flag: Option<i64>,
        domain: Option<String>,
        email_hash: Option<String>,
        message_key: Option<String>,
        set_flag: Option<i64>,
        transfer_rate: Option<i64>,
        tick_size: Option<i64>,
        #[serde(rename = "NFTokenMinter")]
        nftoken_minter: Option<i64>,
    },
    #[serde(rename_all = "PascalCase")]
    TrustSet {
        limit_amount: Amount,
        quality_in: Option<i64>,
        quality_out: Option<i64>,
    },
    #[serde(rename_all = "PascalCase")]
    OfferCreate {
        expiration: Option<i64>,
        offer_sequence: Option<i64>,
        taker_gets: Amount,
        taker_pays: Amount,
    },
    #[serde(rename_all = "PascalCase")]
    Payment {
        amount: Amount,
        destination: String,
        destination_tag: Option<i64>,
        invoice_id: Option<String>,
        paths: Option<Vec<Vec<PathStep>>>,
        send_max: Option<Amount>,
        deliver_min: Option<Amount>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Memo {
    pub memo_data: Option<String>,
    pub memo_format: Option<String>,
    pub memo_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Signer {
    pub account: String,
    pub txn_signature: String,
    pub signing_pub_key: String,
}

impl From<Transaction> for String {
    fn from(val: Transaction) -> Self {
        to_json_skip_nulls(&val)
    }
}

impl From<&Transaction> for String {
    fn from(val: &Transaction) -> Self {
        to_json_skip_nulls(val)
    }
}

impl From<Transaction> for Value {
    fn from(val: Transaction) -> Self {
        to_value_skip_nulls(&val)
    }
}

impl From<&Transaction> for Value {
    fn from(val: &Transaction) -> Self {
        to_value_skip_nulls(val)
    }
}

/// Trait for implementing transaction signing.
///
/// # Example Implementation
/// ```rust
/// use hex;
/// use anyhow::Result;
/// use serde_json;
/// use ripple_keypairs::{PublicKey, PrivateKey};
/// use rippled_binary_codec::serialize::serialize_tx;
///
/// const STX_PREFIX: &str = "53545800"; // STX (535458) + null separator (00)
///
/// struct Wallet {
///     pub public_key: PublicKey,
///     pub private_key: PrivateKey,
/// }
///
/// impl SigningContext for Wallet {
///     type Error = anyhow::Error;
///
///     fn sign_transaction(&self, tx: &Transaction) -> Result<String, Self::Error> {
///         // 1. Convert to JSON and add public key
///         let mut tx_json: serde_json::Value = tx.into();
///         tx_json["SigningPubKey"] = self.public_key.to_string().into();
///
///         // 2. Serialize for signing
///         let json_str = serde_json::to_string(&tx_json)?;
///         let tx_hex = serialize_tx(json_str, true).ok_or_else(|| {
///             anyhow::anyhow!("Failed to serialize transaction for signing")
///         })?;
///
///         // 3. Create signing blob with XRPL STX prefix
///         let signing_hex = format!("{}{}", STX_PREFIX, tx_hex);
///         let signing_bytes = hex::decode(&signing_hex)?;
///
///         // 4. Sign the full signing blob
///         let signature = self.private_key.sign(&signing_bytes);
///         tx_json["TxnSignature"] = signature.to_string().into();
///
///         // 5. Serialize final signed transaction
///         let final_json = serde_json::to_string(&tx_json)?;
///         let final_bytes = serialize_tx(final_json, false).ok_or_else(|| {
///             anyhow::anyhow!("Failed to serialize final transaction")
///         })?;
///         Ok(final_bytes)
///     }
/// }
///
/// // Usage:
/// let wallet = Wallet::from_seed("sSecret...")?;
/// let payment = Transaction {
///     account: "rAccount...".to_string(),
///     sequence: Some(1),
///     // ... other fields
///     transaction_type: TransactionType::Payment { /* ... */ },
/// };
/// let signed_blob = payment.sign_with(&wallet)?;
/// let submit_request = SubmitRequest {
///     tx_blob: signed_blob,
///     fail_hard: Some(false)
/// };
/// ```
pub trait SigningContext {
    type Error;

    fn sign_transaction(&self, tx: &Transaction)
        -> Result<String, Self::Error>;
}

/// Adds signing capability to Transaction objects
pub trait Signable {
    fn sign_with<C: SigningContext>(
        &self,
        context: &C,
    ) -> Result<String, C::Error>;
}

impl Signable for Transaction {
    fn sign_with<C: SigningContext>(
        &self,
        context: &C,
    ) -> Result<String, C::Error> {
        context.sign_transaction(self)
    }
}
