// Integration tests for X402 MCP Server
//
// Epic 8, Phase 3: Test infrastructure
// Tests the ServerHandler implementation and tool routing

use rmcp::{handler::server::ServerHandler, model::ProtocolVersion};
use x402_mcp_server::server::X402McpServer;

#[test]
fn test_server_info() {
    let server = X402McpServer::new();
    let info = server.get_info();

    // Verify server has proper identification
    assert_eq!(info.protocol_version, ProtocolVersion::V_2024_11_05);
    assert!(info.instructions.is_some());
    assert!(info.instructions.unwrap().contains("x402-dev"));
}

#[test]
fn test_server_capabilities() {
    let server = X402McpServer::new();
    let info = server.get_info();

    // Verify that tools capability is enabled
    assert!(info.capabilities.tools.is_some());
}

#[test]
fn test_protocol_version() {
    let server = X402McpServer::new();
    let info = server.get_info();

    // Verify V_2024_11_05 protocol version
    assert_eq!(info.protocol_version, ProtocolVersion::V_2024_11_05);
}
