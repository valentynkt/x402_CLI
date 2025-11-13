// Testing tools for x402-mcp-server
//
// Epic 8, Phase 2: Testing workflow tools
// - x402__testing_run_suite: Execute YAML test suites
// - x402__testing_check_compliance: Validate 402 endpoint compliance

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use x402_core::testing::SuiteResult;

/// Parameters for running a test suite
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TestSuiteParams {
    /// Path to YAML test suite file
    pub suite: String,

    /// Output JSON format (default: false)
    #[serde(default)]
    pub json: bool,

    /// Quiet mode - minimal output (default: false)
    #[serde(default)]
    pub quiet: bool,

    /// Generate JUnit XML report (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub junit: Option<String>,
}

/// Single test result
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TestResultItem {
    /// Test name
    pub name: String,

    /// Test URL
    pub url: String,

    /// HTTP method
    pub method: String,

    /// Test passed or failed
    pub passed: bool,

    /// Test duration in milliseconds
    pub duration_ms: u64,

    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Test suite execution response
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TestSuiteResponse {
    /// Execution status: "passed" or "failed"
    pub status: String,

    /// Total number of tests
    pub total: usize,

    /// Number of tests passed
    pub passed: usize,

    /// Number of tests failed
    pub failed: usize,

    /// Total duration in milliseconds
    pub duration_ms: u64,

    /// Individual test results
    pub tests: Vec<TestResultItem>,

    /// Human-readable summary
    pub summary: String,
}

/// Parameters for compliance checking
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CheckComplianceParams {
    /// URL to check for 402 compliance
    pub url: String,

    /// Timeout in seconds (default: 30)
    #[serde(default = "default_timeout")]
    pub timeout: u64,

    /// Expected pricing (optional validation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_pricing: Option<f64>,
}

fn default_timeout() -> u64 {
    30
}

/// Compliance check result
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ComplianceCheckResponse {
    /// Compliance status: "compliant", "non_compliant", or "error"
    pub status: String,

    /// HTTP status code received
    pub status_code: u16,

    /// Whether WWW-Authenticate header is present
    pub has_www_authenticate: bool,

    /// Parsed invoice details (if present)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<serde_json::Value>,

    /// List of compliance issues found
    pub issues: Vec<String>,

    /// Human-readable summary
    pub summary: String,
}

/// Convert SuiteResult to TestSuiteResponse
pub fn convert_suite_result(result: SuiteResult) -> TestSuiteResponse {
    let tests: Vec<TestResultItem> = result
        .tests
        .iter()
        .map(|test| TestResultItem {
            name: test.name.clone(),
            url: test.url.clone(),
            method: test.method.clone(),
            passed: test.passed,
            duration_ms: test.duration.as_millis() as u64,
            error: test.error.clone(),
        })
        .collect();

    let status = if result.failed == 0 {
        "passed"
    } else {
        "failed"
    };

    let summary = format!(
        "{} of {} tests passed in {}ms",
        result.passed,
        result.total,
        result.duration.as_millis()
    );

    TestSuiteResponse {
        status: status.to_string(),
        total: result.total,
        passed: result.passed,
        failed: result.failed,
        duration_ms: result.duration.as_millis() as u64,
        tests,
        summary,
    }
}
