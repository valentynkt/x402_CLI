//! Custom assertion helpers for x402-dev test validation
//!
//! Provides domain-specific assertion functions for validating
//! x402 invoices, diagnostic output, JSON responses, and CLI output formatting.
//!
//! # Examples
//!
//! ```no_run
//! use helpers::assertions::*;
//!
//! #[test]
//! fn test_invoice_validation() {
//!     let output = r#"{"amount": 1000, "currency": "SOL"}"#;
//!     assert_invoice_valid(output);
//! }
//! ```

use serde_json::Value;

/// Assert that a string contains a valid x402 invoice
///
/// Validates that the invoice has required fields:
/// - amount (positive integer)
/// - currency (string, typically "SOL")
/// - address (string, payment destination)
/// - expires_at (ISO 8601 timestamp string)
///
/// # Example
///
/// ```no_run
/// let output = r#"{"amount": 1000, "currency": "SOL", "address": "abc123", "expires_at": "2024-12-31T23:59:59Z"}"#;
/// assert_invoice_valid(output);
/// ```
///
/// # Panics
///
/// Panics if the invoice is invalid or missing required fields
pub fn assert_invoice_valid(output: &str) {
    let json: Value = serde_json::from_str(output)
        .unwrap_or_else(|_| panic!("Failed to parse invoice JSON: {}", output));

    // Check required fields exist
    assert!(
        json.get("amount").is_some(),
        "Invoice missing 'amount' field: {}",
        output
    );

    assert!(
        json.get("currency").is_some(),
        "Invoice missing 'currency' field: {}",
        output
    );

    assert!(
        json.get("address").is_some(),
        "Invoice missing 'address' field: {}",
        output
    );

    assert!(
        json.get("expires_at").is_some(),
        "Invoice missing 'expires_at' field: {}",
        output
    );

    // Validate amount is positive
    let amount = json["amount"].as_u64()
        .unwrap_or_else(|| panic!("Invoice 'amount' is not a valid positive integer: {}", output));

    assert!(
        amount > 0,
        "Invoice amount must be positive, got: {}",
        amount
    );

    // Validate currency is a string
    assert!(
        json["currency"].is_string(),
        "Invoice 'currency' must be a string: {}",
        output
    );

    // Validate address is a non-empty string
    let address = json["address"].as_str()
        .unwrap_or_else(|| panic!("Invoice 'address' is not a string: {}", output));

    assert!(
        !address.is_empty(),
        "Invoice 'address' cannot be empty"
    );

    // Validate expires_at is a string (ISO 8601 format)
    assert!(
        json["expires_at"].is_string(),
        "Invoice 'expires_at' must be a string: {}",
        output
    );
}

/// Assert that diagnostic output follows the expected format
///
/// Validates that the diagnostic output contains:
/// - Section headers (e.g., "System Diagnostics", "Dependencies")
/// - Status indicators (✓, ✗, ⚠)
/// - Proper formatting and structure
///
/// # Example
///
/// ```no_run
/// let output = "System Diagnostics\n✓ Rust: 1.70.0\n✓ Cargo: 1.70.0\n";
/// assert_diagnostic_format(output);
/// ```
pub fn assert_diagnostic_format(output: &str) {
    // Check for common diagnostic sections
    let has_sections = output.contains("Diagnostics")
        || output.contains("System")
        || output.contains("Dependencies")
        || output.contains("Configuration");

    assert!(
        has_sections,
        "Diagnostic output should contain section headers, got: {}",
        output
    );

    // Check for status indicators
    let has_indicators = output.contains('✓')
        || output.contains('✗')
        || output.contains('⚠')
        || output.contains("[OK]")
        || output.contains("[ERROR]")
        || output.contains("[WARN]");

    assert!(
        has_indicators,
        "Diagnostic output should contain status indicators, got: {}",
        output
    );
}

/// Assert that JSON output has expected structure
///
/// Validates that the JSON contains specific fields with correct types.
///
/// # Example
///
/// ```no_run
/// let json = r#"{"status": "ok", "message": "Success"}"#;
/// assert_json_structure(json, &["status", "message"]);
/// ```
pub fn assert_json_structure(output: &str, required_fields: &[&str]) {
    let json: Value = serde_json::from_str(output)
        .unwrap_or_else(|_| panic!("Failed to parse JSON: {}", output));

    for field in required_fields {
        assert!(
            json.get(field).is_some(),
            "JSON missing required field '{}': {}",
            field,
            output
        );
    }
}

/// Assert exit code matches expectation
///
/// Exit codes:
/// - 0: Success
/// - 1: Failure/Error
/// - 2: Warnings
///
/// # Example
///
/// ```no_run
/// assert_exit_code_success(0);
/// assert_exit_code_failure(1);
/// assert_exit_code_warnings(2);
/// ```
pub fn assert_exit_code_success(code: i32) {
    assert_eq!(code, 0, "Expected success exit code (0), got {}", code);
}

pub fn assert_exit_code_failure(code: i32) {
    assert_eq!(code, 1, "Expected failure exit code (1), got {}", code);
}

pub fn assert_exit_code_warnings(code: i32) {
    assert_eq!(code, 2, "Expected warnings exit code (2), got {}", code);
}

/// Assert that output contains color codes
///
/// Validates that the output includes ANSI color codes for formatting.
/// Color codes:
/// - Green: \x1b[32m (success)
/// - Yellow: \x1b[33m (warning)
/// - Red: \x1b[31m (error)
///
/// # Example
///
/// ```no_run
/// let output = "\x1b[32mSuccess\x1b[0m";
/// assert_contains_color_green(output);
/// ```
pub fn assert_contains_color_green(output: &str) {
    assert!(
        output.contains("\x1b[32m") || output.contains("✓"),
        "Output should contain green color codes or success indicator: {}",
        output
    );
}

pub fn assert_contains_color_yellow(output: &str) {
    assert!(
        output.contains("\x1b[33m") || output.contains("⚠"),
        "Output should contain yellow color codes or warning indicator: {}",
        output
    );
}

pub fn assert_contains_color_red(output: &str) {
    assert!(
        output.contains("\x1b[31m") || output.contains("✗"),
        "Output should contain red color codes or error indicator: {}",
        output
    );
}

/// Assert that output contains a payment amount
///
/// Validates that the output includes a lamports amount or SOL amount.
///
/// # Example
///
/// ```no_run
/// let output = "Payment required: 1000 lamports";
/// assert_contains_payment_amount(output);
/// ```
pub fn assert_contains_payment_amount(output: &str) {
    let has_amount = output.contains("lamports")
        || output.contains("SOL")
        || output.contains("amount");

    assert!(
        has_amount,
        "Output should contain payment amount information: {}",
        output
    );
}

/// Assert that output contains an invoice
///
/// Validates that the output includes invoice-related information.
///
/// # Example
///
/// ```no_run
/// let output = r#"{"amount": 1000, "currency": "SOL"}"#;
/// assert_contains_invoice(output);
/// ```
pub fn assert_contains_invoice(output: &str) {
    let has_invoice = output.contains("invoice")
        || output.contains("amount")
        || output.contains("currency")
        || (output.contains("lamports") && output.contains("SOL"));

    assert!(
        has_invoice,
        "Output should contain invoice information: {}",
        output
    );
}

/// Assert that output indicates payment is required
///
/// # Example
///
/// ```no_run
/// let output = "402 Payment Required";
/// assert_payment_required(output);
/// ```
pub fn assert_payment_required(output: &str) {
    let has_payment_required = output.contains("Payment Required")
        || output.contains("payment required")
        || output.contains("Payment required")
        || output.contains("402");

    assert!(
        has_payment_required,
        "Output should indicate payment is required: {}",
        output
    );
}

/// Assert that JSON contains specific key-value pair
///
/// # Example
///
/// ```no_run
/// let json = r#"{"status": "ok"}"#;
/// assert_json_field_equals(json, "status", "ok");
/// ```
pub fn assert_json_field_equals(output: &str, field: &str, expected: &str) {
    let json: Value = serde_json::from_str(output)
        .unwrap_or_else(|_| panic!("Failed to parse JSON: {}", output));

    let actual = json[field].as_str()
        .unwrap_or_else(|| panic!("Field '{}' is not a string in: {}", field, output));

    assert_eq!(
        actual, expected,
        "Expected field '{}' to be '{}', got '{}' in: {}",
        field, expected, actual, output
    );
}

/// Assert that JSON field exists and is a number
///
/// # Example
///
/// ```no_run
/// let json = r#"{"amount": 1000}"#;
/// assert_json_field_is_number(json, "amount");
/// ```
pub fn assert_json_field_is_number(output: &str, field: &str) {
    let json: Value = serde_json::from_str(output)
        .unwrap_or_else(|_| panic!("Failed to parse JSON: {}", output));

    assert!(
        json[field].is_number(),
        "Expected field '{}' to be a number in: {}",
        field, output
    );
}

/// Assert that output contains a valid URL
///
/// # Example
///
/// ```no_run
/// let output = "Server at http://localhost:8402";
/// assert_contains_url(output);
/// ```
pub fn assert_contains_url(output: &str) {
    let has_url = output.contains("http://") || output.contains("https://");

    assert!(
        has_url,
        "Output should contain a URL: {}",
        output
    );
}

/// Assert that output contains a timestamp
///
/// # Example
///
/// ```no_run
/// let output = r#"{"timestamp": "2024-01-01T00:00:00Z"}"#;
/// assert_contains_timestamp(output);
/// ```
pub fn assert_contains_timestamp(output: &str) {
    let has_timestamp = output.contains("timestamp")
        || output.contains("T") && output.contains("Z")
        || output.contains("expires_at");

    assert!(
        has_timestamp,
        "Output should contain timestamp information: {}",
        output
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_invoice() {
        let invoice = r#"{
            "amount": 1000,
            "currency": "SOL",
            "address": "test-address",
            "expires_at": "2024-12-31T23:59:59Z"
        }"#;

        assert_invoice_valid(invoice);
    }

    #[test]
    #[should_panic(expected = "missing 'amount'")]
    fn test_invalid_invoice_missing_amount() {
        let invoice = r#"{
            "currency": "SOL",
            "address": "test-address",
            "expires_at": "2024-12-31T23:59:59Z"
        }"#;

        assert_invoice_valid(invoice);
    }

    #[test]
    fn test_diagnostic_format() {
        let output = "System Diagnostics\n✓ Rust: 1.70.0\n✗ Missing dependency\n";
        assert_diagnostic_format(output);
    }

    #[test]
    fn test_json_structure() {
        let json = r#"{"status": "ok", "message": "Success", "code": 200}"#;
        assert_json_structure(json, &["status", "message", "code"]);
    }

    #[test]
    fn test_exit_codes() {
        assert_exit_code_success(0);
        assert_exit_code_failure(1);
        assert_exit_code_warnings(2);
    }

    #[test]
    fn test_color_assertions() {
        let green_output = "\x1b[32mSuccess\x1b[0m";
        assert_contains_color_green(green_output);

        let yellow_output = "\x1b[33mWarning\x1b[0m";
        assert_contains_color_yellow(yellow_output);

        let red_output = "\x1b[31mError\x1b[0m";
        assert_contains_color_red(red_output);
    }

    #[test]
    fn test_payment_assertions() {
        let output = "Payment required: 1000 lamports (0.000001 SOL)";
        assert_contains_payment_amount(output);
        assert_payment_required(output);
    }

    #[test]
    fn test_json_field_assertions() {
        let json = r#"{"status": "ok", "amount": 1000}"#;
        assert_json_field_equals(json, "status", "ok");
        assert_json_field_is_number(json, "amount");
    }

    #[test]
    fn test_url_assertion() {
        let output = "Server running at http://localhost:8402";
        assert_contains_url(output);
    }

    #[test]
    fn test_timestamp_assertion() {
        let output = r#"{"timestamp": "2024-01-01T00:00:00Z"}"#;
        assert_contains_timestamp(output);
    }
}
