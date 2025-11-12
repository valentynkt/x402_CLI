//! Test infrastructure helpers for x402-dev integration testing
//!
//! This module provides reusable test utilities including:
//! - HTTP mock servers using wiremock
//! - CLI command runner wrappers using assert_cmd
//! - Custom assertions for validation
//!
//! # Example Usage
//!
//! ```no_run
//! use helpers::{mock_server, cli_runner, assertions};
//!
//! #[tokio::test]
//! async fn test_check_command() {
//!     // Setup mock 402 server
//!     let server = mock_server::mock_402_server().await;
//!
//!     // Run CLI command
//!     let result = cli_runner::run_check(&server.uri());
//!
//!     // Validate output
//!     assertions::assert_contains_invoice(&result.stdout);
//! }
//! ```

pub mod mock_server;
pub mod cli_runner;
pub mod assertions;

// Re-export commonly used items for convenience
pub use mock_server::{MockServerExt, mock_402_server, mock_200_server};
pub use cli_runner::{CliRunner, CommandResult};
pub use assertions::{assert_invoice_valid, assert_diagnostic_format, assert_json_structure};
