use crate::types::transaction::{
    Memo, MemoWrapper, Signer, SignerWrapper, Transaction, TransactionType,
};

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("Account cannot be empty")]
    EmptyAccount,
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    #[error("Destination cannot be empty")]
    EmptyDestination,
    #[error("Sequence number is required")]
    MissingSequence,
    #[error("Fee is required")]
    MissingFee,
    #[error("Invalid field: {0}")]
    InvalidField(String),
}

pub struct TransactionBuilder<T> {
    account: String,
    account_txn_id: Option<String>,
    fee: Option<String>,
    flags: Option<u32>,
    last_ledger_sequence: Option<u32>,
    memos: Option<Vec<MemoWrapper>>,
    sequence: Option<u32>,
    signers: Option<Vec<SignerWrapper>>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    pub(crate) transaction_type: T,
}

pub trait TransactionTypeBuilder {
    type TransactionType;

    fn build_transaction_type(
        self,
    ) -> Result<Self::TransactionType, BuildError>;
    fn validate_specific_fields(&self) -> Result<(), BuildError>;
}

impl<T: TransactionTypeBuilder<TransactionType = TransactionType>>
    TransactionBuilder<T>
{
    pub fn init(account: String, transaction_type: T) -> Self {
        Self {
            account,
            account_txn_id: None,
            fee: None,
            flags: None,
            last_ledger_sequence: None,
            memos: None,
            sequence: None,
            signers: None,
            source_tag: None,
            ticket_sequence: None,
            transaction_type,
        }
    }

    pub fn with_sequence(mut self, sequence: u32) -> Self {
        self.sequence = Some(sequence);
        self
    }

    pub fn with_fee(mut self, fee: impl Into<String>) -> Self {
        self.fee = Some(fee.into());
        self
    }

    pub fn with_flags(mut self, flags: u32) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn with_last_ledger_sequence(mut self, sequence: u32) -> Self {
        self.last_ledger_sequence = Some(sequence);
        self
    }

    pub fn with_memos(mut self, memos: Vec<Memo>) -> Self {
        let memos: Vec<MemoWrapper> =
            memos.into_iter().map(|memo| MemoWrapper { memo }).collect();
        self.memos = Some(memos);
        self
    }

    pub fn with_signers(mut self, signers: Vec<Signer>) -> Self {
        let signers: Vec<SignerWrapper> = signers
            .into_iter()
            .map(|signer| SignerWrapper { signer })
            .collect();
        self.signers = Some(signers);
        self
    }

    pub fn with_source_tag(mut self, tag: u32) -> Self {
        self.source_tag = Some(tag);
        self
    }

    pub fn with_ticket_sequence(mut self, sequence: u32) -> Self {
        self.ticket_sequence = Some(sequence);
        self
    }

    pub fn with_account_txn_id(mut self, id: String) -> Self {
        self.account_txn_id = Some(id);
        self
    }

    pub fn build(self) -> Result<Transaction, BuildError> {
        validate_account(&self.account)?;
        self.transaction_type.validate_specific_fields()?;
        let transaction_type =
            self.transaction_type.build_transaction_type()?;

        Ok(Transaction {
            account: self.account,
            account_txn_id: self.account_txn_id,
            fee: self.fee,
            flags: self.flags,
            last_ledger_sequence: self.last_ledger_sequence,
            memos: self.memos,
            sequence: self.sequence,
            signers: self.signers,
            source_tag: self.source_tag,
            ticket_sequence: self.ticket_sequence,
            signing_pub_key: None,
            txn_signature: None,
            hash: None,
            transaction_type,
        })
    }
}

pub fn validate_account(account: &str) -> Result<(), BuildError> {
    if account.is_empty() {
        return Err(BuildError::EmptyAccount);
    }
    Ok(())
}

pub fn validate_destination(destination: &str) -> Result<(), BuildError> {
    if destination.is_empty() {
        return Err(BuildError::EmptyDestination);
    }
    Ok(())
}
