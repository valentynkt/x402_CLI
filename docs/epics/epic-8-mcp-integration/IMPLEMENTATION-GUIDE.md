# Epic 8: Rust MCP Server Implementation Guide

**Target:** Developers building the x402-mcp-server (Rust implementation)
**Duration:** 6 days (26 hours)
**Prerequisites:** Rust 1.85.0+, x402-dev source code access

**Architecture:** Rust MCP server with direct library integration (not subprocess approach)

**⚠️ DOCUMENT STATUS:** This guide is being migrated from TypeScript to Rust architecture. Sections above show the correct Rust implementation patterns. Some TypeScript code examples below are retained for reference but should be translated to Rust during implementation. See TECHNICAL-APPENDIX.md Section A for complete Rust patterns.

---

## Quick Start

### Prerequisites Checklist

- [ ] Rust 1.85.0+ installed (`rustc --version`)
- [ ] Cargo package manager (`cargo --version`)
- [ ] x402-dev source code cloned (`git clone ...`)
- [ ] Git configured
- [ ] Basic Rust knowledge (async/await, Result<T, E>)
- [ ] MCP concepts understood

### Initial Setup (10 minutes)

```bash
# 1. Create Rust project
cargo new x402-mcp-server
cd x402-mcp-server

# 2. Add dependencies to Cargo.toml
cat >> Cargo.toml << 'EOF'

[dependencies]
# MCP SDK
rmcp = "0.8.5"

# Async runtime
tokio = { version = "1.35", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# x402 libraries (local path dependencies)
x402-core = { path = "../x402-dev/crates/x402-core" }
x402-server = { path = "../x402-dev/crates/x402-server" }
x402-cli = { path = "../x402-dev/crates/x402-cli" }

[dev-dependencies]
criterion = "0.5"
EOF

# 3. Create project structure
mkdir -p src/tools tests benches
```

---

## Phase 1: Foundation (Days 1-2)

### Days 1-2: Project Setup & First 3 Tools

**Goal:** Working Rust MCP server with 3 simple tools

#### Step 1.1: Configure Cargo.toml

Update `Cargo.toml` with complete metadata:
```toml
[package]
name = "x402-mcp-server"
version = "0.1.0"
edition = "2024"
rust-version = "1.85.0"
authors = ["x402-dev Team"]
license = "MIT OR Apache-2.0"
description = "MCP server for x402-dev protocol testing toolkit"
repository = "https://github.com/x402-dev/x402-mcp-server"
keywords = ["mcp", "x402", "testing", "solana", "payment"]
categories = ["development-tools", "testing"]

[[bin]]
name = "x402-mcp-server"
path = "src/main.rs"

[dependencies]
rmcp = "0.8.5"
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
thiserror = "1.0"

# Local x402 dependencies
x402-core = { path = "../x402-dev/crates/x402-core" }
x402-server = { path = "../x402-dev/crates/x402-server" }
x402-cli = { path = "../x402-dev/crates/x402-cli" }

[dev-dependencies]
criterion = { version = "0.5", features = ["async_tokio"] }
tokio-test = "0.4"

[[bench]]
name = "tool_benchmarks"
harness = false
```

#### Step 1.2: Create Project Structure

```
x402-mcp-server/
├── src/
│   ├── main.rs            # Entry point + stdio transport
│   ├── tools/
│   │   ├── mod.rs         # Tool module
│   │   ├── mock.rs        # Mock server tools
│   │   ├── testing.rs     # Testing tools
│   │   └── policy.rs      # Policy tools
│   ├── error.rs           # Error types + MCP translation
│   └── lib.rs             # Library exports (optional)
├── tests/
│   ├── integration_test.rs
│   └── tool_tests.rs
├── benches/
│   └── tool_benchmarks.rs
├── Cargo.toml
└── README.md
```

#### Step 1.3: Configure Cargo Test

Add to `Cargo.toml`:
```toml
[profile.test]
opt-level = 1  # Faster test compilation

[profile.bench]
opt-level = 3  # Maximum performance for benchmarks
```

**Validation:**
- ✅ `cargo build` succeeds
- ✅ `cargo test` runs (no tests yet)
- ✅ `cargo clippy` passes
- ✅ `cargo fmt --check` passes

---

### Day 1: stdio Transport & First Tool

**Goal:** MCP server accepts connections, implements server_mock_start tool

#### Step 2.1: Implement stdio Transport

`src/main.rs`:
```rust
use rmcp::{Server, ServerBuilder};
use tokio;

mod tools;
mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Log to stderr only (stdout is for JSON-RPC)
    eprintln!("x402-mcp-server starting...");

    // Build MCP server with stdio transport
    let server = ServerBuilder::new("x402-dev-mcp")
        .version("0.1.0")
        .description("MCP server for x402-dev protocol testing")
        .build();

    // Register tools (starts with 1, add more later)
    tools::register_all(&server).await?;

    // Start stdio transport (blocking)
    server.serve_stdio().await?;

    Ok(())
}
```

**Important:** All logging MUST go to stderr (`eprintln!`), not stdout (stdout is for JSON-RPC messages).

#### Step 2.2: Implement First Tool (server_mock_start)

`src/tools/mod.rs`:
```rust
pub mod mock;
pub mod testing;
pub mod policy;

use rmcp::Server;

/// Register all MCP tools with the server
pub async fn register_all(server: &Server) -> anyhow::Result<()> {
    // Register mock server tools
    mock::register(server).await?;

    // Register testing tools (Phase 2)
    // testing::register(server).await?;

    // Register policy tools (Phase 2)
    // policy::register(server).await?;

    Ok(())
}
```

`src/tools/mock.rs`:
```rust
use rmcp::{tool, Server, CallToolResult, TextContent};
use serde_json::json;
use x402_server::{start_server, MockServerConfig};
use x402_cli::config::{load_merged_config, CliOverrides};

/// Register mock server tools
pub async fn register(server: &Server) -> anyhow::Result<()> {
    server.register_tool(server_mock_start).await?;
    Ok(())
}

/// Start x402-dev mock payment server
#[tool(
    name = "x402__server_mock_start",
    description = "Start mock payment server for testing",
)]
pub async fn server_mock_start(
    #[arg(default = 3402, description = "Port number (1024-65535)")]
    port: u16,

    #[arg(default = 0.01, description = "Pricing per request in SOL")]
    pricing: f64,
) -> anyhow::Result<CallToolResult> {
    // Load configuration with overrides
    let cli_overrides = CliOverrides {
        port: Some(port),
        pricing: Some(pricing),
        ..Default::default()
    };

    let config = load_merged_config(Some(&cli_overrides))?;

    // Start server (direct library call, no subprocess!)
    let server_config = MockServerConfig {
        port,
        config: config.into(),
        ..Default::default()
    };

    start_server(server_config).await?;

    // Return structured JSON response
    Ok(CallToolResult {
        isError: false,
        content: vec![TextContent {
            type_: "text".to_string(),
            text: json!({
                "status": "started",
                "pid": std::process::id(),
                "port": port,
                "server_url": format!("http://localhost:{}", port),
                "config": {
                    "pricing": pricing
                }
            }).to_string()
        }]
    })
}
```

**Key Difference from TypeScript:** Direct function call (0ms overhead) vs subprocess spawn (50-200ms)

#### Step 2.3: Create Error Translation Layer

```rust
use rmcp::CallToolResult, TextContent};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum McpError {
    #[error("Port {port} is already in use")]
    PortInUse { port: u16 },

    #[error("Invalid port number: {port}")]
    InvalidPort { port: u16 },

    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),

    #[error("YAML parse error: {0}")]
    YamlParse(String),

    #[error("x402-core error: {0}")]
    X402Core(#[from] x402_core::Error),
}

impl From<McpError> for CallToolResult {
    fn from(err: McpError) -> Self {
        let (code, message, suggestion) = match &err {
            McpError::PortInUse { port } => (
                "E3001",
                format!("Port {} is already in use", port),
                "Stop existing server with x402__server_mock_stop or use different port".to_string()
            ),
            McpError::InvalidPort { port } => (
                "E3002",
                format!("Invalid port number: {}", port),
                "Use port between 1024-65535".to_string()
            ),
            McpError::Network(e) => (
                "E4003",
                format!("Network error: {}", e),
                "Check endpoint URL and network connectivity".to_string()
            ),
            McpError::YamlParse(e) => (
                "E4001",
                format!("Invalid YAML: {}", e),
                "Check YAML syntax and structure".to_string()
            ),
            McpError::X402Core(e) => (
                "E9003",
                format!("x402-core error: {}", e),
                "Check x402-dev logs for details".to_string()
            ),
        };

        CallToolResult {
            isError: true,
            content: vec![TextContent {
                type_: "text".to_string(),
                text: json!({
                    "error": code,
                    "message": message,
                    "suggestion": suggestion,
                    "docs_link": format!("https://docs.x402-dev.com/errors/{}", code)
                }).to_string()
            }]
        }
    }
}
```

**Validation:**
- ✅ stdio transport responds to MCP handshake
- ✅ First tool (server_mock_start) callable from Claude Code
- ✅ Error translator converts x402-core errors to MCP format
- ✅ Sub-millisecond latency confirmed with benchmark

---

### Day 2: Add Two More Tools

**Goal:** 3 tools working (mock_start, policy_validate, config_show)

#### Step 3.1: Implement `x402__policy_validate`

`src/tools/policy.rs`:
```rust
use rmcp::{tool, Server, CallToolResult, TextContent};
use serde_json::json;
use x402_core::policy::validate_policies;

/// Register policy tools
pub async fn register(server: &Server) -> anyhow::Result<()> {
    server.register_tool(policy_validate).await?;
    Ok(())
}

/// Validate payment policy YAML
#[tool(
    name = "x402__policy_validate",
    description = "Validate payment policy YAML for syntax and logical conflicts",
)]
pub async fn policy_validate(
    #[arg(description = "Policy YAML content (inline, not file path)")]
    policy_yaml: String,
) -> anyhow::Result<CallToolResult> {
    // Parse and validate policy (in-memory, no temp files)
    let validation_result = validate_policies(&policy_yaml)?;

    Ok(CallToolResult {
        isError: false,
        content: vec![TextContent {
            type_: "text".to_string(),
            text: json!({
                "valid": validation_result.is_valid,
                "has_errors": validation_result.errors.len() > 0,
                "has_warnings": validation_result.warnings.len() > 0,
                "issues": validation_result.all_issues()
            }).to_string()
        }]
    })
}
```

**Key Advantage:** No temp file needed (works with String directly)

#### Step 3.2: Implement `x402__config_show`

`src/tools/config.rs`:
```rust
use rmcp::{tool, Server, CallToolResult, TextContent};
use serde_json::json;
use x402_cli::config::load_merged_config;

pub async fn register(server: &Server) -> anyhow::Result<()> {
    server.register_tool(config_show).await?;
    Ok(())
}

/// Show merged configuration (all tiers)
#[tool(
    name = "x402__config_show",
    description = "Display merged configuration from all sources",
)]
pub async fn config_show() -> anyhow::Result<CallToolResult> {
    let config = load_merged_config(None)?;

    Ok(CallToolResult {
        isError: false,
        content: vec![TextContent {
            type_: "text".to_string(),
            text: json!({
                "config": config,
                "sources": vec!["CLI flags", "Environment", "Project", "Global", "Defaults"]
            }).to_string()
        }]
    })
}
```

#### Step 3.3: Update Tool Registration

`src/tools/mod.rs`:
```rust
pub mod mock;
pub mod testing;
pub mod policy;
pub mod config;

use rmcp::Server;

pub async fn register_all(server: &Server) -> anyhow::Result<()> {
    mock::register(server).await?;
    policy::register(server).await?;
    config::register(server).await?;

    // testing::register(server).await?;  // Phase 2

    eprintln!("Registered 3 tools successfully");
    Ok(())
}
```

#### Step 3.4: Test with Claude Code

```bash
# 1. Build project
cargo build --release

# 2. Add to Claude Code
claude mcp add x402-mcp /path/to/x402-mcp-server/target/release/x402-mcp-server

# 3. Test in Claude Code
# Ask Claude: "Start an x402 mock server on port 3402"
```

**Validation:**
- ✅ Tool callable from Claude Code
- ✅ Server starts successfully
- ✅ PID file created
- ✅ Error handling works (port in use)

---

## Phase 2: Core Tools (Days 3-4)

**Goal:** Implement remaining 4 tools with Rust direct library integration

### Day 3: Complete Mock Server Tools

#### Step 4.1: Implement `x402__server_mock_stop`

`src/tools/mock.rs` (add to existing file):
```rust
/// Stop running mock payment server
#[tool(
    name = "x402__server_mock_stop",
    description = "Stop the running mock payment server gracefully",
)]
pub async fn server_mock_stop() -> anyhow::Result<CallToolResult> {
    // Direct library call (no PID file reading needed with direct integration)
    match x402_server::stop_server().await {
        Ok(server_info) => Ok(CallToolResult {
            isError: false,
            content: vec![TextContent {
                type_: "text".to_string(),
                text: json!({
                    "status": "stopped",
                    "pid": server_info.pid,
                    "stopped_at": chrono::Utc::now().to_rfc3339(),
                    "uptime_seconds": server_info.uptime.as_secs()
                }).to_string()
            }]
        }),
        Err(x402_server::Error::ServerNotRunning) => Ok(CallToolResult {
            isError: true,
            content: vec![TextContent {
                type_: "text".to_string(),
                text: json!({
                    "error": "E3004",
                    "message": "No mock server is currently running",
                    "suggestion": "Start server first with x402__server_mock_start"
                }).to_string()
            }]
        }),
        Err(e) => Err(e.into())
    }
}
```

#### Step 4.2: Implement `x402__server_mock_status`

```rust
/// Check mock server status
#[tool(
    name = "x402__server_mock_status",
    description = "Check if mock payment server is running and get current status",
)]
pub async fn server_mock_status() -> anyhow::Result<CallToolResult> {
    match x402_server::server_status().await? {
        Some(status) => Ok(CallToolResult {
            isError: false,
            content: vec![TextContent {
                type_: "text".to_string(),
                text: json!({
                    "status": "running",
                    "pid": status.pid,
                    "port": status.port,
                    "server_url": format!("http://localhost:{}", status.port),
                    "uptime_seconds": status.uptime.as_secs(),
                    "started_at": status.started_at.to_rfc3339(),
                    "config": {
                        "pricing": status.config.pricing,
                        "simulation_mode": status.config.simulation_mode.to_string()
                    }
                }).to_string()
            }]
        }),
        None => Ok(CallToolResult {
            isError: false,
            content: vec![TextContent {
                type_: "text".to_string(),
                text: json!({
                    "status": "not_running",
                    "message": "No mock server is currently running"
                }).to_string()
            }]
        })
    }
}
```

#### Step 4.3: Update Tool Registration

`src/tools/mock.rs`:
```rust
pub async fn register(server: &Server) -> anyhow::Result<()> {
    server.register_tool(server_mock_start).await?;
    server.register_tool(server_mock_stop).await?;
    server.register_tool(server_mock_status).await?;
    Ok(())
}
```

**Validation:**
- ✅ All 3 mock server tools working
- ✅ Integration test: start → status → stop → status
- ✅ Error handling for "server not running"

---

### Day 4: Testing Tools

#### Step 5.1: Implement `x402__testing_run_suite`

`src/tools/testing.rs`:
```rust
use rmcp::{tool, Server, CallToolResult, TextContent};
use serde_json::json;
use x402_core::testing::{TestSuite, execute_test_suite};

pub async fn register(server: &Server) -> anyhow::Result<()> {
    server.register_tool(testing_run_suite).await?;
    server.register_tool(testing_check_compliance).await?;
    Ok(())
}

/// Execute YAML test suite
#[tool(
    name = "x402__testing_run_suite",
    description = "Execute YAML test suite to validate payment-protected endpoints",
)]
pub async fn testing_run_suite(
    #[arg(description = "YAML test suite content (inline, not file path)")]
    suite_yaml: String,

    #[arg(default = "json", description = "Output format: json, junit, or human")]
    output_format: String,

    #[arg(default = false, description = "Suppress verbose output")]
    quiet: bool,
) -> anyhow::Result<CallToolResult> {
    // Parse YAML into TestSuite (in-memory, no temp files!)
    let suite = TestSuite::from_yaml_str(&suite_yaml)
        .map_err(|e| anyhow::anyhow!("Invalid test suite YAML: {}", e))?;

    // Execute tests (direct library call)
    let result = execute_test_suite(&suite).await?;

    // Return structured JSON (not text parsing!)
    Ok(CallToolResult {
        isError: false,
        content: vec![TextContent {
            type_: "text".to_string(),
            text: json!({
                "summary": {
                    "total_tests": result.total,
                    "passed": result.passed,
                    "failed": result.failed,
                    "skipped": result.skipped,
                    "duration_ms": result.duration.as_millis()
                },
                "tests": result.test_results.iter().map(|t| json!({
                    "name": t.name,
                    "status": t.status.to_string(),
                    "duration_ms": t.duration.as_millis(),
                    "error": t.error.as_ref(),
                    "assertions_passed": t.assertions_passed,
                    "assertions_failed": t.assertions_failed
                })).collect::<Vec<_>>(),
                "exit_code": if result.failed > 0 { 1 } else { 0 }
            }).to_string()
        }]
    })
}
```

**Key Advantage:** No temp files needed - work with data structures directly!

#### Step 5.2: Implement `x402__testing_check_compliance`

```rust
use x402_core::testing::check_compliance;

/// Validate HTTP 402 endpoint compliance
#[tool(
    name = "x402__testing_check_compliance",
    description = "Validate HTTP 402 endpoint compliance with x402 protocol",
)]
pub async fn testing_check_compliance(
    #[arg(description = "Endpoint URL to check")]
    url: String,

    #[arg(description = "Expected recipient address (optional)")]
    expected_recipient: Option<String>,

    #[arg(description = "Expected payment amount (optional)")]
    expected_amount: Option<f64>,

    #[arg(default = 10, description = "Timeout in seconds")]
    timeout_seconds: u64,
) -> anyhow::Result<CallToolResult> {
    // Direct library call with timeout
    let timeout_duration = tokio::time::Duration::from_secs(timeout_seconds);
    let result = tokio::time::timeout(
        timeout_duration,
        check_compliance(&url, expected_recipient.as_deref(), expected_amount)
    ).await
        .map_err(|_| anyhow::anyhow!("Request timed out after {} seconds", timeout_seconds))??;

    Ok(CallToolResult {
        isError: false,
        content: vec![TextContent {
            type_: "text".to_string(),
            text: json!({
                "compliant": result.is_compliant(),
                "checks": result.checks.iter().map(|check| json!({
                    "name": check.name,
                    "passed": check.passed,
                    "value": check.value,
                    "error": check.error
                })).collect::<Vec<_>>(),
                "passed": result.passed_count(),
                "total": result.total_count(),
                "warnings": result.warnings,
                "suggestions": result.suggestions
            }).to_string()
        }]
    })
}
```

**Validation:**
- ✅ Test suite execution works (in-memory YAML parsing)
- ✅ Compliance checking works with timeout
- ✅ No temp file management needed
- ✅ Structured error responses

---

### Day 4 (continued): Policy Tools

#### Step 5.3: Implement `x402__policy_generate_express`

`src/tools/policy.rs` (add to existing):
```rust
use x402_core::policy::{generate_middleware, Framework};

/// Generate Express.js middleware from policy
#[tool(
    name = "x402__policy_generate_express",
    description = "Generate Express.js middleware code from payment policy YAML",
)]
pub async fn policy_generate_express(
    #[arg(description = "Policy YAML content (inline)")]
    policy_yaml: String,

    #[arg(default = "policy.yaml", description = "Source filename for comments")]
    filename: String,
) -> anyhow::Result<CallToolResult> {
    // Parse policy (in-memory)
    let policy = x402_core::policy::Policy::from_yaml_str(&policy_yaml)?;

    // Validate first
    let validation = validate_policies(&policy_yaml)?;
    if !validation.is_valid {
        return Ok(CallToolResult {
            isError: true,
            content: vec![TextContent {
                type_: "text".to_string(),
                text: json!({
                    "error": "E5002",
                    "message": "Cannot generate code from invalid policy",
                    "suggestion": "Fix validation errors first using x402__policy_validate",
                    "validation_errors": validation.errors
                }).to_string()
            }]
        });
    }

    // Generate middleware code
    let generated_code = generate_middleware(&policy, Framework::Express, Some(&filename))?;

    Ok(CallToolResult {
        isError: false,
        content: vec![TextContent {
            type_: "text".to_string(),
            text: json!({
                "generated_code": generated_code,
                "lines": generated_code.lines().count(),
                "size_bytes": generated_code.len(),
                "framework": "express"
            }).to_string()
        }]
    })
}
```

#### Step 5.4: Update Policy Module Registration

```rust
pub async fn register(server: &Server) -> anyhow::Result<()> {
    server.register_tool(policy_validate).await?;
    server.register_tool(policy_generate_express).await?;
    Ok(())
}
```

**Validation:**
- ✅ All 7 tools implemented
- ✅ No subprocess overhead (<1ms P95 latency)
- ✅ Type-safe Rust → Rust integration
- ✅ 60%+ test coverage

---

### Day 4 (final): Integration Tests

#### Create Integration Test Suite

`tests/integration_test.rs`:
```rust
use x402_mcp_server::tools;
use rmcp::Server;

#[tokio::test]
async fn test_complete_workflow() {
    // Setup server
    let server = create_test_server().await;

    // 1. Start mock server
    let start_result = tools::mock::server_mock_start(3402, 0.01).await.unwrap();
    assert!(!start_result.isError);

    // 2. Check status
    let status_result = tools::mock::server_mock_status().await.unwrap();
    assert!(!status_result.isError);

    // 3. Validate policy
    let policy_yaml = r#"
    policies:
      - type: rate_limit
        pattern: "/api/*"
        max_requests: 100
    "#.to_string();
    let validate_result = tools::policy::policy_validate(policy_yaml.clone()).await.unwrap();
    assert!(!validate_result.isError);

    // 4. Check compliance
    let compliance_result = tools::testing::testing_check_compliance(
        "http://localhost:3402/api".to_string(),
        None,
        None,
        10
    ).await.unwrap();
    assert!(!compliance_result.isError);

    // 5. Stop server
    let stop_result = tools::mock::server_mock_stop().await.unwrap();
    assert!(!stop_result.isError);
}

#[tokio::test]
async fn test_error_handling() {
    // Test server not running error
    let status = tools::mock::server_mock_status().await.unwrap();
    // Should succeed but show not_running status

    // Test invalid port
    let result = tools::mock::server_mock_start(99, 0.01).await;
    assert!(result.is_err());  // Should fail with E3002
}
```

**Validation:**
- ✅ End-to-end workflow test passes
- ✅ Error paths tested
- ✅ All 7 tools covered
- ✅ Integration tests pass with `cargo test --test integration_test`

---

## Phase 3: Testing & Documentation (Day 5)

**Goal:** Achieve production quality with comprehensive tests and documentation

### Day 5 Morning: Increase Test Coverage

#### Unit Tests for Error Translation

`tests/error_translation_test.rs`:
```rust
use x402_mcp_server::error::McpError;
use x402_core::Error as CoreError;

#[test]
fn test_port_in_use_error() {
    let core_error = CoreError::PortInUse { port: 3402, pid: 12345 };
    let mcp_result: CallToolResult = core_error.into();

    assert!(mcp_result.isError);
    let text = &mcp_result.content[0].text;
    assert!(text.contains("E3001"));
    assert!(text.contains("Port 3402"));
    assert!(text.contains("x402__server_mock_stop"));
}

#[test]
fn test_invalid_yaml_error() {
    let core_error = CoreError::YamlParse("missing field `type`".to_string());
    let mcp_result: CallToolResult = core_error.into();

    assert!(mcp_result.isError);
    assert!(mcp_result.content[0].text.contains("E4001"));
}
```

#### Performance Benchmarks

`benches/tool_benchmarks.rs`:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use x402_mcp_server::tools;

fn benchmark_mock_start(c: &mut Criterion) {
    c.bench_function("server_mock_start", |b| {
        b.iter(|| {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                tools::mock::server_mock_start(black_box(3402), black_box(0.01)).await
            })
        });
    });
}

fn benchmark_policy_validate(c: &mut Criterion) {
    let policy_yaml = r#"
    policies:
      - type: rate_limit
        pattern: "/api/*"
        max_requests: 100
    "#;

    c.bench_function("policy_validate", |b| {
        b.iter(|| {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                tools::policy::policy_validate(black_box(policy_yaml.to_string())).await
            })
        });
    });
}

criterion_group!(benches, benchmark_mock_start, benchmark_policy_validate);
criterion_main!(benches);
```

**Run Benchmarks:**
```bash
cargo bench --bench tool_benchmarks
```

**Expected Results:**
- `server_mock_start`: <1ms P95 latency
- `policy_validate`: <1ms P95 latency
- All tools: sub-millisecond overhead

### Day 5 Afternoon: Documentation

#### Create Comprehensive README.md

```markdown
# x402-mcp-server

Rust MCP server for x402-dev payment protocol testing toolkit.

## Installation

\`\`\`bash
# Install from crates.io
cargo install x402-mcp-server

# Add to Claude Code
claude mcp add x402-mcp x402-mcp-server
\`\`\`

## Quick Start

Ask Claude Code:
\`\`\`
"Start an x402 mock server on port 3402 and validate my payment endpoint"
\`\`\`

## Features

- ✅ **7 workflow tools** for payment testing
- ✅ **<1ms latency** (direct library integration)
- ✅ **Type-safe** Rust implementation
- ✅ **Zero subprocess overhead**
- ✅ **Structured errors** with AI-friendly suggestions

## Architecture

Direct Rust library integration (not subprocess):
- x402-server → start_server() (0ms overhead)
- x402-core → validate_policies() (0ms overhead)
- No temp files, no command injection, no text parsing

See [API-REFERENCE.md](docs/API-REFERENCE.md) for complete tool documentation.
\`\`\`

#### Generate API Documentation

```bash
# Generate rustdoc
cargo doc --no-deps --open

# Publish to docs.rs (automatic on crates.io publish)
```

### Day 5 Evening: Example Workflows

#### Workflow 1: Complete Payment API Testing

`examples/payment_workflow.rs`:
```rust
//! Complete workflow: Start server → Validate policy → Test endpoint → Stop server

use x402_mcp_server::tools;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 Starting payment API testing workflow...\n");

    // 1. Start mock server
    println!("1️⃣ Starting mock server...");
    let start_result = tools::mock::server_mock_start(3402, 0.01).await?;
    println!("   ✅ Server started: {}\n", start_result.content[0].text);

    // 2. Validate policy
    println!("2️⃣ Validating payment policy...");
    let policy_yaml = r#"
    policies:
      - type: rate_limit
        pattern: "/api/*"
        max_requests: 100
        window: 3600
    "#.to_string();
    let validate_result = tools::policy::policy_validate(policy_yaml.clone()).await?;
    println!("   ✅ Policy valid: {}\n", validate_result.content[0].text);

    // 3. Generate Express middleware
    println!("3️⃣ Generating Express middleware...");
    let generate_result = tools::policy::policy_generate_express(
        policy_yaml,
        "policy.yaml".to_string()
    ).await?;
    println!("   ✅ Middleware generated ({} lines)\n",
        generate_result.content[0].text.lines().count());

    // 4. Check endpoint compliance
    println!("4️⃣ Checking endpoint compliance...");
    let check_result = tools::testing::testing_check_compliance(
        "http://localhost:3402/api".to_string(),
        None,
        None,
        10
    ).await?;
    println!("   ✅ Compliance check: {}\n", check_result.content[0].text);

    // 5. Stop server
    println!("5️⃣ Stopping mock server...");
    let stop_result = tools::mock::server_mock_stop().await?;
    println!("   ✅ Server stopped: {}\n", stop_result.content[0].text);

    println!("🎉 Workflow completed successfully!");
    Ok(())
}
```

**Run Example:**
```bash
cargo run --example payment_workflow
```

**Validation:**
- ✅ 80%+ test coverage achieved (`cargo tarpaulin`)
- ✅ Benchmarks pass (<1ms P95)
- ✅ Documentation complete (README + rustdoc)
- ✅ Example workflows work end-to-end

---

## Phase 4: Production Release (Day 6)

**Goal:** Security audit, crates.io publication, MCP directory listing

### Day 6 Morning: Security Audit

#### Run Security Scans

```bash
# 1. Audit dependencies for vulnerabilities
cargo audit

# 2. Check for unsafe code
rg "unsafe " src/

# 3. Run clippy with pedantic lints
cargo clippy -- -W clippy::all -W clippy::pedantic

# 4. Format check
cargo fmt --check

# 5. Check for TODO/FIXME
rg "TODO|FIXME" src/
```

#### Security Checklist

- ✅ No `unsafe` blocks in codebase
- ✅ Zero critical dependencies vulnerabilities (`cargo audit`)
- ✅ All clippy warnings resolved
- ✅ No hardcoded secrets or credentials
- ✅ Input validation on all tool parameters
- ✅ Error messages don't leak sensitive information
- ✅ Rate limiting implemented (if needed)
- ✅ Timeout enforcement on all async operations

### Day 6 Afternoon: Crates.io Publication

#### Prepare Cargo.toml for Publication

```toml
[package]
name = "x402-mcp-server"
version = "0.1.0"
edition = "2024"
rust-version = "1.85.0"
authors = ["x402-dev Team <dev@x402.io>"]
license = "MIT OR Apache-2.0"
description = "Rust MCP server for x402-dev payment protocol testing toolkit with direct library integration"
repository = "https://github.com/x402-dev/x402-mcp-server"
homepage = "https://x402.io/mcp"
documentation = "https://docs.rs/x402-mcp-server"
readme = "README.md"
keywords = ["mcp", "x402", "testing", "solana", "payment"]
categories = ["development-tools", "testing", "web-programming"]

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

#### Publish to Crates.io

```bash
# 1. Dry run (test packaging)
cargo publish --dry-run

# 2. Review generated package
cargo package --list

# 3. Actually publish
cargo publish

# 4. Verify on crates.io
open https://crates.io/crates/x402-mcp-server
```

### Day 6 Evening: MCP Directory & Announcement

#### Submit to MCP Directory

Create `mcp-directory.json`:
```json
{
  "name": "x402-mcp",
  "vendor": "x402-dev",
  "description": "Rust MCP server for x402 payment protocol testing with <1ms latency",
  "version": "0.1.0",
  "homepage": "https://x402.io/mcp",
  "repository": "https://github.com/x402-dev/x402-mcp-server",
  "install_command": "cargo install x402-mcp-server",
  "integration_command": "claude mcp add x402-mcp x402-mcp-server",
  "tools": [
    "x402__server_mock_start",
    "x402__server_mock_stop",
    "x402__server_mock_status",
    "x402__testing_run_suite",
    "x402__testing_check_compliance",
    "x402__policy_validate",
    "x402__policy_generate_express"
  ],
  "tags": ["testing", "payment", "solana", "x402", "mock-server"]
}
```

#### Community Announcement

**Post to:**
- r/ClaudeAI (Reddit)
- r/solana (Reddit)
- x402-dev Discord
- Twitter/X with demo video
- Solana Hackathon Discord (Sept-Oct 2025)

**Announcement Template:**
```
🚀 Introducing x402-mcp-server: Rust MCP Testing Toolkit

Built with Rust for 10-1000x faster performance (<1ms vs 50-200ms)

✅ 7 workflow tools for payment API testing
✅ Direct library integration (zero subprocess overhead)
✅ Type-safe with compile-time guarantees
✅ Perfect for AI-assisted development

Install: cargo install x402-mcp-server
Docs: https://docs.rs/x402-mcp-server
GitHub: https://github.com/x402-dev/x402-mcp-server

#Rust #MCP #Solana #x402 #AI
```

**Validation:**
- ✅ Security audit passed (0 critical vulnerabilities)
- ✅ Published to crates.io
- ✅ MCP directory submission complete
- ✅ Community announcements posted
- ✅ Documentation live on docs.rs

---

**🎉 PROJECT COMPLETE! All 7 tools implemented, tested, documented, and published!**

---

## Troubleshooting Guide

### Common Issues & Solutions

#### Issue 1: Compilation Errors with rmcp

**Symptom:**
```
error[E0433]: failed to resolve: use of undeclared crate or module `rmcp`
```

**Solution:**
```bash
# Verify rmcp is in Cargo.toml dependencies
cargo tree | grep rmcp

# If missing, add to Cargo.toml:
rmcp = "^0.8"

# Clean and rebuild
cargo clean
cargo build --release
```

---

#### Issue 2: x402-core Function Not Found

**Symptom:**
```
error[E0425]: cannot find function `start_server` in crate `x402_server`
```

**Solution:**
- Check that x402-dev source code is cloned at correct path
- Verify path dependencies in Cargo.toml:
  ```toml
  x402-core = { path = "../x402-dev/crates/x402-core" }
  x402-server = { path = "../x402-dev/crates/x402-server" }
  x402-cli = { path = "../x402-dev/crates/x402-cli" }
  ```
- Ensure x402-dev builds successfully: `cd ../x402-dev && cargo build`

---

#### Issue 3: MCP stdio Transport Not Working

**Symptom:**
- Claude Code shows "MCP server not responding"
- No JSON-RPC messages appearing

**Solution:**
1. Check that logging goes to stderr ONLY:
   ```rust
   // ✅ CORRECT: Log to stderr
   eprintln!("Starting server...");

   // ❌ WRONG: Never use println! (stdout is for JSON-RPC)
   println!("Starting server...");  // BREAKS MCP!
   ```

2. Verify stdio transport setup:
   ```rust
   server.serve_stdio().await?;  // Blocks until shutdown
   ```

3. Test manually:
   ```bash
   # Run server directly (should wait for JSON-RPC input)
   ./target/release/x402-mcp-server

   # Send MCP initialize request
   echo '{"jsonrpc":"2.0","id":1,"method":"initialize"}' | ./target/release/x402-mcp-server
   ```

---

#### Issue 4: Tool Not Appearing in Claude Code

**Symptom:**
- Tool implemented but not showing in MCP tool list
- Claude Code doesn't recognize tool name

**Solution:**
1. Verify tool registration:
   ```rust
   // In src/tools/mod.rs
   pub async fn register_all(server: &Server) -> anyhow::Result<()> {
       mock::register(server).await?;  // Registered?
       testing::register(server).await?;
       policy::register(server).await?;
       Ok(())
   }
   ```

2. Check tool attribute:
   ```rust
   #[tool(
       name = "x402__server_mock_start",  // Must start with x402__
       description = "Start mock payment server",
   )]
   pub async fn server_mock_start(...)
   ```

3. Restart Claude Code after rebuilding server

---

#### Issue 5: Performance Below Target (<1ms)

**Symptom:**
- Benchmarks show >5ms P95 latency
- Tools feel slow

**Solution:**
1. Ensure `--release` build:
   ```bash
   cargo build --release  # NOT cargo build (debug mode is 10x slower)
   cargo bench            # Benchmarks automatically use --release
   ```

2. Profile with criterion:
   ```bash
   cargo bench --bench tool_benchmarks
   # Review reports in target/criterion/
   ```

3. Check for accidental blocking operations:
   ```rust
   // ❌ WRONG: Blocking I/O in async function
   let config = std::fs::read_to_string("config.yaml")?;

   // ✅ CORRECT: Use tokio async I/O
   let config = tokio::fs::read_to_string("config.yaml").await?;
   ```

---

#### Issue 6: Test Command Exits Entire Process

**Symptom:**
```
error: process didn't exit successfully: `x402-mcp-server` (signal: 11, SIGSEGV: invalid memory reference)
```

**Solution:**
- This means Day 0 refactoring wasn't completed!
- The test command still calls `std::process::exit()`
- **MUST refactor test command** to return `Result<TestResult>` instead of calling `exit()`
- See TECHNICAL-APPENDIX.md Section A.4 for refactoring details

---

#### Issue 7: Async Runtime Panic

**Symptom:**
```
thread 'tokio-runtime-worker' panicked at 'Cannot start a runtime from within a runtime'
```

**Solution:**
- Using `#[tokio::main]` in tool functions (wrong!)
- Tool functions should be async but NOT create new runtimes:
  ```rust
  // ✅ CORRECT
  pub async fn server_mock_start(...) -> Result<CallToolResult> {
      x402_server::start_server(...).await  // Just await, don't create runtime
  }

  // ❌ WRONG
  pub async fn server_mock_start(...) -> Result<CallToolResult> {
      tokio::runtime::Runtime::new()?.block_on(async {  // DON'T DO THIS!
          x402_server::start_server(...).await
      })
  }
  ```

---

#### Issue 8: Error Translation Not Working

**Symptom:**
- Errors returned as generic "Unknown error" instead of structured E3001, E4001, etc.

**Solution:**
1. Implement `From<x402_core::Error> for CallToolResult`:
   ```rust
   impl From<x402_core::Error> for CallToolResult {
       fn from(err: x402_core::Error) -> Self {
           // Map specific errors to MCP error codes
       }
   }
   ```

2. Use `?` operator to automatically convert:
   ```rust
   let result = x402_core::some_function()?;  // Auto-converts via From trait
   ```

---

#### Issue 9: cargo audit Failures

**Symptom:**
```
warning: 3 vulnerabilities found!
```

**Solution:**
```bash
# Update vulnerable dependencies
cargo update

# If specific crate needs updating:
cargo update -p <crate-name>

# Check advisory details:
cargo audit --deny warnings

# If unmaintained crate, find alternative or accept risk
```

---

#### Issue 10: Integration Test Failures

**Symptom:**
```
test test_complete_workflow ... FAILED
```

**Solution:**
1. Check test isolation (each test should clean up):
   ```rust
   #[tokio::test]
   async fn test_workflow() {
       // Setup
       cleanup_test_servers().await;

       // Test
       // ...

       // Teardown
       cleanup_test_servers().await;
   }
   ```

2. Run tests sequentially if they share resources:
   ```bash
   cargo test -- --test-threads=1
   ```

3. Check test timeouts:
   ```rust
   #[tokio::test(flavor = "multi_thread")]
   #[timeout(Duration::from_secs(30))]  // Add timeout
   async fn test_long_running() { ... }
   ```

---

### Performance Debugging

If tools are slower than expected (<1ms target):

1. **Profile with perf** (Linux):
   ```bash
   cargo build --release
   perf record --call-graph dwarf ./target/release/x402-mcp-server
   perf report
   ```

2. **Use criterion flamegraphs**:
   ```bash
   cargo install cargo-flamegraph
   cargo flamegraph --bench tool_benchmarks
   ```

3. **Check for common bottlenecks**:
   - Unnecessary cloning (`clone()` everywhere)
   - Synchronous I/O in async functions
   - Excessive allocations
   - Unoptimized JSON serialization

---

### Getting Help

If issues persist:

1. **Check x402-dev issues**: https://github.com/x402-dev/x402-dev/issues
2. **Check rmcp SDK docs**: https://docs.rs/rmcp
3. **Review TECHNICAL-APPENDIX.md** for architecture details
4. **Enable debug logging**:
   ```rust
   env_logger::init();  // Add to main.rs
   RUST_LOG=debug ./target/release/x402-mcp-server
   ```

---

## Common Patterns

### Pattern 1: Tool Implementation Template

```typescript
// 1. Define schema
export const myToolSchema = z.object({
  param1: z.string(),
  param2: z.number().optional()
});

// 2. Implement tool
export async function myTool(params: z.infer<typeof myToolSchema>): Promise<ToolResponse> {
  // Validate
  const validated = myToolSchema.parse(params);

  // Execute
  const result = await execX402Dev('command', [validated.param1]);

  // Return
  return successResponse(result);
}

// 3. Register in server
server.setRequestHandler(CallToolRequestSchema, async (request) => {
  if (request.params.name === 'x402__my_tool') {
    return await myTool(request.params.arguments);
  }
});
```

### Pattern 2: Error Handling

```typescript
try {
  const result = await execX402Dev('command', args);
  return successResponse(result);
} catch (err) {
  return errorResponse({
    code: 'E3XXX',
    message: 'Error description',
    suggestion: 'How to fix',
    context: { command: 'command', args, error: err.message }
  });
}
```

### Pattern 3: Subprocess with Timeout

```typescript
const result = await execX402Dev('command', args, {
  timeout: 30000,  // 30 seconds
  background: false,
  cwd: '/some/path'
});
```

---

## Testing Strategy

### Unit Tests (80%+ coverage)

```typescript
describe('serverMockStart', () => {
  it('validates parameters', () => {
    expect(() => mockStartSchema.parse({ port: 99 })).toThrow();
    expect(() => mockStartSchema.parse({ port: 70000 })).toThrow();
  });

  it('detects already running server', async () => {
    // Mock PID file
    await writePidFile(12345);
    const result = await serverMockStart({ port: 3402 });
    expect(result.error.code).toBe('E3001');
  });
});
```

### Integration Tests (60%+ coverage)

```typescript
describe('End-to-end workflow', () => {
  beforeAll(async () => {
    // Ensure clean state
    await stopAllServers();
  });

  it('completes full workflow', async () => {
    // Test actual tool execution
  });
});
```

---

**Total:** ~3,500 words | Step-by-step implementation guide

For API specifications, see `API-REFERENCE.md`.
For technical details, see `TECHNICAL-APPENDIX.md`.
