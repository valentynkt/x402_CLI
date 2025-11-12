use anyhow::Result;
use crate::cli::ConfigArgs;
use crate::config::{load_merged_config_with_sources, CliOverrides};

/// Run the config command
pub async fn run(args: &ConfigArgs) -> Result<()> {
    match &args.command {
        crate::cli::ConfigCommands::Show => show_config(args).await,
    }
}

/// Display the merged configuration with sources
async fn show_config(args: &ConfigArgs) -> Result<()> {
    // Build CLI overrides from global flags
    let cli_overrides = CliOverrides {
        port: args.port,
        solana_rpc: args.solana_rpc.clone(),
        log_level: args.log_level,
        pricing: None, // Story 2.2: Configurable pricing (not implemented yet)
    };

    // Load merged config with source tracking
    let config_with_sources = load_merged_config_with_sources(Some(&cli_overrides))?;

    println!("x402-dev Configuration");
    println!("=====================");
    println!();
    println!("Configuration Priority:");
    println!("  CLI flags > Environment variables > Project config > Global config > Defaults");
    println!();
    println!("Current Configuration:");
    println!(
        "  port: {} (source: {})",
        config_with_sources.config.port, config_with_sources.port_source
    );
    println!(
        "  solana_rpc: {} (source: {})",
        config_with_sources.config.solana_rpc, config_with_sources.solana_rpc_source
    );
    println!(
        "  log_level: {} (source: {})",
        config_with_sources.config.log_level, config_with_sources.log_level_source
    );
    println!();
    println!("Config File Locations:");
    println!("  Global: ~/.x402dev/config.yaml");
    println!("  Project: ./.x402dev.yaml");
    println!();
    println!("Environment Variables:");
    println!("  X402_DEV_PORT");
    println!("  X402_DEV_SOLANA_RPC");
    println!("  X402_DEV_LOG_LEVEL");

    Ok(())
}
