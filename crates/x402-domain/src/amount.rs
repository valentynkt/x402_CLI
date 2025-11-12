use crate::error::{DomainError, DomainResult};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::ops::{Add, Sub};
use std::str::FromStr;

/// Type-safe amount using Decimal (NOT f64!) to prevent floating-point errors
///
/// # Example
/// ```
/// use x402_domain::Amount;
/// use rust_decimal::Decimal;
///
/// let amount = Amount::from_usdc_lamports(1_000_000).unwrap(); // 1 USDC
/// assert_eq!(amount.to_usdc_lamports(), 1_000_000);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Amount(Decimal);

impl Amount {
    /// Creates a new Amount from any type convertible to Decimal
    pub fn new(value: impl Into<Decimal>) -> DomainResult<Self> {
        let decimal = value.into();
        if decimal.is_sign_negative() {
            return Err(DomainError::InvalidAmount("cannot be negative".into()));
        }
        Ok(Self(decimal))
    }

    /// Creates an Amount from USDC lamports (6 decimals)
    /// 1 USDC = 1,000,000 lamports
    pub fn from_usdc_lamports(lamports: u64) -> DomainResult<Self> {
        let decimal = Decimal::from(lamports) / Decimal::from(1_000_000);
        Ok(Self(decimal))
    }

    /// Converts Amount to USDC lamports (6 decimals)
    pub fn to_usdc_lamports(&self) -> u64 {
        let lamports_decimal = self.0 * Decimal::from(1_000_000);
        lamports_decimal.to_u64().unwrap_or(0)
    }

    /// Creates an Amount from a decimal string (e.g., "1.50")
    pub fn from_decimal_str(s: &str) -> DomainResult<Self> {
        let decimal = Decimal::from_str(s)
            .map_err(|_| DomainError::InvalidAmount("invalid decimal format".into()))?;
        Self::new(decimal)
    }

    /// Returns the inner Decimal value
    pub fn as_decimal(&self) -> Decimal {
        self.0
    }

    /// Returns zero amount
    pub fn zero() -> Self {
        Self(Decimal::ZERO)
    }

    /// Checks if amount is zero
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Checked addition to prevent overflow
    pub fn checked_add(&self, other: &Amount) -> DomainResult<Amount> {
        self.0.checked_add(other.0)
            .map(Amount)
            .ok_or_else(|| DomainError::ArithmeticOverflow("addition overflow".into()))
    }

    /// Checked subtraction to prevent underflow
    pub fn checked_sub(&self, other: &Amount) -> DomainResult<Amount> {
        let result = self.0.checked_sub(other.0)
            .ok_or_else(|| DomainError::ArithmeticOverflow("subtraction overflow".into()))?;

        if result.is_sign_negative() {
            return Err(DomainError::InvalidAmount("result would be negative".into()));
        }

        Ok(Amount(result))
    }

    /// Checked multiplication
    pub fn checked_mul(&self, multiplier: impl Into<Decimal>) -> DomainResult<Amount> {
        let mult = multiplier.into();
        self.0.checked_mul(mult)
            .map(Amount)
            .ok_or_else(|| DomainError::ArithmeticOverflow("multiplication overflow".into()))
    }

    /// Checked division
    pub fn checked_div(&self, divisor: impl Into<Decimal>) -> DomainResult<Amount> {
        let div = divisor.into();
        if div.is_zero() {
            return Err(DomainError::ArithmeticOverflow("division by zero".into()));
        }

        self.0.checked_div(div)
            .map(Amount)
            .ok_or_else(|| DomainError::ArithmeticOverflow("division overflow".into()))
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Amount {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_decimal_str(s)
    }
}

// Implement arithmetic operators using checked operations
impl Add for Amount {
    type Output = DomainResult<Amount>;

    fn add(self, other: Amount) -> Self::Output {
        self.checked_add(&other)
    }
}

impl Sub for Amount {
    type Output = DomainResult<Amount>;

    fn sub(self, other: Amount) -> Self::Output {
        self.checked_sub(&other)
    }
}

/// Currency types supported by the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    /// USD Coin (6 decimals)
    USDC,
    /// Solana native token (9 decimals)
    SOL,
}

impl Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Currency::USDC => write!(f, "USDC"),
            Currency::SOL => write!(f, "SOL"),
        }
    }
}

impl FromStr for Currency {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "USDC" => Ok(Currency::USDC),
            "SOL" => Ok(Currency::SOL),
            _ => Err(DomainError::InvalidAmount(format!("unknown currency: {}", s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_creation() {
        let amount = Amount::new(Decimal::from(100)).unwrap();
        assert_eq!(amount.as_decimal(), Decimal::from(100));

        // Cannot be negative
        assert!(Amount::new(Decimal::from(-1)).is_err());
    }

    #[test]
    fn test_usdc_lamports_conversion() {
        // 1 USDC = 1,000,000 lamports
        let amount = Amount::from_usdc_lamports(1_000_000).unwrap();
        assert_eq!(amount.to_usdc_lamports(), 1_000_000);

        // 0.5 USDC = 500,000 lamports
        let half = Amount::from_usdc_lamports(500_000).unwrap();
        assert_eq!(half.to_usdc_lamports(), 500_000);
    }

    #[test]
    fn test_decimal_string_parsing() {
        let amount = Amount::from_decimal_str("1.50").unwrap();
        assert_eq!(amount.as_decimal(), Decimal::from_str("1.50").unwrap());

        assert!(Amount::from_decimal_str("invalid").is_err());
    }

    #[test]
    fn test_zero_amount() {
        let zero = Amount::zero();
        assert!(zero.is_zero());
        assert_eq!(zero.to_usdc_lamports(), 0);
    }

    #[test]
    fn test_checked_addition() {
        let a = Amount::new(Decimal::from(100)).unwrap();
        let b = Amount::new(Decimal::from(50)).unwrap();

        let sum = a.checked_add(&b).unwrap();
        assert_eq!(sum.as_decimal(), Decimal::from(150));
    }

    #[test]
    fn test_checked_subtraction() {
        let a = Amount::new(Decimal::from(100)).unwrap();
        let b = Amount::new(Decimal::from(50)).unwrap();

        let diff = a.checked_sub(&b).unwrap();
        assert_eq!(diff.as_decimal(), Decimal::from(50));

        // Cannot subtract to negative
        assert!(b.checked_sub(&a).is_err());
    }

    #[test]
    fn test_checked_multiplication() {
        let amount = Amount::new(Decimal::from(10)).unwrap();
        let result = amount.checked_mul(Decimal::from(3)).unwrap();
        assert_eq!(result.as_decimal(), Decimal::from(30));
    }

    #[test]
    fn test_checked_division() {
        let amount = Amount::new(Decimal::from(100)).unwrap();
        let result = amount.checked_div(Decimal::from(4)).unwrap();
        assert_eq!(result.as_decimal(), Decimal::from(25));

        // Cannot divide by zero
        assert!(amount.checked_div(Decimal::ZERO).is_err());
    }

    #[test]
    fn test_amount_ordering() {
        let small = Amount::new(Decimal::from(10)).unwrap();
        let large = Amount::new(Decimal::from(100)).unwrap();

        assert!(small < large);
        assert!(large > small);
    }

    #[test]
    fn test_currency() {
        assert_eq!(Currency::USDC.to_string(), "USDC");
        assert_eq!(Currency::SOL.to_string(), "SOL");

        let usdc: Currency = "usdc".parse().unwrap();
        assert_eq!(usdc, Currency::USDC);

        assert!("BTC".parse::<Currency>().is_err());
    }

    #[test]
    fn test_serialization() {
        let amount = Amount::new(Decimal::from_str("123.456").unwrap()).unwrap();
        let json = serde_json::to_string(&amount).unwrap();

        let deserialized: Amount = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, amount);
    }

    #[test]
    fn test_no_floating_point_errors() {
        // This would fail with f64: 0.1 + 0.2 != 0.3
        let a = Amount::from_decimal_str("0.1").unwrap();
        let b = Amount::from_decimal_str("0.2").unwrap();
        let expected = Amount::from_decimal_str("0.3").unwrap();

        let sum = a.checked_add(&b).unwrap();
        assert_eq!(sum, expected);
    }
}
