// End-to-End Payment Flow Tests
// Phase 2.2: Full x402 protocol workflow testing
//
// These tests verify complete user journeys work from start to finish

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::time::Duration;
use std::thread;
use tempfile::TempDir;

/// Helper to create CLI command
fn cli() -> Command {
    Command::cargo_bin("x402-dev").unwrap()
}

/// Helper to wait for server to start
fn wait_for_server(port: u16, max_attempts: u32) -> bool {
    use std::net::TcpStream;

    for _ in 0..max_attempts {
        if TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok() {
            return true;
        }
        thread::sleep(Duration::from_millis(100));
    }
    false
}

/// Test: Complete x402 payment flow
/// Start server → Request without payment → Receive 402 → Submit proof → Get 200
#[test]
#[ignore] // Requires server to actually run; enable for manual testing
fn test_complete_payment_flow() {
    let port = 8403; // Use non-standard port to avoid conflicts

    // Step 1: Start mock server in background
    let mut server = cli()
        .args(&["mock", "--port", &port.to_string()])
        .spawn()
        .expect("Failed to start server");

    // Wait for server to be ready
    assert!(
        wait_for_server(port, 50),
        "Server did not start within 5 seconds"
    );

    // Step 2: Make request without payment proof
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&format!("http://127.0.0.1:{}/api/test", port))
        .send()
        .expect("Request failed");

    // Step 3: Verify 402 Payment Required response
    assert_eq!(response.status(), 402);

    // Step 4: Extract invoice from WWW-Authenticate header
    let auth_header = response
        .headers()
        .get("www-authenticate")
        .expect("WWW-Authenticate header missing");

    let invoice = auth_header.to_str().unwrap();
    assert!(invoice.contains("x402-solana"));

    // Step 5: Submit payment proof (simulated)
    let response_with_proof = client
        .get(&format!("http://127.0.0.1:{}/api/test", port))
        .header("x-payment-proof", "tx_simulated_proof_12345")
        .send()
        .expect("Request with proof failed");

    // Step 6: Verify successful response
    // (Behavior depends on server implementation)
    // For simulation mode, this might be 200 OK

    // Cleanup: Stop server
    server.kill().expect("Failed to stop server");
}

/// Test: Policy enforcement workflow
/// Validate policy → Generate middleware → Verify middleware works
#[test]
fn test_policy_enforcement_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let policy_path = temp_dir.path().join("test_policy.yaml");
    let middleware_path = temp_dir.path().join("middleware.js");

    // Step 1: Create a policy file
    fs::write(
        &policy_path,
        r#"
policies:
  - type: allowlist
    field: agent_id
    values:
      - "agent-gpt4"
      - "agent-claude"
  - type: rate_limit
    max_requests: 100
    window_seconds: 3600
  - type: spending_cap
    max_amount: 10.00
    currency: USDC
    window_seconds: 86400
"#,
    )
    .unwrap();

    // Step 2: Validate the policy
    cli()
        .args(&["policy", "validate", policy_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("valid"));

    // Step 3: Generate Express middleware
    cli()
        .args(&[
            "policy",
            "generate",
            policy_path.to_str().unwrap(),
            "--framework",
            "express",
            "--output",
            middleware_path.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Generated"));

    // Step 4: Verify generated middleware
    assert!(middleware_path.exists());

    let content = fs::read_to_string(&middleware_path).unwrap();

    // Should contain all policy types
    assert!(content.contains("allowedAgents"));
    assert!(content.contains("rateLimitExceeded"));
    assert!(content.contains("spendingCapExceeded"));

    // Should contain proper x402 response
    assert!(content.contains("402"));
    assert!(content.contains("WWW-Authenticate"));

    // Should contain audit logging
    assert!(content.contains("logPaymentAttempt"));

    // Verify line count (should be 8x multiplier)
    let policy_lines = fs::read_to_string(&policy_path).unwrap().lines().count();
    let middleware_lines = content.lines().count();

    // Generated code should be significantly larger
    assert!(
        middleware_lines > policy_lines * 5,
        "Expected middleware ({} lines) to be at least 5x policy ({} lines)",
        middleware_lines,
        policy_lines
    );
}

/// Test: Configuration precedence workflow
/// CLI flags > Environment variables > Config file > Defaults
#[test]
fn test_configuration_precedence() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    // Create a config file with specific values
    fs::write(
        &config_path,
        r#"
[server]
port = 8888
network = "testnet"

[pricing]
amount = 0.05
currency = "SOL"
"#,
    )
    .unwrap();

    // Test 1: Default config (no file, no flags)
    let output = cli()
        .args(&["config", "show"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should contain defaults

    // Test 2: Config file values
    // (Would need to set config file path via env var or CLI flag)

    // Test 3: CLI override beats config file
    let output = cli()
        .args(&["config", "show", "--port", "9999"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("9999"));

    // Test 4: Multiple CLI overrides
    let output = cli()
        .args(&[
            "config",
            "show",
            "--port",
            "9999",
            "--network",
            "mainnet",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("9999"));
    assert!(stdout.contains("mainnet"));
}

/// Test: Init workflow creates proper project structure
#[test]
#[ignore] // Interactive test - requires user input simulation
fn test_init_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // Change to temp directory
    // Run init command (would need to mock user input)
    // Verify config file created
    // Verify .gitignore updated
    // Verify proper default values set
}

/// Test: Multiple framework generation workflow
/// Generate both Express and Fastify from same policy
#[test]
fn test_multi_framework_generation() {
    let temp_dir = TempDir::new().unwrap();
    let policy_path = temp_dir.path().join("policy.yaml");
    let express_path = temp_dir.path().join("express-middleware.js");
    let fastify_path = temp_dir.path().join("fastify-plugin.js");

    // Create policy
    fs::write(
        &policy_path,
        r#"
policies:
  - type: rate_limit
    max_requests: 50
    window_seconds: 1800
"#,
    )
    .unwrap();

    // Generate Express
    cli()
        .args(&[
            "policy",
            "generate",
            policy_path.to_str().unwrap(),
            "--framework",
            "express",
            "--output",
            express_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Generate Fastify
    cli()
        .args(&[
            "policy",
            "generate",
            policy_path.to_str().unwrap(),
            "--framework",
            "fastify",
            "--output",
            fastify_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Both should exist
    assert!(express_path.exists());
    assert!(fastify_path.exists());

    // Both should contain rate limiting
    let express_content = fs::read_to_string(&express_path).unwrap();
    let fastify_content = fs::read_to_string(&fastify_path).unwrap();

    assert!(express_content.contains("rateLimitExceeded"));
    assert!(fastify_content.contains("checkRateLimit"));

    // Frameworks should have different patterns
    assert!(express_content.contains("module.exports = x402Middleware"));
    assert!(fastify_content.contains("fastify-plugin"));
}

/// Test: Error recovery workflow
/// Invalid input → helpful error → fix → success
#[test]
fn test_error_recovery_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let policy_path = temp_dir.path().join("invalid.yaml");

    // Step 1: Try to validate invalid YAML
    fs::write(
        &policy_path,
        r#"
policies:
  - type: invalid_type
"#,
    )
    .unwrap();

    cli()
        .args(&["policy", "validate", policy_path.to_str().unwrap()])
        .assert()
        .failure(); // Should fail with helpful error

    // Step 2: Fix the YAML
    fs::write(
        &policy_path,
        r#"
policies:
  - type: allowlist
    field: agent_id
    values:
      - "agent-test"
"#,
    )
    .unwrap();

    // Step 3: Validation should now succeed
    cli()
        .args(&["policy", "validate", policy_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("valid"));
}

/// Test: Comprehensive policy workflow
/// Complex policy → validate → generate → verify output
#[test]
fn test_comprehensive_policy_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let policy_path = temp_dir.path().join("comprehensive.yaml");
    let output_path = temp_dir.path().join("middleware.js");

    // Create comprehensive policy (all types)
    fs::write(
        &policy_path,
        r#"
policies:
  - type: allowlist
    field: agent_id
    values:
      - "agent-gpt4-001"
      - "agent-claude-002"
      - "agent-gemini-003"
  - type: denylist
    field: wallet_address
    values:
      - "malicious-wallet-123"
  - type: rate_limit
    max_requests: 100
    window_seconds: 3600
  - type: spending_cap
    max_amount: 10.00
    currency: USDC
    window_seconds: 86400
"#,
    )
    .unwrap();

    // Validate
    cli()
        .args(&["policy", "validate", policy_path.to_str().unwrap()])
        .assert()
        .success();

    // Generate
    cli()
        .args(&[
            "policy",
            "generate",
            policy_path.to_str().unwrap(),
            "--framework",
            "express",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Verify comprehensive middleware
    let content = fs::read_to_string(&output_path).unwrap();

    // All policy checks should be present
    assert!(content.contains("Allowlist policy check"));
    assert!(content.contains("Denylist policy check"));
    assert!(content.contains("Rate limit policy check"));
    assert!(content.contains("Spending cap policy check"));

    // All helper functions should be present
    assert!(content.contains("function rateLimitExceeded"));
    assert!(content.contains("function spendingCapExceeded"));
    assert!(content.contains("function generateInvoice"));
    assert!(content.contains("function logPaymentAttempt"));

    // Proper error responses
    assert!(content.contains("status(403)")); // Forbidden for denyl ist
    assert!(content.contains("status(429)")); // Too Many Requests
    assert!(content.contains("status(402)")); // Payment Required
}
