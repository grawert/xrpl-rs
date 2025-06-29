use super::{
    TransactionBuilder, TransactionTypeBuilder, BuildError,
    validate_destination,
};
use crate::types::transaction::TransactionType;
use crate::types::{Amount, PathStep};

pub struct Payment {
    pub destination: String,
    pub amount: Amount,
    pub destination_tag: Option<i64>,
    pub invoice_id: Option<String>,
    pub paths: Option<Vec<Vec<PathStep>>>,
    pub send_max: Option<Amount>,
    pub deliver_min: Option<Amount>,
}

pub type PaymentBuilder = TransactionBuilder<Payment>;

/// Create a new payment transaction
/// ```ignore
/// let memo = Memo {
///     memo_data: Some("72656e74".to_string()),
///     memo_type: Some("746578742f706c61696e".to_string()),
///     memo_format: None,
///     };
///
/// let payment = PaymentBuilder::new(
///     account.clone().into(),
///     destination.into(),
///     Amount::Xrpl(amount.to_string()),
/// )
/// .with_sequence(sequence)
/// .with_fee(fee.to_string())
/// .with_destination_tag(destination_tag)
/// .with_memos(vec![memo])
/// .build()?;
/// ```
impl PaymentBuilder {
    pub fn new(account: String, destination: String, amount: Amount) -> Self {
        Self::init(
            account,
            Payment {
                destination,
                amount,
                destination_tag: None,
                invoice_id: None,
                paths: None,
                send_max: None,
                deliver_min: None,
            },
        )
    }

    pub fn with_destination_tag(mut self, tag: i64) -> Self {
        self.transaction_type.destination_tag = Some(tag);
        self
    }

    pub fn with_invoice_id(mut self, id: String) -> Self {
        self.transaction_type.invoice_id = Some(id);
        self
    }

    pub fn with_send_max(mut self, amount: Amount) -> Self {
        self.transaction_type.send_max = Some(amount);
        self
    }

    pub fn with_deliver_min(mut self, amount: Amount) -> Self {
        self.transaction_type.deliver_min = Some(amount);
        self
    }

    pub fn add_path(mut self, path: Vec<PathStep>) -> Self {
        self.transaction_type.paths.get_or_insert_with(Vec::new).push(path);
        self
    }
}

impl TransactionTypeBuilder for Payment {
    type TransactionType = TransactionType;

    fn build_transaction_type(
        self,
    ) -> Result<Self::TransactionType, BuildError> {
        Ok(TransactionType::Payment {
            amount: self.amount,
            destination: self.destination,
            destination_tag: self.destination_tag,
            invoice_id: self.invoice_id,
            paths: self.paths,
            send_max: self.send_max,
            deliver_min: self.deliver_min,
        })
    }

    fn validate_specific_fields(&self) -> Result<(), BuildError> {
        validate_destination(&self.destination)?;

        if let Amount::Xrpl(ref amount_str) = self.amount {
            if amount_str.is_empty() || amount_str == "0" {
                return Err(BuildError::InvalidAmount(
                    "XRP amount cannot be zero or empty".to_string(),
                ));
            }
        }

        if let (Amount::Xrpl(ref amount), Some(Amount::Xrpl(ref send_max))) =
            (&self.amount, &self.send_max)
        {
            if let (Ok(amount_val), Ok(send_max_val)) =
                (amount.parse::<u64>(), send_max.parse::<u64>())
            {
                if send_max_val < amount_val {
                    return Err(BuildError::InvalidAmount(
                        "SendMax cannot be less than Amount for XRP-to-XRP payments".to_string()
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::transaction::Memo;

    #[test]
    fn test_payment_builder_basic() {
        let payment = PaymentBuilder::new(
            "rAccount123".to_string(),
            "rDestination456".to_string(),
            Amount::Xrpl("1000000".to_string()),
        )
        .with_sequence(1)
        .with_fee("10")
        .build()
        .expect("Should build valid payment");

        assert_eq!(payment.account, "rAccount123");
        assert_eq!(payment.sequence, Some(1));
        assert_eq!(payment.fee, Some("10".to_string()));

        if let TransactionType::Payment { destination, amount, .. } =
            payment.transaction_type
        {
            assert_eq!(destination, "rDestination456");
            assert_eq!(amount, Amount::Xrpl("1000000".to_string()));
        } else {
            panic!("Expected Payment transaction type");
        }
    }

    #[test]
    fn test_payment_builder_with_memo() {
        let memo = Memo {
            memo_data: Some("48656c6c6f".to_string()), // "Hello" in hex
            memo_format: None,
            memo_type: None,
        };

        let payment = PaymentBuilder::new(
            "rAccount123".to_string(),
            "rDestination456".to_string(),
            Amount::Xrpl("1000000".to_string()),
        )
        .with_sequence(1)
        .with_fee("10")
        .with_memos(vec![memo])
        .build()
        .expect("Should build valid payment");

        assert!(payment.memos.is_some());
        assert_eq!(payment.memos.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_payment_builder_with_destination_tag() {
        let payment = PaymentBuilder::new(
            "rAccount123".to_string(),
            "rDestination456".to_string(),
            Amount::Xrpl("1000000".to_string()),
        )
        .with_destination_tag(12345)
        .with_sequence(1)
        .with_fee("10")
        .build()
        .expect("Should build valid payment");

        if let TransactionType::Payment { destination_tag, .. } =
            payment.transaction_type
        {
            assert_eq!(destination_tag, Some(12345));
        } else {
            panic!("Expected Payment transaction type");
        }
    }

    #[test]
    fn test_payment_builder_validation_empty_account() {
        let result = PaymentBuilder::new(
            "".to_string(),
            "rDestination456".to_string(),
            Amount::Xrpl("1000000".to_string()),
        )
        .build();

        assert!(matches!(result, Err(BuildError::EmptyAccount)));
    }

    #[test]
    fn test_payment_builder_validation_empty_destination() {
        let result = PaymentBuilder::new(
            "rAccount123".to_string(),
            "".to_string(),
            Amount::Xrpl("1000000".to_string()),
        )
        .build();

        assert!(matches!(result, Err(BuildError::EmptyDestination)));
    }
}
