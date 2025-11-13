// x402-mcp-server library
//
// Epic 8: MCP Server Integration for x402-dev
// Exposes server and tools for testing

pub mod server;
pub mod tools;
pub mod types;
pub mod utils;

// Re-export main server type for testing
pub use server::X402McpServer;

// Re-export tool types for testing
pub use tools::{
    mock_server::{MockStartParams, MockStartResponse, MockStatusResponse},
    policy::{
        convert_validation_report, PolicyGenerateParams, PolicyGenerateResponse, PolicyIssue,
        PolicyValidateParams, PolicyValidateResponse,
    },
    testing::{
        convert_suite_result, CheckComplianceParams, ComplianceCheckResponse, TestResultItem,
        TestSuiteParams, TestSuiteResponse,
    },
};
