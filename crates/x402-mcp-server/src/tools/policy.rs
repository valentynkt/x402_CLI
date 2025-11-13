// Policy validation tools for x402-mcp-server
//
// Epic 8, Phase 1: Policy management tools
// - x402__policy_validate: Validate policy YAML files

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use x402_core::policy::{IssueType, ValidationReport};

/// Parameters for policy validation
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolicyValidateParams {
    /// Path to policy YAML file
    pub policy_file: String,
}

/// Single validation issue
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolicyIssue {
    /// Issue type: "error", "warning", or "info"
    pub issue_type: String,

    /// Issue message
    pub message: String,

    /// Additional details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    /// Resolution suggestions
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub suggestions: Vec<String>,
}

/// Response from policy validation
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolicyValidateResponse {
    /// Validation status: "valid", "warnings", or "invalid"
    pub status: String,

    /// List of validation issues
    pub issues: Vec<PolicyIssue>,

    /// Number of errors found
    pub error_count: usize,

    /// Number of warnings found
    pub warning_count: usize,

    /// Human-readable summary
    pub summary: String,
}

/// Convert x402_core IssueType to string
fn issue_type_to_string(issue_type: &IssueType) -> String {
    match issue_type {
        IssueType::Error => "error".to_string(),
        IssueType::Warning => "warning".to_string(),
        IssueType::Info => "info".to_string(),
    }
}

/// Convert ValidationReport to PolicyValidateResponse
pub fn convert_validation_report(report: ValidationReport) -> PolicyValidateResponse {
    let issues: Vec<PolicyIssue> = report
        .issues
        .iter()
        .map(|issue| PolicyIssue {
            issue_type: issue_type_to_string(&issue.issue_type),
            message: issue.message.clone(),
            details: issue.details.clone(),
            suggestions: issue
                .suggestions
                .iter()
                .map(|s| format!("{}: {}", s.description, s.action))
                .collect(),
        })
        .collect();

    let error_count = issues.iter().filter(|i| i.issue_type == "error").count();
    let warning_count = issues.iter().filter(|i| i.issue_type == "warning").count();

    let status = if error_count > 0 {
        "invalid"
    } else if warning_count > 0 {
        "warnings"
    } else {
        "valid"
    };

    let summary = if error_count > 0 {
        format!(
            "Policy validation failed: {} errors, {} warnings",
            error_count, warning_count
        )
    } else if warning_count > 0 {
        format!("Policy validation passed with {} warnings", warning_count)
    } else {
        "Policy validation passed successfully".to_string()
    };

    PolicyValidateResponse {
        status: status.to_string(),
        issues,
        error_count,
        warning_count,
        summary,
    }
}

/// Parameters for policy code generation
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolicyGenerateParams {
    /// Path to policy YAML file
    pub policy_file: String,

    /// Target framework: "express" or "fastify"
    pub framework: String,

    /// Optional output file path (if not provided, returns code as string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

/// Response from policy code generation
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolicyGenerateResponse {
    /// Generation status: "success" or "error"
    pub status: String,

    /// Generated code (if no output file specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Output file path (if written to file)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_file: Option<String>,

    /// Number of policies processed
    pub policy_count: usize,

    /// Human-readable summary
    pub summary: String,
}
