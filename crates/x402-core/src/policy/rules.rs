// Policy rule definitions
//
// Defines the structure for policy rules as specified in FR-5.1
use serde::{Deserialize, Serialize};

// Re-export types from types.rs
pub use super::types::{PolicyRule, PolicyType};

/// Pricing configuration for generated middleware
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingConfig {
    #[serde(default = "default_amount")]
    pub amount: f64,
    #[serde(default = "default_currency")]
    pub currency: String,
    pub memo_prefix: Option<String>,
}

fn default_amount() -> f64 {
    0.01
}

fn default_currency() -> String {
    "USDC".to_string()
}

impl Default for PricingConfig {
    fn default() -> Self {
        Self {
            amount: 0.01,
            currency: "USDC".to_string(),
            memo_prefix: None,
        }
    }
}

/// Audit configuration for generated middleware
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_format")]
    pub format: String,
    pub destination: Option<String>,
}

fn default_enabled() -> bool {
    true
}

fn default_format() -> String {
    "json".to_string()
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            format: "json".to_string(),
            destination: None,
        }
    }
}

/// Complete policy file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyFile {
    pub policies: Vec<PolicyRule>,

    #[serde(default)]
    pub pricing: PricingConfig,

    #[serde(default)]
    pub audit: AuditConfig,
}
