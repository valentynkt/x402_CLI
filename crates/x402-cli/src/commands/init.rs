use anyhow::{Context, Result};
use dialoguer::{Confirm, Input, Select};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

use crate::cli::InitArgs;
use crate::config::Config;

/// Configuration for YAML serialization (includes all fields)
#[derive(Serialize)]
struct ProjectConfig {
    port: u16,
    solana_rpc: String,
    log_level: String,
}

impl From<Config> for ProjectConfig {
    fn from(config: Config) -> Self {
        ProjectConfig {
            port: config.port,
            solana_rpc: config.solana_rpc,
            log_level: config.log_level,
        }
    }
}

/// Run the init command
pub async fn run(_args: &InitArgs) -> Result<()> {
    println!("x402-dev Project Initialization");
    println!("================================\n");

    // Check if .x402dev.yaml already exists
    let config_path = PathBuf::from(".x402dev.yaml");
    if config_path.exists() {
        println!("⚠️  Configuration file already exists: .x402dev.yaml\n");

        let overwrite = Confirm::new()
            .with_prompt("Do you want to overwrite it?")
            .default(false)
            .interact()?;

        if !overwrite {
            println!("\nℹ️  Initialization cancelled. Existing configuration preserved.");
            return Ok(());
        }
        println!();
    }

    // Gather configuration from user
    println!("Please provide the following configuration:\n");

    // Port configuration
    let port: u16 = Input::new()
        .with_prompt("Mock server port")
        .default(8402)
        .validate_with(|input: &u16| -> Result<(), &str> {
            if *input >= 1024 {
                Ok(())
            } else {
                Err("Port must be 1024 or higher")
            }
        })
        .interact_text()?;

    // Solana network selection
    let networks = vec!["devnet", "testnet", "mainnet-beta"];
    let network_idx = Select::new()
        .with_prompt("Solana network")
        .items(&networks)
        .default(0)
        .interact()?;

    let solana_rpc = match networks[network_idx] {
        "devnet" => "https://api.devnet.solana.com",
        "testnet" => "https://api.testnet.solana.com",
        "mainnet-beta" => "https://api.mainnet-beta.solana.com",
        _ => "https://api.devnet.solana.com",
    }
    .to_string();

    // Log level selection
    let log_levels = vec!["error", "warn", "info", "debug", "trace"];
    let log_level_idx = Select::new()
        .with_prompt("Log level")
        .items(&log_levels)
        .default(2) // "info" is at index 2
        .interact()?;

    let log_level = log_levels[log_level_idx].to_string();

    // Create configuration
    let config = Config {
        port,
        solana_rpc,
        log_level,
    };

    // Validate configuration
    config.validate().context("Configuration validation failed")?;

    // Convert to ProjectConfig for serialization
    let project_config: ProjectConfig = config.into();

    // Serialize to YAML
    let yaml = serde_yaml::to_string(&project_config)
        .context("Failed to serialize configuration to YAML")?;

    // Write to file
    fs::write(&config_path, yaml)
        .with_context(|| format!("Failed to write configuration file: {:?}", config_path))?;

    println!("\n✅ Configuration file created successfully!");
    println!("   File: .x402dev.yaml");
    println!("\n📝 Configuration:");
    println!("   Port: {}", project_config.port);
    println!("   Solana RPC: {}", project_config.solana_rpc);
    println!("   Log Level: {}", project_config.log_level);
    println!("\n💡 Next steps:");
    println!("   1. Run 'x402-dev config show' to verify configuration");
    println!("   2. Run 'x402-dev doctor' to check system health");
    println!("   3. Start developing with x402-dev!");

    Ok(())
}
