// CLI Command Integration Tests
// Phase 1.3: End-to-end testing of CLI commands
//
// Tests user-facing commands to ensure they work correctly

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper to create a test command
fn cli() -> Command {
    Command::cargo_bin("x402-dev").unwrap()
}

/// Test: x402-dev --version shows version
#[test]
fn test_version_command() {
    cli()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("x402-dev"));
}

/// Test: x402-dev --help shows usage
#[test]
fn test_help_command() {
    cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("x402 Protocol"))
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("mock"))
        .stdout(predicate::str::contains("policy"))
        .stdout(predicate::str::contains("config"));
}

/// Test: x402-dev mock --help shows mock server help
#[test]
fn test_mock_help() {
    cli()
        .args(&["mock", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("mock server"))
        .stdout(predicate::str::contains("--port"));
}

/// Test: x402-dev config show displays configuration
#[test]
fn test_config_show() {
    cli()
        .args(&["config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Configuration"))
        .stdout(predicate::str::contains("port:"))
        .stdout(predicate::str::contains("solana_rpc:").or(predicate::str::contains("rpc:")));
}

/// Test: x402-dev config show with CLI overrides
#[test]
fn test_config_show_with_overrides() {
    cli()
        .args(&["config", "show", "--port", "9999"])
        .assert()
        .success()
        .stdout(predicate::str::contains("9999"));
}

/// Test: x402-dev policy validate with valid YAML
#[test]
fn test_policy_validate_success() {
    // Given: A valid policy file
    let temp_dir = TempDir::new().unwrap();
    let policy_path = temp_dir.path().join("policy.yaml");

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

    // When: Running validate command
    // Then: Should succeed
    cli()
        .args(&["policy", "validate", policy_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("valid"));
}

/// Test: x402-dev policy validate with invalid YAML
#[test]
fn test_policy_validate_invalid_yaml() {
    // Given: An invalid policy file
    let temp_dir = TempDir::new().unwrap();
    let policy_path = temp_dir.path().join("invalid.yaml");

    fs::write(
        &policy_path,
        r#"
policies:
  - type: unknown_type
    field: test
"#,
    )
    .unwrap();

    // When: Running validate command
    // Then: Should fail with error
    cli()
        .args(&["policy", "validate", policy_path.to_str().unwrap()])
        .assert()
        .failure();
}

/// Test: x402-dev policy validate with conflicting policies
#[test]
fn test_policy_validate_conflicts() {
    // Given: Conflicting allowlist and denylist
    let temp_dir = TempDir::new().unwrap();
    let policy_path = temp_dir.path().join("conflict.yaml");

    fs::write(
        &policy_path,
        r#"
policies:
  - type: allowlist
    field: agent_id
    values:
      - "agent-test"
  - type: denylist
    field: agent_id
    values:
      - "agent-test"
"#,
    )
    .unwrap();

    // When: Running validate command
    // Then: Should detect conflict
    cli()
        .args(&["policy", "validate", policy_path.to_str().unwrap()])
        .assert()
        .failure()
        .stdout(predicate::str::contains("CONFLICT").or(predicate::str::contains("ERROR")))
        .stderr(predicate::str::contains("Policy validation failed"));
}

/// Test: x402-dev policy generate creates Express middleware
#[test]
fn test_policy_generate_express() {
    // Given: A valid policy file
    let temp_dir = TempDir::new().unwrap();
    let policy_path = temp_dir.path().join("policy.yaml");
    let output_path = temp_dir.path().join("middleware.js");

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

    // When: Generating Express middleware
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
        .success()
        .stdout(predicate::str::contains("Generated"));

    // Then: Output file should exist and contain middleware code
    assert!(output_path.exists());
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("x402Middleware"));
    assert!(content.contains("module.exports"));
}

/// Test: x402-dev policy generate creates Fastify plugin
#[test]
fn test_policy_generate_fastify() {
    // Given: A valid policy file
    let temp_dir = TempDir::new().unwrap();
    let policy_path = temp_dir.path().join("policy.yaml");
    let output_path = temp_dir.path().join("plugin.js");

    fs::write(
        &policy_path,
        r#"
policies:
  - type: rate_limit
    max_requests: 100
    window_seconds: 3600
"#,
    )
    .unwrap();

    // When: Generating Fastify plugin
    cli()
        .args(&[
            "policy",
            "generate",
            policy_path.to_str().unwrap(),
            "--framework",
            "fastify",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Then: Output file should contain Fastify plugin code
    assert!(output_path.exists());
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("fastify-plugin"));
    assert!(content.contains("module.exports"));
}

/// Test: x402-dev policy generate without output prints to stdout
#[test]
fn test_policy_generate_stdout() {
    // Given: A valid policy file
    let temp_dir = TempDir::new().unwrap();
    let policy_path = temp_dir.path().join("policy.yaml");

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

    // When: Generating without --output
    // Then: Should print to stdout
    cli()
        .args(&[
            "policy",
            "generate",
            policy_path.to_str().unwrap(),
            "--framework",
            "express",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("x402Middleware"));
}

/// Test: Invalid command shows helpful error
#[test]
fn test_invalid_command() {
    cli().args(&["invalid-command"]).assert().failure().stderr(
        predicate::str::contains("unrecognized subcommand").or(predicate::str::contains("error")),
    );
}

/// Test: Missing required arguments shows error
#[test]
fn test_missing_required_args() {
    cli()
        .args(&["policy", "validate"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

/// Test: x402-dev version command (alternative format)
#[test]
fn test_version_subcommand() {
    cli()
        .arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("x402-dev").or(predicate::str::contains("version")));
}

/// Test: Policy file not found error
#[test]
fn test_policy_file_not_found() {
    cli()
        .args(&["policy", "validate", "/nonexistent/policy.yaml"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("I/O operation failed")
                .or(predicate::str::contains("entity not found")),
        );
}

/// Test: Generate with invalid framework
#[test]
fn test_generate_invalid_framework() {
    let temp_dir = TempDir::new().unwrap();
    let policy_path = temp_dir.path().join("policy.yaml");

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

    cli()
        .args(&[
            "policy",
            "generate",
            policy_path.to_str().unwrap(),
            "--framework",
            "invalid",
        ])
        .assert()
        .failure();
}

/// Test: Validate then generate workflow
#[test]
fn test_validate_then_generate_workflow() {
    // Given: A comprehensive policy file
    let temp_dir = TempDir::new().unwrap();
    let policy_path = temp_dir.path().join("policy.yaml");
    let output_path = temp_dir.path().join("middleware.js");

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

    // When: First validating
    cli()
        .args(&["policy", "validate", policy_path.to_str().unwrap()])
        .assert()
        .success();

    // Then: Generating should also succeed
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

    // And: Output should contain all policy types
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("allowlist"));
    assert!(content.contains("rateLimitExceeded"));
    assert!(content.contains("spendingCapExceeded"));
}

/// Test: Exit codes are correct
#[test]
fn test_exit_codes() {
    // Success case: exit code 0
    cli().arg("--help").assert().code(0);

    // Failure case: non-zero exit code
    cli()
        .args(&["policy", "validate", "/nonexistent.yaml"])
        .assert()
        .code(predicate::ne(0));
}

/// Test: Verbose flag increases output
#[test]
fn test_verbose_flag() {
    cli()
        .args(&["--verbose", "config", "show"])
        .assert()
        .success();
    // Note: Actual verbose output would depend on implementation
}

/// Test: Debug flag provides debug information
#[test]
fn test_debug_flag() {
    cli()
        .args(&["--debug", "config", "show"])
        .assert()
        .success();
    // Note: Debug output would depend on implementation
}
