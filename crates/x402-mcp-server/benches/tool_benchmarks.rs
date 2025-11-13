// Performance benchmarks for x402-mcp-server tools
//
// Epic 8, Phase 3: Performance validation
// Target: <1ms P95 latency for all tool operations

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::json;
use std::time::Duration;
use x402_core::policy::{IssueType, ResolutionSuggestion, ValidationIssue, ValidationReport};
use x402_core::testing::{SuiteResult, TestResult};
use x402_mcp_server::{
    convert_suite_result, convert_validation_report, MockStartParams, MockStartResponse,
    PolicyValidateParams, PolicyValidateResponse, TestSuiteParams, TestSuiteResponse,
};

// Benchmark parameter deserialization
fn bench_param_deserialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("param_deserialization");

    // Mock server start params
    let mock_start_json = json!({
        "port": 3000,
        "pricing": 0.001,
        "simulation_mode": "success"
    });
    group.bench_function("mock_start_params", |b| {
        b.iter(|| {
            let _params: MockStartParams =
                serde_json::from_value(black_box(mock_start_json.clone())).unwrap();
        });
    });

    // Policy validate params
    let policy_json = json!({
        "policy_file": "/path/to/policy.yaml"
    });
    group.bench_function("policy_validate_params", |b| {
        b.iter(|| {
            let _params: PolicyValidateParams =
                serde_json::from_value(black_box(policy_json.clone())).unwrap();
        });
    });

    // Test suite params
    let suite_json = json!({
        "suite": "/path/to/suite.yaml",
        "json": false,
        "quiet": false
    });
    group.bench_function("test_suite_params", |b| {
        b.iter(|| {
            let _params: TestSuiteParams =
                serde_json::from_value(black_box(suite_json.clone())).unwrap();
        });
    });

    group.finish();
}

// Benchmark response serialization
fn bench_response_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("response_serialization");

    // Mock start response
    let mock_response = MockStartResponse {
        status: "started".to_string(),
        port: 3000,
        pid: Some(12345),
        message: "Server started".to_string(),
    };
    group.bench_function("mock_start_response", |b| {
        b.iter(|| {
            let _json = serde_json::to_value(black_box(&mock_response)).unwrap();
        });
    });

    // Policy validate response
    let policy_response = PolicyValidateResponse {
        status: "valid".to_string(),
        issues: vec![],
        error_count: 0,
        warning_count: 0,
        summary: "Validation passed".to_string(),
    };
    group.bench_function("policy_validate_response", |b| {
        b.iter(|| {
            let _json = serde_json::to_value(black_box(&policy_response)).unwrap();
        });
    });

    // Test suite response
    let suite_response = TestSuiteResponse {
        status: "passed".to_string(),
        total: 10,
        passed: 10,
        failed: 0,
        duration_ms: 1000,
        tests: vec![],
        summary: "All tests passed".to_string(),
    };
    group.bench_function("test_suite_response", |b| {
        b.iter(|| {
            let _json = serde_json::to_value(black_box(&suite_response)).unwrap();
        });
    });

    group.finish();
}

// Benchmark conversion functions
fn bench_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("conversions");

    // Validation report conversion - no issues
    let empty_report = ValidationReport {
        issues: vec![],
        has_errors: false,
        has_warnings: false,
    };
    group.bench_function("convert_validation_empty", |b| {
        b.iter(|| {
            let _response = convert_validation_report(black_box(empty_report.clone()));
        });
    });

    // Validation report conversion - with issues
    let report_with_issues = ValidationReport {
        issues: vec![
            ValidationIssue {
                issue_type: IssueType::Error,
                message: "Error 1".to_string(),
                details: Some("Details".to_string()),
                suggestions: vec![ResolutionSuggestion {
                    description: "Fix".to_string(),
                    action: "Do this".to_string(),
                }],
                policy_indices: vec![0],
            },
            ValidationIssue {
                issue_type: IssueType::Warning,
                message: "Warning 1".to_string(),
                details: None,
                suggestions: vec![],
                policy_indices: vec![1],
            },
        ],
        has_errors: true,
        has_warnings: true,
    };
    group.bench_function("convert_validation_with_issues", |b| {
        b.iter(|| {
            let _response = convert_validation_report(black_box(report_with_issues.clone()));
        });
    });

    // Suite result conversion - small suite
    group.bench_function("convert_suite_small", |b| {
        b.iter(|| {
            let small_suite = SuiteResult {
                total: 5,
                passed: 5,
                failed: 0,
                duration: Duration::from_millis(500),
                tests: (0..5)
                    .map(|i| TestResult {
                        name: format!("test{}", i),
                        url: format!("http://example.com/{}", i),
                        method: "GET".to_string(),
                        passed: true,
                        duration: Duration::from_millis(100),
                        assertions: vec![],
                        error: None,
                    })
                    .collect(),
            };
            let _response = convert_suite_result(black_box(small_suite));
        });
    });

    // Suite result conversion - large suite
    group.bench_function("convert_suite_large", |b| {
        b.iter(|| {
            let large_suite = SuiteResult {
                total: 100,
                passed: 95,
                failed: 5,
                duration: Duration::from_millis(10000),
                tests: (0..100)
                    .map(|i| TestResult {
                        name: format!("test{}", i),
                        url: format!("http://example.com/{}", i),
                        method: "GET".to_string(),
                        passed: i < 95,
                        duration: Duration::from_millis(100),
                        assertions: vec![],
                        error: if i >= 95 {
                            Some("Failed".to_string())
                        } else {
                            None
                        },
                    })
                    .collect(),
            };
            let _response = convert_suite_result(black_box(large_suite));
        });
    });

    group.finish();
}

// Benchmark end-to-end operation (deser -> process -> ser)
fn bench_end_to_end(c: &mut Criterion) {
    let mut group = c.benchmark_group("end_to_end");

    // Mock server workflow
    group.bench_function("mock_server_workflow", |b| {
        b.iter(|| {
            // Deserialize params
            let json = json!({"port": 3000, "pricing": 0.001, "simulation_mode": "success"});
            let _params: MockStartParams = serde_json::from_value(json).unwrap();

            // Create response
            let response = MockStartResponse {
                status: "started".to_string(),
                port: 3000,
                pid: Some(12345),
                message: "Server started".to_string(),
            };

            // Serialize response
            let _json = serde_json::to_value(&response).unwrap();
        });
    });

    // Policy validation workflow
    group.bench_function("policy_validation_workflow", |b| {
        b.iter(|| {
            // Deserialize params
            let json = json!({"policy_file": "/path/to/policy.yaml"});
            let _params: PolicyValidateParams = serde_json::from_value(json).unwrap();

            // Process (convert report)
            let report = ValidationReport {
                issues: vec![],
                has_errors: false,
                has_warnings: false,
            };
            let response = convert_validation_report(report);

            // Serialize response
            let _json = serde_json::to_value(&response).unwrap();
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_param_deserialization,
    bench_response_serialization,
    bench_conversions,
    bench_end_to_end
);
criterion_main!(benches);
