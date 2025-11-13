//! Conversion traits for PricingConfig compatibility
//!
//! This module provides conversions between the canonical PricingConfig type
//! and the three existing implementations in CLI, Policy Rules, and Codegen.
//!
//! **Phase 1 (Wave 1)**: Only additive conversions. Old types remain intact.
//! **Phase 2 (Wave 2)**: Remove old types after all usage sites are updated.

#[cfg(test)]
use crate::ResourcePath;
use crate::{Amount, PricingConfig};
use std::collections::HashMap;

/// CLI PricingConfig conversion (from x402-cli/src/config.rs)
///
/// CLI version structure:
/// ```ignore
/// pub struct PricingConfig {
///     pub default: f64,
///     pub per_resource: HashMap<String, f64>,
/// }
/// ```
impl PricingConfig {
    /// Converts from CLI PricingConfig
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use x402_domain::PricingConfig;
    /// use x402_cli::config::PricingConfig as CliPricingConfig;
    ///
    /// let cli_config = CliPricingConfig {
    ///     default: 0.01,
    ///     per_resource: HashMap::new(),
    /// };
    ///
    /// let canonical = PricingConfig::from_cli(cli_config);
    /// ```
    pub fn from_cli(default_f64: f64, per_resource_f64: HashMap<String, f64>) -> Self {
        // Convert default amount from f64 to Amount
        let default =
            Amount::from_decimal_str(&default_f64.to_string()).unwrap_or_else(|_| Amount::zero());

        // Convert per-resource pricing
        let mut per_resource = HashMap::new();
        for (path, amount_f64) in per_resource_f64 {
            if let Ok(amount) = Amount::from_decimal_str(&amount_f64.to_string()) {
                per_resource.insert(path, amount);
            }
        }

        Self {
            default,
            per_resource,
            currency: "USDC".to_string(), // CLI doesn't have currency field, default to USDC
            memo_prefix: None,            // CLI doesn't have memo_prefix
        }
    }

    /// Converts to CLI PricingConfig format
    ///
    /// Returns (default: f64, per_resource: HashMap<String, f64>)
    pub fn to_cli(&self) -> (f64, HashMap<String, f64>) {
        let default_f64 = self
            .default
            .as_decimal()
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.01);

        let per_resource_f64: HashMap<String, f64> = self
            .per_resource
            .iter()
            .filter_map(|(path, amount)| {
                let f64_val = amount.as_decimal().to_string().parse::<f64>().ok()?;
                Some((path.clone(), f64_val))
            })
            .collect();

        (default_f64, per_resource_f64)
    }
}

/// Policy Rules PricingConfig conversion (from x402-core/src/policy/rules.rs)
///
/// Policy Rules version structure:
/// ```ignore
/// pub struct PricingConfig {
///     pub amount: f64,
///     pub currency: String,
///     pub memo_prefix: Option<String>,
/// }
/// ```
impl PricingConfig {
    /// Converts from Policy Rules PricingConfig
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use x402_domain::PricingConfig;
    ///
    /// let canonical = PricingConfig::from_policy_rules(0.01, "USDC".to_string(), None);
    /// ```
    pub fn from_policy_rules(
        amount_f64: f64,
        currency: String,
        memo_prefix: Option<String>,
    ) -> Self {
        let default =
            Amount::from_decimal_str(&amount_f64.to_string()).unwrap_or_else(|_| Amount::zero());

        Self {
            default,
            per_resource: HashMap::new(), // Policy rules version doesn't have per-resource
            currency,
            memo_prefix,
        }
    }

    /// Converts to Policy Rules PricingConfig format
    ///
    /// Returns (amount: f64, currency: String, memo_prefix: Option<String>)
    pub fn to_policy_rules(&self) -> (f64, String, Option<String>) {
        let amount_f64 = self
            .default
            .as_decimal()
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.01);

        (amount_f64, self.currency.clone(), self.memo_prefix.clone())
    }
}

/// Codegen PricingConfig conversion (from x402-core/src/policy/codegen_types.rs)
///
/// Codegen version structure (identical to Policy Rules):
/// ```ignore
/// pub struct PricingConfig {
///     pub amount: f64,
///     pub currency: String,
///     pub memo_prefix: Option<String>,
/// }
/// ```
impl PricingConfig {
    /// Converts from Codegen PricingConfig
    ///
    /// Note: This is identical to Policy Rules format
    pub fn from_codegen(amount_f64: f64, currency: String, memo_prefix: Option<String>) -> Self {
        Self::from_policy_rules(amount_f64, currency, memo_prefix)
    }

    /// Converts to Codegen PricingConfig format
    ///
    /// Returns (amount: f64, currency: String, memo_prefix: Option<String>)
    pub fn to_codegen(&self) -> (f64, String, Option<String>) {
        self.to_policy_rules()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cli_conversion() {
        let mut per_resource = HashMap::new();
        per_resource.insert("/api/premium".to_string(), 0.05);
        per_resource.insert("/api/*".to_string(), 0.02);

        let canonical = PricingConfig::from_cli(0.01, per_resource);

        assert_eq!(canonical.default, Amount::from_decimal_str("0.01").unwrap());
        assert_eq!(canonical.per_resource.len(), 2);
        assert_eq!(canonical.currency, "USDC");
        assert_eq!(canonical.memo_prefix, None);
    }

    #[test]
    fn test_to_cli_conversion() {
        let canonical = PricingConfig::new(Amount::from_decimal_str("0.01").unwrap())
            .with_resource_price(
                ResourcePath::new("/api/premium").unwrap(),
                Amount::from_decimal_str("0.05").unwrap(),
            );

        let (default_f64, per_resource) = canonical.to_cli();

        assert!((default_f64 - 0.01).abs() < 0.0001);
        assert_eq!(per_resource.len(), 1);
        assert!(per_resource.contains_key("/api/premium"));
    }

    #[test]
    fn test_from_policy_rules_conversion() {
        let canonical =
            PricingConfig::from_policy_rules(0.02, "SOL".to_string(), Some("x402:".to_string()));

        assert_eq!(canonical.default, Amount::from_decimal_str("0.02").unwrap());
        assert_eq!(canonical.currency, "SOL");
        assert_eq!(canonical.memo_prefix, Some("x402:".to_string()));
        assert_eq!(canonical.per_resource.len(), 0); // Policy rules doesn't have per-resource
    }

    #[test]
    fn test_to_policy_rules_conversion() {
        let canonical = PricingConfig::new(Amount::from_decimal_str("0.03").unwrap())
            .with_currency("USDC")
            .with_memo_prefix("test:");

        let (amount, currency, memo_prefix) = canonical.to_policy_rules();

        assert!((amount - 0.03).abs() < 0.0001);
        assert_eq!(currency, "USDC");
        assert_eq!(memo_prefix, Some("test:".to_string()));
    }

    #[test]
    fn test_from_codegen_conversion() {
        let canonical = PricingConfig::from_codegen(0.01, "USDC".to_string(), None);

        assert_eq!(canonical.default, Amount::from_decimal_str("0.01").unwrap());
        assert_eq!(canonical.currency, "USDC");
    }

    #[test]
    fn test_to_codegen_conversion() {
        let canonical = PricingConfig::default();
        let (amount, currency, memo_prefix) = canonical.to_codegen();

        assert!((amount - 0.01).abs() < 0.0001);
        assert_eq!(currency, "USDC");
        assert_eq!(memo_prefix, None);
    }

    #[test]
    fn test_roundtrip_cli() {
        let mut original_per_resource = HashMap::new();
        original_per_resource.insert("/api/test".to_string(), 0.02);

        let canonical = PricingConfig::from_cli(0.01, original_per_resource.clone());
        let (default_back, per_resource_back) = canonical.to_cli();

        assert!((default_back - 0.01).abs() < 0.0001);
        assert_eq!(per_resource_back.len(), original_per_resource.len());
    }

    #[test]
    fn test_roundtrip_policy_rules() {
        let canonical =
            PricingConfig::from_policy_rules(0.05, "SOL".to_string(), Some("prefix:".to_string()));

        let (amount, currency, memo_prefix) = canonical.to_policy_rules();

        assert!((amount - 0.05).abs() < 0.0001);
        assert_eq!(currency, "SOL");
        assert_eq!(memo_prefix, Some("prefix:".to_string()));
    }

    #[test]
    fn test_invalid_f64_conversion() {
        // Test that NaN/Infinity are handled gracefully
        let canonical = PricingConfig::from_cli(f64::NAN, HashMap::new());
        assert_eq!(canonical.default, Amount::zero());
    }
}
