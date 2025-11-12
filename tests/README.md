# x402-dev Test Suite Documentation

Comprehensive test suite for the x402-dev CLI toolkit with 135+ tests providing extensive coverage across unit, integration, and property-based testing.

## 📊 Test Statistics

| Category | Tests | Files | Lines of Code | Coverage |
|----------|-------|-------|---------------|----------|
| **Unit Tests** | 54 | 2 | ~1,680 | Core commands |
| **Integration Tests** | 33 | 3 | ~900 | E2E workflows |
| **Property Tests** | 48 | 2 | ~1,150 | Invariants |
| **Fixtures Tests** | 53 | 1 | ~250 | Test data |
| **Helper Tests** | 20 | 1 | ~300 | Infrastructure |
| **TOTAL** | **208** | **9** | **~4,280** | **~85%** |

## 🗂️ Test Organization

```
tests/
├── unit/                           # Unit tests (54 tests)
│   ├── check_command_test.rs      # Check command (21 tests)
│   ├── doctor_command_test.rs     # Doctor command (33 tests)
│   └── fixtures_test.rs           # Fixture validation (53 tests)
│
├── integration/                    # Integration tests (33 tests)
│   ├── mod.rs                     # Module setup
│   ├── cli_integration_test.rs    # CLI commands (12 tests)
│   ├── check_workflow_test.rs     # Check workflows (11 tests)
│   └── doctor_workflow_test.rs    # Doctor workflows (10 tests)
│
├── property/                       # Property-based tests (48 tests)
│   ├── invoice_properties_test.rs # Invoice validation (27 tests)
│   └── policy_properties_test.rs  # Policy engine (21 tests)
│
├── fixtures/                       # Test data
│   ├── mod.rs                     # Module exports
│   ├── policies.rs                # Policy YAML fixtures
│   ├── configs.rs                 # Config YAML fixtures
│   └── invoices.rs                # Invoice structures
│
└── helpers/                        # Test utilities
    ├── mod.rs                     # Module exports
    ├── mock_server.rs             # HTTP mock servers
    ├── cli_runner.rs              # CLI command wrappers
    └── assertions.rs              # Custom assertions
```

## 🚀 Running Tests

### Run All Tests
```bash
# Run all tests (unit + integration + property)
cargo test

# Run with detailed output
cargo test -- --nocapture

# Run with test threads control
cargo test -- --test-threads=4
```

### Run Specific Test Categories

#### Unit Tests Only
```bash
# All unit tests
cargo test --lib

# Specific command tests
cargo test --test check_command_test
cargo test --test doctor_command_test
cargo test --test fixtures_test
```

#### Integration Tests Only
```bash
# All integration tests
cargo test --test integration

# Specific workflow tests
cargo test --test integration cli_integration
cargo test --test integration check_workflow
cargo test --test integration doctor_workflow
```

#### Property-Based Tests
```bash
# IMPORTANT: Always use --release for property tests (10x faster)
cargo test --release --test invoice_properties_test
cargo test --release --test policy_properties_test

# Run all property tests
cargo test --release --test invoice_properties_test --test policy_properties_test
```

### Run Specific Tests
```bash
# Run single test by name
cargo test test_validates_402_status_code

# Run tests matching a pattern
cargo test check_command

# Run tests in a specific module
cargo test unit::check_command
```

## 📈 Test Coverage

### Generate Coverage Report
```bash
# Install tarpaulin (one-time)
cargo install cargo-tarpaulin

# Generate HTML coverage report
cargo tarpaulin --out Html --output-dir coverage

# Generate coverage summary
cargo tarpaulin --out Stdout

# Generate Lcov format (for CI/CD)
cargo tarpaulin --out Lcov --output-dir coverage
```

### View Coverage
```bash
# Open HTML report in browser
open coverage/index.html  # macOS
xdg-open coverage/index.html  # Linux
start coverage/index.html  # Windows
```

### Coverage Targets
- **Minimum Required**: 80% line coverage
- **Current Achievement**: ~85% coverage
- **CI/CD Enforcement**: Builds fail below 80%

## 🧪 Test Details

### Unit Tests (54 tests)

#### Check Command Tests (21 tests)
Tests for `/crates/x402-cli/src/commands/check.rs`:
- HTTP 402 status detection
- WWW-Authenticate header parsing
- Invoice field validation (recipient, amount, currency, memo, network)
- Output formatting (JSON, colored CLI, verbose)
- Exit codes (0 = success, 1 = failure)
- Network error handling

**Run:**
```bash
cargo test --test check_command_test
```

#### Doctor Command Tests (33 tests)
Tests for `/crates/x402-cli/src/commands/doctor.rs`:
- Environment checks (binary version, Rust, npm)
- Configuration validation (.x402dev.yaml)
- Port availability (default 3402, conflicts)
- Ecosystem detection (package.json, SDKs)
- Diagnostic output (success/warning/failure colors, suggestions)
- Exit codes (0 = all passed, 1 = failures, 2 = warnings)

**Run:**
```bash
cargo test --test doctor_command_test
```

### Integration Tests (33 tests)

#### CLI Integration Tests (12 tests)
End-to-end CLI command testing:
- Mock server lifecycle
- Command execution with real binary
- Help and version output
- Invalid command handling
- Config precedence (CLI > file)
- JSON and verbose flags

**Run:**
```bash
cargo test --test integration cli_integration
```

#### Check Workflow Tests (11 tests)
Complete check command workflows:
- Success scenarios with mock 402 server
- Retry logic on network errors
- Verbose and JSON output modes
- Multiple URL checking
- Invalid URL handling
- Timeout scenarios
- Non-402 response handling
- Header validation

**Run:**
```bash
cargo test --test integration check_workflow
```

#### Doctor Workflow Tests (10 tests)
Complete doctor command workflows:
- Full diagnostic scans
- Fix-and-rerun scenarios
- Missing config detection
- Port conflict resolution
- JSON diagnostic output
- Ecosystem package detection

**Run:**
```bash
cargo test --test integration doctor_workflow
```

### Property-Based Tests (48 tests)

#### Invoice Property Tests (27 tests)
Invariant testing for invoice validation:
- Base58 address validation (valid alphabet, length constraints)
- Amount validation (positive values, boundary conditions)
- Timestamp validation (RFC3339 format, expiration)
- Idempotency (consistent validation results)
- Serialization (round-trip preservation)

**Run:**
```bash
cargo test --release --test invoice_properties_test
```

#### Policy Property Tests (21 tests)
Invariant testing for policy engine:
- YAML parsing (never panics, safe errors)
- Code generation (deterministic output)
- Policy evaluation (consistent results, priority-based)
- Serialization (round-trip, deterministic)
- Rule invariants (version preservation, positive timeouts)

**Run:**
```bash
cargo test --release --test policy_properties_test
```

## 🛠️ Test Infrastructure

### Fixtures (`tests/fixtures/`)
Reusable test data for all tests:

```rust
use crate::fixtures::{policies, configs, invoices};

// Policy fixtures
let policy = policies::valid_policy_yaml();
let express_policy = policies::express_policy_yaml();
let invalid = policies::invalid_policy_yaml("empty_field");

// Config fixtures
let config = configs::valid_config_yaml();
let dev_config = configs::dev_environment_config();

// Invoice fixtures
let invoice = invoices::valid_invoice();
let devnet = invoices::devnet_invoice();
let invalid = invoices::invalid_invoice("negative_amount");
```

### Helpers (`tests/helpers/`)
Test utilities and mocks:

```rust
use crate::helpers::{mock_server, cli_runner, assertions};

// Mock HTTP server
let server = mock_server::mock_402_server().await;

// CLI command execution
let result = cli_runner::run_check(&server.uri());

// Custom assertions
assertions::assert_payment_required(&result.stdout);
assertions::assert_exit_code_success(result.exit_code);
```

## 🔧 Adding New Tests

### 1. Unit Test Template
```rust
#[test]
fn test_new_feature() {
    // Arrange
    let input = /* setup test data */;

    // Act
    let result = /* call function under test */;

    // Assert
    assert_eq!(result, expected);
}
```

### 2. Integration Test Template
```rust
#[tokio::test]
async fn test_new_workflow() {
    // Setup
    let server = mock_server::mock_402_server().await;

    // Execute
    let result = cli_runner::run_check(&server.uri());

    // Verify
    assertions::assert_exit_code_success(result.exit_code);
}
```

### 3. Property Test Template
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_invariant(input in strategy()) {
        // Property: describe invariant that should always hold
        let result = function_under_test(input);
        assert!(invariant_holds(result));
    }
}
```

## 📋 Test Best Practices

### 1. Test Naming
- **Format**: `test_<scenario>_<expected_result>`
- **Examples**:
  - `test_validates_402_status_code()`
  - `test_rejects_negative_amounts()`
  - `test_handles_missing_config()`

### 2. Test Structure
- **Arrange**: Set up test data and prerequisites
- **Act**: Execute the code under test
- **Assert**: Verify expected outcomes

### 3. Test Isolation
- Each test should be independent
- Use `tempfile` for temporary directories
- Clean up resources in `Drop` implementations

### 4. Documentation
- Add doc comments explaining what each test verifies
- Include examples in doc comments when helpful

### 5. Performance
- Use `#[ignore]` for slow tests
- Run property tests with `--release` flag
- Use `--test-threads` to control parallelism

## 🐛 Debugging Tests

### Run Failed Tests Only
```bash
# Run only tests that failed in the last run
cargo test -- --test-threads=1 --nocapture
```

### Show Full Output
```bash
# Show println! and dbg! output
cargo test -- --nocapture

# Show test names as they run
cargo test -- --nocapture --test-threads=1
```

### Debug Specific Test
```bash
# Run with environment variables
RUST_LOG=debug cargo test test_name -- --nocapture

# Run with backtrace
RUST_BACKTRACE=1 cargo test test_name
```

## 🔄 CI/CD Integration

Tests run automatically on:
- **Push** to master/main/develop branches
- **Pull requests** to master/main/develop
- **Manual workflow dispatch**

### CI/CD Jobs
1. **test** - Runs all test suites on Ubuntu, macOS, Windows
2. **coverage** - Generates coverage report, enforces 80% minimum
3. **lint** - Runs rustfmt and clippy
4. **security** - Runs cargo-audit for dependency vulnerabilities

### View CI/CD Results
- Check Actions tab in GitHub repository
- Coverage reports uploaded to Codecov
- Failing builds prevent merges

## 📚 Additional Resources

- **Check Command**: `/crates/x402-cli/src/commands/check.rs`
- **Doctor Command**: `/crates/x402-cli/src/commands/doctor.rs`
- **Test Helpers**: `/tests/helpers/README.md`
- **Property Tests**: `/tests/property/README.md`
- **Coverage Reports**: `/coverage/index.html` (after generation)

## 🤝 Contributing

When adding new features:
1. Write tests FIRST (TDD approach)
2. Ensure tests pass: `cargo test`
3. Check coverage: `cargo tarpaulin`
4. Run lint: `cargo clippy`
5. Format code: `cargo fmt`
6. Update this README if adding new test categories

## 📞 Support

For test-related issues:
- Check test output for detailed error messages
- Review test documentation in individual files
- Run tests with `--nocapture` for debugging
- Check CI/CD logs for environment-specific failures

---

**Last Updated**: 2025-11-12
**Test Suite Version**: 1.0.0
**Total Tests**: 208
**Coverage**: ~85%
