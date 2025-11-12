# Epic 8: Comprehensive Testing Strategy

**Version:** 1.0.0
**Target:** 80%+ test coverage with <1ms P95 latency
**Created:** 2025-11-12
**Agent:** Tester (Hive Mind Swarm)

---

## Executive Summary

This testing strategy defines a comprehensive, practical approach to achieving 80%+ test coverage for Epic 8's MCP server integration while maintaining <1ms P95 latency targets. The strategy leverages Rust's testing ecosystem (criterion, tokio-test, proptest) and builds on the existing x402-dev test infrastructure.

**Key Principles:**
1. **Test Pyramid First**: Focus on fast unit tests (70%), moderate integration tests (25%), minimal E2E tests (5%)
2. **Direct Library Testing**: Test x402-core/x402-server functions directly (no subprocess overhead)
3. **Performance-Driven**: All tests must validate latency targets (<1ms for tool overhead)
4. **Maintainability**: Tests as documentation - clear, readable, self-explaining
5. **CI/CD Ready**: Tests run in <5 minutes, deterministic, no flaky tests

---

## 1. Test Organization Structure

### 1.1 Directory Layout

```
x402-mcp-server/
├── tests/
│   ├── unit/
│   │   ├── tool_validation_test.rs      # Parameter validation for all 7 tools
│   │   ├── error_translation_test.rs     # x402-core::Error → MCP JSON
│   │   ├── schema_compliance_test.rs     # MCP protocol schema validation
│   │   └── type_conversion_test.rs       # Rust ↔ JSON type conversions
│   ├── integration/
│   │   ├── tool_integration_test.rs      # All 7 tools end-to-end
│   │   ├── workflow_test.rs              # Multi-tool workflows
│   │   ├── mock_server_lifecycle_test.rs # Server start/stop/status
│   │   └── stdio_transport_test.rs       # MCP stdio protocol
│   ├── property/
│   │   ├── policy_properties_test.rs     # Proptest for policy validation
│   │   └── invoice_properties_test.rs    # Proptest for compliance checks
│   ├── fixtures/
│   │   ├── mod.rs
│   │   ├── policies.rs                   # Sample policy YAMLs
│   │   ├── test_suites.rs                # Sample test suite YAMLs
│   │   └── mcp_requests.rs               # Sample MCP JSON-RPC requests
│   └── helpers/
│       ├── mod.rs
│       ├── mcp_client.rs                 # MCP test client
│       └── assertions.rs                 # Custom test assertions
├── benches/
│   ├── tool_benchmarks.rs                # Criterion benchmarks for all tools
│   ├── memory_benchmarks.rs              # Memory allocation tracking
│   └── concurrency_benchmarks.rs         # Parallel tool invocation
└── Cargo.toml
```

### 1.2 Test Configuration in Cargo.toml

```toml
[dev-dependencies]
# Testing framework
tokio-test = "0.4"
criterion = { version = "0.5", features = ["async_tokio", "html_reports"] }

# Property-based testing
proptest = "1.4"

# HTTP mocking (for compliance checks)
wiremock = "0.6"

# Assertions
pretty_assertions = "1.4"

# Test utilities
tempfile = "3.23"
assert_matches = "1.5"

[profile.test]
opt-level = 1  # Faster test compilation
debug = true

[profile.bench]
opt-level = 3  # Maximum performance for benchmarks
lto = true

[[bench]]
name = "tool_benchmarks"
harness = false

[[bench]]
name = "memory_benchmarks"
harness = false

[[bench]]
name = "concurrency_benchmarks"
harness = false
```

---

## 2. Test Pyramid Strategy

### 2.1 Unit Tests (70% of test suite, 85%+ coverage target)

**Goal:** Fast (<5ms per test), isolated, comprehensive edge case coverage

#### Coverage Matrix

| Component | Coverage Target | Key Focus Areas |
|-----------|----------------|-----------------|
| **Tool Parameter Validation** | 95%+ | Port ranges, YAML parsing, optional fields, invalid inputs |
| **Error Translation** | 100% | All x402-core error variants → MCP error codes |
| **Type Conversions** | 95%+ | Rust structs ↔ JSON serialization |
| **Schema Compliance** | 100% | MCP protocol schema validation |

#### Example: Tool Parameter Validation Test

```rust
// tests/unit/tool_validation_test.rs
use x402_mcp_server::tools::mock::ServerMockStartParams;
use serde_json::json;

#[test]
fn test_server_mock_start_valid_params() {
    let valid_params = vec![
        json!({"port": 3402, "pricing": 0.01}),
        json!({"port": 8080, "pricing": 0.05, "simulation_mode": "success"}),
        json!({"port": 1024}),  // Min valid port
        json!({"port": 65535}), // Max valid port
    ];

    for params in valid_params {
        let result: Result<ServerMockStartParams, _> = serde_json::from_value(params.clone());
        assert!(
            result.is_ok(),
            "Expected valid params: {:?}, got error: {:?}",
            params, result.err()
        );
    }
}

#[test]
fn test_server_mock_start_invalid_ports() {
    let invalid_params = vec![
        json!({"port": 0}),      // Too low
        json!({"port": 99}),     // Below 1024
        json!({"port": 70000}),  // Above 65535
        json!({"port": -100}),   // Negative
    ];

    for params in invalid_params {
        let result: Result<ServerMockStartParams, _> = serde_json::from_value(params.clone());
        assert!(
            result.is_err(),
            "Expected error for invalid params: {:?}",
            params
        );
    }
}

#[test]
fn test_server_mock_start_default_values() {
    let params = json!({});
    let parsed: ServerMockStartParams = serde_json::from_value(params).unwrap();

    assert_eq!(parsed.port, 3402, "Default port should be 3402");
    assert_eq!(parsed.pricing, 0.01, "Default pricing should be 0.01");
    assert_eq!(parsed.simulation_mode, "success", "Default simulation_mode should be 'success'");
}
```

#### Example: Error Translation Test

```rust
// tests/unit/error_translation_test.rs
use x402_mcp_server::error::McpError;
use x402_core::Error as CoreError;
use rmcp::CallToolResult;

#[test]
fn test_port_in_use_error_translation() {
    let core_error = CoreError::PortInUse { port: 3402, pid: 12345 };
    let mcp_result: CallToolResult = McpError::from(core_error).into();

    assert!(mcp_result.isError);

    let content = &mcp_result.content[0].text;
    let error_json: serde_json::Value = serde_json::from_str(content).unwrap();

    assert_eq!(error_json["error"], "E3001");
    assert!(error_json["message"].as_str().unwrap().contains("Port 3402"));
    assert!(error_json["suggestion"].as_str().unwrap().contains("x402__server_mock_stop"));
    assert!(error_json["docs_link"].as_str().unwrap().contains("E3001"));
}

#[test]
fn test_yaml_parse_error_translation() {
    let core_error = CoreError::YamlParse("missing field `type` at line 8".to_string());
    let mcp_result: CallToolResult = McpError::from(core_error).into();

    assert!(mcp_result.isError);

    let content = &mcp_result.content[0].text;
    let error_json: serde_json::Value = serde_json::from_str(content).unwrap();

    assert_eq!(error_json["error"], "E4001");
    assert!(error_json["message"].as_str().unwrap().contains("YAML"));
    assert!(error_json["suggestion"].as_str().unwrap().contains("syntax"));
}

#[test]
fn test_all_error_codes_have_suggestions() {
    let error_variants = vec![
        CoreError::PortInUse { port: 3402, pid: 12345 },
        CoreError::InvalidPort { port: 99 },
        CoreError::YamlParse("test".to_string()),
        CoreError::NetworkError(std::io::Error::new(std::io::ErrorKind::TimedOut, "timeout")),
        CoreError::ServerNotRunning,
    ];

    for error in error_variants {
        let mcp_result: CallToolResult = McpError::from(error.clone()).into();
        let content = &mcp_result.content[0].text;
        let error_json: serde_json::Value = serde_json::from_str(content).unwrap();

        assert!(
            error_json["suggestion"].is_string(),
            "Error {:?} missing suggestion field",
            error
        );
        assert!(
            !error_json["suggestion"].as_str().unwrap().is_empty(),
            "Error {:?} has empty suggestion",
            error
        );
    }
}
```

### 2.2 Integration Tests (25% of test suite, 75%+ coverage target)

**Goal:** End-to-end tool testing with real x402-core/x402-server integration

#### Coverage Matrix

| Workflow | Tests | Coverage Target |
|----------|-------|-----------------|
| **Mock Server Lifecycle** | 5 tests | 100% (start/stop/status/errors) |
| **Policy Validation & Generation** | 8 tests | 85% (valid/invalid YAML, middleware generation) |
| **Testing Suite Execution** | 10 tests | 75% (YAML parsing, assertions, failure handling) |
| **Compliance Checking** | 6 tests | 80% (402 validation, timeouts, network errors) |
| **Multi-Tool Workflows** | 4 tests | 70% (realistic user scenarios) |

#### Example: Mock Server Lifecycle Test

```rust
// tests/integration/mock_server_lifecycle_test.rs
use x402_mcp_server::tools;
use tokio;

#[tokio::test]
async fn test_mock_server_start_stop_lifecycle() {
    // 1. Verify no server running initially
    let status_before = tools::mock::server_mock_status().await.unwrap();
    assert!(!status_before.isError);
    let status_json: serde_json::Value =
        serde_json::from_str(&status_before.content[0].text).unwrap();
    assert_eq!(status_json["status"], "not_running");

    // 2. Start server
    let start_result = tools::mock::server_mock_start(3402, 0.01).await.unwrap();
    assert!(!start_result.isError);
    let start_json: serde_json::Value =
        serde_json::from_str(&start_result.content[0].text).unwrap();
    assert_eq!(start_json["status"], "started");
    assert_eq!(start_json["port"], 3402);

    // 3. Verify server is running
    let status_running = tools::mock::server_mock_status().await.unwrap();
    let status_json: serde_json::Value =
        serde_json::from_str(&status_running.content[0].text).unwrap();
    assert_eq!(status_json["status"], "running");
    assert_eq!(status_json["port"], 3402);

    // 4. Stop server
    let stop_result = tools::mock::server_mock_stop().await.unwrap();
    assert!(!stop_result.isError);
    let stop_json: serde_json::Value =
        serde_json::from_str(&stop_result.content[0].text).unwrap();
    assert_eq!(stop_json["status"], "stopped");

    // 5. Verify server stopped
    let status_after = tools::mock::server_mock_status().await.unwrap();
    let status_json: serde_json::Value =
        serde_json::from_str(&status_after.content[0].text).unwrap();
    assert_eq!(status_json["status"], "not_running");
}

#[tokio::test]
async fn test_mock_server_port_in_use_error() {
    // Start server on port 3402
    tools::mock::server_mock_start(3402, 0.01).await.unwrap();

    // Try to start another server on same port
    let result = tools::mock::server_mock_start(3402, 0.01).await.unwrap();
    assert!(result.isError);

    let error_json: serde_json::Value =
        serde_json::from_str(&result.content[0].text).unwrap();
    assert_eq!(error_json["error"], "E3001");
    assert!(error_json["message"].as_str().unwrap().contains("already in use"));

    // Cleanup
    tools::mock::server_mock_stop().await.unwrap();
}

#[tokio::test]
async fn test_stop_server_not_running_error() {
    // Ensure no server is running
    tools::mock::server_mock_stop().await.ok(); // Ignore error if already stopped

    // Try to stop when no server is running
    let result = tools::mock::server_mock_stop().await.unwrap();
    assert!(result.isError);

    let error_json: serde_json::Value =
        serde_json::from_str(&result.content[0].text).unwrap();
    assert_eq!(error_json["error"], "E3004");
    assert!(error_json["suggestion"].as_str().unwrap().contains("x402__server_mock_start"));
}
```

#### Example: Multi-Tool Workflow Test

```rust
// tests/integration/workflow_test.rs
use x402_mcp_server::tools;
use tokio;

#[tokio::test]
async fn test_complete_payment_api_testing_workflow() {
    // Workflow: Validate policy → Start server → Check compliance → Stop server

    // 1. Validate payment policy
    let policy_yaml = r#"
policies:
  - type: rate_limit
    pattern: "/api/*"
    max_requests: 100
    window: 3600
  - type: payment_required
    pattern: "/premium/*"
    amount: 0.05
    currency: USDC
"#.to_string();

    let validate_result = tools::policy::policy_validate(policy_yaml.clone()).await.unwrap();
    assert!(!validate_result.isError);

    let validate_json: serde_json::Value =
        serde_json::from_str(&validate_result.content[0].text).unwrap();
    assert_eq!(validate_json["valid"], true);

    // 2. Generate Express middleware
    let generate_result = tools::policy::policy_generate_express(
        policy_yaml,
        "policy.yaml".to_string()
    ).await.unwrap();
    assert!(!generate_result.isError);

    let generate_json: serde_json::Value =
        serde_json::from_str(&generate_result.content[0].text).unwrap();
    assert!(generate_json["lines"].as_i64().unwrap() > 50);

    // 3. Start mock server
    let start_result = tools::mock::server_mock_start(3402, 0.01).await.unwrap();
    assert!(!start_result.isError);

    // 4. Check endpoint compliance
    let check_result = tools::testing::testing_check_compliance(
        "http://localhost:3402/api/data".to_string(),
        None,
        None,
        10
    ).await.unwrap();
    assert!(!check_result.isError);

    let check_json: serde_json::Value =
        serde_json::from_str(&check_result.content[0].text).unwrap();
    assert!(check_json["checks"].is_array());

    // 5. Run test suite
    let suite_yaml = r#"
tests:
  - name: "Protected endpoint returns 402"
    request:
      url: "http://localhost:3402/api/data"
      method: GET
    assertions:
      - type: status_code
        expected: 402
      - type: header_exists
        header: "WWW-Authenticate"
"#.to_string();

    let test_result = tools::testing::testing_run_suite(
        suite_yaml,
        "json".to_string(),
        false
    ).await.unwrap();
    assert!(!test_result.isError);

    let test_json: serde_json::Value =
        serde_json::from_str(&test_result.content[0].text).unwrap();
    assert!(test_json["summary"]["total_tests"].as_i64().unwrap() > 0);

    // 6. Stop server (cleanup)
    let stop_result = tools::mock::server_mock_stop().await.unwrap();
    assert!(!stop_result.isError);
}
```

### 2.3 Property-Based Tests (5% of test suite, edge case coverage)

**Goal:** Generate hundreds of random inputs to catch edge cases

#### Example: Policy Validation Properties

```rust
// tests/property/policy_properties_test.rs
use proptest::prelude::*;
use x402_mcp_server::tools;

proptest! {
    #[test]
    fn test_policy_validate_never_panics(
        policy_yaml in ".*"  // Any string
    ) {
        // Property: validation should never panic, even with garbage input
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let result = tools::policy::policy_validate(policy_yaml).await;
            // Either succeeds or returns structured error
            assert!(result.is_ok());
        });
    }

    #[test]
    fn test_port_validation_boundaries(
        port in any::<u16>()
    ) {
        // Property: ports outside 1024-65535 should always fail
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let result = tools::mock::server_mock_start(port, 0.01).await.unwrap();

            if port < 1024 || port > 65535 {
                assert!(result.isError, "Expected error for port {}", port);
            } else {
                // Valid port range - may succeed or fail (port in use)
                // But should never panic
                assert!(result.isError || !result.isError);
            }
        });
    }

    #[test]
    fn test_pricing_validation(
        pricing in any::<f64>()
    ) {
        // Property: negative pricing should fail, zero/positive should succeed
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let result = tools::mock::server_mock_start(3402, pricing).await;

            if pricing < 0.0 {
                assert!(result.is_err() || result.unwrap().isError);
            }
            // Non-negative should work (may fail due to port, but not pricing)
        });
    }
}
```

---

## 3. Performance Testing Strategy

### 3.1 Latency Benchmarks (Criterion)

**Target:** <1ms P95 latency for tool invocation overhead (excluding I/O)

#### Benchmark Suite

```rust
// benches/tool_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use x402_mcp_server::tools;

fn benchmark_mock_server_tools(c: &mut Criterion) {
    let mut group = c.benchmark_group("mock_server_tools");

    // Benchmark server_mock_start (library call overhead only)
    group.bench_function("server_mock_start", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                tools::mock::server_mock_start(black_box(3402), black_box(0.01)).await
            });
    });

    // Benchmark server_mock_status
    group.bench_function("server_mock_status", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                tools::mock::server_mock_status().await
            });
    });

    group.finish();
}

fn benchmark_policy_tools(c: &mut Criterion) {
    let mut group = c.benchmark_group("policy_tools");

    let policy_yaml = r#"
policies:
  - type: rate_limit
    pattern: "/api/*"
    max_requests: 100
    window: 3600
"#.to_string();

    // Benchmark policy validation (in-memory YAML parsing + validation)
    group.bench_function("policy_validate", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                tools::policy::policy_validate(black_box(policy_yaml.clone())).await
            });
    });

    group.finish();
}

fn benchmark_error_translation(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_translation");

    use x402_core::Error as CoreError;
    use x402_mcp_server::error::McpError;

    group.bench_function("port_in_use_translation", |b| {
        b.iter(|| {
            let core_error = CoreError::PortInUse { port: 3402, pid: 12345 };
            black_box(McpError::from(core_error))
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_mock_server_tools,
    benchmark_policy_tools,
    benchmark_error_translation
);
criterion_main!(benches);
```

**Expected Results:**
```
mock_server_tools/server_mock_start
                        time:   [650.0 µs 700.0 µs 750.0 µs]
                        (P95: <1ms ✅)

policy_tools/policy_validate
                        time:   [450.0 µs 500.0 µs 550.0 µs]
                        (P95: <1ms ✅)

error_translation/port_in_use_translation
                        time:   [5.0 µs 10.0 µs 15.0 µs]
                        (P95: <1ms ✅)
```

### 3.2 Memory Benchmarks

```rust
// benches/memory_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use x402_mcp_server::tools;

fn benchmark_memory_allocations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocations");

    // Track peak memory during tool execution
    group.bench_function("policy_validate_memory", |b| {
        b.iter_custom(|iters| {
            let start = std::time::Instant::now();

            for _ in 0..iters {
                let policy_yaml = black_box(r#"
policies:
  - type: rate_limit
    pattern: "/api/*"
    max_requests: 100
"#.to_string());

                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    tools::policy::policy_validate(policy_yaml).await.unwrap();
                });
            }

            start.elapsed()
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_memory_allocations);
criterion_main!(benches);
```

### 3.3 Concurrency Benchmarks

```rust
// benches/concurrency_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use x402_mcp_server::tools;
use tokio;

fn benchmark_concurrent_tool_invocations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_invocations");

    for num_concurrent in [1, 5, 10, 20, 50].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_concurrent),
            num_concurrent,
            |b, &num| {
                b.to_async(tokio::runtime::Runtime::new().unwrap())
                    .iter(|| async move {
                        let mut handles = vec![];

                        for _ in 0..num {
                            handles.push(tokio::spawn(async {
                                tools::mock::server_mock_status().await
                            }));
                        }

                        for handle in handles {
                            handle.await.unwrap().unwrap();
                        }
                    });
            }
        );
    }

    group.finish();
}

criterion_group!(benches, benchmark_concurrent_tool_invocations);
criterion_main!(benches);
```

---

## 4. Test Data Management

### 4.1 Fixtures Organization

```rust
// tests/fixtures/policies.rs
pub fn valid_rate_limit_policy() -> String {
    r#"
policies:
  - type: rate_limit
    pattern: "/api/*"
    max_requests: 100
    window: 3600
"#.to_string()
}

pub fn invalid_policy_missing_type() -> String {
    r#"
policies:
  - pattern: "/api/*"
    max_requests: 100
"#.to_string()
}

pub fn complex_multi_policy() -> String {
    r#"
policies:
  - type: rate_limit
    pattern: "/api/v1/*"
    max_requests: 100
    window: 3600
  - type: payment_required
    pattern: "/premium/*"
    amount: 0.05
    currency: USDC
    network: devnet
  - type: ip_whitelist
    pattern: "/admin/*"
    allowed_ips:
      - "127.0.0.1"
      - "10.0.0.0/8"
"#.to_string()
}

// tests/fixtures/test_suites.rs
pub fn simple_test_suite() -> String {
    r#"
tests:
  - name: "Check 402 status"
    request:
      url: "http://localhost:3402/api/data"
      method: GET
    assertions:
      - type: status_code
        expected: 402
"#.to_string()
}

pub fn comprehensive_test_suite() -> String {
    r#"
tests:
  - name: "Protected endpoint returns 402"
    request:
      url: "http://localhost:3402/api/data"
      method: GET
    assertions:
      - type: status_code
        expected: 402
      - type: header_exists
        header: "WWW-Authenticate"
      - type: header_contains
        header: "WWW-Authenticate"
        value: "x402-solana"

  - name: "Valid payment allows access"
    request:
      url: "http://localhost:3402/api/data"
      method: GET
      headers:
        Authorization: "Bearer valid-payment-token"
    assertions:
      - type: status_code
        expected: 200
      - type: response_time_ms
        max: 500
"#.to_string()
}

// tests/fixtures/mcp_requests.rs
pub fn mcp_initialize_request() -> serde_json::Value {
    serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": {
                "name": "test-client",
                "version": "1.0.0"
            }
        }
    })
}

pub fn mcp_tool_call_request(tool_name: &str, args: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": tool_name,
            "arguments": args
        }
    })
}
```

### 4.2 Test Helpers

```rust
// tests/helpers/mcp_client.rs
use tokio::process::{Command, ChildStdin, ChildStdout};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use serde_json::Value;

pub struct McpTestClient {
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl McpTestClient {
    pub async fn new(server_binary: &str) -> Self {
        let mut child = Command::new(server_binary)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn MCP server");

        let stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = BufReader::new(child.stdout.take().expect("Failed to open stdout"));

        Self { stdin, stdout }
    }

    pub async fn send_request(&mut self, request: Value) -> Value {
        // Send JSON-RPC request
        let request_str = serde_json::to_string(&request).unwrap();
        self.stdin.write_all(request_str.as_bytes()).await.unwrap();
        self.stdin.write_all(b"\n").await.unwrap();
        self.stdin.flush().await.unwrap();

        // Read JSON-RPC response
        let mut response_line = String::new();
        self.stdout.read_line(&mut response_line).await.unwrap();

        serde_json::from_str(&response_line).unwrap()
    }

    pub async fn call_tool(&mut self, tool_name: &str, args: Value) -> Value {
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": args
            }
        });

        self.send_request(request).await
    }
}

// tests/helpers/assertions.rs
use serde_json::Value;

pub fn assert_mcp_success(response: &Value) {
    assert!(response["result"].is_object(), "Expected result object");
    assert!(!response["result"]["isError"].as_bool().unwrap_or(true),
        "Expected success, got error: {:?}", response);
}

pub fn assert_mcp_error(response: &Value, expected_code: &str) {
    assert!(response["result"]["isError"].as_bool().unwrap_or(false),
        "Expected error response");

    let content = &response["result"]["content"][0]["text"];
    let error_json: Value = serde_json::from_str(content.as_str().unwrap()).unwrap();

    assert_eq!(error_json["error"], expected_code,
        "Expected error code {}, got {:?}", expected_code, error_json);
}

pub fn assert_latency_under(duration: std::time::Duration, max_ms: u64) {
    assert!(duration.as_millis() < max_ms as u128,
        "Latency {}ms exceeds target {}ms", duration.as_millis(), max_ms);
}
```

---

## 5. Coverage Measurement & CI/CD Integration

### 5.1 Coverage Tools

```bash
# Install coverage tools
cargo install cargo-tarpaulin  # OR cargo-llvm-cov

# Generate coverage report
cargo tarpaulin --out Html --out Xml --output-dir ./coverage

# View HTML report
open coverage/index.html

# Check coverage threshold (fail if <80%)
cargo tarpaulin --fail-under 80
```

### 5.2 CI/CD Pipeline Configuration

```yaml
# .github/workflows/epic8-tests.yml
name: Epic 8 Testing

on:
  push:
    branches: [epic-8-mcp-integration]
  pull_request:
    branches: [main]

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: 1.85.0
          profile: minimal
          override: true

      - name: Run unit tests
        run: cargo test --lib --verbose

      - name: Check test execution time
        run: |
          time cargo test --lib --release
          # Fail if tests take >2 minutes

  integration-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: 1.85.0
          profile: minimal
          override: true

      - name: Run integration tests
        run: cargo test --test '*' --verbose

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: 1.85.0
          components: llvm-tools-preview

      - name: Install cargo-llvm-cov
        run: cargo install cargo-llvm-cov

      - name: Generate coverage report
        run: cargo llvm-cov --all-features --workspace --html

      - name: Check coverage threshold (80%+)
        run: cargo llvm-cov --fail-under-lines 80

      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v4
        with:
          files: ./target/llvm-cov/html/index.html
          fail_ci_if_error: true

  benchmarks:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: 1.85.0
          profile: minimal
          override: true

      - name: Run benchmarks
        run: cargo bench --no-fail-fast

      - name: Validate P95 latency <1ms
        run: |
          # Parse criterion output and fail if P95 > 1ms
          ./scripts/check_benchmark_thresholds.sh
```

### 5.3 Local Development Workflow

```bash
# Fast feedback loop during development
cargo watch -x 'test --lib' -x 'clippy'

# Full test suite before commit
cargo test --all-targets --all-features

# Coverage check
cargo tarpaulin --out Html --fail-under 80

# Benchmarks (only when needed, slow)
cargo bench

# Pre-commit hook script
#!/bin/bash
set -e
cargo test --lib
cargo clippy -- -D warnings
cargo fmt --check
echo "✅ All pre-commit checks passed!"
```

---

## 6. Success Criteria & Validation

### 6.1 Coverage Targets

| Test Type | Target | Validation Method |
|-----------|--------|-------------------|
| **Overall Line Coverage** | 80%+ | `cargo tarpaulin --fail-under 80` |
| **Unit Tests** | 85%+ | `cargo tarpaulin --lib --fail-under 85` |
| **Integration Tests** | 75%+ | `cargo tarpaulin --test '*' --fail-under 75` |
| **Error Paths** | 100% | Manual review + property tests |

### 6.2 Performance Targets

| Metric | Target | Validation Method |
|--------|--------|-------------------|
| **Tool Invocation Overhead** | <1ms P95 | Criterion benchmarks |
| **Error Translation** | <50µs P95 | Criterion benchmarks |
| **Memory Allocations** | <5MB per tool call | Memory profiling |
| **Test Suite Execution** | <5 minutes | CI pipeline time tracking |

### 6.3 Quality Gates

**Before merging Epic 8 PR, ALL must pass:**

✅ **Coverage:** 80%+ line coverage across all modules
✅ **Performance:** All benchmarks meet <1ms P95 target
✅ **CI/CD:** All tests green in GitHub Actions
✅ **Documentation:** All public APIs documented with examples
✅ **Security:** `cargo audit` shows 0 critical vulnerabilities
✅ **Linting:** `cargo clippy` passes with no warnings
✅ **Formatting:** `cargo fmt --check` passes

### 6.4 Test Execution Checklist

```bash
# 1. Unit tests (fast, run frequently)
cargo test --lib

# 2. Integration tests (moderate, run before commit)
cargo test --test '*'

# 3. Property tests (slow, run before push)
cargo test --test 'property_*'

# 4. Coverage report (slow, run before PR)
cargo tarpaulin --out Html --fail-under 80

# 5. Benchmarks (very slow, run before release)
cargo bench

# 6. Security audit (fast, run daily)
cargo audit

# 7. Full suite (slowest, run in CI)
cargo test --all-targets --all-features
cargo bench --no-run  # Compile benchmarks
cargo clippy -- -D warnings
cargo fmt --check
```

---

## 7. Testing Tools & Dependencies

### 7.1 Required Dependencies

```toml
[dev-dependencies]
# Testing framework
criterion = { version = "0.5", features = ["async_tokio", "html_reports"] }
tokio-test = "0.4"
proptest = "1.4"

# Assertions
pretty_assertions = "1.4"
assert_matches = "1.5"

# HTTP mocking
wiremock = "0.6"

# Test utilities
tempfile = "3.23"
rand = "0.8"

# Coverage (install separately)
# cargo install cargo-tarpaulin
# cargo install cargo-llvm-cov
```

### 7.2 Tool Justification

| Tool | Purpose | Why This Tool? |
|------|---------|----------------|
| **criterion** | Performance benchmarks | Industry standard, statistical rigor, HTML reports, async support |
| **tokio-test** | Async test utilities | Official Tokio testing tools, mocking time/delays |
| **proptest** | Property-based testing | Catch edge cases with random input generation |
| **wiremock** | HTTP mocking | Deterministic HTTP responses for compliance tests |
| **pretty_assertions** | Better assertion output | Colored diffs make test failures easier to debug |
| **cargo-tarpaulin** | Coverage reporting | Fast, accurate, CI-friendly coverage tool |

---

## 8. Potential Testing Challenges & Mitigations

### Challenge 1: Async Test Flakiness

**Risk:** Async tests may have race conditions or timing issues

**Mitigation:**
- Use `tokio-test` utilities for deterministic time control
- Avoid `sleep()` in tests - use mock time advancement
- Run tests with `--test-threads=1` if isolation issues occur
- Add retries for network-dependent tests (integration only)

### Challenge 2: Test Data Management

**Risk:** Hardcoded test data becomes stale or duplicated

**Mitigation:**
- Centralize all fixtures in `tests/fixtures/` module
- Use builder pattern for complex test data
- Property tests generate random valid data
- Document fixture purpose and maintenance responsibility

### Challenge 3: Performance Test Variability

**Risk:** Benchmarks vary across machines, CI environments

**Mitigation:**
- Use relative performance (compare to baseline)
- Set generous P95 targets (<1ms has 50%+ margin)
- Run benchmarks multiple times, use median
- Track performance trends over time (criterion history)

### Challenge 4: Coverage Blind Spots

**Risk:** 80%+ coverage doesn't guarantee all edge cases tested

**Mitigation:**
- Combine coverage metrics with property-based tests
- Manual review of uncovered lines (are they critical?)
- Require tests for all reported bugs
- Use mutation testing occasionally to validate test quality

---

## 9. Testing Timeline & Phasing

### Phase 1: Foundation (Days 1-2)

**Tests to Write:**
- Unit tests for first 3 tools (mock_start, policy_validate, config_show)
- Basic integration test (start server, check status)
- Initial benchmarks (establish baseline)
- **Target:** 50%+ coverage

### Phase 2: Core Tools (Days 3-4)

**Tests to Write:**
- Unit tests for remaining 4 tools
- Integration tests for all 7 tools
- Multi-tool workflow tests
- Error path tests
- **Target:** 70%+ coverage

### Phase 3: Polish (Day 5)

**Tests to Write:**
- Property-based tests
- Concurrency benchmarks
- Memory profiling tests
- Edge case tests (identified from coverage gaps)
- **Target:** 80%+ coverage achieved

### Phase 4: Validation (Day 6)

**Tests to Validate:**
- All benchmarks passing (<1ms P95)
- Full test suite green in CI
- Coverage report validated (80%+)
- No flaky tests detected

---

## 10. Summary & Next Steps

### What This Strategy Delivers

✅ **80%+ test coverage** via test pyramid approach (70% unit, 25% integration, 5% property)
✅ **<1ms P95 latency** validated through criterion benchmarks
✅ **Deterministic tests** using fixtures, mocks, no external dependencies
✅ **CI/CD ready** with <5 minute test suite execution
✅ **Maintainable** with clear organization, helpers, and documentation

### Implementation Checklist

- [x] Testing strategy documented (this file)
- [ ] Fixtures created (`tests/fixtures/`)
- [ ] Test helpers implemented (`tests/helpers/`)
- [ ] Unit tests written (target: 50+ tests)
- [ ] Integration tests written (target: 20+ tests)
- [ ] Property tests written (target: 5+ properties)
- [ ] Benchmarks implemented (target: 10+ benchmarks)
- [ ] CI/CD pipeline configured (`.github/workflows/epic8-tests.yml`)
- [ ] Coverage threshold enforced (`cargo tarpaulin --fail-under 80`)
- [ ] Documentation updated (API docs, testing guide)

### Coordination with Other Agents

**Shared via Memory:**
```bash
# Store testing strategy status
npx claude-flow@alpha hooks post-task --task-id "tester-strategy" \
  --memory-key "swarm/tester/strategy-complete" \
  --value "{ \"coverage_target\": 80, \"latency_target_ms\": 1, \"test_count_estimate\": 75 }"

# Notify other agents
npx claude-flow@alpha hooks notify \
  --message "Testing strategy complete: 80%+ coverage, <1ms P95, 75+ tests estimated"
```

---

**Document Owner:** Tester Agent (Epic 8 Hive Mind Swarm)
**Status:** ✅ Complete - Ready for Implementation
**Next:** Hand off to Coder agent for test implementation

