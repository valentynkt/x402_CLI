// Unit tests for testing workflow tools
//
// Epic 8, Phase 3: Testing tool testing
// Tests: x402__testing_run_suite, x402__testing_check_compliance

use serde_json::json;
use std::time::Duration;
use x402_core::testing::{SuiteResult, TestResult};
use x402_mcp_server::{
    convert_suite_result, CheckComplianceParams, ComplianceCheckResponse, TestResultItem,
    TestSuiteParams, TestSuiteResponse,
};

#[test]
fn test_suite_params_deserialization() {
    let json = json!({
        "suite": "/path/to/suite.yaml"
    });

    let params: TestSuiteParams = serde_json::from_value(json).unwrap();
    assert_eq!(params.suite, "/path/to/suite.yaml");
}

#[test]
fn test_suite_response_serialization() {
    let response = TestSuiteResponse {
        status: "passed".to_string(),
        total: 10,
        passed: 10,
        failed: 0,
        duration_ms: 1500,
        tests: vec![],
        summary: "10 of 10 tests passed in 1500ms".to_string(),
    };

    let json = serde_json::to_value(&response).unwrap();
    assert_eq!(json["status"], "passed");
    assert_eq!(json["total"], 10);
    assert_eq!(json["passed"], 10);
    assert_eq!(json["failed"], 0);
}

#[test]
fn test_suite_response_with_failures() {
    let response = TestSuiteResponse {
        status: "failed".to_string(),
        total: 10,
        passed: 7,
        failed: 3,
        duration_ms: 2000,
        tests: vec![],
        summary: "7 of 10 tests passed in 2000ms".to_string(),
    };

    assert_eq!(response.status, "failed");
    assert_eq!(response.failed, 3);
}

#[test]
fn test_result_item_serialization() {
    let item = TestResultItem {
        name: "test1".to_string(),
        url: "http://example.com".to_string(),
        method: "GET".to_string(),
        passed: true,
        duration_ms: 100,
        error: None,
    };

    let json = serde_json::to_value(&item).unwrap();
    assert_eq!(json["name"], "test1");
    assert_eq!(json["passed"], true);
    assert!(json["error"].is_null());
}

#[test]
fn test_result_item_with_error() {
    let item = TestResultItem {
        name: "test2".to_string(),
        url: "http://example.com".to_string(),
        method: "POST".to_string(),
        passed: false,
        duration_ms: 200,
        error: Some("Connection timeout".to_string()),
    };

    let json = serde_json::to_value(&item).unwrap();
    assert_eq!(json["passed"], false);
    assert_eq!(json["error"], "Connection timeout");
}

#[test]
fn test_convert_suite_result_all_passed() {
    let suite_result = SuiteResult {
        total: 3,
        passed: 3,
        failed: 0,
        duration: Duration::from_millis(500),
        tests: vec![
            TestResult {
                name: "test1".to_string(),
                url: "http://example.com/1".to_string(),
                method: "GET".to_string(),
                passed: true,
                duration: Duration::from_millis(100),
                assertions: vec![],
                error: None,
            },
            TestResult {
                name: "test2".to_string(),
                url: "http://example.com/2".to_string(),
                method: "POST".to_string(),
                passed: true,
                duration: Duration::from_millis(150),
                assertions: vec![],
                error: None,
            },
            TestResult {
                name: "test3".to_string(),
                url: "http://example.com/3".to_string(),
                method: "GET".to_string(),
                passed: true,
                duration: Duration::from_millis(250),
                assertions: vec![],
                error: None,
            },
        ],
    };

    let response = convert_suite_result(suite_result);
    assert_eq!(response.status, "passed");
    assert_eq!(response.total, 3);
    assert_eq!(response.passed, 3);
    assert_eq!(response.failed, 0);
    assert_eq!(response.tests.len(), 3);
    assert_eq!(response.duration_ms, 500);
}

#[test]
fn test_convert_suite_result_with_failures() {
    let suite_result = SuiteResult {
        total: 2,
        passed: 1,
        failed: 1,
        duration: Duration::from_millis(300),
        tests: vec![
            TestResult {
                name: "test1".to_string(),
                url: "http://example.com/1".to_string(),
                method: "GET".to_string(),
                passed: true,
                duration: Duration::from_millis(100),
                assertions: vec![],
                error: None,
            },
            TestResult {
                name: "test2".to_string(),
                url: "http://example.com/2".to_string(),
                method: "POST".to_string(),
                passed: false,
                duration: Duration::from_millis(200),
                assertions: vec![],
                error: Some("Failed assertion".to_string()),
            },
        ],
    };

    let response = convert_suite_result(suite_result);
    assert_eq!(response.status, "failed");
    assert_eq!(response.failed, 1);
    assert_eq!(
        response.tests[1].error,
        Some("Failed assertion".to_string())
    );
}

#[test]
fn test_compliance_params_deserialization() {
    let json = json!({
        "url": "http://localhost:3000/api/test",
        "timeout": 10,
        "expected_pricing": 0.001
    });

    let params: CheckComplianceParams = serde_json::from_value(json).unwrap();
    assert_eq!(params.url, "http://localhost:3000/api/test");
    assert_eq!(params.timeout, 10);
    assert_eq!(params.expected_pricing, Some(0.001));
}

#[test]
fn test_compliance_params_default_timeout() {
    let json = json!({
        "url": "http://localhost:3000/api/test"
    });

    let params: CheckComplianceParams = serde_json::from_value(json).unwrap();
    assert_eq!(params.timeout, 30); // Default timeout
    assert_eq!(params.expected_pricing, None);
}

#[test]
fn test_compliance_response_compliant() {
    let response = ComplianceCheckResponse {
        status: "compliant".to_string(),
        status_code: 402,
        has_www_authenticate: true,
        invoice: Some(json!({"amount": 0.001})),
        issues: vec![],
        summary: "Endpoint is 402 compliant".to_string(),
    };

    let json = serde_json::to_value(&response).unwrap();
    assert_eq!(json["status"], "compliant");
    assert_eq!(json["status_code"], 402);
    assert_eq!(json["has_www_authenticate"], true);
    assert_eq!(json["issues"].as_array().unwrap().len(), 0);
}

#[test]
fn test_compliance_response_non_compliant() {
    let response = ComplianceCheckResponse {
        status: "non_compliant".to_string(),
        status_code: 200,
        has_www_authenticate: false,
        invoice: None,
        issues: vec![
            "Expected status 402, got 200".to_string(),
            "Missing WWW-Authenticate header".to_string(),
        ],
        summary: "Endpoint failed compliance check".to_string(),
    };

    assert_eq!(response.status, "non_compliant");
    assert_eq!(response.issues.len(), 2);
    assert!(!response.has_www_authenticate);
}

#[test]
fn test_compliance_response_error() {
    let response = ComplianceCheckResponse {
        status: "error".to_string(),
        status_code: 0,
        has_www_authenticate: false,
        invoice: None,
        issues: vec!["Connection refused".to_string()],
        summary: "Failed to connect to endpoint".to_string(),
    };

    assert_eq!(response.status, "error");
    assert_eq!(response.status_code, 0);
}

#[test]
fn test_suite_response_summary_format() {
    let response = TestSuiteResponse {
        status: "passed".to_string(),
        total: 5,
        passed: 5,
        failed: 0,
        duration_ms: 1234,
        tests: vec![],
        summary: "5 of 5 tests passed in 1234ms".to_string(),
    };

    assert!(response.summary.contains("5 of 5"));
    assert!(response.summary.contains("1234ms"));
}
