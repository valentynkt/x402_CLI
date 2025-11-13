// x402-mcp-server - MCP server for x402-dev payment protocol testing
//
// Epic 8: MCP Server Integration (Phase 1, Day 1)
//
// Architecture: Rust MCP Server with direct library integration
// - rmcp procedural macros for MCP protocol handling
// - Direct function calls to x402-core/x402-server (0ms latency)
// - stdio transport for Claude Code integration
// - Type safety end-to-end with Rust types

use anyhow::Result;
use rmcp::{transport::stdio, ServiceExt};
use tracing::info;

mod server;
mod tools;
mod types;
mod utils;

use crate::server::X402McpServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging (stderr only, stdout reserved for MCP stdio)
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("x402-mcp-server v{} starting", env!("CARGO_PKG_VERSION"));

    // Create MCP server instance
    let server = X402McpServer::new();

    info!("MCP server initialized with stdio transport");

    // Start MCP server with stdio transport
    // This blocks until the server is shut down (via SIGINT/SIGTERM or client disconnect)
    let service = server.serve(stdio()).await?;

    info!("Server started, waiting for requests...");

    let quit_reason = service.waiting().await?;

    info!("MCP server shutdown: {:?}", quit_reason);
    Ok(())
}
