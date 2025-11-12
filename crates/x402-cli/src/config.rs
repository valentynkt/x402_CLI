use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

/// Log level for application logging
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    /// Returns true if this level is at least as verbose as `other`
    pub fn is_at_least(&self, other: LogLevel) -> bool {
        use LogLevel::*;
        let self_level = match self {
            Error => 0,
            Warn => 1,
            Info => 2,
            Debug => 3,
            Trace => 4,
        };
        let other_level = match other {
            Error => 0,
            Warn => 1,
            Info => 2,
            Debug => 3,
            Trace => 4,
        };
        self_level >= other_level
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Error => write!(f, "error"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Trace => write!(f, "trace"),
        }
    }
}

impl FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "error" => Ok(LogLevel::Error),
            "warn" => Ok(LogLevel::Warn),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            "trace" => Ok(LogLevel::Trace),
            _ => Err(format!("Invalid log level: '{}'. Valid values: error, warn, info, debug, trace", s)),
        }
    }
}


/// Simulation mode for payment verification (Story 2.3)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum SimulationMode {
    #[default]
    Success,
    Failure,
    Timeout,
}

impl fmt::Display for SimulationMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimulationMode::Success => write!(f, "success"),
            SimulationMode::Failure => write!(f, "failure"),
            SimulationMode::Timeout => write!(f, "timeout"),
        }
    }
}

impl FromStr for SimulationMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "success" => Ok(SimulationMode::Success),
            "fail" | "failure" => Ok(SimulationMode::Failure),
            "timeout" => Ok(SimulationMode::Timeout),
            _ => Err(format!("Invalid simulation mode: '{}'. Valid values: success, failure, timeout", s)),
        }
    }
}


/// Configuration schema for x402-dev
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_solana_rpc")]
    pub solana_rpc: String,

    #[serde(default)]
    pub log_level: LogLevel,

    #[serde(default)]
    pub pricing: PricingConfig,

    #[serde(default)]
    pub simulation_mode: SimulationMode,

    #[serde(default = "default_timeout_ms")]
    pub timeout_delay_ms: u64,
}

// Default value functions for serde
fn default_port() -> u16 {
    8402
}

fn default_solana_rpc() -> String {
    "https://api.devnet.solana.com".to_string()
}

fn default_timeout_ms() -> u64 {
    5000
}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: default_port(),
            solana_rpc: default_solana_rpc(),
            log_level: LogLevel::default(),
            pricing: PricingConfig::default(),
            simulation_mode: SimulationMode::default(),
            timeout_delay_ms: default_timeout_ms(),
        }
    }
}

/// Pricing configuration for mock server endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingConfig {
    /// Default pricing for all endpoints (in SOL/USDC)
    #[serde(default = "default_pricing_amount")]
    pub default: f64,

    /// Per-resource pricing rules (supports exact match and wildcard patterns)
    #[serde(default)]
    pub per_resource: HashMap<String, f64>,
}

fn default_pricing_amount() -> f64 {
    0.01
}

impl Default for PricingConfig {
    fn default() -> Self {
        PricingConfig {
            default: default_pricing_amount(),
            per_resource: HashMap::new(),
        }
    }
}

impl PricingConfig {
    /// Validate pricing configuration values
    pub fn validate(&self) -> Result<()> {
        // Validate default pricing
        if self.default < 0.0 {
            anyhow::bail!(
                "Default pricing must be non-negative. Got: {}\nFix: Set default pricing to a non-negative value, e.g., 0.01",
                self.default
            );
        }
        if self.default > 100.0 {
            anyhow::bail!(
                "Default pricing must be <= 100 SOL. Got: {}\nFix: Set default pricing to a reasonable value, e.g., 0.01",
                self.default
            );
        }

        // Validate per-resource pricing
        for (path, amount) in &self.per_resource {
            if *amount < 0.0 {
                anyhow::bail!(
                    "Pricing for {} must be non-negative. Got: {}\nFix: Set pricing to a non-negative value",
                    path,
                    amount
                );
            }
            if *amount > 100.0 {
                anyhow::bail!(
                    "Pricing for {} must be <= 100 SOL. Got: {}\nFix: Set pricing to a reasonable value",
                    path,
                    amount
                );
            }
        }

        Ok(())
    }
}

/// Pricing matcher for route-based pricing
pub struct PricingMatcher {
    config: PricingConfig,
}

impl PricingMatcher {
    /// Create a new pricing matcher
    pub fn new(config: PricingConfig) -> Self {
        PricingMatcher { config }
    }

    /// Get the price for a given request path
    ///
    /// Priority order:
    /// 1. Exact match (e.g., "/api/data" matches "/api/data")
    /// 2. Prefix match with wildcard (e.g., "/api/*" matches "/api/users")
    /// 3. Default pricing
    pub fn get_price_for_path(&self, path: &str) -> f64 {
        // Priority 1: Exact match
        if let Some(&amount) = self.config.per_resource.get(path) {
            return amount;
        }

        // Priority 2: Prefix match (wildcard patterns)
        let mut matches: Vec<(&str, f64)> = Vec::new();
        for (pattern, &amount) in &self.config.per_resource {
            if pattern.ends_with("/*") {
                let prefix = &pattern[..pattern.len() - 2];
                if path.starts_with(prefix) {
                    matches.push((prefix, amount));
                }
            }
        }

        // If multiple wildcards match, use the longest (most specific) prefix
        if !matches.is_empty() {
            matches.sort_by_key(|(prefix, _)| std::cmp::Reverse(prefix.len()));
            return matches[0].1;
        }

        // Priority 3: Default pricing
        self.config.default
    }
}

impl Config {
    /// Merge another config into this one, overwriting existing values
    pub fn merge(&mut self, other: Config) {
        self.port = other.port;
        self.solana_rpc = other.solana_rpc.clone();
        self.log_level = other.log_level;
        self.pricing = other.pricing.clone();
        self.simulation_mode = other.simulation_mode;
        self.timeout_delay_ms = other.timeout_delay_ms;
    }

    /// Validate configuration values
    pub fn validate(&self) -> Result<()> {
        // Validate port range
        if !(1024..=65535).contains(&self.port) {
            anyhow::bail!(
                "Invalid port: {}. Port must be between 1024 and 65535.\nFix: Set port to a value in the valid range, e.g., 8402",
                self.port
            );
        }

        // Validate Solana RPC URL format
        if !self.solana_rpc.starts_with("http://") && !self.solana_rpc.starts_with("https://") {
            anyhow::bail!(
                "Invalid Solana RPC URL: {}. URL must start with http:// or https://.\nFix: Use a valid URL, e.g., https://api.devnet.solana.com",
                self.solana_rpc
            );
        }

        // Log level validation is now compile-time enforced by the LogLevel enum
        // No runtime validation needed

        // Validate pricing configuration
        self.pricing.validate()?;

        // Validate timeout delay (100ms to 60s)
        if self.timeout_delay_ms < 100 || self.timeout_delay_ms > 60000 {
            anyhow::bail!(
                "Invalid timeout delay: {} ms. Must be between 100ms and 60000ms (1 minute).\n\
                Fix: Set timeout_delay_ms to a reasonable value between 100 and 60000",
                self.timeout_delay_ms
            );
        }

        Ok(())
    }
}

/// Get the global config directory (~/.x402dev/)
fn get_config_dir() -> Result<PathBuf> {
    directories::BaseDirs::new()
        .map(|dirs| dirs.home_dir().join(".x402dev"))
        .context("Could not determine home directory")
}

/// Load global config from ~/.x402dev/config.yaml
fn load_global_config() -> Result<Option<Config>> {
    let config_dir = get_config_dir()?;
    let config_path = config_dir.join("config.yaml");

    if !config_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read global config file: {:?}", config_path))?;

    let config: Config = serde_yaml::from_str(&content).with_context(|| {
        format!(
            "Failed to parse global config file: {:?}\nFix: Ensure the YAML syntax is valid",
            config_path
        )
    })?;

    Ok(Some(config))
}

/// Load project config from ./.x402dev.yaml
fn load_project_config() -> Result<Option<Config>> {
    let config_path = PathBuf::from(".x402dev.yaml");

    if !config_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read project config file: {:?}", config_path))?;

    let config: Config = serde_yaml::from_str(&content).with_context(|| {
        format!(
            "Failed to parse project config file: {:?}\nFix: Ensure the YAML syntax is valid",
            config_path
        )
    })?;

    Ok(Some(config))
}

/// CLI override options
#[derive(Debug, Default)]
pub struct CliOverrides {
    pub port: Option<u16>,
    pub solana_rpc: Option<String>,
    pub log_level: Option<LogLevel>,
    pub pricing: Option<f64>,
}

impl Config {
    /// Merge environment variables into config
    pub fn merge_env(&mut self) -> Result<()> {
        if let Ok(port_str) = env::var("X402_DEV_PORT") {
            self.port = port_str
                .parse()
                .context("Invalid X402_DEV_PORT value. Must be a valid port number.")?;
        }

        if let Ok(rpc) = env::var("X402_DEV_SOLANA_RPC") {
            self.solana_rpc = rpc;
        }

        if let Ok(level) = env::var("X402_DEV_LOG_LEVEL") {
            self.log_level = level.parse().map_err(|e: String| anyhow::anyhow!(e))?;
        }

        Ok(())
    }

    /// Merge CLI overrides into config
    pub fn merge_cli(&mut self, cli: &CliOverrides) {
        if let Some(port) = cli.port {
            self.port = port;
        }
        if let Some(ref rpc) = cli.solana_rpc {
            self.solana_rpc = rpc.clone();
        }
        if let Some(level) = cli.log_level {
            self.log_level = level;
        }
        if let Some(pricing) = cli.pricing {
            self.pricing.default = pricing;
        }
    }
}

/// Load merged configuration with priority: CLI > ENV > project > global > defaults
pub fn load_merged_config(cli_overrides: Option<&CliOverrides>) -> Result<Config> {
    // Step 1: Start with defaults
    let mut config = Config::default();

    // Step 2: Apply global config (~/.x402dev/config.yaml)
    if let Some(global) = load_global_config()? {
        config.merge(global);
    }

    // Step 3: Apply project config (./.x402dev.yaml)
    if let Some(project) = load_project_config()? {
        config.merge(project);
    }

    // Step 4: Apply environment variables
    config.merge_env()?;

    // Step 5: Apply CLI flags (highest priority)
    if let Some(cli) = cli_overrides {
        config.merge_cli(cli);
    }

    // Step 6: Validate final config
    config.validate()?;

    Ok(config)
}

/// Configuration source tracking for display purposes
#[derive(Debug, Clone)]
pub struct ConfigWithSources {
    pub config: Config,
    pub port_source: String,
    pub solana_rpc_source: String,
    pub log_level_source: String,
    /// Reserved for Epic 2: Will be displayed in config show command
    /// when per-resource pricing tracking is implemented
    #[allow(dead_code)]
    pub pricing_source: String,
}

/// Load merged configuration with source tracking
pub fn load_merged_config_with_sources(
    cli_overrides: Option<&CliOverrides>,
) -> Result<ConfigWithSources> {
    let defaults = Config::default();
    let mut config = defaults.clone();

    let mut port_source = "default".to_string();
    let mut solana_rpc_source = "default".to_string();
    let mut log_level_source = "default".to_string();
    let mut pricing_source = "default".to_string();

    // Global config
    if let Some(global) = load_global_config()? {
        if global.port != defaults.port {
            config.port = global.port;
            port_source = "global (~/.x402dev/config.yaml)".to_string();
        }
        if global.solana_rpc != defaults.solana_rpc {
            config.solana_rpc = global.solana_rpc.clone();
            solana_rpc_source = "global (~/.x402dev/config.yaml)".to_string();
        }
        if global.log_level != defaults.log_level {
            config.log_level = global.log_level;
            log_level_source = "global (~/.x402dev/config.yaml)".to_string();
        }
    }

    // Project config
    if let Some(project) = load_project_config()? {
        if project.port != defaults.port {
            config.port = project.port;
            port_source = "project (.x402dev.yaml)".to_string();
        }
        if project.solana_rpc != defaults.solana_rpc {
            config.solana_rpc = project.solana_rpc.clone();
            solana_rpc_source = "project (.x402dev.yaml)".to_string();
        }
        if project.log_level != defaults.log_level {
            config.log_level = project.log_level;
            log_level_source = "project (.x402dev.yaml)".to_string();
        }
    }

    // Environment variables
    if let Ok(port_str) = env::var("X402_DEV_PORT") {
        config.port = port_str
            .parse()
            .context("Invalid X402_DEV_PORT value. Must be a valid port number.")?;
        port_source = "environment (X402_DEV_PORT)".to_string();
    }
    if let Ok(rpc) = env::var("X402_DEV_SOLANA_RPC") {
        config.solana_rpc = rpc;
        solana_rpc_source = "environment (X402_DEV_SOLANA_RPC)".to_string();
    }
    if let Ok(level) = env::var("X402_DEV_LOG_LEVEL") {
        config.log_level = level.parse().map_err(|e: String| anyhow::anyhow!(e))?;
        log_level_source = "environment (X402_DEV_LOG_LEVEL)".to_string();
    }

    // CLI overrides
    if let Some(cli) = cli_overrides {
        if let Some(port) = cli.port {
            config.port = port;
            port_source = "CLI flag (--port)".to_string();
        }
        if let Some(ref rpc) = cli.solana_rpc {
            config.solana_rpc = rpc.clone();
            solana_rpc_source = "CLI flag (--solana-rpc)".to_string();
        }
        if let Some(level) = cli.log_level {
            config.log_level = level;
            log_level_source = "CLI flag (--log-level)".to_string();
        }
        if let Some(pricing) = cli.pricing {
            config.pricing.default = pricing;
            pricing_source = "CLI flag (--pricing)".to_string();
        }
    }

    // Validate
    config.validate()?;

    Ok(ConfigWithSources {
        config,
        port_source,
        solana_rpc_source,
        log_level_source,
        pricing_source,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pricing_config_validation() {
        // Valid config
        let config = PricingConfig {
            default: 0.01,
            per_resource: HashMap::new(),
        };
        assert!(config.validate().is_ok());

        // Invalid negative default
        let config = PricingConfig {
            default: -0.01,
            per_resource: HashMap::new(),
        };
        assert!(config.validate().is_err());

        // Invalid too high default
        let config = PricingConfig {
            default: 101.0,
            per_resource: HashMap::new(),
        };
        assert!(config.validate().is_err());

        // Invalid per-resource pricing
        let mut per_resource = HashMap::new();
        per_resource.insert("/api/data".to_string(), -0.05);
        let config = PricingConfig {
            default: 0.01,
            per_resource,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_pricing_matcher_exact_match() {
        let mut per_resource = HashMap::new();
        per_resource.insert("/api/data".to_string(), 0.05);
        per_resource.insert("/api/premium".to_string(), 0.10);

        let config = PricingConfig {
            default: 0.01,
            per_resource,
        };
        let matcher = PricingMatcher::new(config);

        // Exact matches
        assert_eq!(matcher.get_price_for_path("/api/data"), 0.05);
        assert_eq!(matcher.get_price_for_path("/api/premium"), 0.10);

        // No match - should use default
        assert_eq!(matcher.get_price_for_path("/random"), 0.01);
    }

    #[test]
    fn test_pricing_matcher_prefix_match() {
        let mut per_resource = HashMap::new();
        per_resource.insert("/api/admin/*".to_string(), 0.20);
        per_resource.insert("/api/*".to_string(), 0.03);

        let config = PricingConfig {
            default: 0.01,
            per_resource,
        };
        let matcher = PricingMatcher::new(config);

        // Prefix matches - should use longest matching prefix
        assert_eq!(matcher.get_price_for_path("/api/admin/users"), 0.20);
        assert_eq!(matcher.get_price_for_path("/api/admin/settings"), 0.20);
        assert_eq!(matcher.get_price_for_path("/api/users"), 0.03);
        assert_eq!(matcher.get_price_for_path("/api/posts"), 0.03);

        // No match
        assert_eq!(matcher.get_price_for_path("/public/status"), 0.01);
    }

    #[test]
    fn test_pricing_matcher_priority() {
        let mut per_resource = HashMap::new();
        per_resource.insert("/api/*".to_string(), 0.03);
        per_resource.insert("/api/data".to_string(), 0.05);

        let config = PricingConfig {
            default: 0.01,
            per_resource,
        };
        let matcher = PricingMatcher::new(config);

        // Exact match should take priority over prefix
        assert_eq!(matcher.get_price_for_path("/api/data"), 0.05);

        // Prefix match for other paths
        assert_eq!(matcher.get_price_for_path("/api/users"), 0.03);

        // Default for no match
        assert_eq!(matcher.get_price_for_path("/other"), 0.01);
    }

    #[test]
    fn test_pricing_matcher_default_fallback() {
        let config = PricingConfig::default();
        let matcher = PricingMatcher::new(config);

        // All paths should return default
        assert_eq!(matcher.get_price_for_path("/any/path"), 0.01);
        assert_eq!(matcher.get_price_for_path("/api/data"), 0.01);
        assert_eq!(matcher.get_price_for_path("/"), 0.01);
    }

    #[test]
    fn test_pricing_matcher_longest_prefix() {
        let mut per_resource = HashMap::new();
        per_resource.insert("/api/*".to_string(), 0.03);
        per_resource.insert("/api/admin/*".to_string(), 0.20);
        per_resource.insert("/api/admin/super/*".to_string(), 0.50);

        let config = PricingConfig {
            default: 0.01,
            per_resource,
        };
        let matcher = PricingMatcher::new(config);

        // Should use longest matching prefix
        assert_eq!(matcher.get_price_for_path("/api/users"), 0.03);
        assert_eq!(matcher.get_price_for_path("/api/admin/users"), 0.20);
        assert_eq!(matcher.get_price_for_path("/api/admin/super/users"), 0.50);
    }

    // ============================================================================
    // LogLevel Enum Tests
    // ============================================================================

    #[test]
    fn test_log_level_from_str() {
        assert_eq!("error".parse::<LogLevel>().unwrap(), LogLevel::Error);
        assert_eq!("warn".parse::<LogLevel>().unwrap(), LogLevel::Warn);
        assert_eq!("info".parse::<LogLevel>().unwrap(), LogLevel::Info);
        assert_eq!("debug".parse::<LogLevel>().unwrap(), LogLevel::Debug);
        assert_eq!("trace".parse::<LogLevel>().unwrap(), LogLevel::Trace);

        // Case insensitive
        assert_eq!("ERROR".parse::<LogLevel>().unwrap(), LogLevel::Error);
        assert_eq!("Debug".parse::<LogLevel>().unwrap(), LogLevel::Debug);

        // Invalid values
        assert!("invalid".parse::<LogLevel>().is_err());
        assert!("".parse::<LogLevel>().is_err());
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(LogLevel::Error.to_string(), "error");
        assert_eq!(LogLevel::Warn.to_string(), "warn");
        assert_eq!(LogLevel::Info.to_string(), "info");
        assert_eq!(LogLevel::Debug.to_string(), "debug");
        assert_eq!(LogLevel::Trace.to_string(), "trace");
    }

    #[test]
    fn test_log_level_default() {
        assert_eq!(LogLevel::default(), LogLevel::Info);
    }

    #[test]
    fn test_log_level_comparison() {
        // Test is_at_least method
        assert!(LogLevel::Error.is_at_least(LogLevel::Error));
        assert!(!LogLevel::Error.is_at_least(LogLevel::Warn));

        assert!(LogLevel::Debug.is_at_least(LogLevel::Info));
        assert!(LogLevel::Debug.is_at_least(LogLevel::Debug));
        assert!(!LogLevel::Debug.is_at_least(LogLevel::Trace));

        assert!(LogLevel::Trace.is_at_least(LogLevel::Error));
        assert!(LogLevel::Trace.is_at_least(LogLevel::Trace));
    }

    #[test]
    fn test_log_level_ordering() {
        // Test that more verbose levels are "greater"
        assert!(LogLevel::Trace.is_at_least(LogLevel::Error));
        assert!(LogLevel::Debug.is_at_least(LogLevel::Warn));
        assert!(LogLevel::Info.is_at_least(LogLevel::Error));
        assert!(!LogLevel::Error.is_at_least(LogLevel::Info));
    }

    #[test]
    fn test_log_level_serde() {
        // Test serialization
        let level = LogLevel::Debug;
        let yaml = serde_yaml::to_string(&level).unwrap();
        assert_eq!(yaml.trim(), "debug");

        // Test deserialization
        let yaml = "log_level: info\n";
        #[derive(Deserialize)]
        struct TestConfig {
            log_level: LogLevel,
        }
        let config: TestConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.log_level, LogLevel::Info);
    }

    // ============================================================================
    // SimulationMode Enum Tests
    // ============================================================================

    #[test]
    fn test_simulation_mode_from_str() {
        assert_eq!("success".parse::<SimulationMode>().unwrap(), SimulationMode::Success);
        assert_eq!("failure".parse::<SimulationMode>().unwrap(), SimulationMode::Failure);
        assert_eq!("fail".parse::<SimulationMode>().unwrap(), SimulationMode::Failure); // Alias
        assert_eq!("timeout".parse::<SimulationMode>().unwrap(), SimulationMode::Timeout);

        // Case insensitive
        assert_eq!("SUCCESS".parse::<SimulationMode>().unwrap(), SimulationMode::Success);
        assert_eq!("Failure".parse::<SimulationMode>().unwrap(), SimulationMode::Failure);

        // Invalid values
        assert!("invalid".parse::<SimulationMode>().is_err());
        assert!("".parse::<SimulationMode>().is_err());
    }

    #[test]
    fn test_simulation_mode_display() {
        assert_eq!(SimulationMode::Success.to_string(), "success");
        assert_eq!(SimulationMode::Failure.to_string(), "failure");
        assert_eq!(SimulationMode::Timeout.to_string(), "timeout");
    }

    #[test]
    fn test_simulation_mode_default() {
        assert_eq!(SimulationMode::default(), SimulationMode::Success);
    }

    #[test]
    fn test_simulation_mode_serde() {
        // Test serialization
        let mode = SimulationMode::Failure;
        let yaml = serde_yaml::to_string(&mode).unwrap();
        assert_eq!(yaml.trim(), "failure");

        // Test deserialization
        let yaml = "simulation_mode: timeout\n";
        #[derive(Deserialize)]
        struct TestConfig {
            simulation_mode: SimulationMode,
        }
        let config: TestConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.simulation_mode, SimulationMode::Timeout);
    }

    // ============================================================================
    // Config Integration Tests
    // ============================================================================

    #[test]
    fn test_config_with_log_level_enum() {
        let config = Config {
            port: 8402,
            solana_rpc: "https://api.devnet.solana.com".to_string(),
            log_level: LogLevel::Debug,
            pricing: PricingConfig::default(),
            simulation_mode: SimulationMode::Success,
            timeout_delay_ms: 5000,
        };

        assert_eq!(config.log_level, LogLevel::Debug);
        assert_eq!(config.log_level.to_string(), "debug");
    }

    #[test]
    fn test_config_yaml_serialization() {
        let config = Config {
            port: 8402,
            solana_rpc: "https://api.devnet.solana.com".to_string(),
            log_level: LogLevel::Info,
            pricing: PricingConfig::default(),
            simulation_mode: SimulationMode::Success,
            timeout_delay_ms: 5000,
        };

        let yaml = serde_yaml::to_string(&config).unwrap();
        assert!(yaml.contains("log_level: info"));
        assert!(yaml.contains("simulation_mode: success"));
    }

    #[test]
    fn test_config_yaml_deserialization() {
        let yaml = r#"
port: 8402
solana_rpc: "https://api.devnet.solana.com"
log_level: debug
simulation_mode: failure
timeout_delay_ms: 5000
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.log_level, LogLevel::Debug);
        assert_eq!(config.simulation_mode, SimulationMode::Failure);
    }
}
