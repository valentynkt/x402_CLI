use proptest::prelude::*;
use serde::{Deserialize, Serialize};

/// Invoice structure matching the main application
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct X402Invoice {
    amount: u64,
    currency: String,
    address: String,
    expires_at: String,
}

/// Validation result for invoice properties
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValidationResult {
    Valid,
    InvalidBase58,
    InvalidAmount,
    #[allow(dead_code)]
    InvalidMemo,
    #[allow(dead_code)]
    InvalidNetwork,
    InvalidTimestamp,
}

// ============================================================================
// CUSTOM STRATEGIES FOR PROPERTY GENERATION
// ============================================================================

/// Generate valid Base58 characters (Bitcoin alphabet)
fn valid_base58_char() -> impl Strategy<Value = char> {
    prop_oneof![
        prop::char::range('1', '9'),
        prop::char::range('A', 'H'),
        prop::char::range('J', 'N'),
        prop::char::range('P', 'Z'),
        prop::char::range('a', 'k'),
        prop::char::range('m', 'z'),
    ]
}

/// Generate valid Base58 strings (32-44 characters for Solana addresses)
fn valid_base58_address() -> impl Strategy<Value = String> {
    prop::collection::vec(valid_base58_char(), 32..=44)
        .prop_map(|chars| chars.into_iter().collect())
}

/// Generate invalid Base58 strings (contain 0, O, I, l)
fn invalid_base58_address() -> impl Strategy<Value = String> {
    "[0OIl]+[a-zA-Z0-9]*".prop_map(|s| s)
}

/// Generate valid payment amounts (1 to 1 trillion lamports)
fn valid_amount() -> impl Strategy<Value = u64> {
    1u64..=1_000_000_000_000u64
}

/// Generate invalid amounts (zero or boundary cases)
#[allow(dead_code)]
fn invalid_amount() -> impl Strategy<Value = u64> {
    prop_oneof![Just(0u64), Just(u64::MAX)]
}

/// Generate valid memo formats (req- prefix)
#[allow(dead_code)]
fn valid_memo() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9]{8,32}".prop_map(|suffix| format!("req-{}", suffix))
}

/// Generate invalid memo formats (missing req- prefix)
#[allow(dead_code)]
fn invalid_memo() -> impl Strategy<Value = String> {
    "(?!req-)[a-zA-Z0-9]{8,32}".prop_map(|s| s)
}

/// Generate valid Solana networks
fn valid_network() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("devnet".to_string()),
        Just("testnet".to_string()),
        Just("mainnet-beta".to_string()),
    ]
}

/// Generate arbitrary valid invoices
fn arbitrary_valid_invoice() -> impl Strategy<Value = X402Invoice> {
    (valid_amount(), valid_base58_address(), valid_network()).prop_map(
        |(amount, address, _network)| {
            let expires_at = chrono::Utc::now()
                .checked_add_signed(chrono::Duration::minutes(15))
                .unwrap()
                .to_rfc3339();

            X402Invoice {
                amount,
                currency: "SOL".to_string(),
                address,
                expires_at,
            }
        },
    )
}

// ============================================================================
// VALIDATION FUNCTIONS
// ============================================================================

/// Validate Base58 address format
fn validate_base58(address: &str) -> bool {
    const BASE58_ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    if address.len() < 32 || address.len() > 44 {
        return false;
    }

    address.bytes().all(|b| BASE58_ALPHABET.contains(&b))
}

/// Validate payment amount
fn validate_amount(amount: u64) -> bool {
    amount > 0 && amount <= 1_000_000_000_000
}

/// Validate timestamp format (ISO 8601)
fn validate_timestamp(timestamp: &str) -> bool {
    chrono::DateTime::parse_from_rfc3339(timestamp).is_ok()
}

/// Validate invoice expiration
fn validate_not_expired(timestamp: &str) -> bool {
    if let Ok(expires) = chrono::DateTime::parse_from_rfc3339(timestamp) {
        expires.timestamp() > chrono::Utc::now().timestamp()
    } else {
        false
    }
}

/// Comprehensive invoice validation
fn validate_invoice(invoice: &X402Invoice) -> ValidationResult {
    if !validate_base58(&invoice.address) {
        return ValidationResult::InvalidBase58;
    }

    if !validate_amount(invoice.amount) {
        return ValidationResult::InvalidAmount;
    }

    if !validate_timestamp(&invoice.expires_at) {
        return ValidationResult::InvalidTimestamp;
    }

    ValidationResult::Valid
}

// ============================================================================
// PROPERTY TESTS: BASE58 ADDRESS VALIDATION
// ============================================================================

#[cfg(test)]
mod base58_properties {
    use super::*;

    proptest! {
        /// Property: Any valid Base58 string (32-44 chars) should always validate
        #[test]
        fn valid_base58_always_validates(address in valid_base58_address()) {
            prop_assert!(
                validate_base58(&address),
                "Valid Base58 address '{}' should pass validation", address
            );
        }

        /// Property: Strings containing 0, O, I, l should always fail Base58 validation
        #[test]
        fn invalid_base58_always_fails(address in invalid_base58_address()) {
            prop_assert!(
                !validate_base58(&address),
                "Invalid Base58 address '{}' should fail validation", address
            );
        }

        /// Property: Base58 validation respects case and invalid characters
        #[test]
        fn base58_handles_case_transformations(address in valid_base58_address()) {
            let uppercase = address.to_uppercase();
            let lowercase = address.to_lowercase();

            // Uppercase might introduce invalid chars (O, I, L)
            // Lowercase might introduce invalid char (l)
            // At least the original should validate
            let original_valid = validate_base58(&address);
            prop_assert!(original_valid, "Original valid Base58 address should validate");

            // Case transformations may or may not be valid depending on resulting chars
            let _upper_valid = validate_base58(&uppercase);
            let _lower_valid = validate_base58(&lowercase);
            // No assertion - case transforms may introduce invalid Base58 chars
        }

        /// Property: Short addresses (< 32 chars) should always fail
        #[test]
        fn short_addresses_fail(len in 1usize..32) {
            let address = "1".repeat(len);
            prop_assert!(
                !validate_base58(&address),
                "Address with {} chars should fail (minimum 32)", len
            );
        }

        /// Property: Long addresses (> 44 chars) should always fail
        #[test]
        fn long_addresses_fail(len in 45usize..100) {
            let address = "1".repeat(len);
            prop_assert!(
                !validate_base58(&address),
                "Address with {} chars should fail (maximum 44)", len
            );
        }
    }
}

// ============================================================================
// PROPERTY TESTS: AMOUNT VALIDATION
// ============================================================================

#[cfg(test)]
mod amount_properties {
    use super::*;

    proptest! {
        /// Property: Any positive amount (1 to 1 trillion) should validate
        #[test]
        fn positive_amounts_always_validate(amount in valid_amount()) {
            prop_assert!(
                validate_amount(amount),
                "Positive amount {} should be valid", amount
            );
        }

        /// Property: Amount validation is deterministic
        #[test]
        fn amount_validation_is_deterministic(amount in any::<u64>()) {
            let result1 = validate_amount(amount);
            let result2 = validate_amount(amount);
            prop_assert_eq!(result1, result2, "Validation must be deterministic");
        }

        /// Property: Adding amounts should preserve validity
        #[test]
        fn amount_addition_preserves_validity(a in 1u64..500_000_000_000, b in 1u64..500_000_000_000) {
            if let Some(sum) = a.checked_add(b) {
                let a_valid = validate_amount(a);
                let b_valid = validate_amount(b);
                let sum_valid = validate_amount(sum);

                prop_assert!(
                    !a_valid || !b_valid || sum_valid,
                    "If both amounts are valid, their sum should be valid (if no overflow)"
                );
            }
        }
    }

    // Non-property tests for fixed values
    #[test]
    fn zero_amount_always_fails() {
        assert!(!validate_amount(0), "Zero amount should fail validation");
    }

    #[test]
    fn minimum_amount_validates() {
        assert!(
            validate_amount(1),
            "Minimum amount (1 lamport) should validate"
        );
    }

    #[test]
    fn maximum_amount_validates() {
        let max_amount = 1_000_000_000_000u64; // 1000 SOL
        assert!(
            validate_amount(max_amount),
            "Maximum amount should validate"
        );
    }
}

// ============================================================================
// PROPERTY TESTS: TIMESTAMP VALIDATION
// ============================================================================

#[cfg(test)]
mod timestamp_properties {
    use super::*;

    proptest! {
        /// Property: Valid RFC3339 timestamps should always parse
        #[test]
        fn valid_timestamps_always_parse(offset_minutes in 1i64..1440) {
            let timestamp = chrono::Utc::now()
                .checked_add_signed(chrono::Duration::minutes(offset_minutes))
                .unwrap()
                .to_rfc3339();

            prop_assert!(
                validate_timestamp(&timestamp),
                "Valid RFC3339 timestamp should parse: {}", timestamp
            );
        }

        /// Property: Future timestamps should not be expired
        #[test]
        fn future_timestamps_not_expired(offset_minutes in 1i64..1440) {
            let timestamp = chrono::Utc::now()
                .checked_add_signed(chrono::Duration::minutes(offset_minutes))
                .unwrap()
                .to_rfc3339();

            prop_assert!(
                validate_not_expired(&timestamp),
                "Future timestamp should not be expired: {}", timestamp
            );
        }

        /// Property: Past timestamps should be expired
        #[test]
        fn past_timestamps_are_expired(offset_minutes in 1i64..1440) {
            let timestamp = chrono::Utc::now()
                .checked_sub_signed(chrono::Duration::minutes(offset_minutes))
                .unwrap()
                .to_rfc3339();

            prop_assert!(
                !validate_not_expired(&timestamp),
                "Past timestamp should be expired: {}", timestamp
            );
        }

        /// Property: Invalid timestamp formats should fail parsing
        #[test]
        fn invalid_timestamps_fail_parsing(invalid in "[a-z0-9]{10,50}") {
            if !invalid.contains('T') && !invalid.contains('Z') {
                prop_assert!(
                    !validate_timestamp(&invalid),
                    "Invalid timestamp format should fail: {}", invalid
                );
            }
        }
    }
}

// ============================================================================
// PROPERTY TESTS: IDEMPOTENCY AND CONSISTENCY
// ============================================================================

#[cfg(test)]
mod idempotency_properties {
    use super::*;

    proptest! {
        /// Property: Validation is idempotent (same result on repeated calls)
        #[test]
        fn validation_is_idempotent(invoice in arbitrary_valid_invoice()) {
            let result1 = validate_invoice(&invoice);
            let result2 = validate_invoice(&invoice);
            let result3 = validate_invoice(&invoice);

            prop_assert_eq!(result1, result2, "First and second validation must match");
            prop_assert_eq!(result2, result3, "Second and third validation must match");
        }

        /// Property: Cloning an invoice doesn't change validation result
        #[test]
        fn clone_preserves_validation(invoice in arbitrary_valid_invoice()) {
            let cloned = invoice.clone();
            let original_result = validate_invoice(&invoice);
            let cloned_result = validate_invoice(&cloned);

            prop_assert_eq!(
                original_result, cloned_result,
                "Cloning should not affect validation result"
            );
        }

        /// Property: Serialization and deserialization preserve validity
        #[test]
        fn serialization_preserves_validity(invoice in arbitrary_valid_invoice()) {
            let json = serde_json::to_string(&invoice).unwrap();
            let deserialized: X402Invoice = serde_json::from_str(&json).unwrap();

            let original_result = validate_invoice(&invoice);
            let deserialized_result = validate_invoice(&deserialized);

            prop_assert_eq!(
                original_result, deserialized_result,
                "Serialization round-trip should preserve validity"
            );
        }
    }
}

// ============================================================================
// PROPERTY TESTS: COMPREHENSIVE INVOICE VALIDATION
// ============================================================================

#[cfg(test)]
mod comprehensive_validation {
    use super::*;

    proptest! {
        /// Property: Valid invoices should pass all validations
        #[test]
        fn valid_invoices_pass_validation(invoice in arbitrary_valid_invoice()) {
            let result = validate_invoice(&invoice);
            prop_assert_eq!(
                result, ValidationResult::Valid,
                "Valid invoice should pass validation: {:?}", invoice
            );
        }

        /// Property: Invoices with invalid address should fail with InvalidBase58
        #[test]
        fn invalid_address_fails_validation(
            amount in valid_amount(),
            address in invalid_base58_address()
        ) {
            let invoice = X402Invoice {
                amount,
                currency: "SOL".to_string(),
                address,
                expires_at: chrono::Utc::now()
                    .checked_add_signed(chrono::Duration::minutes(15))
                    .unwrap()
                    .to_rfc3339(),
            };

            let result = validate_invoice(&invoice);
            prop_assert_eq!(
                result, ValidationResult::InvalidBase58,
                "Invalid address should result in InvalidBase58"
            );
        }

        /// Property: Invoices with zero amount should fail with InvalidAmount
        #[test]
        fn zero_amount_fails_invoice_validation(address in valid_base58_address()) {
            let invoice = X402Invoice {
                amount: 0,
                currency: "SOL".to_string(),
                address,
                expires_at: chrono::Utc::now()
                    .checked_add_signed(chrono::Duration::minutes(15))
                    .unwrap()
                    .to_rfc3339(),
            };

            let result = validate_invoice(&invoice);
            prop_assert_eq!(
                result, ValidationResult::InvalidAmount,
                "Zero amount should result in InvalidAmount"
            );
        }

        /// Property: Currency should always be "SOL"
        #[test]
        fn currency_is_always_sol(invoice in arbitrary_valid_invoice()) {
            prop_assert_eq!(
                invoice.currency, "SOL",
                "Currency should always be SOL"
            );
        }
    }
}

// ============================================================================
// PROPERTY TESTS: BOUNDARY CONDITIONS
// ============================================================================

#[cfg(test)]
mod boundary_conditions {
    use super::*;

    // Non-property tests for boundary conditions
    #[test]
    fn minimum_address_length_validates() {
        let address = "1".repeat(32);
        assert!(
            validate_base58(&address),
            "32-character address should validate"
        );
    }

    #[test]
    fn maximum_address_length_validates() {
        let address = "1".repeat(44);
        assert!(
            validate_base58(&address),
            "44-character address should validate"
        );
    }

    #[test]
    fn below_minimum_length_fails() {
        let address = "1".repeat(31);
        assert!(
            !validate_base58(&address),
            "31-character address should fail"
        );
    }

    #[test]
    fn above_maximum_length_fails() {
        let address = "1".repeat(45);
        assert!(
            !validate_base58(&address),
            "45-character address should fail"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sanity check: ensure property tests compile and basic validation works
    #[test]
    fn test_basic_validation() {
        // Valid invoice
        let valid_invoice = X402Invoice {
            amount: 1000,
            currency: "SOL".to_string(),
            address: "11111111111111111111111111111111".to_string(),
            expires_at: chrono::Utc::now()
                .checked_add_signed(chrono::Duration::minutes(15))
                .unwrap()
                .to_rfc3339(),
        };
        assert_eq!(validate_invoice(&valid_invoice), ValidationResult::Valid);

        // Invalid amount
        let invalid_amount = X402Invoice {
            amount: 0,
            currency: "SOL".to_string(),
            address: "11111111111111111111111111111111".to_string(),
            expires_at: chrono::Utc::now()
                .checked_add_signed(chrono::Duration::minutes(15))
                .unwrap()
                .to_rfc3339(),
        };
        assert_eq!(
            validate_invoice(&invalid_amount),
            ValidationResult::InvalidAmount
        );

        // Invalid address (contains 'O')
        let invalid_address = X402Invoice {
            amount: 1000,
            currency: "SOL".to_string(),
            address: "O1111111111111111111111111111111".to_string(),
            expires_at: chrono::Utc::now()
                .checked_add_signed(chrono::Duration::minutes(15))
                .unwrap()
                .to_rfc3339(),
        };
        assert_eq!(
            validate_invoice(&invalid_address),
            ValidationResult::InvalidBase58
        );
    }
}
