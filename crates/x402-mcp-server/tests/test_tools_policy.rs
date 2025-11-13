// Unit tests for policy tools
//
// Epic 8, Phase 3: Policy tool testing
// Tests: x402__policy_validate, x402__policy_generate_express

use serde_json::json;
use x402_core::policy::{IssueType, ResolutionSuggestion, ValidationIssue, ValidationReport};
use x402_mcp_server::{
    convert_validation_report, PolicyGenerateParams, PolicyGenerateResponse, PolicyIssue,
    PolicyValidateParams, PolicyValidateResponse,
};

#[test]
fn test_policy_validate_params_deserialization() {
    let json = json!({
        "policy_file": "/path/to/policy.yaml"
    });

    let params: PolicyValidateParams = serde_json::from_value(json).unwrap();
    assert_eq!(params.policy_file, "/path/to/policy.yaml");
}

#[test]
fn test_policy_issue_serialization() {
    let issue = PolicyIssue {
        issue_type: "error".to_string(),
        message: "Test error message".to_string(),
        details: Some("Detailed information".to_string()),
        suggestions: vec!["Fix this".to_string(), "Or fix that".to_string()],
    };

    let json = serde_json::to_value(&issue).unwrap();
    assert_eq!(json["issue_type"], "error");
    assert_eq!(json["message"], "Test error message");
    assert_eq!(json["suggestions"].as_array().unwrap().len(), 2);
}

#[test]
fn test_policy_validate_response_valid() {
    let response = PolicyValidateResponse {
        status: "valid".to_string(),
        issues: vec![],
        error_count: 0,
        warning_count: 0,
        summary: "Policy validation passed successfully".to_string(),
    };

    assert_eq!(response.status, "valid");
    assert_eq!(response.error_count, 0);
    assert_eq!(response.issues.len(), 0);
}

#[test]
fn test_policy_validate_response_with_errors() {
    let response = PolicyValidateResponse {
        status: "invalid".to_string(),
        issues: vec![PolicyIssue {
            issue_type: "error".to_string(),
            message: "Missing required field".to_string(),
            details: None,
            suggestions: vec![],
        }],
        error_count: 1,
        warning_count: 0,
        summary: "Policy validation failed: 1 errors, 0 warnings".to_string(),
    };

    assert_eq!(response.status, "invalid");
    assert_eq!(response.error_count, 1);
    assert_eq!(response.issues.len(), 1);
}

#[test]
fn test_convert_validation_report_no_issues() {
    let report = ValidationReport {
        issues: vec![],
        has_errors: false,
        has_warnings: false,
    };

    let response = convert_validation_report(report);
    assert_eq!(response.status, "valid");
    assert_eq!(response.error_count, 0);
    assert_eq!(response.warning_count, 0);
    assert_eq!(response.summary, "Policy validation passed successfully");
}

#[test]
fn test_convert_validation_report_with_errors() {
    let report = ValidationReport {
        issues: vec![
            ValidationIssue {
                issue_type: IssueType::Error,
                message: "Error 1".to_string(),
                details: None,
                suggestions: vec![],
                policy_indices: vec![],
            },
            ValidationIssue {
                issue_type: IssueType::Warning,
                message: "Warning 1".to_string(),
                details: None,
                suggestions: vec![],
                policy_indices: vec![],
            },
        ],
        has_errors: true,
        has_warnings: true,
    };

    let response = convert_validation_report(report);
    assert_eq!(response.status, "invalid");
    assert_eq!(response.error_count, 1);
    assert_eq!(response.warning_count, 1);
    assert_eq!(response.issues.len(), 2);
}

#[test]
fn test_convert_validation_report_warnings_only() {
    let report = ValidationReport {
        issues: vec![
            ValidationIssue {
                issue_type: IssueType::Warning,
                message: "Warning 1".to_string(),
                details: None,
                suggestions: vec![],
                policy_indices: vec![],
            },
            ValidationIssue {
                issue_type: IssueType::Warning,
                message: "Warning 2".to_string(),
                details: None,
                suggestions: vec![],
                policy_indices: vec![],
            },
        ],
        has_errors: false,
        has_warnings: true,
    };

    let response = convert_validation_report(report);
    assert_eq!(response.status, "warnings");
    assert_eq!(response.error_count, 0);
    assert_eq!(response.warning_count, 2);
    assert_eq!(response.summary, "Policy validation passed with 2 warnings");
}

#[test]
fn test_convert_validation_report_with_suggestions() {
    let report = ValidationReport {
        issues: vec![ValidationIssue {
            issue_type: IssueType::Error,
            message: "Invalid pricing".to_string(),
            details: Some("Pricing must be positive".to_string()),
            suggestions: vec![ResolutionSuggestion {
                description: "Use positive value".to_string(),
                action: "Set pricing: 0.001".to_string(),
            }],
            policy_indices: vec![],
        }],
        has_errors: true,
        has_warnings: false,
    };

    let response = convert_validation_report(report);
    assert_eq!(response.issues[0].suggestions.len(), 1);
    assert!(response.issues[0].suggestions[0].contains("Use positive value"));
}

#[test]
fn test_policy_generate_params_deserialization() {
    let json = json!({
        "policy_file": "/path/to/policy.yaml",
        "framework": "express"
    });

    let params: PolicyGenerateParams = serde_json::from_value(json).unwrap();
    assert_eq!(params.policy_file, "/path/to/policy.yaml");
    assert_eq!(params.framework, "express");
    assert_eq!(params.output, None);
}

#[test]
fn test_policy_generate_params_with_output() {
    let json = json!({
        "policy_file": "/path/to/policy.yaml",
        "framework": "express",
        "output": "/path/to/middleware.js"
    });

    let params: PolicyGenerateParams = serde_json::from_value(json).unwrap();
    assert_eq!(params.output, Some("/path/to/middleware.js".to_string()));
}

#[test]
fn test_policy_generate_response_with_code() {
    let response = PolicyGenerateResponse {
        status: "success".to_string(),
        code: Some("const middleware = () => {};".to_string()),
        output_file: None,
        policy_count: 3,
        summary: "Generated Express middleware for 3 policies".to_string(),
    };

    let json = serde_json::to_value(&response).unwrap();
    assert!(json["code"].is_string());
    assert!(json["output_file"].is_null());
}

#[test]
fn test_policy_generate_response_with_file() {
    let response = PolicyGenerateResponse {
        status: "success".to_string(),
        code: None,
        output_file: Some("/path/to/output.js".to_string()),
        policy_count: 5,
        summary: "Generated Express middleware for 5 policies".to_string(),
    };

    let json = serde_json::to_value(&response).unwrap();
    assert!(json["code"].is_null());
    assert_eq!(json["output_file"], "/path/to/output.js");
    assert_eq!(json["policy_count"], 5);
}
