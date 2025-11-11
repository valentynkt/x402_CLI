// Policy type definitions for code generation

use serde::{Deserialize, Serialize};

/// Policy rule type for code generation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PolicyRule {
    /// Allowlist policy - only specified agents can access
    Allowlist { agents: Vec<String> },

    /// Denylist policy - specified agents are blocked
    Denylist { agents: Vec<String> },

    /// Rate limiting policy
    #[serde(rename = "rate_limit")]
    RateLimit {
        max_requests: u32,
        window: u32, // in seconds
    },

    /// Spending cap policy
    #[serde(rename = "spending_cap")]
    SpendingCap {
        max_amount: f64,
        period: String, // "daily", "weekly", "monthly"
        currency: String, // "SOL", "USDC"
    },
}

/// Complete policy configuration for code generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    pub policies: Vec<PolicyRule>,

    #[serde(default)]
    pub pricing: PricingConfig,

    #[serde(default)]
    pub audit: AuditConfig,
}

/// Pricing configuration for 402 responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingConfig {
    #[serde(default = "default_amount")]
    pub amount: f64,

    #[serde(default = "default_currency")]
    pub currency: String,

    #[serde(default)]
    pub memo_prefix: Option<String>,
}

impl Default for PricingConfig {
    fn default() -> Self {
        Self {
            amount: default_amount(),
            currency: default_currency(),
            memo_prefix: None,
        }
    }
}

fn default_amount() -> f64 {
    0.01
}

fn default_currency() -> String {
    "USDC".to_string()
}

/// Audit logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    #[serde(default = "default_format")]
    pub format: String, // "csv" or "json"

    #[serde(default)]
    pub destination: Option<String>, // file path or "stdout"
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            format: default_format(),
            destination: Some("stdout".to_string()),
        }
    }
}

fn default_enabled() -> bool {
    true
}

fn default_format() -> String {
    "json".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_rule_deserialization() {
        let yaml = r#"
type: allowlist
agents:
  - agent-123
  - agent-456
"#;

        let rule: PolicyRule = serde_yaml::from_str(yaml).unwrap();
        match rule {
            PolicyRule::Allowlist { agents } => {
                assert_eq!(agents.len(), 2);
                assert!(agents.contains(&"agent-123".to_string()));
            }
            _ => panic!("Expected allowlist policy"),
        }
    }

    #[test]
    fn test_rate_limit_deserialization() {
        let yaml = r#"
type: rate_limit
max_requests: 100
window: 3600
"#;

        let rule: PolicyRule = serde_yaml::from_str(yaml).unwrap();
        match rule {
            PolicyRule::RateLimit { max_requests, window } => {
                assert_eq!(max_requests, 100);
                assert_eq!(window, 3600);
            }
            _ => panic!("Expected rate_limit policy"),
        }
    }

    #[test]
    fn test_spending_cap_deserialization() {
        let yaml = r#"
type: spending_cap
max_amount: 10.0
period: daily
currency: USDC
"#;

        let rule: PolicyRule = serde_yaml::from_str(yaml).unwrap();
        match rule {
            PolicyRule::SpendingCap { max_amount, period, currency } => {
                assert_eq!(max_amount, 10.0);
                assert_eq!(period, "daily");
                assert_eq!(currency, "USDC");
            }
            _ => panic!("Expected spending_cap policy"),
        }
    }

    #[test]
    fn test_policy_config_deserialization() {
        let yaml = r#"
policies:
  - type: allowlist
    agents: [agent-1, agent-2]
  - type: rate_limit
    max_requests: 100
    window: 3600
pricing:
  amount: 0.02
  currency: SOL
audit:
  enabled: true
  format: csv
"#;

        let config: PolicyConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.policies.len(), 2);
        assert_eq!(config.pricing.amount, 0.02);
        assert_eq!(config.pricing.currency, "SOL");
        assert_eq!(config.audit.format, "csv");
    }

    #[test]
    fn test_default_pricing() {
        let pricing = PricingConfig::default();
        assert_eq!(pricing.amount, 0.01);
        assert_eq!(pricing.currency, "USDC");
    }

    #[test]
    fn test_default_audit() {
        let audit = AuditConfig::default();
        assert!(audit.enabled);
        assert_eq!(audit.format, "json");
        assert_eq!(audit.destination, Some("stdout".to_string()));
    }
}
