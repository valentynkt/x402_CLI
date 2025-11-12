# Property-Based Testing Suite

This directory contains comprehensive property-based tests using the `proptest` crate for invoice validation and policy engine invariants.

## Overview

Property-based testing validates **invariants** that should hold true for all possible inputs, rather than testing specific cases. This approach provides:

- **Broader coverage**: Tests thousands of generated inputs automatically
- **Edge case discovery**: Finds boundary conditions and corner cases
- **Shrinking**: Automatically minimizes failing test cases to simplest form
- **Regression protection**: Failed cases are saved and replayed

## Test Files

### 1. `invoice_properties_test.rs` (27 tests)

Tests invoice validation invariants for the x402 payment system:

**Base58 Address Validation (6 tests)**
- Valid Base58 strings always validate
- Invalid characters (0, O, I, l) always fail
- Case transformation handling
- Short addresses (< 32 chars) fail
- Long addresses (> 44 chars) fail

**Amount Validation (6 tests)**
- Positive amounts (1 to 1 trillion lamports) validate
- Zero amount fails
- Minimum (1 lamport) and maximum amounts validate
- Validation is deterministic
- Amount addition preserves validity

**Timestamp Validation (4 tests)**
- Valid RFC3339 timestamps parse correctly
- Future timestamps are not expired
- Past timestamps are expired
- Invalid timestamp formats fail parsing

**Idempotency and Consistency (3 tests)**
- Validation is idempotent (same result on repeated calls)
- Cloning preserves validation results
- Serialization round-trip preserves validity

**Comprehensive Invoice Validation (4 tests)**
- Valid invoices pass all validations
- Invalid addresses fail with InvalidBase58
- Zero amounts fail with InvalidAmount
- Currency is always "SOL"

**Boundary Conditions (4 tests)**
- Minimum valid address length (32 chars)
- Maximum valid address length (44 chars)
- Below minimum length fails
- Above maximum length fails

### 2. `policy_properties_test.rs` (21 tests)

Tests policy engine invariants for request authorization:

**YAML Parsing (2 tests)**
- Parsing any string never panics
- Valid YAML handling returns Result type

**Code Generation (5 tests)**
- Code generation is deterministic
- Generated code includes version number
- Generated code includes all rule IDs
- Generated code is never empty
- Generated code structure is consistent

**Policy Evaluation (4 tests)**
- Evaluation is consistent (deterministic)
- Evaluation never panics
- Higher priority rules are evaluated first
- Payment status affects payment-required policies

**Serialization (4 tests)**
- Serialization round-trip preserves policy
- Serialization is deterministic
- Cloning preserves policy equality
- Cloned policies produce same evaluation results

**Invariants (4 tests)**
- Policies with no rules use default action
- Version string is preserved
- Timeout is always positive
- Rule priorities are preserved

**Unit Tests (2 tests)**
- Basic evaluation sanity check
- Code generation sanity check

## Running the Tests

### Run all property tests:
```bash
cargo test --release --test invoice_properties_test --test policy_properties_test
```

### Run specific test file:
```bash
cargo test --release --test invoice_properties_test
cargo test --release --test policy_properties_test
```

### Run specific test:
```bash
cargo test --release valid_base58_always_validates
```

### Run with verbose output:
```bash
cargo test --release -- --nocapture
```

### Important: Always use `--release` flag

Property-based tests generate many test cases and run faster in release mode:
- Debug mode: ~2-3 seconds per test file
- Release mode: ~0.1 seconds per test file

## Test Statistics

- **Total tests**: 48 (27 invoice + 21 policy)
- **Total lines of code**: ~1,150 lines
- **Test cases per property**: 256-1000 (configurable)
- **Execution time (release)**: ~0.1 seconds
- **Test coverage**: Invariants, not specific cases

## Key Features

### 1. Custom Strategies

We define custom `Strategy` implementations to generate domain-specific data:

```rust
/// Generate valid Base58 strings (32-44 characters for Solana addresses)
fn valid_base58_address() -> impl Strategy<Value = String> {
    prop::collection::vec(valid_base58_char(), 32..=44)
        .prop_map(|chars| chars.into_iter().collect())
}
```

### 2. Property Assertions

Tests use `prop_assert!` to check invariants:

```rust
proptest! {
    #[test]
    fn valid_base58_always_validates(address in valid_base58_address()) {
        prop_assert!(
            validate_base58(&address),
            "Valid Base58 address should pass validation"
        );
    }
}
```

### 3. Shrinking

When a test fails, proptest automatically finds the minimal failing case:

```
minimal failing input: address = "1L1111111111111111111111111111i1"
```

### 4. Regression Files

Failed tests are saved to `.proptest-regressions/` for replay:

```bash
tests/property/invoice_properties_test.proptest-regressions
```

## Test Organization

Each test file is organized by test category using modules:

```rust
#[cfg(test)]
mod base58_properties {
    use super::*;

    proptest! {
        // Tests for Base58 validation
    }
}

#[cfg(test)]
mod amount_properties {
    use super::*;

    proptest! {
        // Tests for amount validation
    }
}
```

## Dependencies

```toml
[dev-dependencies]
proptest = "1.4"          # Property-based testing framework
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"        # JSON serialization for tests
serde_yaml = "0.9"        # YAML parsing for policy tests
chrono = "0.4"            # Timestamp validation
```

## Implementation Requirements

### For Invoice Validation

1. **Base58 Validation**: Validate Solana address format (32-44 chars, no 0OIl)
2. **Amount Validation**: Positive amounts only, up to 1 trillion lamports
3. **Timestamp Validation**: RFC3339 format, expiration checking
4. **Idempotency**: Same inputs always produce same results
5. **Serialization**: Round-trip preservation of data

### For Policy Engine

1. **YAML Parsing**: Safe parsing that never panics
2. **Code Generation**: Deterministic code generation from policies
3. **Policy Evaluation**: Consistent rule evaluation with priority handling
4. **Serialization**: Round-trip preservation of policy configuration
5. **Invariants**: Version preservation, positive timeouts, priority ordering

## Best Practices

1. **Test Invariants, Not Cases**: Focus on properties that should always hold
2. **Use Custom Strategies**: Generate realistic domain-specific data
3. **Add Context to Assertions**: Include helpful error messages
4. **Run in Release Mode**: Property tests are much faster when optimized
5. **Review Minimal Failures**: Proptest shrinks failures to simplest form
6. **Commit Regression Files**: Share discovered edge cases with team
7. **Deterministic Results**: Ensure functions are pure (no randomness, no I/O)

## Future Enhancements

Potential areas for additional property-based tests:

1. **Network Validation**: Test devnet/testnet/mainnet-beta handling
2. **Memo Validation**: Test req- prefix requirements
3. **Payment Proof Validation**: Test transaction ID validation
4. **Policy Condition Evaluation**: Test expression parsing and evaluation
5. **Rate Limiting**: Test request count and time window logic
6. **Concurrent Access**: Test thread-safety properties
7. **Error Recovery**: Test error handling and recovery paths

## Troubleshooting

### Test failures with "proptest-regressions"

If tests fail consistently, check the `.proptest-regressions/` directory for saved failing cases. These represent edge cases that should be fixed or documented.

### Slow test execution

Always use `--release` flag:
```bash
cargo test --release --test invoice_properties_test
```

### Type errors with strategies

Ensure `Copy` or `Clone` traits are derived for types used in multiple assertions:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValidationResult { /* ... */ }
```

## References

- [Proptest Documentation](https://docs.rs/proptest/)
- [Property-Based Testing Guide](https://hypothesis.works/articles/what-is-property-based-testing/)
- [Rust Proptest Book](https://proptest-rs.github.io/proptest/proptest/index.html)

---

**Test Results**: ✅ 48/48 passing (27 invoice + 21 policy)
