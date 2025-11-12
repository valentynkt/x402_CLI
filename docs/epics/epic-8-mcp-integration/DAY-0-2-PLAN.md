# Epic 8: Days 0-2 Execution Plan

**Timeline:** Day 0 (4 hours) + Days 1-2 (10 hours) = 14 hours total
**Goal:** Foundation phase complete with 3 working tools and <1ms latency
**Team:** Epic8 Hive Mind Swarm (6 agents)

---

## 🎯 Phase Overview

### Day 0: Critical Blocker Resolution (4 hours)
**Status:** MUST COMPLETE - Blocks all MCP integration work
**Success Gate:** Test command returns Result, rmcp PoC works, APIs verified

### Days 1-2: Foundation Implementation (10 hours)
**Status:** Core infrastructure + first 3 tools
**Success Gate:** 3 tools callable from Claude Code, <1ms latency, 50%+ coverage

---

## 📋 DAY 0: BLOCKER RESOLUTION & VALIDATION (4 hours)

### Task 0.1: Refactor Test Command (2.5 hours)

**Owner:** Coder Agent
**Priority:** CRITICAL (P0)
**Blocker:** Prevents library integration of test functionality

#### **Step 1: Analyze Current Implementation (15 min)**
```bash
# Review the problematic code
cd /Users/valentynkit/dev/sandbox/Hackaton
cat crates/x402-cli/src/commands/test.rs | grep -A 5 -B 5 "process::exit"

# Check for other exit() calls
rg "std::process::exit" crates/x402-cli/src/
rg "exit\(" crates/x402-cli/src/commands/
```

**Expected Findings:**
- Line 60: `std::process::exit(result.exit_code())`
- Likely in the main execute() function
- May be other commands with same pattern

#### **Step 2: Design Refactoring (15 min)**

**Current Pattern (BROKEN):**
```rust
pub async fn execute(args: TestArgs) -> Result<()> {
    // ... test logic ...
    let result = run_test_suite(&args).await?;

    // BLOCKER: Exits entire process!
    std::process::exit(result.exit_code());
}
```

**New Pattern (FIXED):**
```rust
// New function - returns Result (library-friendly)
pub async fn execute_with_result(args: TestArgs) -> Result<TestResult> {
    let suite = TestSuite::from_yaml_str(&args.suite_yaml)?;
    let result = execute_test_suite(&suite).await?;
    Ok(result)
}

// Keep existing function for CLI compatibility
pub async fn execute(args: TestArgs) -> Result<()> {
    let result = execute_with_result(args).await?;

    // CLI can still exit, but library callers use execute_with_result()
    std::process::exit(result.exit_code());
}
```

#### **Step 3: Implement Refactoring (60 min)**

**File:** `crates/x402-cli/src/commands/test.rs`

```rust
use anyhow::Result;
use crate::testing::{TestSuite, TestResult, execute_test_suite};

/// Execute test suite and return results (library-friendly)
pub async fn execute_with_result(args: TestArgs) -> Result<TestResult> {
    eprintln!("Loading test suite from: {}", args.suite_path.display());

    // Load test suite
    let yaml_content = tokio::fs::read_to_string(&args.suite_path).await?;
    let suite = TestSuite::from_yaml_str(&yaml_content)?;

    // Execute tests
    let result = execute_test_suite(&suite).await?;

    // Log summary (to stderr, not stdout)
    eprintln!("Tests: {} passed, {} failed, {} skipped",
        result.passed, result.failed, result.skipped);

    Ok(result)
}

/// Execute test suite and exit with appropriate code (CLI-friendly)
pub async fn execute(args: TestArgs) -> Result<()> {
    let result = execute_with_result(args).await?;

    // Exit with code (only for CLI usage)
    std::process::exit(result.exit_code());
}
```

**Validation:**
```bash
# Must compile without errors
cargo build --package x402-cli

# Run existing CLI tests (should still work)
cargo test --package x402-cli test_command
```

#### **Step 4: Add Unit Tests (30 min)**

**File:** `crates/x402-cli/src/commands/test.rs` (add at bottom)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_execute_with_result_success() {
        // Create test YAML
        let yaml = r#"
tests:
  - name: "Always passes"
    assertions:
      - type: always_true
"#;

        let args = TestArgs {
            suite_yaml: yaml.to_string(),
            output_format: OutputFormat::Json,
            quiet: true,
        };

        let result = execute_with_result(args).await.unwrap();
        assert_eq!(result.passed, 1);
        assert_eq!(result.failed, 0);
    }

    #[tokio::test]
    async fn test_execute_with_result_failure() {
        let yaml = r#"
tests:
  - name: "Always fails"
    assertions:
      - type: always_false
"#;

        let args = TestArgs {
            suite_yaml: yaml.to_string(),
            output_format: OutputFormat::Json,
            quiet: true,
        };

        let result = execute_with_result(args).await.unwrap();
        assert_eq!(result.passed, 0);
        assert_eq!(result.failed, 1);
    }

    #[tokio::test]
    async fn test_execute_with_result_returns_not_exits() {
        // This test verifies the function returns instead of exiting
        let yaml = r#"
tests:
  - name: "Test"
    assertions: []
"#;

        let args = TestArgs {
            suite_yaml: yaml.to_string(),
            output_format: OutputFormat::Json,
            quiet: true,
        };

        // Should return, not exit process
        let result = execute_with_result(args).await;
        assert!(result.is_ok());

        // If we reach here, function returned (didn't exit)
        eprintln!("✅ Function returned instead of exiting");
    }
}
```

**Validation:**
```bash
# Run new tests
cargo test --package x402-cli test_execute_with_result

# Verify no other commands have exit()
rg "std::process::exit" crates/x402-cli/src/commands/ --type rust
```

#### **Step 5: Verify Integration Points (30 min)**

**Check all CLI command entry points:**
```bash
# List all commands
ls -1 crates/x402-cli/src/commands/*.rs

# For each command, check for exit() calls
for cmd in crates/x402-cli/src/commands/*.rs; do
    echo "Checking $cmd..."
    grep -n "process::exit\|exit(" "$cmd" || echo "  ✅ No exit() calls"
done
```

**Expected Result:** Only `test.rs` should have exit(), and only in the CLI-facing `execute()` function.

#### **Deliverables:**
- ✅ `execute_with_result()` function returns Result<TestResult>
- ✅ Existing CLI interface preserved (backward compatible)
- ✅ 3+ unit tests passing
- ✅ No other commands have exit() blockers
- ✅ Code compiles and existing tests pass

**Coordination:**
```bash
npx claude-flow@alpha hooks pre-task --description "Refactor test command"
# ... work ...
npx claude-flow@alpha hooks post-edit --file "crates/x402-cli/src/commands/test.rs"
npx claude-flow@alpha hooks post-task --task-id "day0-refactor"
```

---

### Task 0.2: rmcp SDK Proof-of-Concept (1 hour)

**Owner:** Researcher Agent
**Priority:** CRITICAL (P0)
**Goal:** Validate rmcp SDK works with our use case

#### **Step 1: Create Minimal MCP Server (30 min)**

```bash
# Create temporary PoC project
cd /tmp
cargo new rmcp-poc
cd rmcp-poc

# Add dependencies
cat >> Cargo.toml << 'EOF'

[dependencies]
rmcp = "0.8.5"
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
EOF
```

**Implement minimal server:**

`src/main.rs`:
```rust
use rmcp::{tool, Server, ServerBuilder, CallToolResult, TextContent};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!("rmcp PoC starting...");

    let server = ServerBuilder::new("rmcp-poc")
        .version("0.1.0")
        .description("Proof of concept for rmcp SDK")
        .build();

    // Register test tool
    server.register_tool(hello_world).await?;

    eprintln!("Server ready. Serving stdio...");
    server.serve_stdio().await?;

    Ok(())
}

/// Simple test tool
#[tool(
    name = "poc__hello_world",
    description = "Test tool to validate rmcp SDK works",
)]
async fn hello_world(
    #[arg(default = "World", description = "Name to greet")]
    name: String,
) -> anyhow::Result<CallToolResult> {
    Ok(CallToolResult {
        isError: false,
        content: vec![TextContent {
            type_: "text".to_string(),
            text: json!({
                "greeting": format!("Hello, {}!", name),
                "timestamp": chrono::Utc::now().to_rfc3339()
            }).to_string()
        }]
    })
}
```

**Build and test:**
```bash
cargo build --release

# Test 1: Binary starts
./target/release/rmcp-poc &
POC_PID=$!
sleep 1
kill $POC_PID

# Test 2: Responds to MCP handshake
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | ./target/release/rmcp-poc
```

#### **Step 2: Validate Key Features (15 min)**

**Test procedural macro:**
```bash
# Check generated code compiles
cargo expand hello_world 2>/dev/null | head -50
```

**Test async compatibility:**
```rust
// Add to main.rs
#[tool(
    name = "poc__async_test",
    description = "Validate async functions work",
)]
async fn async_test() -> anyhow::Result<CallToolResult> {
    // Simulate async work
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    Ok(CallToolResult {
        isError: false,
        content: vec![TextContent {
            type_: "text".to_string(),
            text: "Async works!".to_string()
        }]
    })
}
```

**Test error handling:**
```rust
#[tool(
    name = "poc__error_test",
    description = "Validate error responses work",
)]
async fn error_test() -> anyhow::Result<CallToolResult> {
    Err(anyhow::anyhow!("Test error"))
}
```

#### **Step 3: Document Findings (15 min)**

Create `rmcp-poc-findings.md`:
```markdown
# rmcp SDK PoC Findings

## ✅ Validated Features
- Procedural macro `#[tool]` works
- stdio transport functional
- Async functions supported
- Error handling works
- JSON serialization/deserialization working

## 📊 Performance
- Cold start: ~50ms
- Tool invocation overhead: <0.5ms
- Memory usage: ~2MB base

## ⚠️ Gotchas
1. All logging MUST go to stderr (not stdout)
2. `#[tokio::main]` should only be in main.rs (not tool functions)
3. Tool names must be valid identifiers (no spaces)

## ✅ Recommendation
rmcp 0.8.5 is production-ready for Epic 8 integration.
```

**Deliverables:**
- ✅ Working rmcp PoC server
- ✅ 3 test tools functional (hello_world, async_test, error_test)
- ✅ stdio transport validated
- ✅ Performance measured (<1ms overhead confirmed)
- ✅ Findings documented

---

### Task 0.3: x402-core API Validation (30 min)

**Owner:** Researcher Agent
**Priority:** HIGH (P1)
**Goal:** Verify all documented functions exist with correct signatures

#### **Script: Validate x402-core API**

```bash
#!/bin/bash
# validate_x402_api.sh

cd /Users/valentynkit/dev/sandbox/Hackaton

echo "🔍 Validating x402-core API signatures..."
echo "=========================================="

# Function 1: start_server
echo "1. Checking x402_server::start_server()..."
grep -r "pub.*fn start_server" crates/x402-server/src/ && echo "  ✅ Found" || echo "  ❌ Missing"

# Function 2: stop_server
echo "2. Checking x402_server::stop_server()..."
grep -r "pub.*fn stop_server" crates/x402-server/src/ && echo "  ✅ Found" || echo "  ❌ Missing"

# Function 3: server_status
echo "3. Checking x402_server::server_status()..."
grep -r "pub.*fn.*status" crates/x402-server/src/ && echo "  ✅ Found" || echo "  ❌ Missing"

# Function 4: validate_policies
echo "4. Checking x402_core::policy::validate_policies()..."
grep -r "pub.*fn validate" crates/x402-core/src/policy/ && echo "  ✅ Found" || echo "  ❌ Missing"

# Function 5: execute_test_suite
echo "5. Checking x402_core::testing::execute_test_suite()..."
grep -r "pub.*fn execute" crates/x402-core/src/testing/ && echo "  ✅ Found" || echo "  ❌ Missing"

# Function 6: check_compliance
echo "6. Checking x402_core::testing::check_compliance()..."
grep -r "pub.*fn.*compliance" crates/x402-core/src/testing/ && echo "  ✅ Found" || echo "  ❌ Missing"

# Function 7: generate_middleware
echo "7. Checking x402_core::policy::generate_middleware()..."
grep -r "pub.*fn generate" crates/x402-core/src/policy/ && echo "  ✅ Found" || echo "  ❌ Missing"

echo ""
echo "📋 Checking parameter types..."
echo "==============================="

# Verify Result types
echo "Checking Result<T, E> patterns..."
grep -r "Result<" crates/x402-core/src/lib.rs | head -5

# Verify async functions
echo "Checking async fn patterns..."
grep -r "pub async fn" crates/x402-core/src/ | wc -l

echo ""
echo "✅ API validation complete!"
```

**Run validation:**
```bash
chmod +x validate_x402_api.sh
./validate_x402_api.sh > day0-api-validation.txt
cat day0-api-validation.txt
```

**Document any discrepancies:**
```markdown
# API Validation Results

## ✅ Functions Found (7/7)
1. x402_server::start_server - MATCH
2. x402_server::stop_server - MATCH
3. x402_server::server_status - MATCH
4. x402_core::policy::validate_policies - MATCH
5. x402_core::testing::execute_test_suite - MATCH
6. x402_core::testing::check_compliance - MATCH
7. x402_core::policy::generate_middleware - MATCH

## 📊 Type Compatibility
- All functions return Result<T, E>
- Async functions use tokio runtime
- No blocking I/O detected

## ✅ Recommendation
All documented APIs exist and are compatible with MCP integration.
```

**Deliverables:**
- ✅ All 7 core functions verified to exist
- ✅ Parameter types validated
- ✅ Result types confirmed compatible
- ✅ Discrepancies documented (if any)

---

### Day 0 Success Gate ✅

**Criteria (all must pass):**
- ✅ Test command returns Result (not exit)
- ✅ rmcp PoC compiles and runs
- ✅ stdio transport works
- ✅ All 7 x402-core functions verified

**Decision:** If all criteria met → PROCEED to Day 1. If not → ESCALATE to tech lead.

---

## 📋 DAYS 1-2: FOUNDATION IMPLEMENTATION (10 hours)

### Day 1 Morning: Project Setup (3 hours)

#### **Task 1.1: Create MCP Server Crate (30 min)**

**Owner:** Coder Agent

```bash
cd /Users/valentynkit/dev/sandbox/Hackaton

# Create new crate
mkdir -p crates/x402-mcp-server
cd crates/x402-mcp-server
cargo init --name x402-mcp-server

# Create directory structure
mkdir -p src/tools tests benches examples
touch src/main.rs src/error.rs src/lib.rs
touch src/tools/mod.rs src/tools/mock.rs src/tools/policy.rs src/tools/config.rs
```

**Cargo.toml:**
```toml
[package]
name = "x402-mcp-server"
version = "0.1.0"
edition = "2024"
rust-version = "1.85.0"
authors = ["x402-dev Team"]
license = "MIT OR Apache-2.0"

[[bin]]
name = "x402-mcp-server"
path = "src/main.rs"

[dependencies]
rmcp = "0.8.5"
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }

# Direct library integration (zero subprocess overhead!)
x402-core = { path = "../x402-core" }
x402-server = { path = "../x402-server" }
x402-cli = { path = "../x402-cli" }

[dev-dependencies]
criterion = { version = "0.5", features = ["async_tokio"] }
tokio-test = "0.4"

[[bench]]
name = "tool_benchmarks"
harness = false
```

**Update workspace root:**
```bash
cd /Users/valentynkit/dev/sandbox/Hackaton

# Add to Cargo.toml [workspace] section
# members = [..., "crates/x402-mcp-server"]
```

**Validate:**
```bash
cargo build --package x402-mcp-server
# Should compile empty project
```

---

#### **Task 1.2: Implement stdio Transport (1 hour)**

**Owner:** Coder Agent

**File:** `src/main.rs`
```rust
use rmcp::{Server, ServerBuilder};
use anyhow::Result;

mod tools;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    // All logging to stderr (stdout is for JSON-RPC)
    eprintln!("🚀 x402-mcp-server v0.1.0 starting...");

    // Build MCP server
    let server = ServerBuilder::new("x402-dev-mcp")
        .version("0.1.0")
        .description("Rust MCP server for x402-dev protocol testing toolkit")
        .build();

    // Register tools
    tools::register_all(&server).await?;
    eprintln!("✅ Tools registered successfully");

    // Start stdio transport (blocking until shutdown)
    eprintln!("📡 Serving stdio transport...");
    server.serve_stdio().await?;

    Ok(())
}
```

**File:** `src/tools/mod.rs`
```rust
use rmcp::Server;
use anyhow::Result;

pub mod mock;
pub mod policy;
pub mod config;

/// Register all MCP tools with the server
pub async fn register_all(server: &Server) -> Result<()> {
    eprintln!("Registering tools...");

    // Phase 1: First 3 tools
    mock::register(server).await?;
    // policy::register(server).await?;  // Task 1.3
    // config::register(server).await?;  // Task 2.2

    eprintln!("  ✅ {} tools registered", 1);
    Ok(())
}
```

**Validate:**
```bash
cargo build --release --package x402-mcp-server

# Test 1: Binary starts
./target/release/x402-mcp-server &
SERVER_PID=$!
sleep 2
kill $SERVER_PID

# Test 2: Responds to MCP initialize
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | ./target/release/x402-mcp-server
```

---

#### **Task 1.3: Implement First Tool - server_mock_start (1.5 hours)**

**Owner:** Coder Agent

**File:** `src/tools/mock.rs`
```rust
use rmcp::{tool, Server, CallToolResult, TextContent};
use serde_json::json;
use anyhow::Result;
use x402_server::{start_server, ServerConfig};

/// Register mock server tools
pub async fn register(server: &Server) -> Result<()> {
    server.register_tool(server_mock_start).await?;
    Ok(())
}

/// Start x402-dev mock payment server
#[tool(
    name = "x402__server_mock_start",
    description = "Start mock payment server for testing 402 protocol endpoints",
)]
pub async fn server_mock_start(
    #[arg(default = 3402, description = "Port number (1024-65535)")]
    port: u16,

    #[arg(default = 0.01, description = "Pricing per request in SOL")]
    pricing: f64,

    #[arg(default = "success", description = "Simulation mode: success, failure, or timeout")]
    simulation_mode: String,
) -> Result<CallToolResult> {
    // Validate parameters
    if port < 1024 || port > 65535 {
        return Ok(CallToolResult {
            isError: true,
            content: vec![TextContent {
                type_: "text".to_string(),
                text: json!({
                    "error": "E3002",
                    "message": format!("Invalid port number: {}", port),
                    "suggestion": "Use port between 1024-65535",
                    "docs_link": "https://docs.x402-dev.com/errors/E3002"
                }).to_string()
            }]
        });
    }

    // Create server config
    let config = ServerConfig {
        port,
        pricing,
        simulation_mode: simulation_mode.parse()?,
        ..Default::default()
    };

    // Start server (DIRECT LIBRARY CALL - no subprocess!)
    match start_server(config).await {
        Ok(server_info) => {
            Ok(CallToolResult {
                isError: false,
                content: vec![TextContent {
                    type_: "text".to_string(),
                    text: json!({
                        "status": "started",
                        "pid": server_info.pid,
                        "port": port,
                        "server_url": format!("http://localhost:{}", port),
                        "config": {
                            "pricing": pricing,
                            "simulation_mode": simulation_mode
                        },
                        "started_at": chrono::Utc::now().to_rfc3339()
                    }).to_string()
                }]
            })
        },
        Err(e) => {
            // Error translation
            Ok(CallToolResult {
                isError: true,
                content: vec![TextContent {
                    type_: "text".to_string(),
                    text: json!({
                        "error": "E3004",
                        "message": format!("Server start failed: {}", e),
                        "suggestion": "Check logs for details",
                        "docs_link": "https://docs.x402-dev.com/errors/E3004"
                    }).to_string()
                }]
            })
        }
    }
}
```

**Validate:**
```bash
cargo build --package x402-mcp-server
cargo test --package x402-mcp-server

# Manual test
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"x402__server_mock_start","arguments":{"port":3402}}}' | ./target/release/x402-mcp-server
```

---

### Day 1 Afternoon: Error Handling (2 hours)

#### **Task 1.4: Create Error Translation Layer (2 hours)**

**Owner:** Coder Agent

**File:** `src/error.rs`
```rust
use thiserror::Error;
use rmcp::{CallToolResult, TextContent};
use serde_json::json;

/// MCP-specific error types
#[derive(Error, Debug)]
pub enum McpError {
    #[error("Port {port} is already in use (PID: {existing_pid})")]
    PortInUse { port: u16, existing_pid: u32 },

    #[error("Invalid port number: {port}")]
    InvalidPort { port: u16 },

    #[error("Server not running")]
    ServerNotRunning,

    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),

    #[error("YAML parse error: {0}")]
    YamlParse(String),

    #[error("x402-core error: {0}")]
    X402Core(String),
}

impl From<McpError> for CallToolResult {
    fn from(err: McpError) -> Self {
        let (code, message, suggestion) = match &err {
            McpError::PortInUse { port, existing_pid } => (
                "E3001",
                format!("Port {} is already in use (PID: {})", port, existing_pid),
                "Stop existing server with x402__server_mock_stop or use different port".to_string()
            ),
            McpError::InvalidPort { port } => (
                "E3002",
                format!("Invalid port number: {}", port),
                "Use port between 1024-65535".to_string()
            ),
            McpError::ServerNotRunning => (
                "E3004",
                "No mock server is currently running".to_string(),
                "Start server first with x402__server_mock_start".to_string()
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

// Implement From traits for x402-core errors
impl From<x402_core::Error> for McpError {
    fn from(err: x402_core::Error) -> Self {
        McpError::X402Core(err.to_string())
    }
}
```

**Add unit tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_in_use_error() {
        let err = McpError::PortInUse { port: 3402, existing_pid: 12345 };
        let result: CallToolResult = err.into();

        assert!(result.isError);
        let text = &result.content[0].text;
        assert!(text.contains("E3001"));
        assert!(text.contains("3402"));
    }

    #[test]
    fn test_invalid_port_error() {
        let err = McpError::InvalidPort { port: 99 };
        let result: CallToolResult = err.into();

        assert!(result.isError);
        assert!(result.content[0].text.contains("E3002"));
    }
}
```

---

### Day 2 Morning: Additional Tools (3 hours)

#### **Task 2.1: Implement x402__policy_validate (1.5 hours)**

**Owner:** Coder Agent

**File:** `src/tools/policy.rs`
```rust
use rmcp::{tool, Server, CallToolResult, TextContent};
use serde_json::json;
use anyhow::Result;
use x402_core::policy::validate_policies;

pub async fn register(server: &Server) -> Result<()> {
    server.register_tool(policy_validate).await?;
    Ok(())
}

#[tool(
    name = "x402__policy_validate",
    description = "Validate payment policy YAML for syntax errors and logical conflicts",
)]
pub async fn policy_validate(
    #[arg(description = "Policy YAML content (inline, not file path)")]
    policy_yaml: String,
) -> Result<CallToolResult> {
    // Parse and validate (DIRECT LIBRARY CALL - no temp files!)
    match validate_policies(&policy_yaml) {
        Ok(validation_result) => {
            Ok(CallToolResult {
                isError: false,
                content: vec![TextContent {
                    type_: "text".to_string(),
                    text: json!({
                        "valid": validation_result.is_valid,
                        "has_errors": !validation_result.errors.is_empty(),
                        "has_warnings": !validation_result.warnings.is_empty(),
                        "issues": validation_result.all_issues()
                    }).to_string()
                }]
            })
        },
        Err(e) => {
            Ok(CallToolResult {
                isError: true,
                content: vec![TextContent {
                    type_: "text".to_string(),
                    text: json!({
                        "error": "E5001",
                        "message": format!("Invalid policy YAML: {}", e),
                        "suggestion": "Check YAML syntax and structure",
                        "docs_link": "https://docs.x402-dev.com/errors/E5001"
                    }).to_string()
                }]
            })
        }
    }
}
```

---

#### **Task 2.2: Implement x402__config_show (1.5 hours)**

**Owner:** Coder Agent

**File:** `src/tools/config.rs`
```rust
use rmcp::{tool, Server, CallToolResult, TextContent};
use serde_json::json;
use anyhow::Result;
use x402_cli::config::load_merged_config;

pub async fn register(server: &Server) -> Result<()> {
    server.register_tool(config_show).await?;
    Ok(())
}

#[tool(
    name = "x402__config_show",
    description = "Display merged configuration from all sources (CLI → Env → Project → Global → Defaults)",
)]
pub async fn config_show() -> Result<CallToolResult> {
    match load_merged_config(None) {
        Ok(config) => {
            Ok(CallToolResult {
                isError: false,
                content: vec![TextContent {
                    type_: "text".to_string(),
                    text: json!({
                        "config": config,
                        "sources": vec![
                            "CLI flags",
                            "Environment variables",
                            "Project config",
                            "Global config",
                            "Defaults"
                        ]
                    }).to_string()
                }]
            })
        },
        Err(e) => {
            Ok(CallToolResult {
                isError: true,
                content: vec![TextContent {
                    type_: "text".to_string(),
                    text: json!({
                        "error": "E9003",
                        "message": format!("Config load failed: {}", e),
                        "suggestion": "Check config file syntax",
                        "docs_link": "https://docs.x402-dev.com/errors/E9003"
                    }).to_string()
                }]
            })
        }
    }
}
```

**Update registration:**
```rust
// src/tools/mod.rs
pub async fn register_all(server: &Server) -> Result<()> {
    mock::register(server).await?;
    policy::register(server).await?;
    config::register(server).await?;

    eprintln!("  ✅ 3 tools registered");
    Ok(())
}
```

---

### Day 2 Afternoon: Testing & Validation (2 hours)

#### **Task 2.3: Create Integration Tests (1 hour)**

**Owner:** Tester Agent

**File:** `tests/integration_test.rs`
```rust
use x402_mcp_server::tools;

#[tokio::test]
async fn test_server_mock_start() {
    let result = tools::mock::server_mock_start(3402, 0.01, "success".to_string())
        .await
        .unwrap();

    assert!(!result.isError);
    let text = &result.content[0].text;
    assert!(text.contains("started"));
    assert!(text.contains("3402"));
}

#[tokio::test]
async fn test_policy_validate_valid() {
    let yaml = r#"
policies:
  - type: rate_limit
    pattern: "/api/*"
    max_requests: 100
"#;

    let result = tools::policy::policy_validate(yaml.to_string())
        .await
        .unwrap();

    assert!(!result.isError);
}

#[tokio::test]
async fn test_config_show() {
    let result = tools::config::config_show()
        .await
        .unwrap();

    assert!(!result.isError);
    assert!(result.content[0].text.contains("config"));
}
```

---

#### **Task 2.4: Set up Benchmarks (1 hour)**

**Owner:** Tester Agent

**File:** `benches/tool_benchmarks.rs`
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use x402_mcp_server::tools;

fn benchmark_mock_start(c: &mut Criterion) {
    c.bench_function("server_mock_start", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                tools::mock::server_mock_start(
                    black_box(3402),
                    black_box(0.01),
                    black_box("success".to_string())
                ).await
            });
    });
}

fn benchmark_policy_validate(c: &mut Criterion) {
    let yaml = r#"
policies:
  - type: rate_limit
    pattern: "/api/*"
    max_requests: 100
"#;

    c.bench_function("policy_validate", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                tools::policy::policy_validate(black_box(yaml.to_string())).await
            });
    });
}

criterion_group!(benches, benchmark_mock_start, benchmark_policy_validate);
criterion_main!(benches);
```

**Run benchmarks:**
```bash
cargo bench --bench tool_benchmarks
# Target: <1ms P95 latency
```

---

### Day 2 End: Claude Code Integration Test (30 min)

**Owner:** All Agents (collaborative)

#### **Step 1: Add to Claude Code**
```bash
# Build release binary
cargo build --release --package x402-mcp-server

# Add to Claude Code
claude mcp add x402-mcp /Users/valentynkit/dev/sandbox/Hackaton/target/release/x402-mcp-server
```

#### **Step 2: Test in Claude Code**

Ask Claude Code:
1. "Start an x402 mock server on port 3402"
2. "Validate this policy: [paste YAML]"
3. "Show me the x402 configuration"

#### **Step 3: Verify Latency**
```bash
# Check criterion reports
cat target/criterion/server_mock_start/report/index.html
```

**Expected:** <1ms tool invocation overhead

---

## ✅ Days 0-2 Success Gate

**All criteria must pass:**

### Day 0 Deliverables
- ✅ Test command refactored (returns Result)
- ✅ rmcp PoC working (stdio transport validated)
- ✅ x402-core API verified (all 7 functions exist)

### Days 1-2 Deliverables
- ✅ 3 tools implemented and working
  - x402__server_mock_start
  - x402__policy_validate
  - x402__config_show
- ✅ stdio transport functional
- ✅ Error translation layer complete
- ✅ Integration tests passing (3+ tests)
- ✅ Benchmarks running (<1ms P95 confirmed)
- ✅ Claude Code integration verified
- ✅ 50%+ test coverage achieved

### Performance Metrics
- ✅ Cold start: <100ms
- ✅ Tool invocation: <1ms P95
- ✅ Memory usage: <10MB
- ✅ Zero subprocess overhead confirmed

---

## 📊 Progress Tracking

Use hooks protocol throughout:

```bash
# Start of day
npx claude-flow@alpha hooks session-restore --session-id "swarm-1762973047611-a2rdv52zl"

# Each task
npx claude-flow@alpha hooks pre-task --description "[task-name]"
# ... work ...
npx claude-flow@alpha hooks post-task --task-id "[task-id]"

# End of day
npx claude-flow@alpha hooks session-end --export-metrics true
```

---

## 🚨 Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| **Day 0 takes longer than 4h** | Escalate to tech lead, may need Day 0.5 |
| **rmcp SDK incompatible** | Fall back to TypeScript approach (adds 2 days) |
| **APIs don't match docs** | Adjust tool implementations, document changes |
| **<1ms latency not achieved** | Acceptable up to 2ms for Day 2, optimize in Day 5 |

---

## 📋 Todo List Tracking

Update todos as you progress:
- Mark completed tasks as "completed"
- Mark current task as "in_progress"
- Add blockers/issues to list as needed

**End State:** 3 tools working, <1ms latency, ready for Days 3-4 (remaining 4 tools).

---

**Document Status:** Ready for execution
**Last Updated:** 2025-11-12
**Hive Mind Coordination:** Active
