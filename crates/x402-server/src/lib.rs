//! x402-server: HTTP mock facilitator server for x402 payment protocol testing
//!
//! This crate provides the HTTP server infrastructure for testing x402 payment flows.
//! It implements a two-phase payment protocol:
//! 1. Initial request without payment → 402 with invoice
//! 2. Retry with payment proof → Simulated verification response
//!
//! ## Architecture
//!
//! - `server`: HTTP server setup and configuration
//! - `handlers`: Request handlers implementing x402 protocol
//! - `process`: PID management and process lifecycle
//! - `lifecycle`: Start/stop/restart/status commands
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use x402_server::{start_server, MockServerConfig, PricingMatcher, InvoiceGenerator};
//! use x402_server::{Config, PricingConfig, SimulationMode};
//! use std::collections::HashMap;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = Config {
//!         port: 3402,
//!         solana_rpc: "https://api.devnet.solana.com".to_string(),
//!         log_level: "info".to_string(),
//!         pricing: PricingConfig {
//!             default: 0.01,
//!             per_resource: HashMap::new(),
//!         },
//!         simulation_mode: SimulationMode::Success,
//!         timeout_delay_ms: 5000,
//!     };
//!
//!     let server_config = MockServerConfig {
//!         port: 3402,
//!         pricing_matcher: PricingMatcher::new(config.pricing.clone()),
//!         invoice_generator: InvoiceGenerator::new(),
//!         config: config.clone(),
//!     };
//!
//!     start_server(server_config).await
//! }
//! ```

pub mod handlers;
pub mod lifecycle;
pub mod process;
pub mod server;

// Re-export main types for convenience
pub use lifecycle::{restart_server, server_status, start_server, stop_server};
pub use process::ProcessManager;
pub use server::{
    Config, Invoice, InvoiceGenerator, MockServer, MockServerConfig, PricingConfig, PricingMatcher,
    SimulationMode,
};
