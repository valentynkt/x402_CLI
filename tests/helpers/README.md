# Test Helpers Infrastructure

Comprehensive test utilities for x402-dev CLI testing, providing reusable components for HTTP mocking, CLI execution, and custom assertions.

## 📁 Structure

```
tests/helpers/
├── mod.rs           # Module declarations and re-exports
├── mock_server.rs   # HTTP mock servers using wiremock
├── cli_runner.rs    # CLI command execution wrappers
├── assertions.rs    # Custom domain-specific assertions
└── README.md        # This documentation
```

## 🚀 Quick Start

### Basic Usage

```rust
use crate::helpers::{mock_server, cli_runner, assertions};

#[tokio::test]
async fn test_check_command_with_402() {
    // 1. Setup mock server
    let server = mock_server::mock_402_server().await;

    // 2. Run CLI command
    let result = cli_runner::run_check(&server.uri());

    // 3. Validate output
    assertions::assert_payment_required(&result.stdout);
    assertions::assert_contains_invoice(&result.stdout);
    assert!(result.success());
}
```

## 📦 Modules

### 1. Mock Server (`mock_server.rs`)

HTTP mock servers for testing x402 payment protocol responses.

#### Available Functions

```rust
// Basic mock servers
mock_402_server() -> MockServer
mock_200_server() -> MockServer

// Custom amount
mock_402_server_with_amount(amount_lamports: u64) -> MockServer
mock_200_server_with_data(data: &str) -> MockServer

// Custom invoice
mock_server_with_invoice(invoice: serde_json::Value) -> MockServer

// Error scenarios
mock_timeout_server() -> MockServer
mock_error_server(status_code: u16) -> MockServer

// Specific endpoints
mock_health_endpoint() -> MockServer
mock_payment_verification_server() -> MockServer
```

#### Examples

**Testing 402 Payment Required:**
```rust
#[tokio::test]
async fn test_402_response() {
    let server = mock_402_server().await;
    let response = reqwest::get(&server.uri()).await.unwrap();

    assert_eq!(response.status(), 402);
    assert!(response.headers().contains_key("www-authenticate"));
    assert!(response.headers().contains_key("x-402-invoice"));
}
```

**Testing with custom payment amount:**
```rust
#[tokio::test]
async fn test_custom_amount() {
    let server = mock_402_server_with_amount(5000).await;
    let response = reqwest::get(&server.uri()).await.unwrap();
    let body = response.text().await.unwrap();

    assert!(body.contains("5000"));
}
```

**Testing network errors:**
```rust
#[tokio::test]
async fn test_error_handling() {
    let server = mock_error_server(500).await;
    let response = reqwest::get(&server.uri()).await.unwrap();

    assert_eq!(response.status(), 500);
}
```

**Testing timeouts:**
```rust
#[tokio::test]
async fn test_timeout() {
    let server = mock_timeout_server().await;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(100))
        .build()
        .unwrap();

    let result = client.get(&server.uri()).await;
    assert!(result.is_err()); // Should timeout
}
```

### 2. CLI Runner (`cli_runner.rs`)

Wrapper around `assert_cmd` for easier CLI testing.

#### Available Functions

```rust
// Direct command runners
run_check(url: &str) -> CommandResult
run_check_json(url: &str) -> CommandResult
run_doctor() -> CommandResult
run_doctor_json() -> CommandResult
run_test(suite_path: &str) -> CommandResult
run_test_json(suite_path: &str) -> CommandResult
run_help() -> CommandResult
run_version() -> CommandResult
run_custom(args: &[&str]) -> CommandResult

// Builder pattern
CliRunner::new()
    .arg("check")
    .arg("http://example.com")
    .env("DEBUG", "1")
    .run()
```

#### CommandResult API

```rust
pub struct CommandResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub raw_output: Output,
}

// Methods
result.success() -> bool           // exit_code == 0
result.failed() -> bool            // exit_code != 0
result.has_warnings() -> bool      // exit_code == 2
result.json() -> Result<Value>     // Parse stdout as JSON
result.stdout_contains(&str) -> bool
result.stderr_contains(&str) -> bool
```

#### Examples

**Testing check command:**
```rust
#[test]
fn test_check_command() {
    let result = run_check("http://localhost:8402/data");

    assert!(result.success());
    assert!(result.stdout_contains("402"));
}
```

**Testing with JSON output:**
```rust
#[test]
fn test_check_json_output() {
    let result = run_check_json("http://localhost:8402/data");
    let json = result.json().unwrap();

    assert_eq!(json["status"], "payment_required");
    assert!(json["invoice"].is_object());
}
```

**Using the builder pattern:**
```rust
#[test]
fn test_custom_command() {
    let result = CliRunner::new()
        .arg("check")
        .arg("http://example.com")
        .arg("--json")
        .env("X402_DEBUG", "1")
        .run();

    assert!(result.success());
}
```

**Testing error scenarios:**
```rust
#[test]
fn test_invalid_url() {
    let result = run_check("invalid-url");

    assert!(result.failed());
    assert_stderr_contains(&result, "error");
}
```

### 3. Assertions (`assertions.rs`)

Domain-specific assertions for x402-dev testing.

#### Invoice Validation

```rust
assert_invoice_valid(output: &str)
```
Validates x402 invoice structure with required fields:
- `amount` (positive integer)
- `currency` (string, typically "SOL")
- `address` (non-empty string)
- `expires_at` (ISO 8601 timestamp)

**Example:**
```rust
let invoice = r#"{
    "amount": 1000,
    "currency": "SOL",
    "address": "test-address",
    "expires_at": "2024-12-31T23:59:59Z"
}"#;
assert_invoice_valid(invoice);
```

#### Diagnostic Output

```rust
assert_diagnostic_format(output: &str)
```
Validates diagnostic output contains:
- Section headers (e.g., "System Diagnostics")
- Status indicators (✓, ✗, ⚠)

**Example:**
```rust
let output = "System Diagnostics\n✓ Rust: 1.70.0\n✗ Missing dependency\n";
assert_diagnostic_format(output);
```

#### JSON Structure

```rust
assert_json_structure(output: &str, required_fields: &[&str])
assert_json_field_equals(output: &str, field: &str, expected: &str)
assert_json_field_is_number(output: &str, field: &str)
```

**Examples:**
```rust
let json = r#"{"status": "ok", "message": "Success"}"#;
assert_json_structure(json, &["status", "message"]);
assert_json_field_equals(json, "status", "ok");

let json_with_amount = r#"{"amount": 1000}"#;
assert_json_field_is_number(json_with_amount, "amount");
```

#### Exit Codes

```rust
assert_exit_code_success(code: i32)  // code == 0
assert_exit_code_failure(code: i32)  // code == 1
assert_exit_code_warnings(code: i32) // code == 2
```

**Example:**
```rust
let result = run_check("http://localhost:8402");
assert_exit_code_success(result.exit_code);
```

#### Color Output

```rust
assert_contains_color_green(output: &str)   // Success messages
assert_contains_color_yellow(output: &str)  // Warning messages
assert_contains_color_red(output: &str)     // Error messages
```

**Example:**
```rust
let output = "\x1b[32mSuccess\x1b[0m";
assert_contains_color_green(output);
```

#### Payment Information

```rust
assert_contains_payment_amount(output: &str)
assert_contains_invoice(output: &str)
assert_payment_required(output: &str)
```

**Examples:**
```rust
let output = "Payment required: 1000 lamports (0.000001 SOL)";
assert_contains_payment_amount(output);
assert_payment_required(output);
```

#### Utility Assertions

```rust
assert_contains_url(output: &str)
assert_contains_timestamp(output: &str)
```

## 🎯 Complete Test Examples

### Example 1: Testing check command with mock server

```rust
#[tokio::test]
async fn test_check_command_end_to_end() {
    // Setup mock 402 server
    let server = mock_server::mock_402_server().await;

    // Run check command
    let result = cli_runner::run_check(&server.uri());

    // Validate exit code
    assertions::assert_exit_code_success(result.exit_code);

    // Validate payment required
    assertions::assert_payment_required(&result.stdout);

    // Validate invoice present
    assertions::assert_contains_invoice(&result.stdout);
    assertions::assert_contains_payment_amount(&result.stdout);
}
```

### Example 2: Testing doctor command

```rust
#[test]
fn test_doctor_command_diagnostics() {
    // Run doctor command
    let result = cli_runner::run_doctor();

    // Should succeed
    assert!(result.success());

    // Should have diagnostic format
    assertions::assert_diagnostic_format(&result.stdout);

    // Should contain system info
    assert!(result.stdout_contains("Rust"));
    assert!(result.stdout_contains("Cargo"));
}
```

### Example 3: Testing with JSON output

```rust
#[tokio::test]
async fn test_check_json_output() {
    let server = mock_server::mock_402_server_with_amount(2000).await;

    let result = cli_runner::run_check_json(&server.uri());
    let json = result.json().expect("Invalid JSON");

    // Validate JSON structure
    assertions::assert_json_structure(
        &result.stdout,
        &["status", "invoice", "message"]
    );

    // Validate specific fields
    assertions::assert_json_field_equals(&result.stdout, "status", "payment_required");

    // Validate invoice in JSON
    let invoice_str = json["invoice"].to_string();
    assertions::assert_invoice_valid(&invoice_str);
}
```

### Example 4: Testing error handling

```rust
#[tokio::test]
async fn test_network_error_handling() {
    let server = mock_server::mock_error_server(500).await;

    let result = cli_runner::run_check(&server.uri());

    // Should handle error gracefully
    assertions::assert_exit_code_failure(result.exit_code);

    // Should show error message
    assertions::assert_contains_color_red(&result.stderr);
    assert!(result.stderr_contains("error"));
}
```

### Example 5: Testing timeout scenarios

```rust
#[tokio::test]
async fn test_timeout_handling() {
    let server = mock_server::mock_timeout_server().await;

    let result = cli_runner::CliRunner::new()
        .arg("check")
        .arg(&server.uri())
        .arg("--timeout")
        .arg("100")
        .run();

    // Should timeout and fail
    assertions::assert_exit_code_failure(result.exit_code);
    assert!(result.stderr_contains("timeout"));
}
```

## 🔧 Integration with Existing Tests

To use these helpers in your tests:

```rust
// At the top of your test file
#[path = "../helpers/mod.rs"]
mod helpers;

use crate::helpers::{mock_server, cli_runner, assertions};

// Then use the helpers as shown in examples above
```

## 📊 Test Coverage

The helper modules include comprehensive unit tests:

- **mock_server.rs**: 3 unit tests covering basic functionality
- **cli_runner.rs**: 3 unit tests covering CommandResult
- **assertions.rs**: 10 unit tests covering all assertion functions

Run tests:
```bash
cargo test --test integration
```

## 🎨 Best Practices

1. **Always use mock servers** instead of real endpoints in tests
2. **Test both success and failure cases** for robustness
3. **Use specific assertions** instead of generic string contains
4. **Test JSON output** when available for structured validation
5. **Verify exit codes** to ensure proper error handling
6. **Use the builder pattern** for complex CLI scenarios

## 🐛 Troubleshooting

### Mock server not responding
- Ensure you're using `.await` for async mock server creation
- Check that the tokio runtime is properly configured

### CLI command not found
- Ensure the binary is built: `cargo build`
- Check that the binary name matches in `Command::cargo_bin()`

### JSON parsing errors
- Use `result.json()` instead of manual parsing
- Verify the command uses `--json` flag for JSON output

### Timeout tests failing
- Adjust timeout duration based on CI environment
- Use `#[ignore]` for slow tests in CI

## 📝 Contributing

When adding new helpers:

1. Add comprehensive doc comments with examples
2. Include unit tests in the same file
3. Update this README with usage examples
4. Follow existing patterns for consistency

## 📚 Related Documentation

- [Epic 4 Test Specification](../epic4_check_command_spec.md)
- [Integration Test Report](../../docs/EPIC-4-INTEGRATION-TEST-REPORT.md)
- [Testing Architecture](../../docs/TESTING_ARCHITECTURE.md)
