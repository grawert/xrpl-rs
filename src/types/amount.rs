use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Amount {
    Xrpl(String),
    IssuedCurrency { value: String, currency: String, issuer: String },
}

impl Default for Amount {
    fn default() -> Self {
        Amount::Xrpl("0".into())
    }
}

impl Amount {
    pub fn xrp<T: Into<String>>(value: T) -> Self {
        let value_str = value.into();

        if let Ok(xrp_amount) = value_str.parse::<f64>() {
            let drops = (xrp_amount * 1_000_000.0).round() as u64;
            Amount::Xrpl(drops.to_string())
        } else {
            Amount::Xrpl(value_str)
        }
    }

    pub fn xrp_from_decimal(xrp_amount: f64) -> Self {
        Amount::from(xrp_amount)
    }

    pub fn drops(drops: u64) -> Self {
        Amount::from(drops)
    }

    pub fn issued_currency<V, C, I>(value: V, currency: C, issuer: I) -> Self
    where
        V: Into<String>,
        C: Into<String>,
        I: Into<String>,
    {
        Amount::IssuedCurrency {
            value: value.into(),
            currency: currency.into(),
            issuer: issuer.into(),
        }
    }

    pub fn issued_currency_decimal<C, I>(
        value: f64,
        currency: C,
        issuer: I,
    ) -> Self
    where
        C: Into<String>,
        I: Into<String>,
    {
        Amount::IssuedCurrency {
            value: value.to_string(),
            currency: currency.into(),
            issuer: issuer.into(),
        }
    }

    pub fn value(&self) -> &str {
        match self {
            Amount::Xrpl(value) => value,
            Amount::IssuedCurrency { value, .. } => value,
        }
    }

    pub fn currency(&self) -> &str {
        match self {
            Amount::Xrpl(_) => "XRP",
            Amount::IssuedCurrency { currency, .. } => currency,
        }
    }

    pub fn to_drops(&self) -> Option<u64> {
        match self {
            Amount::Xrpl(value) => value.parse().ok(),
            Amount::IssuedCurrency { .. } => None,
        }
    }

    pub fn to_decimal(&self) -> Option<f64> {
        match self {
            Amount::Xrpl(value) => value
                .parse::<u64>()
                .ok()
                .map(|drops| drops as f64 / 1_000_000.0),
            Amount::IssuedCurrency { value, .. } => value.parse().ok(),
        }
    }
}

impl From<u64> for Amount {
    fn from(drops: u64) -> Self {
        Amount::Xrpl(drops.to_string())
    }
}

impl From<i64> for Amount {
    fn from(drops: i64) -> Self {
        Amount::Xrpl(drops.to_string())
    }
}

impl From<f64> for Amount {
    fn from(xrp_amount: f64) -> Self {
        let drops = (xrp_amount * 1_000_000.0).round() as u64;
        Amount::Xrpl(drops.to_string())
    }
}

impl From<f32> for Amount {
    fn from(xrp_amount: f32) -> Self {
        Amount::from(xrp_amount as f64)
    }
}

impl From<&str> for Amount {
    fn from(value: &str) -> Self {
        Amount::Xrpl(value.to_string())
    }
}

impl From<String> for Amount {
    fn from(value: String) -> Self {
        Amount::Xrpl(value)
    }
}

impl FromStr for Amount {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| c.is_ascii_digit() || c == '.') {
            return Ok(Amount::Xrpl(s.to_string()));
        }

        Ok(Amount::Xrpl(s.to_string()))
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Amount::Xrpl(value) => {
                if let Ok(drops) = value.parse::<u64>() {
                    write!(f, "{:.6} XRP", drops as f64 / 1_000_000.0)
                } else {
                    write!(f, "{} XRP", value)
                }
            }
            Amount::IssuedCurrency { value, currency, issuer } => {
                write!(
                    f,
                    "{} {} ({})",
                    value,
                    currency,
                    if issuer.len() > 8 {
                        format!("{}...", &issuer[..8])
                    } else {
                        issuer.clone()
                    }
                )
            }
        }
    }
}

impl TryFrom<Amount> for u64 {
    type Error = String;

    fn try_from(amount: Amount) -> Result<Self, Self::Error> {
        match amount {
            Amount::Xrpl(value) => {
                value.parse().map_err(|_| "Invalid XRP amount".to_string())
            }
            Amount::IssuedCurrency { .. } => {
                Err("Cannot convert issued currency to u64".to_string())
            }
        }
    }
}

impl TryFrom<Amount> for f64 {
    type Error = String;

    fn try_from(amount: Amount) -> Result<Self, Self::Error> {
        match amount {
            Amount::Xrpl(value) => {
                let drops: u64 =
                    value.parse().map_err(|_| "Invalid XRP amount")?;
                Ok(drops as f64 / 1_000_000.0)
            }
            Amount::IssuedCurrency { value, .. } => {
                value.parse().map_err(|_| "Invalid currency amount".to_string())
            }
        }
    }
}

#[macro_export]
macro_rules! xrp {
    ($amount:expr) => {
        Amount::xrp($amount)
    };
}

#[macro_export]
macro_rules! xrp_decimal {
    ($amount:expr) => {
        Amount::xrp_from_decimal($amount)
    };
}

#[macro_export]
macro_rules! issued {
    ($value:expr, $currency:expr, $issuer:expr) => {
        Amount::issued_currency($value, $currency, $issuer)
    };
}

#[macro_export]
macro_rules! issued_decimal {
    ($value:expr, $currency:expr, $issuer:expr) => {
        Amount::issued_currency_decimal($value, $currency, $issuer)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversions() {
        let amount1 = Amount::from(1000000u64);
        let amount2 = Amount::xrp("1");
        let amount3 = xrp!("1");
        let amount4: Amount = "1000000".into();

        assert_eq!(amount1, amount2);
        assert_eq!(amount2, amount3);
        assert_eq!(amount1, amount4);

        let amount5 = Amount::from(1.0f64);
        let amount6 = Amount::xrp_from_decimal(1.0);
        let amount7 = xrp_decimal!(1.0);
        let amount8 = Amount::drops(1000000u64);

        assert_eq!(amount1, amount5);
        assert_eq!(amount5, amount6);
        assert_eq!(amount6, amount7);
        assert_eq!(amount8, amount7);

        let half_xrp = Amount::from(0.5f64);
        assert_eq!(half_xrp.to_drops().unwrap(), 500000);
        assert_eq!(half_xrp.to_decimal().unwrap(), 0.5);

        let precise = Amount::from(1.123456f64);
        assert_eq!(precise.to_drops().unwrap(), 1123456);

        let usd = Amount::issued_currency(
            "100.5",
            "USD",
            "rXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
        );
        let usd2 = issued!("100.5", "USD", "rXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        let usd3 = Amount::issued_currency_decimal(
            100.5,
            "USD",
            "rXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
        );
        let usd4 =
            issued_decimal!(100.5, "USD", "rXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

        assert_eq!(usd, usd2);
        assert_eq!(usd.value(), usd3.value());
        assert_eq!(usd3.value(), usd4.value());

        assert_eq!(amount1.value(), "1000000");
        assert_eq!(amount1.currency(), "XRP");
        assert_eq!(usd.currency(), "USD");

        let drops: u64 = amount1.clone().try_into().unwrap();
        assert_eq!(drops, 1000000);

        let xrp_decimal: f64 = amount1.try_into().unwrap();
        assert_eq!(xrp_decimal, 1.0);

        let zero = Amount::from(0.0f64);
        assert_eq!(zero.to_drops().unwrap(), 0);

        let max_precision = Amount::from(1.999999f64);
        assert_eq!(max_precision.to_drops().unwrap(), 1999999);
    }
}
