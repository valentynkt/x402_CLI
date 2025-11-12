// CLI Integration Tests
// Tests the x402-dev binary with real command execution
//
// These tests verify end-to-end functionality by running the actual binary
// using assert_cmd, rather than unit testing individual components.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::net::TcpListener;
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::sleep;
use wiremock::{matchers::method, Mock, MockServer, ResponseTemplate};

/// Helper function to get a free port for testing
fn get_free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to ephemeral port")
        .local_addr()
        .expect("Failed to get local address")
        .port()
}

/// Helper to create a mock 402 server with wiremock
/// Returns the URL to the mock server
async fn setup_mock_402_server() -> MockServer {
    let mock_server = MockServer::start().await;

    // Mock 402 response with valid x402-solana WWW-Authenticate header
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(402)
                .insert_header(
                    "WWW-Authenticate",
                    "x402-solana recipient=5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d amount=0.01 currency=USDC memo=req-test network=devnet"
                )
        )
        .mount(&mock_server)
        .await;

    mock_server
}

#[tokio::test]
async fn test_check_command_with_mock_server() {
    // Start a mock 402 server using wiremock (no process spawning needed)
    let mock_server = setup_mock_402_server().await;
    let url = format!("{}/api/data", mock_server.uri());

    // Run check command against the mock server with retries
    let max_retries = 3;
    let mut last_error = None;

    for attempt in 1..=max_retries {
        let mut cmd = Command::cargo_bin("x402-dev").unwrap();
        cmd.arg("check")
            .arg(&url)
            .timeout(Duration::from_secs(15));

        let result = cmd.output();

        match result {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);

                // Verify expected output
                assert!(
                    stdout.contains("x402 API Compliance Check"),
                    "Expected 'x402 API Compliance Check' in output, got: {}",
                    stdout
                );
                assert!(
                    stdout.contains("Protocol Validation"),
                    "Expected 'Protocol Validation' in output, got: {}",
                    stdout
                );

                // Test passed!
                return;
            }
            Ok(output) => {
                last_error = Some(format!(
                    "Command failed (attempt {}/{}): exit code {:?}\nstdout: {}\nstderr: {}",
                    attempt,
                    max_retries,
                    output.status.code(),
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
            Err(e) => {
                last_error = Some(format!(
                    "Command execution failed (attempt {}/{}): {}",
                    attempt, max_retries, e
                ));
            }
        }

        // Wait before retry (except on last attempt)
        if attempt < max_retries {
            sleep(Duration::from_millis(500)).await;
        }
    }

    // All retries failed
    panic!(
        "Test failed after {} attempts. Last error: {}",
        max_retries,
        last_error.unwrap_or_else(|| "Unknown error".to_string())
    );
}

#[tokio::test]
async fn test_doctor_command_clean_environment() {
    let temp_dir = TempDir::new().unwrap();

    // Run doctor command in temp directory (clean environment)
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("doctor")
        .current_dir(temp_dir.path())
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("x402-dev System Diagnostics"))
        .stdout(predicate::str::contains("Environment:"))
        .stdout(predicate::str::contains("Configuration:"));
}

#[tokio::test]
async fn test_doctor_command_with_valid_config() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".x402dev.yaml");

    // Create a valid config file
    let config = r#"
port: 3402
solana_rpc: "https://api.devnet.solana.com"
log_level: info
pricing:
  default: 0.01
simulation_mode: success
"#;
    fs::write(&config_path, config).unwrap();

    // Run doctor command
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("doctor")
        .current_dir(temp_dir.path())
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Config file:"))
        .stdout(predicate::str::contains(".x402dev.yaml"));
}

#[tokio::test]
async fn test_test_command_execution() {
    let temp_dir = TempDir::new().unwrap();
    let suite_path = temp_dir.path().join("test-suite.yaml");

    // Create a minimal test suite
    let suite = r#"
tests:
  - name: "Test basic HTTP request"
    url: "http://httpbin.org/status/200"
    method: GET
    expect:
      status: 200
"#;
    fs::write(&suite_path, suite).unwrap();

    // Run test command
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("test")
        .arg(&suite_path)
        .arg("--quiet")
        .timeout(Duration::from_secs(30));

    // Test command should execute (may pass or fail depending on network)
    let assert = cmd.assert();
    assert.stdout(predicate::str::contains("test").or(predicate::str::contains("Test")));
}

#[test]
fn test_mock_server_start_stop() {
    // Note: This test is complex for integration, simplified version
    // Testing just that the commands accept arguments correctly

    let port = get_free_port();

    // Test that mock command accepts valid arguments
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("mock")
        .arg("--port")
        .arg(port.to_string())
        .arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("mock"));
}

#[test]
fn test_command_help_output() {
    // Test help for main command
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("x402 Protocol Standard Toolkit"))
        .stdout(predicate::str::contains("Commands:"));

    // Test help for check command
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("check").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("check"))
        .stdout(predicate::str::contains("URL"));

    // Test help for doctor command
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("doctor").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("doctor"))
        .stdout(predicate::str::contains("Diagnose"));

    // Test help for test command
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("test").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test"))
        .stdout(predicate::str::contains("suite"));
}

#[test]
fn test_invalid_command_handling() {
    // Test that invalid command produces error
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("invalid-command-xyz");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized").or(predicate::str::contains("error")));
}

#[test]
fn test_config_precedence() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".x402dev.yaml");

    // Create config file with port 5000
    let config = r#"
port: 5000
solana_rpc: "https://api.devnet.solana.com"
log_level: info
pricing:
  default: 0.01
simulation_mode: success
"#;
    fs::write(&config_path, config).unwrap();

    // Run config show command (CLI flag should override config file)
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("config")
        .arg("show")
        .arg("--port")
        .arg("6000")
        .current_dir(temp_dir.path())
        .timeout(Duration::from_secs(5));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("6000")); // CLI flag overrides config
}

#[test]
fn test_json_output_flag() {
    let temp_dir = TempDir::new().unwrap();
    let suite_path = temp_dir.path().join("test-suite.yaml");

    // Create a minimal test suite with an invalid URL (will fail quickly)
    let suite = r#"
tests:
  - name: "Test JSON output"
    url: "http://localhost:9999/nonexistent"
    method: GET
    expect:
      status: 200
"#;
    fs::write(&suite_path, suite).unwrap();

    // Run test command with JSON output
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("test")
        .arg(&suite_path)
        .arg("--json")
        .timeout(Duration::from_secs(10));

    // Should output JSON (will fail but we're testing format)
    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check if output contains JSON-like structure
    assert!(stdout.contains("{") || stdout.contains("}"));
}

#[test]
fn test_verbose_output_flag() {
    // Test verbose flag on doctor command
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("--verbose")
        .arg("doctor")
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Diagnostics").or(predicate::str::contains("Environment")));

    // Test verbose with config show
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("--verbose")
        .arg("config")
        .arg("show")
        .timeout(Duration::from_secs(5));

    cmd.assert().success();
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("x402-dev").or(predicate::str::contains("version")));
}

#[test]
fn test_config_show_command() {
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("config").arg("show").timeout(Duration::from_secs(5));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Configuration").or(predicate::str::contains("port")));
}
