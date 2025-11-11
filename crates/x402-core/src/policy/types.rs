// Policy type definitions for x402-dev

use serde::{Deserialize, Serialize};

/// Complete policy configuration from YAML file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    pub policies: Vec<PolicyRule>,
}

/// Individual policy rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PolicyRule {
    Allowlist {
        field: String,
        values: Vec<String>,
    },
    Denylist {
        field: String,
        values: Vec<String>,
    },
    RateLimit {
        max_requests: u32,
        window_seconds: u32,
    },
    SpendingCap {
        max_amount: f64,
        currency: String,
        window_seconds: u32,
    },
}

/// Policy action result
#[derive(Debug, Clone, PartialEq)]
pub enum PolicyAction {
    Allow,
    Deny(String), // Deny with reason
}

/// Policy type enumeration for categorization
#[derive(Debug, Clone, PartialEq)]
pub enum PolicyType {
    Allowlist,
    Denylist,
    RateLimit,
    SpendingCap,
}

impl PolicyRule {
    /// Get the type of this policy rule
    pub fn policy_type(&self) -> PolicyType {
        match self {
            PolicyRule::Allowlist { .. } => PolicyType::Allowlist,
            PolicyRule::Denylist { .. } => PolicyType::Denylist,
            PolicyRule::RateLimit { .. } => PolicyType::RateLimit,
            PolicyRule::SpendingCap { .. } => PolicyType::SpendingCap,
        }
    }

    /// Validate policy rule configuration
    pub fn validate(&self) -> Result<(), String> {
        match self {
            PolicyRule::Allowlist { field, values } | PolicyRule::Denylist { field, values } => {
                if field.is_empty() {
                    return Err("Field name cannot be empty".to_string());
                }
                if values.is_empty() {
                    return Err("Values list cannot be empty".to_string());
                }
                Ok(())
            }
            PolicyRule::RateLimit {
                max_requests,
                window_seconds,
            } => {
                if *max_requests == 0 {
                    return Err("max_requests must be greater than 0".to_string());
                }
                if *window_seconds == 0 {
                    return Err("window_seconds must be greater than 0".to_string());
                }
                Ok(())
            }
            PolicyRule::SpendingCap {
                max_amount,
                currency,
                window_seconds,
            } => {
                if *max_amount <= 0.0 {
                    return Err("max_amount must be positive".to_string());
                }
                if currency.is_empty() {
                    return Err("currency cannot be empty".to_string());
                }
                if *window_seconds == 0 {
                    return Err("window_seconds must be greater than 0".to_string());
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allowlist_validation() {
        let policy = PolicyRule::Allowlist {
            field: "agent_id".to_string(),
            values: vec!["agent-123".to_string()],
        };
        assert!(policy.validate().is_ok());
    }

    #[test]
    fn test_rate_limit_validation() {
        let policy = PolicyRule::RateLimit {
            max_requests: 100,
            window_seconds: 3600,
        };
        assert!(policy.validate().is_ok());
    }

    #[test]
    fn test_spending_cap_validation() {
        let policy = PolicyRule::SpendingCap {
            max_amount: 10.0,
            currency: "USDC".to_string(),
            window_seconds: 86400,
        };
        assert!(policy.validate().is_ok());
    }

    #[test]
    fn test_invalid_rate_limit() {
        let policy = PolicyRule::RateLimit {
            max_requests: 0,
            window_seconds: 3600,
        };
        assert!(policy.validate().is_err());
    }
}
