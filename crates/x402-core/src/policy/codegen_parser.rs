// Policy file parser for code generation

use crate::policy::codegen_types::PolicyConfig;
use std::path::Path;

/// Parse a policy YAML file
pub fn parse_policy_file(path: &Path) -> Result<PolicyConfig, anyhow::Error> {
    let contents = std::fs::read_to_string(path)?;
    parse_policy_yaml(&contents)
}

/// Parse policy YAML string
pub fn parse_policy_yaml(yaml: &str) -> Result<PolicyConfig, anyhow::Error> {
    let config: PolicyConfig = serde_yaml::from_str(yaml)
        .map_err(|e| anyhow::anyhow!("Failed to parse policy YAML: {}", e))?;

    // Validate the policy configuration
    validate_policy_config(&config)?;

    Ok(config)
}

/// Validate policy configuration for conflicts and issues
fn validate_policy_config(config: &PolicyConfig) -> Result<(), anyhow::Error> {
    use crate::policy::codegen_types::PolicyRule;

    // Check for conflicting allowlist and denylist
    let has_allowlist = config.policies.iter().any(|p| matches!(p, PolicyRule::Allowlist { .. }));
    let has_denylist = config.policies.iter().any(|p| matches!(p, PolicyRule::Denylist { .. }));

    if has_allowlist && has_denylist {
        // Check for overlapping agents
        let allowlist_agents: Vec<String> = config.policies.iter()
            .filter_map(|p| match p {
                PolicyRule::Allowlist { agents } => Some(agents.clone()),
                _ => None,
            })
            .flatten()
            .collect();

        let denylist_agents: Vec<String> = config.policies.iter()
            .filter_map(|p| match p {
                PolicyRule::Denylist { agents } => Some(agents.clone()),
                _ => None,
            })
            .flatten()
            .collect();

        for agent in &allowlist_agents {
            if denylist_agents.contains(agent) {
                return Err(anyhow::anyhow!(
                    "Policy conflict: agent '{}' is in both allowlist and denylist",
                    agent
                ));
            }
        }
    }

    // Validate rate limit values
    for policy in &config.policies {
        if let PolicyRule::RateLimit { max_requests, window } = policy {
            if *max_requests == 0 {
                return Err(anyhow::anyhow!("Rate limit max_requests must be greater than 0"));
            }
            if *window == 0 {
                return Err(anyhow::anyhow!("Rate limit window must be greater than 0"));
            }
        }
    }

    // Validate spending cap values
    for policy in &config.policies {
        if let PolicyRule::SpendingCap { max_amount, period, currency } = policy {
            if *max_amount <= 0.0 {
                return Err(anyhow::anyhow!("Spending cap max_amount must be greater than 0"));
            }
            if !["daily", "weekly", "monthly"].contains(&period.as_str()) {
                return Err(anyhow::anyhow!(
                    "Spending cap period must be one of: daily, weekly, monthly (got: {})",
                    period
                ));
            }
            if currency.is_empty() {
                return Err(anyhow::anyhow!("Spending cap currency cannot be empty"));
            }
        }
    }

    // Validate pricing
    if config.pricing.amount <= 0.0 {
        return Err(anyhow::anyhow!("Pricing amount must be greater than 0"));
    }

    // Validate audit config
    if config.audit.enabled {
        if !["csv", "json"].contains(&config.audit.format.as_str()) {
            return Err(anyhow::anyhow!(
                "Audit format must be 'csv' or 'json' (got: {})",
                config.audit.format
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_policy() {
        let yaml = r#"
policies:
  - type: allowlist
    agents: [agent-1, agent-2]
  - type: rate_limit
    max_requests: 100
    window: 3600
"#;

        let result = parse_policy_yaml(yaml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_invalid_yaml() {
        let yaml = "invalid: [unclosed";
        let result = parse_policy_yaml(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_conflicting_policies() {
        let yaml = r#"
policies:
  - type: allowlist
    agents: [agent-1, agent-2]
  - type: denylist
    agents: [agent-2, agent-3]
"#;

        let result = parse_policy_yaml(yaml);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("agent-2"));
    }

    #[test]
    fn test_validate_zero_rate_limit() {
        let yaml = r#"
policies:
  - type: rate_limit
    max_requests: 0
    window: 3600
"#;

        let result = parse_policy_yaml(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_invalid_period() {
        let yaml = r#"
policies:
  - type: spending_cap
    max_amount: 10.0
    period: yearly
    currency: USDC
"#;

        let result = parse_policy_yaml(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_negative_amount() {
        let yaml = r#"
policies: []
pricing:
  amount: -1.0
  currency: USDC
"#;

        let result = parse_policy_yaml(yaml);
        assert!(result.is_err());
    }
}
