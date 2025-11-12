//! Integration tests for test helper infrastructure
//!
//! These tests verify that all helper modules work correctly together.

use crate::helpers::{mock_server, cli_runner, assertions};

#[tokio::test]
async fn test_mock_402_server_integration() {
    let server = mock_server::mock_402_server().await;
    let response = reqwest::get(&server.uri()).await.unwrap();

    assert_eq!(response.status(), 402);
    assert!(response.headers().contains_key("www-authenticate"));
    assert!(response.headers().contains_key("x-402-invoice"));
}

#[tokio::test]
async fn test_mock_200_server_integration() {
    let server = mock_server::mock_200_server().await;
    let response = reqwest::get(&server.uri()).await.unwrap();

    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    assert!(body.contains("Protected content"));
}

#[tokio::test]
async fn test_mock_server_with_custom_amount() {
    let server = mock_server::mock_402_server_with_amount(5000).await;
    let response = reqwest::get(&server.uri()).await.unwrap();

    assert_eq!(response.status(), 402);
    let body = response.text().await.unwrap();
    assert!(body.contains("5000"));
}

#[test]
fn test_cli_runner_help() {
    let result = cli_runner::run_help();
    // Help should show usage information
    assert!(
        result.stdout.contains("Usage") || result.stdout.contains("x402-dev") || result.stderr.contains("Usage"),
        "Expected help output, got stdout: {}, stderr: {}",
        result.stdout,
        result.stderr
    );
}

#[test]
fn test_cli_runner_version() {
    let result = cli_runner::run_version();
    // Version should show version info
    assert!(
        result.stdout.contains("x402-dev") || result.stdout.contains("mcp-server-starter") || result.stderr.contains("x402-dev"),
        "Expected version output, got stdout: {}, stderr: {}",
        result.stdout,
        result.stderr
    );
}

#[test]
fn test_assertions_invoice_validation() {
    let valid_invoice = r#"{
        "amount": 1000,
        "currency": "SOL",
        "address": "test-address",
        "expires_at": "2024-12-31T23:59:59Z"
    }"#;

    assertions::assert_invoice_valid(valid_invoice);
}

#[test]
fn test_assertions_json_structure() {
    let json = r#"{
        "status": "ok",
        "message": "Success",
        "code": 200
    }"#;

    assertions::assert_json_structure(json, &["status", "message", "code"]);
}

#[test]
fn test_assertions_diagnostic_format() {
    let diagnostic = "System Diagnostics\n✓ Rust: 1.70.0\n✗ Missing dependency\n⚠ Warning\n";
    assertions::assert_diagnostic_format(diagnostic);
}

#[test]
fn test_assertions_exit_codes() {
    assertions::assert_exit_code_success(0);
    assertions::assert_exit_code_failure(1);
    assertions::assert_exit_code_warnings(2);
}

#[test]
fn test_assertions_colors() {
    let green = "\x1b[32mSuccess\x1b[0m";
    let yellow = "\x1b[33mWarning\x1b[0m";
    let red = "\x1b[31mError\x1b[0m";

    assertions::assert_contains_color_green(green);
    assertions::assert_contains_color_yellow(yellow);
    assertions::assert_contains_color_red(red);
}

#[test]
fn test_assertions_payment_info() {
    let output = "Payment required: 1000 lamports (0.000001 SOL)";
    assertions::assert_contains_payment_amount(output);
    assertions::assert_payment_required(output);
}

#[test]
fn test_cli_runner_builder() {
    let result = cli_runner::CliRunner::new()
        .arg("--version")
        .run();

    // Should execute without panic
    assert!(result.exit_code != -1);
}

#[tokio::test]
async fn test_mock_server_extensions() {
    use mock_server::MockServerExt;

    let server = mock_server::mock_402_server().await;
    let base_uri = server.base_uri();
    let endpoint_uri = server.endpoint_uri("/test");

    assert!(!base_uri.is_empty());
    assert!(endpoint_uri.contains("/test"));
}

#[test]
fn test_command_result_helpers() {
    let result = cli_runner::CommandResult {
        stdout: "test output".to_string(),
        stderr: "test error".to_string(),
        exit_code: 0,
        raw_output: std::process::Command::new("echo")
            .arg("test")
            .output()
            .unwrap(),
    };

    assert!(result.success());
    assert!(!result.failed());
    assert!(!result.has_warnings());
    assert!(result.stdout_contains("test"));
    assert!(result.stderr_contains("test"));
}

#[test]
fn test_json_parsing() {
    let result = cli_runner::CommandResult {
        stdout: r#"{"status":"ok","code":200}"#.to_string(),
        stderr: "".to_string(),
        exit_code: 0,
        raw_output: std::process::Command::new("echo")
            .output()
            .unwrap(),
    };

    let json = result.json().expect("Failed to parse JSON");
    assert_eq!(json["status"], "ok");
    assert_eq!(json["code"], 200);
}
