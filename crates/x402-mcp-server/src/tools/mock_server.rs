// Mock server tools for x402-mcp-server
//
// Epic 8, Phase 1: Mock server management tools
// - x402__server_mock_start: Start mock payment server
// - x402__server_mock_status: Check server status
// - x402__server_mock_stop: Stop server (Phase 2)

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for starting the mock server
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MockStartParams {
    /// Server port (1024-65535)
    #[serde(default = "default_port")]
    pub port: u16,

    /// Default pricing in SOL/USDC
    #[serde(default = "default_pricing")]
    pub pricing: f64,

    /// Simulation mode: "success", "failure", or "timeout"
    #[serde(default = "default_simulation_mode")]
    pub simulation_mode: String,
}

fn default_port() -> u16 {
    3402
}
fn default_pricing() -> f64 {
    0.01
}
fn default_simulation_mode() -> String {
    "success".to_string()
}

/// Response from starting the mock server
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MockStartResponse {
    /// Server status: "started", "already_running", or "error"
    pub status: String,

    /// Server port
    pub port: u16,

    /// Process ID (if started)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,

    /// Human-readable message
    pub message: String,
}

/// Response from checking server status
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MockStatusResponse {
    /// Server status: "running" or "stopped"
    pub status: String,

    /// Process ID (if running)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,

    /// Port number (if running)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}

// Tool implementations will be added to X402McpServer impl block
