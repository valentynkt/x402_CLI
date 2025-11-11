use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

/// Configuration schema for x402-dev
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_solana_rpc")]
    pub solana_rpc: String,

    #[serde(default = "default_log_level")]
    pub log_level: String,
}

// Default value functions for serde
fn default_port() -> u16 {
    8402
}

fn default_solana_rpc() -> String {
    "https://api.devnet.solana.com".to_string()
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: default_port(),
            solana_rpc: default_solana_rpc(),
            log_level: default_log_level(),
        }
    }
}

impl Config {
    /// Merge another config into this one, overwriting existing values
    pub fn merge(&mut self, other: Config) {
        self.port = other.port;
        self.solana_rpc = other.solana_rpc.clone();
        self.log_level = other.log_level.clone();
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

        // Validate log level
        let valid_levels = ["error", "warn", "info", "debug", "trace"];
        if !valid_levels.contains(&self.log_level.as_str()) {
            anyhow::bail!(
                "Invalid log level: {}. Must be one of: error, warn, info, debug, trace.\nFix: Set log_level to one of the valid values, e.g., info",
                self.log_level
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
    pub log_level: Option<String>,
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
            self.log_level = level;
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
        if let Some(ref level) = cli.log_level {
            self.log_level = level.clone();
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
            config.log_level = global.log_level.clone();
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
            config.log_level = project.log_level.clone();
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
        config.log_level = level;
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
        if let Some(ref level) = cli.log_level {
            config.log_level = level.clone();
            log_level_source = "CLI flag (--log-level)".to_string();
        }
    }

    // Validate
    config.validate()?;

    Ok(ConfigWithSources {
        config,
        port_source,
        solana_rpc_source,
        log_level_source,
    })
}
