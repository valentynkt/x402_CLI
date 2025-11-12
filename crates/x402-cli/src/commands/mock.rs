use anyhow::Result;
use x402_server::{
    restart_server as server_restart, server_status as server_status_check,
    start_server as server_start, stop_server as server_stop, Config, InvoiceGenerator,
    MockServerConfig, PricingConfig, PricingMatcher,
};

use crate::cli::{MockArgs, MockSubcommand};
use crate::config::{load_merged_config, CliOverrides};

// ============================================================================
// CLI Command Handlers
// ============================================================================

/// Handle stop command
pub async fn handle_stop() -> Result<()> {
    server_stop().await
}

/// Handle status command
pub async fn handle_status() -> Result<()> {
    server_status_check().await
}

/// Handle restart command
pub async fn handle_restart(args: &MockArgs) -> Result<()> {
    let server_config = build_server_config(args)?;
    server_restart(server_config).await
}

/// Build server configuration from CLI arguments
fn build_server_config(args: &MockArgs) -> Result<MockServerConfig> {
    let port = args.port;

    // Load configuration with CLI overrides
    let cli_overrides = CliOverrides {
        port: None, // Port is handled separately
        solana_rpc: None,
        log_level: None,
        pricing: args.pricing,
    };

    let config = load_merged_config(Some(&cli_overrides))?;

    // Convert CLI config to server config
    let server_config = Config {
        port,
        solana_rpc: config.solana_rpc.clone(),
        log_level: config.log_level.to_string(),
        pricing: PricingConfig {
            default: config.pricing.default,
            per_resource: config.pricing.per_resource.clone(),
        },
        simulation_mode: match config.simulation_mode {
            crate::config::SimulationMode::Success => x402_server::SimulationMode::Success,
            crate::config::SimulationMode::Failure => x402_server::SimulationMode::Failure,
            crate::config::SimulationMode::Timeout => x402_server::SimulationMode::Timeout,
        },
        timeout_delay_ms: config.timeout_delay_ms,
    };

    // Create pricing matcher
    let pricing_matcher = PricingMatcher::new(server_config.pricing.clone());

    Ok(MockServerConfig {
        port,
        pricing_matcher,
        invoice_generator: InvoiceGenerator::new(),
        config: server_config,
    })
}

/// Main entry point for mock command
pub async fn run(args: &MockArgs) -> Result<()> {
    // Handle subcommands
    match &args.command {
        Some(MockSubcommand::Stop) => handle_stop().await,
        Some(MockSubcommand::Status) => handle_status().await,
        Some(MockSubcommand::Restart) => handle_restart(args).await,
        None => {
            let server_config = build_server_config(args)?;
            server_start(server_config).await
        }
    }
}
