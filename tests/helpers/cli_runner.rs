//! CLI command runner utilities using assert_cmd
//!
//! Provides wrapper functions around assert_cmd::Command for easier
//! testing of the x402-dev CLI tool.
//!
//! # Examples
//!
//! ```no_run
//! use helpers::cli_runner::{run_check, run_doctor};
//!
//! #[test]
//! fn test_check_command() {
//!     let result = run_check("http://localhost:8402");
//!     assert!(result.success());
//!     assert!(result.stdout.contains("402"));
//! }
//! ```

use assert_cmd::Command;
use serde_json::Value;
use std::process::Output;

/// Result of running a CLI command
#[derive(Debug)]
pub struct CommandResult {
    /// Standard output as UTF-8 string
    pub stdout: String,
    /// Standard error as UTF-8 string
    pub stderr: String,
    /// Exit status code
    pub exit_code: i32,
    /// Raw output for advanced assertions
    pub raw_output: Output,
}

impl CommandResult {
    /// Check if the command succeeded (exit code 0)
    pub fn success(&self) -> bool {
        self.exit_code == 0
    }

    /// Check if the command failed (exit code != 0)
    pub fn failed(&self) -> bool {
        !self.success()
    }

    /// Check if the command returned warnings (exit code 2)
    pub fn has_warnings(&self) -> bool {
        self.exit_code == 2
    }

    /// Parse stdout as JSON
    pub fn json(&self) -> Result<Value, serde_json::Error> {
        serde_json::from_str(&self.stdout)
    }

    /// Check if stdout contains a string
    pub fn stdout_contains(&self, needle: &str) -> bool {
        self.stdout.contains(needle)
    }

    /// Check if stderr contains a string
    pub fn stderr_contains(&self, needle: &str) -> bool {
        self.stderr.contains(needle)
    }
}

/// Builder for CLI commands with fluent interface
pub struct CliRunner {
    command: Command,
}

impl CliRunner {
    /// Create a new CLI runner for x402-dev binary
    pub fn new() -> Self {
        Self {
            command: Command::cargo_bin("x402-dev").expect("Failed to find x402-dev binary"),
        }
    }

    /// Add an argument to the command
    pub fn arg(mut self, arg: &str) -> Self {
        self.command.arg(arg);
        self
    }

    /// Add multiple arguments to the command
    pub fn args(mut self, args: &[&str]) -> Self {
        self.command.args(args);
        self
    }

    /// Set an environment variable
    pub fn env(mut self, key: &str, value: &str) -> Self {
        self.command.env(key, value);
        self
    }

    /// Run the command and return the result
    pub fn run(mut self) -> CommandResult {
        let output = self.command
            .output()
            .expect("Failed to execute command");

        CommandResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            raw_output: output,
        }
    }

    /// Run the command and assert success
    pub fn assert_success(mut self) -> CommandResult {
        self.command.assert().success();
        self.run()
    }

    /// Run the command and assert failure
    pub fn assert_failure(mut self) -> CommandResult {
        self.command.assert().failure();
        self.run()
    }
}

impl Default for CliRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// Run the `x402-dev check <url>` command
///
/// # Arguments
///
/// * `url` - The URL to check for x402 payment requirements
///
/// # Example
///
/// ```no_run
/// let result = run_check("http://localhost:8402/data");
/// assert!(result.success());
/// ```
pub fn run_check(url: &str) -> CommandResult {
    CliRunner::new()
        .arg("check")
        .arg(url)
        .run()
}

/// Run the `x402-dev check <url>` command with JSON output
///
/// # Arguments
///
/// * `url` - The URL to check
///
/// # Example
///
/// ```no_run
/// let result = run_check_json("http://localhost:8402/data");
/// let json = result.json().unwrap();
/// assert_eq!(json["status"], "payment_required");
/// ```
pub fn run_check_json(url: &str) -> CommandResult {
    CliRunner::new()
        .arg("check")
        .arg(url)
        .arg("--json")
        .run()
}

/// Run the `x402-dev doctor` command
///
/// # Example
///
/// ```no_run
/// let result = run_doctor();
/// assert!(result.success());
/// assert!(result.stdout.contains("System Diagnostics"));
/// ```
pub fn run_doctor() -> CommandResult {
    CliRunner::new()
        .arg("doctor")
        .run()
}

/// Run the `x402-dev doctor` command with JSON output
///
/// # Example
///
/// ```no_run
/// let result = run_doctor_json();
/// let json = result.json().unwrap();
/// assert!(json.is_object());
/// ```
pub fn run_doctor_json() -> CommandResult {
    CliRunner::new()
        .arg("doctor")
        .arg("--json")
        .run()
}

/// Run the `x402-dev test <suite>` command
///
/// # Arguments
///
/// * `suite_path` - Path to the test suite YAML file
///
/// # Example
///
/// ```no_run
/// let result = run_test("tests/example-suite.yaml");
/// assert!(result.success());
/// ```
pub fn run_test(suite_path: &str) -> CommandResult {
    CliRunner::new()
        .arg("test")
        .arg(suite_path)
        .run()
}

/// Run the `x402-dev test <suite>` command with JSON output
///
/// # Arguments
///
/// * `suite_path` - Path to the test suite YAML file
///
/// # Example
///
/// ```no_run
/// let result = run_test_json("tests/example-suite.yaml");
/// let json = result.json().unwrap();
/// ```
pub fn run_test_json(suite_path: &str) -> CommandResult {
    CliRunner::new()
        .arg("test")
        .arg(suite_path)
        .arg("--json")
        .run()
}

/// Run the `x402-dev` command with no arguments (show help)
///
/// # Example
///
/// ```no_run
/// let result = run_help();
/// assert!(result.stdout.contains("Usage:"));
/// ```
pub fn run_help() -> CommandResult {
    CliRunner::new().run()
}

/// Run the `x402-dev --version` command
///
/// # Example
///
/// ```no_run
/// let result = run_version();
/// assert!(result.stdout.contains("x402-dev"));
/// ```
pub fn run_version() -> CommandResult {
    CliRunner::new()
        .arg("--version")
        .run()
}

/// Run a custom command with arbitrary arguments
///
/// # Example
///
/// ```no_run
/// let result = run_custom(&["check", "http://example.com", "--json"]);
/// ```
pub fn run_custom(args: &[&str]) -> CommandResult {
    CliRunner::new()
        .args(args)
        .run()
}

/// Assert that a command's stdout contains specific text
///
/// # Example
///
/// ```no_run
/// let result = run_check("http://localhost:8402");
/// assert_stdout_contains(&result, "Payment Required");
/// ```
pub fn assert_stdout_contains(result: &CommandResult, needle: &str) {
    assert!(
        result.stdout_contains(needle),
        "Expected stdout to contain '{}', but it didn't.\nStdout: {}",
        needle,
        result.stdout
    );
}

/// Assert that a command's stderr contains specific text
///
/// # Example
///
/// ```no_run
/// let result = run_check("http://invalid-url");
/// assert_stderr_contains(&result, "error");
/// ```
pub fn assert_stderr_contains(result: &CommandResult, needle: &str) {
    assert!(
        result.stderr_contains(needle),
        "Expected stderr to contain '{}', but it didn't.\nStderr: {}",
        needle,
        result.stderr
    );
}

/// Assert that a command exited with a specific code
///
/// Exit codes:
/// - 0: Success
/// - 1: Failure/Error
/// - 2: Warnings
///
/// # Example
///
/// ```no_run
/// let result = run_check("http://localhost:8402");
/// assert_exit_code(&result, 0);
/// ```
pub fn assert_exit_code(result: &CommandResult, expected: i32) {
    assert_eq!(
        result.exit_code, expected,
        "Expected exit code {}, but got {}\nStdout: {}\nStderr: {}",
        expected, result.exit_code, result.stdout, result.stderr
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_result_success() {
        let result = CommandResult {
            stdout: "test".to_string(),
            stderr: "".to_string(),
            exit_code: 0,
            raw_output: std::process::Command::new("echo")
                .arg("test")
                .output()
                .unwrap(),
        };

        assert!(result.success());
        assert!(!result.failed());
        assert!(!result.has_warnings());
    }

    #[test]
    fn test_command_result_warnings() {
        let result = CommandResult {
            stdout: "".to_string(),
            stderr: "warning".to_string(),
            exit_code: 2,
            raw_output: std::process::Command::new("echo")
                .output()
                .unwrap(),
        };

        assert!(!result.success());
        assert!(result.has_warnings());
    }

    #[test]
    fn test_stdout_contains() {
        let result = CommandResult {
            stdout: "Hello World".to_string(),
            stderr: "".to_string(),
            exit_code: 0,
            raw_output: std::process::Command::new("echo")
                .output()
                .unwrap(),
        };

        assert!(result.stdout_contains("Hello"));
        assert!(!result.stdout_contains("Goodbye"));
    }
}
