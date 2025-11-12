//! Canonical pricing configuration for x402-dev
//!
//! This module provides the single source of truth for pricing configuration,
//! used across CLI, policy engine, and code generation contexts.

use crate::amount::Amount;
use crate::error::{DomainError, DomainResult};
use crate::types::ResourcePath;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Canonical pricing configuration used across all contexts
///
/// This type unifies pricing configuration from CLI, policy engine, and code generation.
/// It supports both default pricing and per-resource pricing overrides with wildcard patterns.
///
/// # Examples
///
/// ```
/// use x402_domain::{PricingConfig, Amount, ResourcePath};
///
/// let config = PricingConfig::new(Amount::from_usdc_lamports(10_000).unwrap())
///     .with_resource_price(
///         ResourcePath::new("/api/premium").unwrap(),
///         Amount::from_usdc_lamports(50_000).unwrap()
///     )
///     .with_resource_price(
///         ResourcePath::new("/api/*").unwrap(),
///         Amount::from_usdc_lamports(20_000).unwrap()
///     );
///
/// // Exact match takes priority
/// assert_eq!(config.get_price("/api/premium"), Amount::from_usdc_lamports(50_000).unwrap());
///
/// // Wildcard match
/// assert_eq!(config.get_price("/api/users"), Amount::from_usdc_lamports(20_000).unwrap());
///
/// // Default fallback
/// assert_eq!(config.get_price("/other"), Amount::from_usdc_lamports(10_000).unwrap());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PricingConfig {
    /// Default price when no resource-specific price is set
    pub default: Amount,

    /// Per-resource pricing overrides (supports exact match and wildcard patterns)
    #[serde(default)]
    pub per_resource: HashMap<String, Amount>,

    /// Currency type (e.g., "USDC", "SOL") - for metadata/display purposes
    #[serde(default = "default_currency")]
    pub currency: String,

    /// Optional memo prefix for payment transactions
    #[serde(default)]
    pub memo_prefix: Option<String>,
}

fn default_currency() -> String {
    "USDC".to_string()
}

impl PricingConfig {
    /// Creates a new pricing config with a default price
    ///
    /// # Examples
    ///
    /// ```
    /// use x402_domain::{PricingConfig, Amount};
    ///
    /// let config = PricingConfig::new(Amount::from_usdc_lamports(10_000).unwrap());
    /// assert_eq!(config.default, Amount::from_usdc_lamports(10_000).unwrap());
    /// ```
    pub fn new(default: Amount) -> Self {
        Self {
            default,
            per_resource: HashMap::new(),
            currency: default_currency(),
            memo_prefix: None,
        }
    }

    /// Sets the currency type
    pub fn with_currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = currency.into();
        self
    }

    /// Sets the memo prefix
    pub fn with_memo_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.memo_prefix = Some(prefix.into());
        self
    }

    /// Adds a resource-specific price
    ///
    /// # Examples
    ///
    /// ```
    /// use x402_domain::{PricingConfig, Amount, ResourcePath};
    ///
    /// let config = PricingConfig::default()
    ///     .with_resource_price(
    ///         ResourcePath::new("/api/premium").unwrap(),
    ///         Amount::from_usdc_lamports(50_000).unwrap()
    ///     );
    ///
    /// assert_eq!(config.get_price("/api/premium"), Amount::from_usdc_lamports(50_000).unwrap());
    /// ```
    pub fn with_resource_price(mut self, path: ResourcePath, amount: Amount) -> Self {
        self.per_resource.insert(path.as_str().to_string(), amount);
        self
    }

    /// Gets the price for a given resource path
    ///
    /// Matching priority:
    /// 1. Exact match (e.g., `/api/data` matches `/api/data`)
    /// 2. Longest wildcard prefix match (e.g., `/api/admin/*` over `/api/*`)
    /// 3. Default price
    ///
    /// # Examples
    ///
    /// ```
    /// use x402_domain::{PricingConfig, Amount, ResourcePath};
    ///
    /// let config = PricingConfig::new(Amount::from_usdc_lamports(10_000).unwrap())
    ///     .with_resource_price(
    ///         ResourcePath::new("/api/*").unwrap(),
    ///         Amount::from_usdc_lamports(20_000).unwrap()
    ///     )
    ///     .with_resource_price(
    ///         ResourcePath::new("/api/premium").unwrap(),
    ///         Amount::from_usdc_lamports(50_000).unwrap()
    ///     );
    ///
    /// // Priority 1: Exact match
    /// assert_eq!(config.get_price("/api/premium"), Amount::from_usdc_lamports(50_000).unwrap());
    ///
    /// // Priority 2: Wildcard match
    /// assert_eq!(config.get_price("/api/users"), Amount::from_usdc_lamports(20_000).unwrap());
    ///
    /// // Priority 3: Default
    /// assert_eq!(config.get_price("/other"), Amount::from_usdc_lamports(10_000).unwrap());
    /// ```
    pub fn get_price(&self, path: &str) -> Amount {
        // Priority 1: Try exact match first
        if let Some(&amount) = self.per_resource.get(path) {
            return amount;
        }

        // Priority 2: Try wildcard pattern matching (longest prefix wins)
        let mut matches: Vec<(&str, Amount)> = Vec::new();

        for (pattern, &amount) in &self.per_resource {
            // Check if pattern is a wildcard
            if pattern.ends_with("/*") {
                let prefix = &pattern[..pattern.len() - 2];
                if path.starts_with(prefix) {
                    matches.push((prefix, amount));
                }
            }
        }

        if !matches.is_empty() {
            // Sort by prefix length (descending) to get longest match
            matches.sort_by_key(|(prefix, _)| std::cmp::Reverse(prefix.len()));
            return matches[0].1;
        }

        // Priority 3: Fall back to default
        self.default
    }

    /// Validates the pricing configuration
    ///
    /// Checks:
    /// - Currency is not empty
    /// - Resource paths are valid
    pub fn validate(&self) -> DomainResult<()> {
        if self.currency.is_empty() {
            return Err(DomainError::InvalidAmount("Currency cannot be empty".into()));
        }

        // Validate all resource paths
        for path in self.per_resource.keys() {
            ResourcePath::new(path.clone())?;
        }

        Ok(())
    }

    /// Gets all configured resource paths
    pub fn resource_paths(&self) -> Vec<String> {
        self.per_resource.keys().cloned().collect()
    }

    /// Checks if a resource-specific price is configured
    pub fn has_resource_price(&self, path: &str) -> bool {
        self.per_resource.contains_key(path)
    }
}

impl Default for PricingConfig {
    fn default() -> Self {
        // Default to 0.01 USDC (10,000 lamports)
        let default_amount = Amount::from_usdc_lamports(10_000).unwrap_or_else(|_| Amount::zero());
        Self::new(default_amount).with_currency("USDC")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_pricing() {
        let config = PricingConfig::default();
        assert_eq!(
            config.get_price("/any/path"),
            Amount::from_usdc_lamports(10_000).unwrap()
        );
        assert_eq!(config.currency, "USDC");
    }

    #[test]
    fn test_exact_match_priority() {
        let config = PricingConfig::new(Amount::from_usdc_lamports(10_000).unwrap())
            .with_resource_price(
                ResourcePath::new("/api/*").unwrap(),
                Amount::from_usdc_lamports(20_000).unwrap(),
            )
            .with_resource_price(
                ResourcePath::new("/api/premium").unwrap(),
                Amount::from_usdc_lamports(50_000).unwrap(),
            );

        // Exact match takes priority over wildcard
        assert_eq!(
            config.get_price("/api/premium"),
            Amount::from_usdc_lamports(50_000).unwrap()
        );
    }

    #[test]
    fn test_wildcard_matching() {
        let config = PricingConfig::new(Amount::from_usdc_lamports(10_000).unwrap())
            .with_resource_price(
                ResourcePath::new("/api/*").unwrap(),
                Amount::from_usdc_lamports(20_000).unwrap(),
            );

        assert_eq!(
            config.get_price("/api/users"),
            Amount::from_usdc_lamports(20_000).unwrap()
        );
        assert_eq!(
            config.get_price("/api/posts"),
            Amount::from_usdc_lamports(20_000).unwrap()
        );
        assert_eq!(
            config.get_price("/other"),
            Amount::from_usdc_lamports(10_000).unwrap()
        );
    }

    #[test]
    fn test_longest_wildcard_wins() {
        let config = PricingConfig::new(Amount::from_usdc_lamports(10_000).unwrap())
            .with_resource_price(
                ResourcePath::new("/api/*").unwrap(),
                Amount::from_usdc_lamports(20_000).unwrap(),
            )
            .with_resource_price(
                ResourcePath::new("/api/admin/*").unwrap(),
                Amount::from_usdc_lamports(50_000).unwrap(),
            );

        // Longest matching prefix wins
        assert_eq!(
            config.get_price("/api/admin/users"),
            Amount::from_usdc_lamports(50_000).unwrap()
        );
        assert_eq!(
            config.get_price("/api/users"),
            Amount::from_usdc_lamports(20_000).unwrap()
        );
    }

    #[test]
    fn test_default_fallback() {
        let config = PricingConfig::default();
        assert_eq!(
            config.get_price("/any/path"),
            Amount::from_usdc_lamports(10_000).unwrap()
        );
    }

    #[test]
    fn test_with_currency() {
        let config = PricingConfig::default().with_currency("SOL");
        assert_eq!(config.currency, "SOL");
    }

    #[test]
    fn test_with_memo_prefix() {
        let config = PricingConfig::default().with_memo_prefix("x402:");
        assert_eq!(config.memo_prefix, Some("x402:".to_string()));
    }

    #[test]
    fn test_validation() {
        let config = PricingConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validation_empty_currency() {
        let mut config = PricingConfig::default();
        config.currency = String::new();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_has_resource_price() {
        let config = PricingConfig::default().with_resource_price(
            ResourcePath::new("/api/premium").unwrap(),
            Amount::from_usdc_lamports(50_000).unwrap(),
        );

        assert!(config.has_resource_price("/api/premium"));
        assert!(!config.has_resource_price("/api/users"));
    }

    #[test]
    fn test_resource_paths() {
        let config = PricingConfig::default()
            .with_resource_price(
                ResourcePath::new("/api/premium").unwrap(),
                Amount::from_usdc_lamports(50_000).unwrap(),
            )
            .with_resource_price(
                ResourcePath::new("/api/*").unwrap(),
                Amount::from_usdc_lamports(20_000).unwrap(),
            );

        let paths = config.resource_paths();
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn test_serde_roundtrip() {
        let config = PricingConfig::default()
            .with_currency("SOL")
            .with_memo_prefix("test:")
            .with_resource_price(
                ResourcePath::new("/api/premium").unwrap(),
                Amount::from_usdc_lamports(50_000).unwrap(),
            );

        let yaml = serde_yaml::to_string(&config).unwrap();
        let deserialized: PricingConfig = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_builder_pattern() {
        let config = PricingConfig::new(Amount::from_usdc_lamports(10_000).unwrap())
            .with_currency("USDC")
            .with_memo_prefix("x402:")
            .with_resource_price(
                ResourcePath::new("/api/premium").unwrap(),
                Amount::from_usdc_lamports(50_000).unwrap(),
            );

        assert_eq!(config.default, Amount::from_usdc_lamports(10_000).unwrap());
        assert_eq!(config.currency, "USDC");
        assert_eq!(config.memo_prefix, Some("x402:".to_string()));
    }
}
