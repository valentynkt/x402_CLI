// Policy Parser - YAML parsing and validation

use super::types::{PolicyConfig, PolicyRule, PolicyType};
use serde_yaml;
use std::path::Path;

pub struct PolicyParser;

impl PolicyParser {
    /// Parse policy from YAML string
    pub fn parse_yaml(yaml_content: &str) -> Result<PolicyConfig, String> {
        serde_yaml::from_str(yaml_content)
            .map_err(|e| format!("YAML parsing error: {}", e))
    }

    /// Parse policy from file
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<PolicyConfig, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("File read error: {}", e))?;
        Self::parse_yaml(&content)
    }

    /// Validate policy structure
    pub fn validate_policy(policy: &PolicyRule) -> Result<(), String> {
        // Use the built-in validate method from PolicyRule
        policy.validate()
                if policy.field.is_none() {
                    return Err("Allowlist/Denylist must have 'field' specified".to_string());
                }
                if policy.values.is_none() || policy.values.as_ref().unwrap().is_empty() {
                    return Err("Allowlist/Denylist must have non-empty 'values'".to_string());
                }
            }
            PolicyType::RateLimit => {
                if policy.max_requests.is_none() {
                    return Err("RateLimit must have 'max_requests' specified".to_string());
                }
                if policy.window_seconds.is_none() {
                    return Err("RateLimit must have 'window_seconds' specified".to_string());
                }
                if policy.max_requests.unwrap() == 0 {
                    return Err("RateLimit 'max_requests' must be > 0".to_string());
                }
                if policy.window_seconds.unwrap() == 0 {
                    return Err("RateLimit 'window_seconds' must be > 0".to_string());
                }
            }
            PolicyType::SpendingCap => {
                if policy.max_amount.is_none() {
                    return Err("SpendingCap must have 'max_amount' specified".to_string());
                }
                if policy.currency.is_none() {
                    return Err("SpendingCap must have 'currency' specified".to_string());
                }
                if policy.window_seconds.is_none() {
                    return Err("SpendingCap must have 'window_seconds' specified".to_string());
                }
                if policy.max_amount.unwrap() <= 0.0 {
                    return Err("SpendingCap 'max_amount' must be > 0".to_string());
                }
            }
        }
        Ok(())
    }

    /// Validate entire policy document
    pub fn validate_document(doc: &PolicyDocument) -> Result<(), String> {
        if doc.policies.is_empty() {
            return Err("Policy document must contain at least one policy".to_string());
        }

        for (idx, policy) in doc.policies.iter().enumerate() {
            Self::validate_policy(policy)
                .map_err(|e| format!("Policy {} validation failed: {}", idx, e))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_allowlist() {
        let yaml = r#"
policies:
  - type: allowlist
    field: agent_id
    values:
      - "agent-abc-123"
      - "agent-xyz-789"
"#;
        let result = PolicyParser::parse_yaml(yaml);
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.policies.len(), 1);
        assert_eq!(doc.policies[0].policy_type, PolicyType::Allowlist);
        assert_eq!(doc.policies[0].field, Some("agent_id".to_string()));
        assert_eq!(doc.policies[0].values.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_parse_valid_denylist() {
        let yaml = r#"
policies:
  - type: denylist
    field: wallet_address
    values:
      - "0xBAD123"
      - "0xBAD456"
"#;
        let result = PolicyParser::parse_yaml(yaml);
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.policies[0].policy_type, PolicyType::Denylist);
    }

    #[test]
    fn test_parse_valid_rate_limit() {
        let yaml = r#"
policies:
  - type: rate_limit
    max_requests: 100
    window_seconds: 3600
"#;
        let result = PolicyParser::parse_yaml(yaml);
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.policies[0].policy_type, PolicyType::RateLimit);
        assert_eq!(doc.policies[0].max_requests, Some(100));
        assert_eq!(doc.policies[0].window_seconds, Some(3600));
    }

    #[test]
    fn test_parse_valid_spending_cap() {
        let yaml = r#"
policies:
  - type: spending_cap
    max_amount: 10.00
    currency: USDC
    window_seconds: 86400
"#;
        let result = PolicyParser::parse_yaml(yaml);
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.policies[0].policy_type, PolicyType::SpendingCap);
        assert_eq!(doc.policies[0].max_amount, Some(10.0));
        assert_eq!(doc.policies[0].currency, Some("USDC".to_string()));
    }

    #[test]
    fn test_parse_multiple_policies() {
        let yaml = r#"
policies:
  - type: allowlist
    field: agent_id
    values: ["agent-1"]
  - type: rate_limit
    max_requests: 50
    window_seconds: 1800
  - type: spending_cap
    max_amount: 5.0
    currency: SOL
    window_seconds: 43200
"#;
        let result = PolicyParser::parse_yaml(yaml);
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.policies.len(), 3);
    }

    #[test]
    fn test_parse_invalid_yaml_syntax() {
        let yaml = "invalid: yaml: syntax::: [[[";
        let result = PolicyParser::parse_yaml(yaml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("YAML parsing error"));
    }

    #[test]
    fn test_parse_missing_type() {
        let yaml = r#"
policies:
  - field: agent_id
    values: ["agent-1"]
"#;
        let result = PolicyParser::parse_yaml(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_unknown_policy_type() {
        let yaml = r#"
policies:
  - type: unknown_type
    field: test
"#;
        let result = PolicyParser::parse_yaml(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_allowlist_missing_field() {
        let policy = Policy {
            policy_type: PolicyType::Allowlist,
            field: None,
            values: Some(vec!["agent-1".to_string()]),
            max_requests: None,
            window_seconds: None,
            max_amount: None,
            currency: None,
        };
        let result = PolicyParser::validate_policy(&policy);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must have 'field' specified"));
    }

    #[test]
    fn test_validate_allowlist_empty_values() {
        let policy = Policy {
            policy_type: PolicyType::Allowlist,
            field: Some("agent_id".to_string()),
            values: Some(vec![]),
            max_requests: None,
            window_seconds: None,
            max_amount: None,
            currency: None,
        };
        let result = PolicyParser::validate_policy(&policy);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("non-empty 'values'"));
    }

    #[test]
    fn test_validate_rate_limit_missing_max_requests() {
        let policy = Policy {
            policy_type: PolicyType::RateLimit,
            field: None,
            values: None,
            max_requests: None,
            window_seconds: Some(3600),
            max_amount: None,
            currency: None,
        };
        let result = PolicyParser::validate_policy(&policy);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must have 'max_requests'"));
    }

    #[test]
    fn test_validate_rate_limit_zero_max_requests() {
        let policy = Policy {
            policy_type: PolicyType::RateLimit,
            field: None,
            values: None,
            max_requests: Some(0),
            window_seconds: Some(3600),
            max_amount: None,
            currency: None,
        };
        let result = PolicyParser::validate_policy(&policy);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be > 0"));
    }

    #[test]
    fn test_validate_spending_cap_missing_currency() {
        let policy = Policy {
            policy_type: PolicyType::SpendingCap,
            field: None,
            values: None,
            max_requests: None,
            window_seconds: Some(86400),
            max_amount: Some(10.0),
            currency: None,
        };
        let result = PolicyParser::validate_policy(&policy);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must have 'currency'"));
    }

    #[test]
    fn test_validate_spending_cap_negative_amount() {
        let policy = Policy {
            policy_type: PolicyType::SpendingCap,
            field: None,
            values: None,
            max_requests: None,
            window_seconds: Some(86400),
            max_amount: Some(-5.0),
            currency: Some("USDC".to_string()),
        };
        let result = PolicyParser::validate_policy(&policy);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be > 0"));
    }

    #[test]
    fn test_validate_empty_document() {
        let doc = PolicyDocument {
            policies: vec![],
        };
        let result = PolicyParser::validate_document(&doc);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("at least one policy"));
    }

    #[test]
    fn test_validate_document_with_invalid_policy() {
        let doc = PolicyDocument {
            policies: vec![
                Policy {
                    policy_type: PolicyType::Allowlist,
                    field: Some("agent_id".to_string()),
                    values: Some(vec!["agent-1".to_string()]),
                    max_requests: None,
                    window_seconds: None,
                    max_amount: None,
                    currency: None,
                },
                Policy {
                    policy_type: PolicyType::RateLimit,
                    field: None,
                    values: None,
                    max_requests: None,
                    window_seconds: Some(3600),
                    max_amount: None,
                    currency: None,
                },
            ],
        };
        let result = PolicyParser::validate_document(&doc);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Policy 1"));
    }
}
