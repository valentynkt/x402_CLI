// Common test helpers and fixtures
// Phase 3.1: Reusable test utilities to reduce duplication

use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Sample policy configurations for testing

pub fn sample_allowlist_policy() -> &'static str {
    r#"
policies:
  - type: allowlist
    field: agent_id
    values:
      - "agent-test-1"
      - "agent-test-2"
      - "agent-test-3"
"#
}

pub fn sample_denylist_policy() -> &'static str {
    r#"
policies:
  - type: denylist
    field: agent_id
    values:
      - "malicious-agent"
      - "blocked-agent"
"#
}

pub fn sample_rate_limit_policy() -> &'static str {
    r#"
policies:
  - type: rate_limit
    max_requests: 100
    window_seconds: 3600
"#
}

pub fn sample_spending_cap_policy() -> &'static str {
    r#"
policies:
  - type: spending_cap
    max_amount: 10.00
    currency: USDC
    window_seconds: 86400
"#
}

pub fn sample_comprehensive_policy() -> &'static str {
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
"#
}

pub fn sample_conflicting_policy() -> &'static str {
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
"#
}

/// Test fixtures helper
pub struct TestFixture {
    temp_dir: TempDir,
}

impl TestFixture {
    pub fn new() -> Self {
        Self {
            temp_dir: TempDir::new().expect("Failed to create temp directory"),
        }
    }

    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }

    /// Create a policy file with given content
    pub fn create_policy_file(&self, name: &str, content: &str) -> PathBuf {
        let path = self.temp_dir.path().join(name);
        fs::write(&path, content).expect("Failed to write policy file");
        path
    }

    /// Create an output path for generated code
    pub fn output_path(&self, name: &str) -> PathBuf {
        self.temp_dir.path().join(name)
    }

    /// Create a config file
    pub fn create_config_file(&self, content: &str) -> PathBuf {
        let path = self.temp_dir.path().join("config.toml");
        fs::write(&path, content).expect("Failed to write config file");
        path
    }
}

impl Default for TestFixture {
    fn default() -> Self {
        Self::new()
    }
}

/// HTTP test helpers

pub mod http {
    use std::time::Duration;
    use std::thread;
    use std::net::TcpStream;

    /// Wait for a server to be ready on the given port
    pub fn wait_for_server(port: u16, timeout_ms: u64) -> bool {
        let max_attempts = timeout_ms / 100;

        for _ in 0..max_attempts {
            if TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok() {
                return true;
            }
            thread::sleep(Duration::from_millis(100));
        }
        false
    }

    /// Create a test HTTP client
    pub fn test_client() -> reqwest::blocking::Client {
        reqwest::blocking::ClientBuilder::new()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("Failed to create HTTP client")
    }
}

/// Assertion helpers

pub mod assertions {
    use std::fs;
    use std::path::Path;

    /// Assert that generated middleware contains expected patterns
    pub fn assert_middleware_contains(path: &Path, patterns: &[&str]) {
        assert!(path.exists(), "Middleware file does not exist: {:?}", path);

        let content = fs::read_to_string(path)
            .expect(&format!("Failed to read middleware: {:?}", path));

        for pattern in patterns {
            assert!(
                content.contains(pattern),
                "Middleware missing expected pattern '{}' in file {:?}",
                pattern,
                path
            );
        }
    }

    /// Assert that a policy file is valid YAML
    pub fn assert_valid_yaml(content: &str) -> bool {
        serde_yaml::from_str::<serde_yaml::Value>(content).is_ok()
    }

    /// Assert that middleware has minimum line count
    pub fn assert_min_line_count(path: &Path, min_lines: usize) {
        let content = fs::read_to_string(path)
            .expect(&format!("Failed to read file: {:?}", path));

        let line_count = content.lines().count();

        assert!(
            line_count >= min_lines,
            "Expected at least {} lines, found {} in {:?}",
            min_lines,
            line_count,
            path
        );
    }

    /// Assert HTTP response contains x402 invoice
    pub fn assert_has_x402_invoice(response: &reqwest::blocking::Response) {
        let auth_header = response
            .headers()
            .get("www-authenticate")
            .expect("Missing WWW-Authenticate header");

        let value = auth_header.to_str().unwrap();

        assert!(value.contains("x402-solana"), "Not a valid x402 invoice");
        assert!(value.contains("recipient="), "Missing recipient");
        assert!(value.contains("amount="), "Missing amount");
        assert!(value.contains("currency="), "Missing currency");
        assert!(value.contains("memo="), "Missing memo");
        assert!(value.contains("network="), "Missing network");
    }
}

/// CLI command helpers

pub mod cli {
    use assert_cmd::Command;

    /// Create a new CLI command
    pub fn x402_cmd() -> Command {
        Command::cargo_bin("x402-dev").expect("Failed to find x402-dev binary")
    }

    /// Run a policy validate command
    pub fn validate_policy(path: &str) -> Command {
        let mut cmd = x402_cmd();
        cmd.args(&["policy", "validate", path]);
        cmd
    }

    /// Run a policy generate command
    pub fn generate_middleware(policy_path: &str, framework: &str, output_path: &str) -> Command {
        let mut cmd = x402_cmd();
        cmd.args(&[
            "policy",
            "generate",
            policy_path,
            "--framework",
            framework,
            "--output",
            output_path,
        ]);
        cmd
    }

    /// Run config show command
    pub fn show_config() -> Command {
        let mut cmd = x402_cmd();
        cmd.args(&["config", "show"]);
        cmd
    }
}

/// Policy testing helpers

pub mod policy {
    use x402_core::policy::types::{PolicyConfig, PolicyRule};

    /// Create a test allowlist policy
    pub fn test_allowlist(agents: Vec<&str>) -> PolicyConfig {
        PolicyConfig {
            policies: vec![PolicyRule::Allowlist {
                field: "agent_id".to_string(),
                values: agents.iter().map(|s| s.to_string()).collect(),
            }],
        }
    }

    /// Create a test rate limit policy
    pub fn test_rate_limit(max_requests: u32, window_seconds: u32) -> PolicyConfig {
        PolicyConfig {
            policies: vec![PolicyRule::RateLimit {
                max_requests,
                window_seconds,
            }],
        }
    }

    /// Create a test spending cap policy
    pub fn test_spending_cap(max_amount: f64, currency: &str, window_seconds: u32) -> PolicyConfig {
        PolicyConfig {
            policies: vec![PolicyRule::SpendingCap {
                max_amount,
                currency: currency.to_string(),
                window_seconds,
            }],
        }
    }

    /// Create a comprehensive test policy with all types
    pub fn test_comprehensive() -> PolicyConfig {
        PolicyConfig {
            policies: vec![
                PolicyRule::Allowlist {
                    field: "agent_id".to_string(),
                    values: vec!["agent-test".to_string()],
                },
                PolicyRule::RateLimit {
                    max_requests: 100,
                    window_seconds: 3600,
                },
                PolicyRule::SpendingCap {
                    max_amount: 10.0,
                    currency: "USDC".to_string(),
                    window_seconds: 86400,
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixture_creation() {
        let fixture = TestFixture::new();
        assert!(fixture.path().exists());
    }

    #[test]
    fn test_policy_file_creation() {
        let fixture = TestFixture::new();
        let path = fixture.create_policy_file("test.yaml", sample_allowlist_policy());

        assert!(path.exists());
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("allowlist"));
    }

    #[test]
    fn test_sample_policies_are_valid_yaml() {
        assert!(assertions::assert_valid_yaml(sample_allowlist_policy()));
        assert!(assertions::assert_valid_yaml(sample_denylist_policy()));
        assert!(assertions::assert_valid_yaml(sample_rate_limit_policy()));
        assert!(assertions::assert_valid_yaml(sample_spending_cap_policy()));
        assert!(assertions::assert_valid_yaml(sample_comprehensive_policy()));
    }
}
