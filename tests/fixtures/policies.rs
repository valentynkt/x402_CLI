// Policy YAML fixture generators for testing
//
// Provides sample policy configurations for various frameworks,
// pricing tiers, and edge cases (empty policies, malformed YAML, etc.)

/// Returns a valid policy YAML with multiple policy types
///
/// This fixture includes all policy types: allowlist, denylist, rate_limit, and spending_cap.
/// Use this for testing complete policy parsing and validation.
pub fn valid_policy_yaml() -> &'static str {
    r#"
policies:
  - type: allowlist
    field: agent_id
    values:
      - agent-123
      - agent-456
      - agent-789

  - type: denylist
    field: ip_address
    values:
      - 192.168.1.100
      - 10.0.0.50

  - type: rate_limit
    max_requests: 100
    window_seconds: 3600

  - type: spending_cap
    max_amount: 10.0
    currency: USDC
    window_seconds: 86400
"#
}

/// Returns a minimal valid policy with only one rule
///
/// Use this for testing basic policy parsing with minimal configuration.
pub fn minimal_policy_yaml() -> &'static str {
    r#"
policies:
  - type: allowlist
    field: agent_id
    values:
      - agent-123
"#
}

/// Returns an Express.js framework-specific policy configuration
///
/// Includes API routes and typical Express patterns.
pub fn express_policy_yaml() -> &'static str {
    r#"
policies:
  - type: allowlist
    field: resource_path
    values:
      - /api/v1/users
      - /api/v1/posts
      - /api/v1/comments

  - type: rate_limit
    max_requests: 1000
    window_seconds: 3600

  - type: spending_cap
    max_amount: 50.0
    currency: USDC
    window_seconds: 86400
"#
}

/// Returns a Fastify framework-specific policy configuration
///
/// Includes typical Fastify route patterns and plugins.
pub fn fastify_policy_yaml() -> &'static str {
    r#"
policies:
  - type: allowlist
    field: resource_path
    values:
      - /v1/health
      - /v1/metrics
      - /v1/api/*

  - type: rate_limit
    max_requests: 500
    window_seconds: 60

  - type: denylist
    field: agent_id
    values:
      - blocked-agent-1
      - blocked-agent-2
"#
}

/// Returns a policy with different pricing tiers
///
/// Demonstrates per-resource pricing rules with wildcards.
pub fn policy_with_pricing_tiers() -> &'static str {
    r#"
policies:
  - type: allowlist
    field: resource_path
    values:
      - /api/free/*
      - /api/basic/*
      - /api/premium/*
      - /api/enterprise/*

  - type: rate_limit
    max_requests: 10
    window_seconds: 60

  - type: spending_cap
    max_amount: 5.0
    currency: USDC
    window_seconds: 3600
"#
}

/// Returns an empty policy YAML (no policies defined)
///
/// Use this to test error handling for empty configurations.
pub fn empty_policy_yaml() -> &'static str {
    r#"
policies: []
"#
}

/// Returns malformed YAML with syntax errors
///
/// Use this to test YAML parsing error handling.
pub fn malformed_yaml() -> &'static str {
    r#"
policies:
  - type: allowlist
    field: agent_id
    values:
      - agent-123
    - this is invalid syntax here
  - type: rate_limit
    max_requests: "not a number"
    window_seconds: 3600
"#
}

/// Returns policy YAML missing required fields
///
/// Each variant is missing a different required field for comprehensive testing.
pub fn missing_required_fields(variant: &str) -> String {
    match variant {
        "allowlist_no_field" => r#"
policies:
  - type: allowlist
    values:
      - agent-123
"#
        .to_string(),

        "allowlist_no_values" => r#"
policies:
  - type: allowlist
    field: agent_id
"#
        .to_string(),

        "rate_limit_no_max_requests" => r#"
policies:
  - type: rate_limit
    window_seconds: 3600
"#
        .to_string(),

        "rate_limit_no_window" => r#"
policies:
  - type: rate_limit
    max_requests: 100
"#
        .to_string(),

        "spending_cap_no_amount" => r#"
policies:
  - type: spending_cap
    currency: USDC
    window_seconds: 86400
"#
        .to_string(),

        "spending_cap_no_currency" => r#"
policies:
  - type: spending_cap
    max_amount: 10.0
    window_seconds: 86400
"#
        .to_string(),

        _ => panic!("Unknown variant: {}", variant),
    }
}

/// Returns policy with invalid values
///
/// Use this to test validation of policy rule values.
pub fn invalid_policy_yaml(variant: &str) -> String {
    match variant {
        "empty_field_name" => r#"
policies:
  - type: allowlist
    field: ""
    values:
      - agent-123
"#
        .to_string(),

        "empty_values_list" => r#"
policies:
  - type: allowlist
    field: agent_id
    values: []
"#
        .to_string(),

        "zero_max_requests" => r#"
policies:
  - type: rate_limit
    max_requests: 0
    window_seconds: 3600
"#
        .to_string(),

        "negative_max_requests" => r#"
policies:
  - type: rate_limit
    max_requests: -100
    window_seconds: 3600
"#
        .to_string(),

        "zero_window_seconds" => r#"
policies:
  - type: rate_limit
    max_requests: 100
    window_seconds: 0
"#
        .to_string(),

        "negative_amount" => r#"
policies:
  - type: spending_cap
    max_amount: -10.0
    currency: USDC
    window_seconds: 86400
"#
        .to_string(),

        "zero_amount" => r#"
policies:
  - type: spending_cap
    max_amount: 0.0
    currency: USDC
    window_seconds: 86400
"#
        .to_string(),

        "empty_currency" => r#"
policies:
  - type: spending_cap
    max_amount: 10.0
    currency: ""
    window_seconds: 86400
"#
        .to_string(),

        _ => panic!("Unknown variant: {}", variant),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_policy_yaml_parses() {
        let yaml = valid_policy_yaml();
        assert!(yaml.contains("policies:"));
        assert!(yaml.contains("type: allowlist"));
        assert!(yaml.contains("type: rate_limit"));
    }

    #[test]
    fn test_minimal_policy_yaml() {
        let yaml = minimal_policy_yaml();
        assert!(yaml.contains("policies:"));
        assert_eq!(yaml.matches("type:").count(), 1);
    }

    #[test]
    fn test_framework_specific_policies() {
        let express = express_policy_yaml();
        assert!(express.contains("/api/v1/users"));

        let fastify = fastify_policy_yaml();
        assert!(fastify.contains("/v1/health"));
    }

    #[test]
    fn test_pricing_tiers_policy() {
        let yaml = policy_with_pricing_tiers();
        assert!(yaml.contains("/api/free/*"));
        assert!(yaml.contains("/api/premium/*"));
    }

    #[test]
    fn test_empty_policy() {
        let yaml = empty_policy_yaml();
        assert!(yaml.contains("policies: []"));
    }

    #[test]
    fn test_missing_fields_variants() {
        let variants = vec![
            "allowlist_no_field",
            "allowlist_no_values",
            "rate_limit_no_max_requests",
            "rate_limit_no_window",
            "spending_cap_no_amount",
            "spending_cap_no_currency",
        ];

        for variant in variants {
            let yaml = missing_required_fields(variant);
            assert!(!yaml.is_empty(), "Variant {} should not be empty", variant);
        }
    }

    #[test]
    fn test_invalid_policy_variants() {
        let variants = vec![
            "empty_field_name",
            "empty_values_list",
            "zero_max_requests",
            "negative_max_requests",
            "zero_window_seconds",
            "negative_amount",
            "zero_amount",
            "empty_currency",
        ];

        for variant in variants {
            let yaml = invalid_policy_yaml(variant);
            assert!(!yaml.is_empty(), "Variant {} should not be empty", variant);
        }
    }
}
