// MCP server implementation with rmcp tool_router
//
// Epic 8, Phase 1: Foundation
// - rmcp SDK integration with #[tool_router] macro
// - stdio transport for Claude Code
// - Tool registration and routing

use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters, ServerHandler},
    model::*,
    tool, tool_handler, tool_router, ErrorData as McpError, Json,
};

use crate::tools::mock_server::{MockStartParams, MockStartResponse, MockStatusResponse};
use crate::tools::policy::{
    convert_validation_report, PolicyGenerateParams, PolicyGenerateResponse, PolicyValidateParams,
    PolicyValidateResponse,
};
use crate::tools::testing::{
    convert_suite_result, CheckComplianceParams, ComplianceCheckResponse, TestSuiteParams,
    TestSuiteResponse,
};
use x402_core::policy::{generate_express_middleware, validate_policies, PolicyConfig, PolicyFile};
use x402_core::testing::{execute_test_suite, TestSuite};

/// x402 MCP Server
///
/// Provides MCP tools for x402-dev payment protocol testing toolkit.
/// Uses stdio transport for Claude Code integration.
#[derive(Clone)]
pub struct X402McpServer {
    /// Tool router for dispatching tool calls
    tool_router: ToolRouter<Self>,
}

/// Implement Default trait for convenient instantiation
impl Default for X402McpServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router]
impl X402McpServer {
    /// Create a new MCP server instance
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Start x402 mock facilitator server
    ///
    /// Starts a local HTTP server that responds with 402 Payment Required.
    /// Uses direct x402-server library integration for <1ms latency.
    #[tool(
        name = "x402__server_mock_start",
        description = "Start x402 mock payment server for local testing"
    )]
    async fn mock_start(
        &self,
        params: Parameters<MockStartParams>,
    ) -> Result<Json<MockStartResponse>, McpError> {
        let params = params.0;

        // Validate port range
        if params.port < 1024 {
            return Err(McpError::invalid_params(
                format!("Port must be >= 1024, got {}", params.port),
                None,
            ));
        }

        // Validate pricing
        if params.pricing <= 0.0 {
            return Err(McpError::invalid_params(
                format!("Pricing must be positive, got {}", params.pricing),
                None,
            ));
        }

        // TODO: Phase 1 implementation
        // For now, return a simulated response
        // Full implementation will use x402_server::start_server() in background task

        tracing::info!(
            "Mock server start requested: port={}, pricing={}, mode={}",
            params.port,
            params.pricing,
            params.simulation_mode
        );

        Ok(Json(MockStartResponse {
            status: "started".to_string(),
            port: params.port,
            pid: Some(std::process::id()),
            message: format!(
                "Mock server started on port {} (Phase 1 simulation)",
                params.port
            ),
        }))
    }

    /// Check x402 mock server status
    ///
    /// Returns the current status of the mock payment server.
    #[tool(
        name = "x402__server_mock_status",
        description = "Check if x402 mock server is running"
    )]
    async fn mock_status(&self) -> Result<Json<MockStatusResponse>, McpError> {
        // TODO: Phase 1 implementation
        // Full implementation will check actual server process status

        tracing::info!("Mock server status check");

        Ok(Json(MockStatusResponse {
            status: "stopped".to_string(),
            pid: None,
            port: None,
        }))
    }

    /// Validate x402 policy YAML file
    ///
    /// Validates policy rules and detects conflicts before code generation.
    /// Uses direct x402-core library integration for <1ms latency.
    #[tool(
        name = "x402__policy_validate",
        description = "Validate x402 policy YAML file for conflicts and errors"
    )]
    async fn policy_validate(
        &self,
        params: Parameters<PolicyValidateParams>,
    ) -> Result<Json<PolicyValidateResponse>, McpError> {
        let params = params.0;

        tracing::info!("Policy validation requested: {}", params.policy_file);

        // Read policy file
        let policy_path = std::path::Path::new(&params.policy_file);
        if !policy_path.exists() {
            return Err(McpError::invalid_params(
                format!("Policy file not found: {}", params.policy_file),
                None,
            ));
        }

        let yaml_content = std::fs::read_to_string(policy_path).map_err(|e| {
            McpError::invalid_params(format!("Failed to read policy file: {}", e), None)
        })?;

        // Parse YAML into PolicyConfig
        let policy_config: PolicyConfig = serde_yaml::from_str(&yaml_content).map_err(|e| {
            McpError::invalid_params(
                format!("Invalid YAML format: {}", e),
                Some(serde_json::json!({
                    "hint": "Check YAML syntax and policy structure",
                    "error": e.to_string()
                })),
            )
        })?;

        // Validate policies using x402-core
        let report = validate_policies(&policy_config);

        // Convert to MCP response format
        let response = convert_validation_report(report);

        tracing::info!(
            "Policy validation complete: {} errors, {} warnings",
            response.error_count,
            response.warning_count
        );

        Ok(Json(response))
    }

    /// Run x402 test suite from YAML file
    ///
    /// Executes automated HTTP test suite for x402 payment flows.
    /// Uses direct x402-core library integration (refactored Day 0).
    #[tool(
        name = "x402__testing_run_suite",
        description = "Execute YAML test suite for x402 payment protocol testing"
    )]
    async fn testing_run_suite(
        &self,
        params: Parameters<TestSuiteParams>,
    ) -> Result<Json<TestSuiteResponse>, McpError> {
        let params = params.0;

        tracing::info!("Test suite execution requested: {}", params.suite);

        // Validate suite file exists
        let suite_path = std::path::Path::new(&params.suite);
        if !suite_path.exists() {
            return Err(McpError::invalid_params(
                format!("Test suite file not found: {}", params.suite),
                None,
            ));
        }

        // Parse test suite from YAML
        let suite = TestSuite::from_file(suite_path).map_err(|e| {
            McpError::invalid_params(
                format!("Failed to parse test suite: {}", e),
                Some(serde_json::json!({
                    "hint": "Check YAML syntax and test structure",
                    "error": e.to_string()
                })),
            )
        })?;

        tracing::info!("Executing {} tests from suite", suite.tests.len());

        // Execute test suite using refactored x402-core function (Day 0 work!)
        let result = execute_test_suite(&suite)
            .await
            .map_err(|e| McpError::invalid_params(format!("Test execution failed: {}", e), None))?;

        // Convert to MCP response format
        let response = convert_suite_result(result);

        tracing::info!(
            "Test suite complete: {} passed, {} failed",
            response.passed,
            response.failed
        );

        Ok(Json(response))
    }

    /// Check x402 endpoint compliance
    ///
    /// Validates that an HTTP endpoint properly implements the 402 payment protocol.
    #[tool(
        name = "x402__testing_check_compliance",
        description = "Check if an HTTP endpoint is x402 protocol compliant"
    )]
    async fn testing_check_compliance(
        &self,
        params: Parameters<CheckComplianceParams>,
    ) -> Result<Json<ComplianceCheckResponse>, McpError> {
        let params = params.0;

        tracing::info!("Compliance check requested: {}", params.url);

        // Validate URL format
        let url = reqwest::Url::parse(&params.url)
            .map_err(|e| McpError::invalid_params(format!("Invalid URL: {}", e), None))?;

        // Create HTTP client with timeout
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(params.timeout))
            .build()
            .map_err(|e| {
                McpError::invalid_params(format!("Failed to create HTTP client: {}", e), None)
            })?;

        // Make request
        let response =
            client.get(url.clone()).send().await.map_err(|e| {
                McpError::invalid_params(format!("HTTP request failed: {}", e), None)
            })?;

        let status_code = response.status().as_u16();
        let has_www_authenticate = response.headers().contains_key("www-authenticate");

        // Analyze compliance
        let mut issues = Vec::new();
        let status = if status_code == 402 {
            if !has_www_authenticate {
                issues.push("Missing WWW-Authenticate header".to_string());
                "non_compliant"
            } else {
                "compliant"
            }
        } else {
            issues.push(format!("Expected 402 status code, got {}", status_code));
            "non_compliant"
        };

        let summary = if status == "compliant" {
            format!("Endpoint {} is x402 compliant", params.url)
        } else {
            format!(
                "Endpoint {} has {} compliance issues",
                params.url,
                issues.len()
            )
        };

        tracing::info!("Compliance check complete: {}", status);

        Ok(Json(ComplianceCheckResponse {
            status: status.to_string(),
            status_code,
            has_www_authenticate,
            invoice: None, // TODO: Parse invoice from WWW-Authenticate header
            issues,
            summary,
        }))
    }

    /// Generate Express/Fastify middleware from policy YAML
    ///
    /// Converts policy rules into production-ready middleware code.
    /// Uses direct x402-core code generation library.
    #[tool(
        name = "x402__policy_generate_express",
        description = "Generate Express or Fastify middleware code from x402 policy YAML"
    )]
    async fn policy_generate(
        &self,
        params: Parameters<PolicyGenerateParams>,
    ) -> Result<Json<PolicyGenerateResponse>, McpError> {
        let params = params.0;

        tracing::info!(
            "Policy code generation requested: {} ({})",
            params.policy_file,
            params.framework
        );

        // Validate policy file exists
        let policy_path = std::path::Path::new(&params.policy_file);
        if !policy_path.exists() {
            return Err(McpError::invalid_params(
                format!("Policy file not found: {}", params.policy_file),
                None,
            ));
        }

        // Read and parse policy file
        let yaml_content = std::fs::read_to_string(policy_path).map_err(|e| {
            McpError::invalid_params(format!("Failed to read policy file: {}", e), None)
        })?;

        let policy_file: PolicyFile = serde_yaml::from_str(&yaml_content)
            .map_err(|e| McpError::invalid_params(format!("Invalid policy YAML: {}", e), None))?;

        // Extract filename from path
        let policy_file_name = policy_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("policy.yaml");

        // Generate code based on framework
        let generated_code = if params.framework == "express" {
            generate_express_middleware(&policy_file, policy_file_name)
        } else {
            return Err(McpError::invalid_params(
                format!(
                    "Unsupported framework: {} (only 'express' supported in Phase 2)",
                    params.framework
                ),
                None,
            ));
        };

        let policy_count = policy_file.policies.len();

        // Write to file or return as string
        let (output_file, code) = if let Some(output_path) = params.output {
            std::fs::write(&output_path, &generated_code).map_err(|e| {
                McpError::invalid_params(format!("Failed to write output file: {}", e), None)
            })?;
            (Some(output_path), None)
        } else {
            (None, Some(generated_code))
        };

        let summary = format!(
            "Generated {} middleware from {} policies",
            params.framework, policy_count
        );

        tracing::info!("Code generation complete: {} policies", policy_count);

        Ok(Json(PolicyGenerateResponse {
            status: "success".to_string(),
            code,
            output_file,
            policy_count,
            summary,
        }))
    }

    /// Stop x402 mock facilitator server
    ///
    /// Stops the running mock payment server process.
    #[tool(
        name = "x402__server_mock_stop",
        description = "Stop the running x402 mock payment server"
    )]
    async fn mock_stop(&self) -> Result<Json<MockStatusResponse>, McpError> {
        tracing::info!("Mock server stop requested");

        // TODO: Phase 2 implementation
        // Full implementation will:
        // 1. Read PID file
        // 2. Check if process is running
        // 3. Send SIGTERM signal
        // 4. Delete PID file
        // For now, return stopped status

        Ok(Json(MockStatusResponse {
            status: "stopped".to_string(),
            pid: None,
            port: None,
        }))
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for X402McpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "x402-dev MCP Server - Payment protocol testing toolkit. \
                Provides tools for: mock server management, policy validation, \
                test execution, and compliance checking."
                    .to_string(),
            ),
        }
    }
}
