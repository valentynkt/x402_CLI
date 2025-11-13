// Configuration YAML fixture generators for testing
//
// Provides sample .x402dev.yaml configurations for different environments,
// with valid, minimal, full, and invalid variants.

/// Returns a valid .x402dev.yaml configuration with common settings
///
/// Use this for testing standard configuration loading and merging.
pub fn valid_config_yaml() -> &'static str {
    r#"
port: 8402
solana_rpc: "https://api.devnet.solana.com"
log_level: info
simulation_mode: success
timeout_delay_ms: 5000

pricing:
  default: 0.01
  per_resource:
    /api/data: 0.05
    /api/premium: 0.10
    /api/admin/*: 0.20
"#
}

/// Returns a minimal .x402dev.yaml with only required fields
///
/// Use this to test default value handling for optional fields.
pub fn minimal_config_yaml() -> &'static str {
    r#"
port: 8402
solana_rpc: "https://api.devnet.solana.com"
"#
}

/// Returns a full .x402dev.yaml with all optional fields specified
///
/// Use this for comprehensive configuration testing.
pub fn full_config_yaml() -> &'static str {
    r#"
port: 8080
solana_rpc: "https://api.mainnet-beta.solana.com"
log_level: debug
simulation_mode: success
timeout_delay_ms: 10000

pricing:
  default: 0.02
  per_resource:
    /api/free: 0.0
    /api/basic: 0.01
    /api/standard: 0.05
    /api/premium: 0.10
    /api/enterprise: 0.50
    /api/admin/*: 0.20
    /api/internal/*: 0.0
    /v1/*: 0.03
    /v2/*: 0.04
"#
}

/// Returns a development environment configuration
///
/// Configured for local development with verbose logging.
pub fn dev_environment_config() -> &'static str {
    r#"
port: 8402
solana_rpc: "http://localhost:8899"
log_level: debug
simulation_mode: success
timeout_delay_ms: 5000

pricing:
  default: 0.001
  per_resource:
    /api/test: 0.0
"#
}

/// Returns a test environment configuration
///
/// Configured for automated testing with devnet.
pub fn test_environment_config() -> &'static str {
    r#"
port: 9402
solana_rpc: "https://api.devnet.solana.com"
log_level: warn
simulation_mode: success
timeout_delay_ms: 3000

pricing:
  default: 0.01
  per_resource:
    /test/*: 0.0
"#
}

/// Returns a production environment configuration
///
/// Configured for production deployment with mainnet-beta.
pub fn prod_environment_config() -> &'static str {
    r#"
port: 8402
solana_rpc: "https://api.mainnet-beta.solana.com"
log_level: info
simulation_mode: success
timeout_delay_ms: 5000

pricing:
  default: 0.05
  per_resource:
    /api/v1/*: 0.10
    /api/v2/*: 0.15
"#
}

/// Returns configuration with invalid port (below 1024)
///
/// Use this to test port validation error handling.
pub fn config_with_invalid_port() -> &'static str {
    r#"
port: 80
solana_rpc: "https://api.devnet.solana.com"
log_level: info
"#
}

/// Returns configuration with bad YAML syntax
///
/// Use this to test YAML parsing error handling.
pub fn config_with_bad_syntax() -> &'static str {
    r#"
port: 8402
solana_rpc: "https://api.devnet.solana.com
log_level: info
  - this is invalid
pricing:
  default: not_a_number
"#
}

/// Returns invalid configuration variants
///
/// Each variant tests a different validation error case.
pub fn invalid_config_yaml(variant: &str) -> String {
    match variant {
        "port_too_low" => r#"
port: 1023
solana_rpc: "https://api.devnet.solana.com"
"#
        .to_string(),

        "port_too_high" => r#"
port: 65536
solana_rpc: "https://api.devnet.solana.com"
"#
        .to_string(),

        "invalid_rpc_url" => r#"
port: 8402
solana_rpc: "not-a-valid-url"
log_level: info
"#
        .to_string(),

        "invalid_log_level" => r#"
port: 8402
solana_rpc: "https://api.devnet.solana.com"
log_level: invalid_level
"#
        .to_string(),

        "invalid_simulation_mode" => r#"
port: 8402
solana_rpc: "https://api.devnet.solana.com"
simulation_mode: invalid_mode
"#
        .to_string(),

        "negative_pricing" => r#"
port: 8402
solana_rpc: "https://api.devnet.solana.com"
pricing:
  default: -0.01
"#
        .to_string(),

        "excessive_pricing" => r#"
port: 8402
solana_rpc: "https://api.devnet.solana.com"
pricing:
  default: 101.0
"#
        .to_string(),

        "negative_per_resource_pricing" => r#"
port: 8402
solana_rpc: "https://api.devnet.solana.com"
pricing:
  default: 0.01
  per_resource:
    /api/test: -0.05
"#
        .to_string(),

        "timeout_too_low" => r#"
port: 8402
solana_rpc: "https://api.devnet.solana.com"
timeout_delay_ms: 50
"#
        .to_string(),

        "timeout_too_high" => r#"
port: 8402
solana_rpc: "https://api.devnet.solana.com"
timeout_delay_ms: 61000
"#
        .to_string(),

        _ => panic!("Unknown variant: {}", variant),
    }
}

/// Returns configuration with environment-specific overrides
///
/// Demonstrates configuration layering (global + project + env).
#[allow(dead_code)]
pub fn config_with_env_overrides() -> &'static str {
    r#"
# Project configuration that can be overridden by environment variables:
# - X402_DEV_PORT
# - X402_DEV_SOLANA_RPC
# - X402_DEV_LOG_LEVEL

port: 8402
solana_rpc: "https://api.devnet.solana.com"
log_level: info
simulation_mode: success
timeout_delay_ms: 5000

pricing:
  default: 0.01
"#
}

/// Returns configuration with wildcard pricing patterns
///
/// Demonstrates advanced per-resource pricing with wildcards.
pub fn config_with_wildcard_pricing() -> &'static str {
    r#"
port: 8402
solana_rpc: "https://api.devnet.solana.com"

pricing:
  default: 0.01
  per_resource:
    /api/*: 0.03
    /api/admin/*: 0.20
    /api/admin/super/*: 0.50
    /public/*: 0.0
"#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config_yaml_parses() {
        let yaml = valid_config_yaml();
        assert!(yaml.contains("port:"));
        assert!(yaml.contains("solana_rpc:"));
        assert!(yaml.contains("pricing:"));
    }

    #[test]
    fn test_minimal_config_yaml() {
        let yaml = minimal_config_yaml();
        assert!(yaml.contains("port: 8402"));
        assert!(yaml.contains("solana_rpc:"));
        // Should not contain optional fields
        assert!(!yaml.contains("log_level:"));
    }

    #[test]
    fn test_full_config_yaml() {
        let yaml = full_config_yaml();
        assert!(yaml.contains("log_level:"));
        assert!(yaml.contains("simulation_mode:"));
        assert!(yaml.contains("timeout_delay_ms:"));
        assert!(yaml.contains("per_resource:"));
    }

    #[test]
    fn test_environment_configs() {
        let dev = dev_environment_config();
        assert!(dev.contains("localhost:8899"));
        assert!(dev.contains("log_level: debug"));

        let test = test_environment_config();
        assert!(test.contains("devnet"));
        assert!(test.contains("log_level: warn"));

        let prod = prod_environment_config();
        assert!(prod.contains("mainnet-beta"));
        assert!(prod.contains("log_level: info"));
    }

    #[test]
    fn test_invalid_port_config() {
        let yaml = config_with_invalid_port();
        assert!(yaml.contains("port: 80"));
    }

    #[test]
    fn test_bad_syntax_config() {
        let yaml = config_with_bad_syntax();
        assert!(yaml.contains("not_a_number"));
    }

    #[test]
    fn test_invalid_config_variants() {
        let variants = vec![
            "port_too_low",
            "port_too_high",
            "invalid_rpc_url",
            "invalid_log_level",
            "invalid_simulation_mode",
            "negative_pricing",
            "excessive_pricing",
            "negative_per_resource_pricing",
            "timeout_too_low",
            "timeout_too_high",
        ];

        for variant in variants {
            let yaml = invalid_config_yaml(variant);
            assert!(!yaml.is_empty(), "Variant {} should not be empty", variant);
        }
    }

    #[test]
    fn test_wildcard_pricing_config() {
        let yaml = config_with_wildcard_pricing();
        assert!(yaml.contains("/api/*:"));
        assert!(yaml.contains("/api/admin/*:"));
        assert!(yaml.contains("/api/admin/super/*:"));
    }
}
