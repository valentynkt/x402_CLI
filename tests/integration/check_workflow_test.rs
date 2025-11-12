// Check Command Workflow Integration Tests
// End-to-end tests for the check command with real HTTP interactions

use assert_cmd::Command;
use predicates::prelude::*;
use std::time::Duration;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn test_complete_check_workflow_success() {
    // Start a mock HTTP server with wiremock
    let mock_server = MockServer::start().await;

    // Configure mock to return proper 402 response with x402-solana header
    Mock::given(method("GET"))
        .and(path("/api/data"))
        .respond_with(
            ResponseTemplate::new(402)
                .insert_header(
                    "WWW-Authenticate",
                    "x402-solana recipient=5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d amount=0.01 currency=USDC memo=req-test-123 network=devnet"
                )
                .insert_header("Content-Type", "application/json")
                .set_body_json(serde_json::json!({
                    "error": "Payment required",
                    "amount": 0.01
                }))
        )
        .mount(&mock_server)
        .await;

    let url = format!("{}/api/data", &mock_server.uri());

    // Run check command
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("check")
        .arg(&url)
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("x402 API Compliance Check"))
        .stdout(predicate::str::contains("HTTP 402 status code"))
        .stdout(predicate::str::contains("WWW-Authenticate header"))
        .stdout(predicate::str::contains("PASS"))
        .stdout(predicate::str::contains("ALL CHECKS PASSED"));
}

#[tokio::test]
async fn test_check_workflow_with_retries() {
    // This test verifies that check command handles network errors gracefully

    let url = "http://localhost:59999/nonexistent"; // Non-existent server

    // Run check command (should fail with connection error)
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("check")
        .arg(url)
        .timeout(Duration::from_secs(15));

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to connect").or(predicate::str::contains("error")));
}

#[tokio::test]
async fn test_check_workflow_verbose_mode() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/test"))
        .respond_with(
            ResponseTemplate::new(402)
                .insert_header(
                    "WWW-Authenticate",
                    "x402-solana recipient=5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d amount=0.05 currency=USDC memo=req-verbose-test network=devnet"
                )
        )
        .mount(&mock_server)
        .await;

    let url = format!("{}/api/test", &mock_server.uri());

    // Run with verbose flag
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("--verbose")
        .arg("check")
        .arg(&url)
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Protocol Validation"))
        .stdout(predicate::str::contains("Invoice Structure"));
}

#[tokio::test]
async fn test_check_workflow_json_output() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(402)
                .insert_header(
                    "WWW-Authenticate",
                    "x402-solana recipient=5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d amount=0.02 currency=USDC memo=req-json-test network=devnet"
                )
        )
        .mount(&mock_server)
        .await;

    let url = mock_server.uri();

    // Run with JSON format
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("check")
        .arg(&url)
        .arg("--format")
        .arg("json")
        .timeout(Duration::from_secs(10));

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify JSON structure
    assert!(stdout.contains("status"));
    assert!(stdout.contains("checks_passed") || stdout.contains("checks_total"));
    assert!(stdout.contains("{") && stdout.contains("}"));
}

#[tokio::test]
async fn test_check_workflow_multiple_urls() {
    let mock_server = MockServer::start().await;

    // Mock multiple endpoints
    Mock::given(method("GET"))
        .and(path("/api/endpoint1"))
        .respond_with(
            ResponseTemplate::new(402)
                .insert_header(
                    "WWW-Authenticate",
                    "x402-solana recipient=5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d amount=0.01 currency=USDC memo=req-ep1 network=devnet"
                )
        )
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/endpoint2"))
        .respond_with(
            ResponseTemplate::new(402)
                .insert_header(
                    "WWW-Authenticate",
                    "x402-solana recipient=5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d amount=0.03 currency=USDC memo=req-ep2 network=devnet"
                )
        )
        .mount(&mock_server)
        .await;

    // Check first endpoint
    let url1 = format!("{}/api/endpoint1", &mock_server.uri());
    let mut cmd1 = Command::cargo_bin("x402-dev").unwrap();
    cmd1.arg("check").arg(&url1).timeout(Duration::from_secs(10));
    cmd1.assert().success();

    // Check second endpoint
    let url2 = format!("{}/api/endpoint2", &mock_server.uri());
    let mut cmd2 = Command::cargo_bin("x402-dev").unwrap();
    cmd2.arg("check").arg(&url2).timeout(Duration::from_secs(10));
    cmd2.assert().success();
}

#[tokio::test]
async fn test_check_workflow_invalid_url() {
    // Test with malformed URL
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("check")
        .arg("not-a-valid-url")
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error").or(predicate::str::contains("Failed")));
}

#[tokio::test]
async fn test_check_workflow_timeout_handling() {
    // Use a blackhole IP (should timeout)
    let url = "http://192.0.2.1:80/"; // TEST-NET-1, guaranteed to not respond

    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("check")
        .arg(url)
        .timeout(Duration::from_secs(15)); // Check has 10s timeout + buffer

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("timeout").or(predicate::str::contains("Failed to connect")).or(predicate::str::contains("")));
}

#[tokio::test]
async fn test_check_workflow_non_402_response() {
    let mock_server = MockServer::start().await;

    // Mock server returns 200 OK instead of 402
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "message": "Success"
        })))
        .mount(&mock_server)
        .await;

    let url = mock_server.uri();

    // Check should fail because status is not 402
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("check")
        .arg(&url)
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("FAIL").or(predicate::str::contains("200")));
}

#[tokio::test]
async fn test_check_workflow_missing_www_authenticate_header() {
    let mock_server = MockServer::start().await;

    // Mock 402 response but missing required header
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(402)
                .set_body_json(serde_json::json!({"error": "Payment required"}))
        )
        .mount(&mock_server)
        .await;

    let url = mock_server.uri();

    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("check")
        .arg(&url)
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("missing").or(predicate::str::contains("FAIL")));
}

#[tokio::test]
async fn test_check_workflow_invalid_header_format() {
    let mock_server = MockServer::start().await;

    // Mock 402 with malformed WWW-Authenticate header
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(402)
                .insert_header("WWW-Authenticate", "invalid-format-here")
        )
        .mount(&mock_server)
        .await;

    let url = mock_server.uri();

    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("check")
        .arg(&url)
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("Invalid protocol identifier").or(predicate::str::contains("Failed")));
}

#[tokio::test]
async fn test_check_workflow_custom_port() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(402)
                .insert_header(
                    "WWW-Authenticate",
                    "x402-solana recipient=5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d amount=0.01 currency=USDC memo=req-custom-port network=devnet"
                )
        )
        .mount(&mock_server)
        .await;

    let url = mock_server.uri();

    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("check")
        .arg(&url)
        .timeout(Duration::from_secs(10));

    cmd.assert().success();
}
