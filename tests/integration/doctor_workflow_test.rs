// Doctor Command Workflow Integration Tests
// End-to-end tests for the doctor command diagnostic workflows

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::net::TcpListener;
use std::time::Duration;
use tempfile::TempDir;

#[tokio::test]
async fn test_complete_doctor_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // Create a basic project structure
    let config_path = temp_dir.path().join(".x402dev.yaml");
    let config = r#"
port: 3402
solana_rpc: "https://api.devnet.solana.com"
log_level: info
pricing:
  default: 0.01
simulation_mode: success
"#;
    fs::write(&config_path, config).unwrap();

    // Create package.json with x402 ecosystem packages
    let package_json = temp_dir.path().join("package.json");
    let package_content = r#"{
  "name": "test-project",
  "version": "1.0.0",
  "dependencies": {
    "@corbits/sdk": "^1.0.0",
    "@payai/core": "^1.0.0",
    "@cdp/sdk": "^1.0.0"
  }
}"#;
    fs::write(&package_json, package_content).unwrap();

    // Run doctor command
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("doctor")
        .current_dir(temp_dir.path())
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("x402-dev System Diagnostics"))
        .stdout(predicate::str::contains("Environment:"))
        .stdout(predicate::str::contains("Configuration:"))
        .stdout(predicate::str::contains("x402 Ecosystem:"))
        .stdout(predicate::str::contains("Config file:"))
        .stdout(predicate::str::contains("Corbits SDK"))
        .stdout(predicate::str::contains("PayAI packages"))
        .stdout(predicate::str::contains("CDP SDK"));
}

#[tokio::test]
async fn test_doctor_workflow_fix_and_rerun() {
    let temp_dir = TempDir::new().unwrap();

    // First run: no config (should warn)
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("doctor")
        .current_dir(temp_dir.path())
        .timeout(Duration::from_secs(10));

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should warn about missing config
    assert!(stdout.contains("Not found") || stdout.contains("Warning") || stdout.contains("⚠️"));

    // Fix: Create config file
    let config_path = temp_dir.path().join(".x402dev.yaml");
    let config = r#"
port: 3402
solana_rpc: "https://api.devnet.solana.com"
log_level: info
pricing:
  default: 0.01
simulation_mode: success
"#;
    fs::write(&config_path, config).unwrap();

    // Second run: should detect config
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("doctor")
        .current_dir(temp_dir.path())
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(".x402dev.yaml"));
}

#[tokio::test]
async fn test_doctor_workflow_missing_config() {
    let temp_dir = TempDir::new().unwrap();

    // No config file exists
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("doctor")
        .current_dir(temp_dir.path())
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .success() // Doctor doesn't fail, just reports
        .stdout(predicate::str::contains("Config file:"))
        .stdout(predicate::str::contains("Not found").or(predicate::str::contains("Warning")))
        .stdout(predicate::str::contains("Suggestions:").or(predicate::str::contains("x402-dev init")));
}

#[tokio::test]
async fn test_doctor_workflow_port_conflict() {
    let temp_dir = TempDir::new().unwrap();

    // Bind a listener to create port conflict
    let _listener = TcpListener::bind("127.0.0.1:3402").expect("Failed to bind to port 3402");

    // Create config with that port
    let config_path = temp_dir.path().join(".x402dev.yaml");
    let config = r#"
port: 3402
solana_rpc: "https://api.devnet.solana.com"
log_level: info
pricing:
  default: 0.01
simulation_mode: success
"#;
    fs::write(&config_path, config).unwrap();

    // Run doctor
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("doctor")
        .current_dir(temp_dir.path())
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Port 3402"))
        .stdout(predicate::str::contains("In use").or(predicate::str::contains("Warning")));

    // Listener dropped, port freed
}

#[tokio::test]
async fn test_doctor_workflow_json_output() {
    let temp_dir = TempDir::new().unwrap();

    // Create minimal config
    let config_path = temp_dir.path().join(".x402dev.yaml");
    let config = r#"
port: 4402
solana_rpc: "https://api.devnet.solana.com"
log_level: info
pricing:
  default: 0.01
simulation_mode: success
"#;
    fs::write(&config_path, config).unwrap();

    // Note: doctor command doesn't currently support --json flag
    // This test verifies standard output format
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("doctor")
        .current_dir(temp_dir.path())
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Diagnostics"))
        .stdout(predicate::str::contains("Environment"))
        .stdout(predicate::str::contains("Configuration"));
}

#[tokio::test]
async fn test_doctor_workflow_verbose_mode() {
    let temp_dir = TempDir::new().unwrap();

    // Run with verbose flag
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("--verbose")
        .arg("doctor")
        .current_dir(temp_dir.path())
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("x402-dev System Diagnostics"))
        .stdout(predicate::str::contains("Environment:"));
}

#[tokio::test]
async fn test_doctor_workflow_ecosystem_detection() {
    let temp_dir = TempDir::new().unwrap();

    // Create package.json without x402 packages
    let package_json = temp_dir.path().join("package.json");
    let package_content = r#"{
  "name": "test-project",
  "version": "1.0.0",
  "dependencies": {
    "express": "^4.18.0"
  }
}"#;
    fs::write(&package_json, package_content).unwrap();

    // Run doctor
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("doctor")
        .current_dir(temp_dir.path())
        .timeout(Duration::from_secs(10));

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect missing x402 packages
    assert!(stdout.contains("Corbits SDK"));
    assert!(stdout.contains("PayAI packages"));
    assert!(stdout.contains("CDP SDK"));
    assert!(stdout.contains("Not detected") || stdout.contains("❌"));

    // Should provide installation suggestions
    assert!(stdout.contains("npm install") || stdout.contains("Suggestions"));
}

#[tokio::test]
async fn test_doctor_workflow_invalid_config() {
    let temp_dir = TempDir::new().unwrap();

    // Create invalid YAML config
    let config_path = temp_dir.path().join(".x402dev.yaml");
    let invalid_config = r#"
port: invalid_port_value
solana_rpc: 12345
this is not: valid yaml syntax [
"#;
    fs::write(&config_path, invalid_config).unwrap();

    // Run doctor
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("doctor")
        .current_dir(temp_dir.path())
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .success() // Doctor doesn't fail
        .stdout(predicate::str::contains("Config").or(predicate::str::contains("Invalid")));
}

#[tokio::test]
async fn test_doctor_workflow_with_all_checks_passing() {
    let temp_dir = TempDir::new().unwrap();

    // Create ideal setup
    let config_path = temp_dir.path().join(".x402dev.yaml");
    let config = r#"
port: 5402
solana_rpc: "https://api.devnet.solana.com"
log_level: info
pricing:
  default: 0.01
simulation_mode: success
"#;
    fs::write(&config_path, config).unwrap();

    let package_json = temp_dir.path().join("package.json");
    let package_content = r#"{
  "name": "perfect-project",
  "version": "1.0.0",
  "dependencies": {
    "@corbits/sdk": "^1.0.0",
    "@payai/core": "^1.0.0",
    "@payai/solana": "^1.0.0",
    "@cdp/sdk": "^1.0.0"
  }
}"#;
    fs::write(&package_json, package_content).unwrap();

    // Run doctor
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("doctor")
        .current_dir(temp_dir.path())
        .timeout(Duration::from_secs(10));

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should have minimal warnings
    assert!(stdout.contains("x402-dev binary"));
    assert!(stdout.contains("Config file:"));
    assert!(stdout.contains(".x402dev.yaml"));
    assert!(stdout.contains("Corbits SDK"));
    assert!(stdout.contains("Detected"));
}

#[tokio::test]
async fn test_doctor_workflow_no_package_json() {
    let temp_dir = TempDir::new().unwrap();

    // Only create config, no package.json
    let config_path = temp_dir.path().join(".x402dev.yaml");
    let config = r#"
port: 6402
solana_rpc: "https://api.devnet.solana.com"
log_level: info
pricing:
  default: 0.01
simulation_mode: success
"#;
    fs::write(&config_path, config).unwrap();

    // Run doctor
    let mut cmd = Command::cargo_bin("x402-dev").unwrap();
    cmd.arg("doctor")
        .current_dir(temp_dir.path())
        .timeout(Duration::from_secs(10));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("package.json not found").or(predicate::str::contains("Not detected")))
        .stdout(predicate::str::contains("npm init").or(predicate::str::contains("Suggestions")));
}
