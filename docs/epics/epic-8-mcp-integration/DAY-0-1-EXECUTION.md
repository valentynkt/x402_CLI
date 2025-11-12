# Epic 8: Day 0-1 Detailed Execution Plan

**Status:** Ready for Execution
**Timeline:** 12-13 hours (Option A - Extended Timeline)
**Goal:** Complete all refactoring blockers and establish library foundation

---

## Table of Contents

1. [Prerequisites & Environment Setup](#prerequisites--environment-setup)
2. [Day 0: Core Refactoring (10-12 hours)](#day-0-core-refactoring-10-12-hours)
3. [Day 1: Foundation & PoC (2-3 hours)](#day-1-foundation--poc-2-3-hours)
4. [Validation & Testing](#validation--testing)
5. [Rollback Procedures](#rollback-procedures)
6. [Progress Tracking](#progress-tracking)

---

## Prerequisites & Environment Setup

### Checklist Before Starting

```bash
# ✅ Verify environment
rustc --version  # Should be >= 1.85.0 (have 1.90.0 ✅)
cargo --version
git status      # Should be clean or ready for new branch

# ✅ Create feature branch
git checkout -b epic-8-mcp-server-refactoring
git push -u origin epic-8-mcp-server-refactoring

# ✅ Verify all tests pass BEFORE starting
cd /Users/valentynkit/dev/sandbox/Hackaton
cargo test --all-features --workspace
cargo clippy --all-features -- -D warnings

# ✅ Create checkpoint (safety measure)
git add -A
git commit -m "chore: checkpoint before Epic 8 refactoring"
```

### Files We'll Modify (7 files total)

1. `crates/x402-cli/src/commands/test.rs` - Line 60 exit()
2. `crates/x402-server/src/lifecycle.rs` - Lines 19, 35, 39, 44, 78 (5 exits!)
3. `crates/x402-cli/src/commands/check.rs` - Lines 181, 199, 263 (3 exits)
4. `crates/x402-server/src/server.rs` - Line 223 exit()
5. `crates/x402-cli/src/main.rs` - Line 40 exit()
6. `crates/x402-cli/src/lib.rs` - **NEW FILE** (create library interface)
7. `crates/x402-cli/Cargo.toml` - Add `[lib]` section

---

## Day 0: Core Refactoring (10-12 hours)

### Task 1: Refactor test.rs (2 hours)

**File:** `crates/x402-cli/src/commands/test.rs`
**Blocker:** Line 60 `std::process::exit(result.exit_code())`

#### Step 1.1: Add Library Function (30 min)

Read the current implementation:

```bash
cat crates/x402-cli/src/commands/test.rs | head -n 80
```

Add new public function BEFORE existing `execute()`:

```rust
// NEW: Library-friendly version that returns result
pub async fn execute_with_result(args: &TestArgs) -> Result<x402_core::testing::SuiteResult> {
    use x402_core::testing::{execute_test_suite, TestSuite};
    use std::path::Path;

    let test_file = Path::new(&args.test_file);
    if !test_file.exists() {
        anyhow::bail!("Test file not found: {}", args.test_file);
    }

    let suite = TestSuite::from_yaml_file(test_file)
        .await
        .context("Failed to parse test suite")?;

    let result = execute_test_suite(&suite)
        .await
        .context("Test execution failed")?;

    Ok(result)
}
```

#### Step 1.2: Refactor CLI Function (30 min)

Modify existing `execute()` to use new function:

```rust
// MODIFIED: CLI version wraps library version
pub async fn execute(args: &TestArgs) -> Result<()> {
    match execute_with_result(args).await {
        Ok(result) => {
            // Print results
            println!("\n{}", result);

            // Exit with appropriate code (CLI behavior preserved)
            std::process::exit(result.exit_code());
        }
        Err(e) => {
            eprintln!("❌ Test execution failed: {}", e);
            std::process::exit(1);
        }
    }
}
```

#### Step 1.3: Add Unit Tests (45 min)

Add tests at bottom of test.rs:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_execute_with_result_success() {
        // Create temporary test file
        let test_yaml = r#"
version: "1.0"
tests:
  - name: "basic test"
    request:
      method: GET
      url: "http://example.com"
    expect:
      status: 200
"#;
        let temp_file = "/tmp/test_success.yaml";
        fs::write(temp_file, test_yaml).unwrap();

        let args = TestArgs {
            test_file: temp_file.to_string(),
            verbose: false,
        };

        let result = execute_with_result(&args).await;
        assert!(result.is_ok());

        fs::remove_file(temp_file).ok();
    }

    #[tokio::test]
    async fn test_execute_with_result_file_not_found() {
        let args = TestArgs {
            test_file: "/nonexistent/test.yaml".to_string(),
            verbose: false,
        };

        let result = execute_with_result(&args).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
}
```

#### Step 1.4: Validate & Commit (15 min)

```bash
# Run tests for this module
cargo test -p x402-cli test::tests

# Run full test suite
cargo test --workspace

# Check code quality
cargo clippy -- -D warnings

# Commit atomically
git add crates/x402-cli/src/commands/test.rs
git commit -m "refactor(x402-cli): Add execute_with_result for library use

- Add execute_with_result() returning Result<SuiteResult>
- Refactor CLI execute() to wrap library function
- Add unit tests for library function
- Preserve CLI behavior (exit codes unchanged)

Enables MCP server integration for x402__testing_run_suite tool.
Part of Epic 8, Day 0."
```

**Checkpoint:** ✅ test.rs refactored, tests passing, committed

---

### Task 2: Refactor lifecycle.rs (4 hours) ⚠️ CRITICAL

**File:** `crates/x402-server/src/lifecycle.rs`
**Blockers:** 5 exit() calls across 3 functions

#### Step 2.1: Add ServerInfo Struct (15 min)

Add at top of lifecycle.rs:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Information about a running server instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub pid: u32,
    pub port: u16,
    pub started_at: DateTime<Utc>,
    pub config: crate::config::Config,
}

/// Information about server stop operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopInfo {
    pub stopped_at: DateTime<Utc>,
    pub pid: u32,
    pub was_running: bool,
}

/// Information about server status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusInfo {
    pub is_running: bool,
    pub pid: Option<u32>,
    pub port: Option<u16>,
    pub started_at: Option<DateTime<Utc>>,
}
```

#### Step 2.2: Add State Persistence Helper (30 min)

Add helper functions for persistent state:

```rust
use std::fs;
use std::path::PathBuf;

fn state_file_path() -> PathBuf {
    let state_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("x402-dev");

    fs::create_dir_all(&state_dir).ok();
    state_dir.join("server-state.json")
}

fn save_server_state(info: &ServerInfo) -> Result<()> {
    let json = serde_json::to_string_pretty(info)?;
    fs::write(state_file_path(), json)?;
    Ok(())
}

fn load_server_state() -> Option<ServerInfo> {
    let path = state_file_path();
    if !path.exists() {
        return None;
    }

    let json = fs::read_to_string(path).ok()?;
    serde_json::from_str(&json).ok()
}

fn clear_server_state() -> Result<()> {
    let path = state_file_path();
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}
```

#### Step 2.3: Refactor start_server (60 min)

Add library version BEFORE existing function:

```rust
/// Library version: Start server and return info
pub async fn start_server_with_result(
    server_config: crate::config::MockServerConfig,
) -> Result<ServerInfo> {
    use crate::server::MockServer;

    // Check if already running
    if let Some(existing_state) = load_server_state() {
        if is_server_running(existing_state.pid) {
            anyhow::bail!(
                "Server already running (PID: {}). Stop it first with 'x402-dev mock stop'",
                existing_state.pid
            );
        }
    }

    // Clear stale state
    clear_server_state()?;
    clear_pid_file()?;

    // Write PID file
    write_pid_file(std::process::id())?;

    // Prepare server info
    let info = ServerInfo {
        pid: std::process::id(),
        port: server_config.port,
        started_at: Utc::now(),
        config: server_config.config.clone(),
    };

    // Save state for status queries
    save_server_state(&info)?;

    // Start server as background task
    let server = MockServer::new(server_config);
    tokio::spawn(async move {
        if let Err(e) = server.run().await {
            eprintln!("Server error: {}", e);
            clear_server_state().ok();
        }
    });

    Ok(info)
}
```

Modify existing CLI function:

```rust
/// CLI version: Start server or exit with error code
pub async fn start_server(server_config: crate::config::MockServerConfig) -> Result<()> {
    match start_server_with_result(server_config).await {
        Ok(info) => {
            println!("✅ Server started successfully");
            println!("   PID: {}", info.pid);
            println!("   Port: {}", info.port);
            println!("   URL: http://localhost:{}", info.port);
            Ok(())
        }
        Err(e) if e.to_string().contains("already running") => {
            eprintln!("❌ {}", e);
            std::process::exit(3); // Exit code 3: already running
        }
        Err(e) => {
            eprintln!("❌ Failed to start server: {}", e);
            std::process::exit(1);
        }
    }
}
```

#### Step 2.4: Refactor stop_server (45 min)

Add library version:

```rust
/// Library version: Stop server and return info
pub async fn stop_server_with_result() -> Result<StopInfo> {
    let stopped_at = Utc::now();

    match read_pid_file() {
        Some(pid) => {
            let was_running = is_server_running(pid);

            if was_running {
                // Send SIGTERM to process
                #[cfg(unix)]
                {
                    use nix::sys::signal::{kill, Signal};
                    use nix::unistd::Pid;

                    kill(Pid::from_raw(pid as i32), Signal::SIGTERM)
                        .context("Failed to send SIGTERM")?;
                }

                #[cfg(windows)]
                {
                    // Windows: Use taskkill
                    std::process::Command::new("taskkill")
                        .args(["/PID", &pid.to_string(), "/F"])
                        .output()
                        .context("Failed to kill process")?;
                }

                // Wait for graceful shutdown
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }

            // Clean up state
            clear_pid_file()?;
            clear_server_state()?;

            Ok(StopInfo {
                stopped_at,
                pid,
                was_running,
            })
        }
        None => {
            // No PID file, check state file
            if let Some(state) = load_server_state() {
                clear_server_state()?;
                Ok(StopInfo {
                    stopped_at,
                    pid: state.pid,
                    was_running: false,
                })
            } else {
                anyhow::bail!("Server is not running (no PID file found)")
            }
        }
    }
}
```

Modify CLI function:

```rust
/// CLI version: Stop server or exit with error code
pub async fn stop_server() -> Result<()> {
    match stop_server_with_result().await {
        Ok(info) => {
            if info.was_running {
                println!("✅ Server stopped (PID: {})", info.pid);
            } else {
                println!("⚠️  Server was not running");
            }
            Ok(())
        }
        Err(e) if e.to_string().contains("not running") => {
            eprintln!("❌ {}", e);
            std::process::exit(2); // Exit code 2: not running
        }
        Err(e) => {
            eprintln!("❌ Failed to stop server: {}", e);
            std::process::exit(1);
        }
    }
}
```

#### Step 2.5: Refactor server_status (45 min)

Add library version:

```rust
/// Library version: Get server status as data
pub async fn server_status_with_result() -> Result<StatusInfo> {
    // Check PID file
    let pid = read_pid_file();

    if let Some(pid) = pid {
        let is_running = is_server_running(pid);

        if is_running {
            // Load full state if available
            if let Some(state) = load_server_state() {
                return Ok(StatusInfo {
                    is_running: true,
                    pid: Some(pid),
                    port: Some(state.port),
                    started_at: Some(state.started_at),
                });
            }

            // Fallback: running but no state file
            return Ok(StatusInfo {
                is_running: true,
                pid: Some(pid),
                port: None,
                started_at: None,
            });
        }
    }

    // Not running
    Ok(StatusInfo {
        is_running: false,
        pid: None,
        port: None,
        started_at: None,
    })
}
```

Modify CLI function:

```rust
/// CLI version: Print status and exit with status code
pub async fn server_status() -> Result<()> {
    let status = server_status_with_result().await?;

    if status.is_running {
        println!("✅ Server is running");
        if let Some(pid) = status.pid {
            println!("   PID: {}", pid);
        }
        if let Some(port) = status.port {
            println!("   Port: {}", port);
            println!("   URL: http://localhost:{}", port);
        }
        if let Some(started_at) = status.started_at {
            println!("   Started: {}", started_at.format("%Y-%m-%d %H:%M:%S"));
        }
        std::process::exit(0); // Exit code 0: running
    } else {
        println!("❌ Server is not running");
        std::process::exit(2); // Exit code 2: not running
    }
}
```

#### Step 2.6: Add Dependencies (15 min)

Update `crates/x402-server/Cargo.toml`:

```toml
[dependencies]
# ... existing deps ...
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dirs = "5.0"

# Unix-specific
[target.'cfg(unix)'.dependencies]
nix = { version = "0.29", features = ["signal"] }
```

#### Step 2.7: Integration Tests (30 min)

Create `crates/x402-server/tests/lifecycle_integration.rs`:

```rust
use x402_server::{
    start_server_with_result, stop_server_with_result, server_status_with_result,
    ServerInfo, StopInfo, StatusInfo,
};

#[tokio::test]
async fn test_server_lifecycle() {
    // Stop any existing server
    let _ = stop_server_with_result().await;

    // Check initial status (should be stopped)
    let status = server_status_with_result().await.unwrap();
    assert!(!status.is_running);

    // Start server
    let config = create_test_config();
    let start_info = start_server_with_result(config).await.unwrap();
    assert!(start_info.pid > 0);
    assert_eq!(start_info.port, 3402);

    // Check status (should be running)
    let status = server_status_with_result().await.unwrap();
    assert!(status.is_running);
    assert_eq!(status.pid, Some(start_info.pid));

    // Stop server
    let stop_info = stop_server_with_result().await.unwrap();
    assert_eq!(stop_info.pid, start_info.pid);
    assert!(stop_info.was_running);

    // Check final status (should be stopped)
    let status = server_status_with_result().await.unwrap();
    assert!(!status.is_running);
}

#[tokio::test]
async fn test_start_already_running() {
    let _ = stop_server_with_result().await;

    let config = create_test_config();
    start_server_with_result(config.clone()).await.unwrap();

    // Second start should fail
    let result = start_server_with_result(config).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("already running"));

    stop_server_with_result().await.ok();
}

fn create_test_config() -> x402_server::config::MockServerConfig {
    // TODO: Replace with actual config construction
    unimplemented!("Need to check x402_server config API")
}
```

#### Step 2.8: Validate & Commit (15 min)

```bash
# Run tests
cargo test -p x402-server lifecycle

# Full workspace test
cargo test --workspace

# Commit
git add crates/x402-server/
git commit -m "refactor(x402-server): Add library functions for lifecycle management

- Add ServerInfo, StopInfo, StatusInfo structs
- Add start_server_with_result(), stop_server_with_result(), server_status_with_result()
- Add persistent state management via JSON files
- Refactor CLI functions to wrap library functions
- Add lifecycle integration tests
- Add chrono, serde, dirs dependencies

Fixes 5 exit() blockers in lifecycle.rs.
Enables x402__server_mock_* tools for MCP integration.
Part of Epic 8, Day 0."
```

**Checkpoint:** ✅ lifecycle.rs refactored (3 functions), tests passing, committed

---

### Task 3: Refactor check.rs (2 hours)

**File:** `crates/x402-cli/src/commands/check.rs`
**Blockers:** Lines 181, 199, 263 (3 exit() calls)

#### Step 3.1: Read Current Implementation (15 min)

```bash
# Examine the check command structure
cat crates/x402-cli/src/commands/check.rs | head -n 300
```

#### Step 3.2: Add ComplianceCheckResult Struct (15 min)

Add at top of check.rs:

```rust
use serde::{Deserialize, Serialize};

/// Result of compliance check operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheckResult {
    pub endpoint_url: String,
    pub is_compliant: bool,
    pub checks_passed: usize,
    pub checks_failed: usize,
    pub failures: Vec<ComplianceFailure>,
    pub duration: std::time::Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceFailure {
    pub check_name: String,
    pub expected: String,
    pub actual: String,
    pub severity: FailureSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FailureSeverity {
    Critical,  // Must fix
    Warning,   // Should fix
    Info,      // Nice to have
}
```

#### Step 3.3: Add Library Function (60 min)

Add before existing `execute()`:

```rust
/// Library version: Check compliance and return result
pub async fn check_with_result(args: &CheckArgs) -> Result<ComplianceCheckResult> {
    use std::time::Instant;

    let start = Instant::now();
    let endpoint_url = &args.endpoint;

    let mut checks_passed = 0;
    let mut checks_failed = 0;
    let mut failures = Vec::new();

    // Check 1: Endpoint responds to OPTIONS
    match check_options_support(endpoint_url).await {
        Ok(_) => checks_passed += 1,
        Err(e) => {
            checks_failed += 1;
            failures.push(ComplianceFailure {
                check_name: "OPTIONS support".to_string(),
                expected: "Endpoint should respond to OPTIONS requests".to_string(),
                actual: format!("Failed: {}", e),
                severity: FailureSeverity::Critical,
            });
        }
    }

    // Check 2: 402 Payment Required header
    match check_payment_required_header(endpoint_url).await {
        Ok(_) => checks_passed += 1,
        Err(e) => {
            checks_failed += 1,
            failures.push(ComplianceFailure {
                check_name: "402 Payment Required".to_string(),
                expected: "Should return 402 status with payment info".to_string(),
                actual: format!("Failed: {}", e),
                severity: FailureSeverity::Critical,
            });
        }
    }

    // Check 3: Payment protocol headers
    match check_payment_protocol_headers(endpoint_url).await {
        Ok(_) => checks_passed += 1,
        Err(e) => {
            checks_failed += 1,
            failures.push(ComplianceFailure {
                check_name: "Payment protocol headers".to_string(),
                expected: "Should include X-Accept-Payment headers".to_string(),
                actual: format!("Failed: {}", e),
                severity: FailureSeverity::Warning,
            });
        }
    }

    // TODO: Add more checks as needed

    let duration = start.elapsed();
    let is_compliant = checks_failed == 0;

    Ok(ComplianceCheckResult {
        endpoint_url: endpoint_url.clone(),
        is_compliant,
        checks_passed,
        checks_failed,
        failures,
        duration,
    })
}

// Helper functions for individual checks
async fn check_options_support(url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let response = client.options(url).send().await?;

    if !response.status().is_success() {
        anyhow::bail!("OPTIONS request failed with status: {}", response.status());
    }

    Ok(())
}

async fn check_payment_required_header(url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;

    if response.status() != reqwest::StatusCode::PAYMENT_REQUIRED {
        anyhow::bail!("Expected 402 Payment Required, got: {}", response.status());
    }

    Ok(())
}

async fn check_payment_protocol_headers(url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;

    let headers = response.headers();
    if !headers.contains_key("x-accept-payment") {
        anyhow::bail!("Missing X-Accept-Payment header");
    }

    Ok(())
}
```

#### Step 3.4: Refactor CLI Function (30 min)

Find existing exit() calls and replace with wrapped calls:

```rust
/// CLI version: Check compliance and exit with result code
pub async fn execute(args: &CheckArgs) -> Result<()> {
    match check_with_result(args).await {
        Ok(result) => {
            // Print results
            println!("\n🔍 Compliance Check Results");
            println!("   Endpoint: {}", result.endpoint_url);
            println!("   Duration: {:?}", result.duration);
            println!();

            if result.is_compliant {
                println!("✅ All checks passed ({}/{})",
                    result.checks_passed,
                    result.checks_passed + result.checks_failed
                );
                std::process::exit(0);
            } else {
                println!("❌ Compliance check failed");
                println!("   Passed: {}", result.checks_passed);
                println!("   Failed: {}", result.checks_failed);
                println!();

                for failure in &result.failures {
                    let icon = match failure.severity {
                        FailureSeverity::Critical => "🔴",
                        FailureSeverity::Warning => "🟡",
                        FailureSeverity::Info => "🔵",
                    };
                    println!("{} {}", icon, failure.check_name);
                    println!("   Expected: {}", failure.expected);
                    println!("   Actual: {}", failure.actual);
                    println!();
                }

                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("❌ Compliance check failed: {}", e);
            std::process::exit(1);
        }
    }
}
```

#### Step 3.5: Add Tests (30 min)

Add at bottom of check.rs:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_with_result_structure() {
        // This tests structure, not actual compliance
        // (actual compliance requires mock server)

        let args = CheckArgs {
            endpoint: "http://localhost:3402/protected".to_string(),
        };

        // Will fail but should return proper structure
        let result = check_with_result(&args).await;

        // Should return result (not panic/exit)
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_compliance_result_serialization() {
        let result = ComplianceCheckResult {
            endpoint_url: "http://test.com".to_string(),
            is_compliant: false,
            checks_passed: 1,
            checks_failed: 2,
            failures: vec![
                ComplianceFailure {
                    check_name: "Test check".to_string(),
                    expected: "Expected value".to_string(),
                    actual: "Actual value".to_string(),
                    severity: FailureSeverity::Critical,
                }
            ],
            duration: std::time::Duration::from_secs(1),
        };

        // Should serialize to JSON
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("endpoint_url"));
        assert!(json.contains("is_compliant"));
    }
}
```

#### Step 3.6: Validate & Commit (15 min)

```bash
# Add reqwest if not already present
cargo add reqwest --features json --package x402-cli

# Run tests
cargo test -p x402-cli check::tests

# Commit
git add crates/x402-cli/src/commands/check.rs crates/x402-cli/Cargo.toml
git commit -m "refactor(x402-cli): Add check_with_result for library use

- Add ComplianceCheckResult and ComplianceFailure structs
- Add check_with_result() returning structured compliance data
- Refactor CLI execute() to wrap library function
- Add helper functions for individual compliance checks
- Add unit tests for result structure

Fixes 3 exit() blockers in check.rs.
Enables x402__testing_check_compliance tool.
Part of Epic 8, Day 0."
```

**Checkpoint:** ✅ check.rs refactored, tests passing, committed

---

### Task 4: Refactor server.rs (30 min)

**File:** `crates/x402-server/src/server.rs`
**Blocker:** Line 223 exit() on port bind error

#### Step 4.1: Find and Fix Port Bind Error (20 min)

```bash
# Read the server.rs around line 223
sed -n '210,240p' crates/x402-server/src/server.rs
```

Expected pattern:

```rust
// BEFORE (line ~223)
pub async fn run(&self) -> Result<()> {
    let listener = TcpListener::bind(addr).await.unwrap_or_else(|e| {
        eprintln!("Failed to bind to port {}: {}", port, e);
        std::process::exit(1);  // ❌ EXIT BLOCKER
    });
    // ...
}
```

Replace with proper error handling:

```rust
// AFTER
pub async fn run(&self) -> Result<()> {
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("Failed to bind to port {}", port))?;
    // ✅ Returns error, no exit
    // ...
}
```

#### Step 4.2: Validate & Commit (10 min)

```bash
cargo test -p x402-server

git add crates/x402-server/src/server.rs
git commit -m "refactor(x402-server): Replace exit() with error return in port bind

- Replace unwrap_or_else with with_context for port bind error
- Enables proper error propagation to MCP server
- Fixes exit() blocker at server.rs:223

Part of Epic 8, Day 0."
```

**Checkpoint:** ✅ server.rs refactored, committed

---

### Task 5: Refactor main.rs (15 min)

**File:** `crates/x402-cli/src/main.rs`
**Blocker:** Line 40 exit() on config load error

#### Step 5.1: Fix Config Load Error (10 min)

```bash
# Read main.rs around line 40
sed -n '30,50p' crates/x402-cli/src/main.rs
```

Expected pattern:

```rust
// BEFORE (line ~40)
let config = load_merged_config(Some(&cli_overrides)).unwrap_or_else(|e| {
    eprintln!("Failed to load config: {}", e);
    std::process::exit(1);  // ❌ EXIT BLOCKER
});
```

Replace with:

```rust
// AFTER
let config = load_merged_config(Some(&cli_overrides))
    .context("Failed to load configuration")?;
// ✅ Returns error, propagates up
```

Note: This is fine for CLI, but we need config accessible from library. That's next task!

#### Step 5.2: Commit (5 min)

```bash
git add crates/x402-cli/src/main.rs
git commit -m "refactor(x402-cli): Replace exit() with error return in config load

- Use context() instead of unwrap_or_else with exit
- Enables error propagation in CLI
- Fixes exit() blocker at main.rs:40

Part of Epic 8, Day 0."
```

**Checkpoint:** ✅ main.rs refactored, committed

---

### Task 6: Create x402-cli Library Interface (1 hour)

**Files:** `crates/x402-cli/src/lib.rs` (NEW), `crates/x402-cli/Cargo.toml`

#### Step 6.1: Create lib.rs (30 min)

Create new file `crates/x402-cli/src/lib.rs`:

```rust
//! x402-cli Library Interface
//!
//! This crate provides both a CLI binary and a library interface.
//! The library exposes functionality needed by the x402-mcp-server.

// Re-export config module (most important for MCP server)
pub mod config {
    pub use crate::config::*;
}

// Re-export command modules with library functions
pub mod commands {
    pub mod test {
        pub use crate::commands::test::{execute_with_result, TestArgs};
    }

    pub mod check {
        pub use crate::commands::check::{
            check_with_result,
            CheckArgs,
            ComplianceCheckResult,
            ComplianceFailure,
            FailureSeverity,
        };
    }
}

// Re-export commonly used types
pub use config::{
    Config,
    CliOverrides,
    load_merged_config,
};

pub use commands::test::{execute_with_result as execute_test_suite, TestArgs};
pub use commands::check::{check_with_result as check_compliance, CheckArgs};
```

Wait, we need to fix module visibility. Update existing files:

**In `crates/x402-cli/src/config.rs`:**

```rust
// Change from:
pub(crate) struct Config { ... }

// To:
pub struct Config { ... }  // Make public for library use
pub struct CliOverrides { ... }  // Already public
pub fn load_merged_config(...) -> Result<Config> { ... }  // Already public
```

#### Step 6.2: Update Cargo.toml (15 min)

Edit `crates/x402-cli/Cargo.toml`:

```toml
[package]
name = "x402-cli"
version = "0.1.0"
edition = "2024"

# Add library section
[lib]
name = "x402_cli"
path = "src/lib.rs"

# Keep binary section
[[bin]]
name = "x402-dev"
path = "src/main.rs"

[dependencies]
# ... existing dependencies ...
```

#### Step 6.3: Fix Module Structure (10 min)

We need to expose modules in main.rs. Edit `crates/x402-cli/src/main.rs`:

```rust
// At the top, make modules public for lib.rs
pub mod config;
pub mod commands;

use config::{load_merged_config, CliOverrides};
// ... rest of main.rs
```

#### Step 6.4: Test Library Interface (5 min)

```bash
# Test that library compiles
cargo build -p x402-cli --lib

# Test that binary still works
cargo build -p x402-cli --bin x402-dev

# Test importing as library
cd /tmp
cargo init --lib test-x402-import
cd test-x402-import

# Add to Cargo.toml:
# [dependencies]
# x402-cli = { path = "/Users/valentynkit/dev/sandbox/Hackaton/crates/x402-cli" }

# Create src/lib.rs:
cat > src/lib.rs << 'EOF'
use x402_cli::Config;

pub fn test_import() {
    let _: Config;
    println!("Import works!");
}
EOF

cargo build
```

#### Step 6.5: Commit (5 min)

```bash
cd /Users/valentynkit/dev/sandbox/Hackaton

git add crates/x402-cli/src/lib.rs \
        crates/x402-cli/src/main.rs \
        crates/x402-cli/src/config.rs \
        crates/x402-cli/Cargo.toml

git commit -m "feat(x402-cli): Add library interface for MCP integration

- Create lib.rs exposing config and command modules
- Add [lib] section to Cargo.toml
- Make Config and command functions public
- Enable x402-cli to be used as both binary and library

Enables x402-mcp-server to import config and commands directly.
Part of Epic 8, Day 0."
```

**Checkpoint:** ✅ x402-cli library interface created, tested, committed

---

## Day 1: Foundation & PoC (2-3 hours)

### Task 7: Create x402-mcp-server Skeleton (1 hour)

#### Step 7.1: Create Crate (15 min)

```bash
cd /Users/valentynkit/dev/sandbox/Hackaton/crates
cargo new x402-mcp-server --bin
cd x402-mcp-server
```

#### Step 7.2: Setup Cargo.toml (15 min)

Edit `crates/x402-mcp-server/Cargo.toml`:

```toml
[package]
name = "x402-mcp-server"
version = "0.1.0"
edition = "2024"
rust-version = "1.85.0"

[[bin]]
name = "x402-mcp-server"
path = "src/main.rs"

[dependencies]
# MCP SDK
rmcp = "0.8.5"

# Async runtime
tokio = { workspace = true }

# Serialization
serde = { workspace = true }
serde_json = { workspace = true }

# Error handling
anyhow = { workspace = true }
thiserror = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# x402 dependencies (direct library integration)
x402-core = { path = "../x402-core" }
x402-server = { path = "../x402-server" }
x402-cli = { path = "../x402-cli" }

[dev-dependencies]
tokio-test = "0.4"
```

#### Step 7.3: Create Main Entry Point (20 min)

Create `crates/x402-mcp-server/src/main.rs`:

```rust
use anyhow::Result;
use rmcp::{Server, ServerBuilder};
use tracing::info;

mod tools;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .with_writer(std::io::stderr)
        .init();

    info!("🚀 x402-mcp-server v0.1.0 starting...");

    // Build MCP server
    let server = ServerBuilder::new("x402-dev-mcp")
        .version(env!("CARGO_PKG_VERSION"))
        .description("Rust MCP server for x402-dev payment protocol testing")
        .build();

    info!("📋 Registering tools...");
    tools::register_all(&server).await?;
    info!("✅ All tools registered successfully");

    info!("📡 Starting stdio transport...");
    server.serve_stdio().await?;

    Ok(())
}
```

#### Step 7.4: Create Tool Module Structure (10 min)

Create `crates/x402-mcp-server/src/tools/mod.rs`:

```rust
use anyhow::Result;
use rmcp::Server;

pub mod mock;
pub mod testing;
pub mod policy;

/// Register all tools with the MCP server
pub async fn register_all(server: &Server) -> Result<()> {
    mock::register(server).await?;
    testing::register(server).await?;
    policy::register(server).await?;
    Ok(())
}
```

Create `crates/x402-mcp-server/src/error.rs`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum McpError {
    #[error("Server error (E3{code:03}): {message}")]
    Server { code: u16, message: String },

    #[error("Testing error (E4{code:03}): {message}")]
    Testing { code: u16, message: String },

    #[error("Policy error (E5{code:03}): {message}")]
    Policy { code: u16, message: String },

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl McpError {
    pub fn error_code(&self) -> String {
        match self {
            McpError::Server { code, .. } => format!("E3{:03}", code),
            McpError::Testing { code, .. } => format!("E4{:03}", code),
            McpError::Policy { code, .. } => format!("E5{:03}", code),
            McpError::Internal(_) => "E9999".to_string(),
        }
    }
}
```

#### Step 7.5: Add to Workspace (5 min)

Edit root `Cargo.toml`:

```toml
[workspace]
members = [
    ".",
    "crates/*",
    "crates/x402-mcp-server",  # Add explicitly if needed
]
```

#### Step 7.6: Test Build (5 min)

```bash
cd /Users/valentynkit/dev/sandbox/Hackaton
cargo build -p x402-mcp-server
```

This will fail because we haven't created tool modules yet - that's Day 2!

---

### Task 8: rmcp Proof-of-Concept (1 hour)

Create minimal working tool to validate rmcp SDK.

#### Step 8.1: Create Mock Tools Stub (30 min)

Create `crates/x402-mcp-server/src/tools/mock.rs`:

```rust
use anyhow::Result;
use rmcp::{tool, CallToolResult, Server, TextContent};
use serde_json::json;
use tracing::{info, error};

pub async fn register(server: &Server) -> Result<()> {
    server.add_tool(server_mock_status()).await?;
    info!("✅ Registered mock server tools");
    Ok(())
}

#[tool(
    name = "x402__server_mock_status",
    description = "Check mock payment server status (returns running state, PID, port)"
)]
pub async fn server_mock_status() -> Result<CallToolResult> {
    info!("🔍 Checking server status...");

    // Call library function (no subprocess!)
    match x402_server::server_status_with_result().await {
        Ok(status) => {
            let response = json!({
                "status": if status.is_running { "running" } else { "stopped" },
                "is_running": status.is_running,
                "pid": status.pid,
                "port": status.port,
                "started_at": status.started_at.map(|t| t.to_rfc3339()),
            });

            Ok(CallToolResult {
                isError: false,
                content: vec![TextContent {
                    type_: "text".to_string(),
                    text: response.to_string(),
                }],
            })
        }
        Err(e) => {
            error!("Failed to get server status: {}", e);
            Ok(CallToolResult {
                isError: true,
                content: vec![TextContent {
                    type_: "text".to_string(),
                    text: json!({
                        "error": "E3004",
                        "message": format!("Failed to get server status: {}", e),
                    }).to_string(),
                }],
            })
        }
    }
}
```

Create stubs for other modules:

`crates/x402-mcp-server/src/tools/testing.rs`:

```rust
use anyhow::Result;
use rmcp::Server;

pub async fn register(_server: &Server) -> Result<()> {
    // TODO: Day 2 - implement testing tools
    Ok(())
}
```

`crates/x402-mcp-server/src/tools/policy.rs`:

```rust
use anyhow::Result;
use rmcp::Server;

pub async fn register(_server: &Server) -> Result<()> {
    // TODO: Day 2 - implement policy tools
    Ok(())
}
```

#### Step 8.2: Test Build (10 min)

```bash
cargo build -p x402-mcp-server
cargo run -p x402-mcp-server
```

Should start and wait for stdio input. Test with:

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | cargo run -p x402-mcp-server
```

#### Step 8.3: Test with Claude Desktop (15 min)

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "x402-dev": {
      "command": "/Users/valentynkit/.cargo/bin/cargo",
      "args": [
        "run",
        "--manifest-path",
        "/Users/valentynkit/dev/sandbox/Hackaton/Cargo.toml",
        "-p",
        "x402-mcp-server",
        "--release"
      ]
    }
  }
}
```

Restart Claude Desktop, check server shows up in Tools menu.

#### Step 8.4: Manual Integration Test (5 min)

In Claude Desktop, try:

```
Use the x402__server_mock_status tool to check the server status
```

Should get JSON response with status info!

#### Step 8.5: Commit PoC (5 min)

```bash
git add crates/x402-mcp-server/
git commit -m "feat(x402-mcp-server): Add rmcp proof-of-concept

- Create x402-mcp-server crate with rmcp SDK
- Implement x402__server_mock_status tool (PoC)
- Add tool registration system
- Add error types and logging
- Validate rmcp SDK integration works
- Test with Claude Desktop successfully

Part of Epic 8, Day 1."
```

**Checkpoint:** ✅ rmcp PoC complete, 1 tool working, committed

---

### Task 9: API Validation Script (30 min)

Create script to validate all refactored functions exist.

#### Step 9.1: Create Validation Script (20 min)

Create `crates/x402-mcp-server/scripts/validate-apis.sh`:

```bash
#!/bin/bash
set -e

echo "🔍 Validating x402 API availability for MCP integration..."
echo ""

# Test imports in a temporary test file
cat > /tmp/x402_api_test.rs << 'EOF'
// Test that all required APIs are importable

use x402_cli::{
    execute_test_suite,
    check_compliance,
    Config,
    load_merged_config,
};

use x402_server::{
    start_server_with_result,
    stop_server_with_result,
    server_status_with_result,
    ServerInfo,
    StopInfo,
    StatusInfo,
};

use x402_core::{
    policy::validate_policies,
    testing::execute_test_suite as core_execute_test_suite,
};

fn main() {
    println!("✅ All APIs importable");
}
EOF

# Try to compile the test
echo "📦 Testing x402-cli APIs..."
cd /Users/valentynkit/dev/sandbox/Hackaton
cargo build -p x402-cli --lib
echo "✅ x402-cli library builds"

echo "📦 Testing x402-server APIs..."
cargo build -p x402-server --lib
echo "✅ x402-server library builds"

echo "📦 Testing x402-core APIs..."
cargo build -p x402-core --lib
echo "✅ x402-core library builds"

echo ""
echo "🎉 All required APIs validated successfully!"
echo ""
echo "Available functions:"
echo "  • x402_cli::execute_with_result()"
echo "  • x402_cli::check_with_result()"
echo "  • x402_server::start_server_with_result()"
echo "  • x402_server::stop_server_with_result()"
echo "  • x402_server::server_status_with_result()"
echo "  • x402_core::testing::execute_test_suite()"
echo "  • x402_core::policy::validate_policies()"
echo ""
```

Make executable:

```bash
chmod +x crates/x402-mcp-server/scripts/validate-apis.sh
```

#### Step 9.2: Run Validation (5 min)

```bash
./crates/x402-mcp-server/scripts/validate-apis.sh
```

Should complete with ✅ for all APIs.

#### Step 9.3: Commit (5 min)

```bash
git add crates/x402-mcp-server/scripts/
git commit -m "chore(x402-mcp-server): Add API validation script

- Create validate-apis.sh to check all required functions exist
- Validates library builds for x402-cli, x402-server, x402-core
- Documents available API surface for MCP tools

Part of Epic 8, Day 1."
```

**Checkpoint:** ✅ API validation script created and passing

---

## Validation & Testing

### Full Workspace Test

After completing all Day 0-1 tasks:

```bash
cd /Users/valentynkit/dev/sandbox/Hackaton

# Clean build
cargo clean

# Build all crates
cargo build --workspace --all-features

# Run all tests
cargo test --workspace --all-features

# Check code quality
cargo clippy --workspace --all-features -- -D warnings

# Check formatting
cargo fmt --all -- --check

# Run benchmarks (if any)
cargo bench --no-run
```

### Integration Test Checklist

Create `docs/epics/epic-8-mcp-integration/DAY-0-1-CHECKLIST.md`:

```markdown
# Day 0-1 Completion Checklist

## Day 0: Refactoring (10-12 hours)

- [ ] Task 1: test.rs refactored (2h)
  - [ ] execute_with_result() added
  - [ ] Tests passing
  - [ ] Committed

- [ ] Task 2: lifecycle.rs refactored (4h)
  - [ ] ServerInfo/StopInfo/StatusInfo structs added
  - [ ] start_server_with_result() added
  - [ ] stop_server_with_result() added
  - [ ] server_status_with_result() added
  - [ ] State persistence implemented
  - [ ] Integration tests passing
  - [ ] Committed

- [ ] Task 3: check.rs refactored (2h)
  - [ ] ComplianceCheckResult struct added
  - [ ] check_with_result() added
  - [ ] Tests passing
  - [ ] Committed

- [ ] Task 4: server.rs refactored (30min)
  - [ ] Port bind exit() removed
  - [ ] Committed

- [ ] Task 5: main.rs refactored (15min)
  - [ ] Config load exit() removed
  - [ ] Committed

- [ ] Task 6: x402-cli library created (1h)
  - [ ] lib.rs created
  - [ ] Cargo.toml updated with [lib]
  - [ ] Modules exposed properly
  - [ ] Test imports work
  - [ ] Committed

## Day 1: Foundation (2-3 hours)

- [ ] Task 7: x402-mcp-server skeleton (1h)
  - [ ] Crate created
  - [ ] Cargo.toml configured
  - [ ] Main entry point created
  - [ ] Tool module structure set up
  - [ ] Builds successfully

- [ ] Task 8: rmcp PoC (1h)
  - [ ] server_mock_status implemented
  - [ ] Tests with Claude Desktop successful
  - [ ] Committed

- [ ] Task 9: API validation (30min)
  - [ ] Validation script created
  - [ ] All APIs confirmed importable
  - [ ] Committed

## Final Validation

- [ ] All workspace tests pass
- [ ] No clippy warnings
- [ ] Code formatted
- [ ] All commits pushed to feature branch
- [ ] Ready for Day 2 (tool implementation)

## Exit Criteria

✅ Zero exit() calls in library functions
✅ x402-cli importable as library
✅ All refactored functions have tests
✅ rmcp PoC working with 1 tool
✅ All changes committed atomically
```

---

## Rollback Procedures

### If Tests Fail After Task N

```bash
# Find last good commit
git log --oneline

# Reset to last checkpoint
git reset --hard <commit-hash>

# Or: Revert specific commit
git revert <commit-hash>

# Re-run tests
cargo test --workspace
```

### If Build Breaks

```bash
# Check what changed
git diff HEAD~1

# Stash changes temporarily
git stash

# Verify build works without changes
cargo build --workspace

# Apply changes back
git stash pop

# Debug incrementally
```

### Nuclear Option (Start Over)

```bash
# Return to branch point
git checkout master
git branch -D epic-8-mcp-server-refactoring
git checkout -b epic-8-mcp-server-refactoring

# Start from Task 1 again
```

---

## Progress Tracking

### Time Tracking Template

Copy to `docs/epics/epic-8-mcp-integration/DAY-0-1-ACTUAL-TIME.md`:

```markdown
# Actual Time Tracking

| Task | Estimated | Actual | Variance | Notes |
|------|-----------|--------|----------|-------|
| test.rs | 2h | ___h | ___% | |
| lifecycle.rs | 4h | ___h | ___% | |
| check.rs | 2h | ___h | ___% | |
| server.rs | 0.5h | ___h | ___% | |
| main.rs | 0.25h | ___h | ___% | |
| lib.rs | 1h | ___h | ___% | |
| mcp skeleton | 1h | ___h | ___% | |
| rmcp PoC | 1h | ___h | ___% | |
| validation | 0.5h | ___h | ___% | |
| **Total** | **12.25h** | **___h** | **___% | |
```

### Checkpoint Commands

After each task:

```bash
# Status check
echo "✅ Task N completed at $(date)"
git log -1 --oneline
cargo test --workspace | tail -n 5

# Update tracking
echo "Task N: $(git log -1 --format='%h %s')" >> progress.log
```

---

## Next Steps After Day 0-1

Once checklist is 100% complete:

1. **Code Review:** Self-review all diffs before proceeding
2. **Documentation:** Update API-REFERENCE.md with actual function signatures
3. **Day 2 Planning:** Ready to implement remaining 6 tools
4. **Push Branch:** `git push origin epic-8-mcp-server-refactoring`

**Estimated:** Day 0-1 = 12-13 hours
**Remaining:** Days 2-7 = 30-31 hours
**Total:** 42-43 hours (Option A)

---

## Success Metrics

### Day 0 Success Criteria

- ✅ All 11 exit() calls refactored
- ✅ 8 new functions created (_with_result variants)
- ✅ x402-cli lib.rs created and working
- ✅ Zero breaking changes to CLI behavior
- ✅ All existing tests pass
- ✅ New tests added (8+ test functions)

### Day 1 Success Criteria

- ✅ x402-mcp-server builds successfully
- ✅ rmcp integration validated
- ✅ 1 tool working end-to-end (server_mock_status)
- ✅ Claude Desktop integration confirmed
- ✅ API validation script passes

---

**Document Version:** 1.0
**Created:** 2025-11-12
**Part of:** Epic 8 - Option A Extended Timeline
**Next:** Days 2-3 Implementation Plan
