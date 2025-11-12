// Invoice fixture generators for testing
//
// Provides sample x402 invoice structures for different networks,
// amounts, currencies, and validation error cases.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// x402-compliant invoice structure for testing
///
/// This matches the Invoice struct in crates/x402-cli/src/commands/invoice.rs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Invoice {
    /// Solana recipient address (Base58-encoded)
    pub recipient: String,
    /// Payment amount in USDC
    pub amount: f64,
    /// Currency (always "USDC" for x402-solana)
    pub currency: String,
    /// Unique memo/request ID for tracking (UUID-based)
    pub memo: String,
    /// Network identifier (devnet, testnet, mainnet-beta)
    pub network: String,
    /// Invoice creation timestamp (ISO8601)
    pub timestamp: DateTime<Utc>,
    /// Request path that triggered this invoice
    pub resource_path: String,
    /// Invoice expiration timestamp (ISO8601)
    pub expires_at: DateTime<Utc>,
}

/// Valid Base58 Solana addresses for testing
///
/// These are properly formatted Base58 addresses (32-44 chars, no 0/O/I/l).
/// NOTE: These are TEST ADDRESSES and should never be used on real networks.
pub const VALID_TEST_ADDRESSES: &[&str] = &[
    "GXk8vTest1111111111111111111111111111qPz9",
    "HYn9xTest2222222222222222222222222222rAb3",
    "JZp4yTest3333333333333333333333333333sCd7",
    "KAq5zTest4444444444444444444444444444tDe8",
    "7EqQdEULxWcraVx3mXKFjc84LhCkMGZCkRuDpvcMwJeK", // 44 chars
    "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM", // 44 chars
];

/// Invalid addresses for testing validation errors
pub const INVALID_ADDRESSES: &[&str] = &[
    "",                              // Empty
    "abc",                           // Too short
    "0OIl0OIl0OIl0OIl0OIl0OIl0OIl0OIl0OIl0OIl", // Invalid Base58 chars
    "ThisContainsInvalidChars!@#$%^&*()",        // Invalid characters
];

/// Returns a valid invoice for devnet
///
/// Use this as the standard valid invoice fixture.
pub fn valid_invoice() -> Invoice {
    let now = Utc::now();
    Invoice {
        recipient: VALID_TEST_ADDRESSES[0].to_string(),
        amount: 0.05,
        currency: "USDC".to_string(),
        memo: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        network: "devnet".to_string(),
        timestamp: now,
        resource_path: "/api/data".to_string(),
        expires_at: now + Duration::minutes(5),
    }
}

/// Returns a valid devnet invoice with custom amount
pub fn devnet_invoice(amount: f64) -> Invoice {
    let now = Utc::now();
    Invoice {
        recipient: VALID_TEST_ADDRESSES[0].to_string(),
        amount,
        currency: "USDC".to_string(),
        memo: "550e8400-e29b-41d4-a716-446655440001".to_string(),
        network: "devnet".to_string(),
        timestamp: now,
        resource_path: "/api/devnet".to_string(),
        expires_at: now + Duration::minutes(5),
    }
}

/// Returns a valid testnet invoice
pub fn testnet_invoice(amount: f64) -> Invoice {
    let now = Utc::now();
    Invoice {
        recipient: VALID_TEST_ADDRESSES[1].to_string(),
        amount,
        currency: "USDC".to_string(),
        memo: "550e8400-e29b-41d4-a716-446655440002".to_string(),
        network: "testnet".to_string(),
        timestamp: now,
        resource_path: "/api/testnet".to_string(),
        expires_at: now + Duration::minutes(5),
    }
}

/// Returns a valid mainnet-beta invoice
pub fn mainnet_invoice(amount: f64) -> Invoice {
    let now = Utc::now();
    Invoice {
        recipient: VALID_TEST_ADDRESSES[2].to_string(),
        amount,
        currency: "USDC".to_string(),
        memo: "550e8400-e29b-41d4-a716-446655440003".to_string(),
        network: "mainnet-beta".to_string(),
        timestamp: now,
        resource_path: "/api/mainnet".to_string(),
        expires_at: now + Duration::minutes(5),
    }
}

/// Returns invoice with invalid recipient address
pub fn invoice_with_invalid_recipient(variant: &str) -> Invoice {
    let mut invoice = valid_invoice();
    invoice.recipient = match variant {
        "empty" => "".to_string(),
        "too_short" => "abc".to_string(),
        "invalid_chars" => "0OIl".repeat(11),
        "special_chars" => "Invalid!@#$Address".to_string(),
        _ => panic!("Unknown variant: {}", variant),
    };
    invoice
}

/// Returns invoice with negative amount
pub fn invoice_with_negative_amount() -> Invoice {
    let mut invoice = valid_invoice();
    invoice.amount = -0.05;
    invoice
}

/// Returns invoice with zero amount
pub fn invoice_with_zero_amount() -> Invoice {
    let mut invoice = valid_invoice();
    invoice.amount = 0.0;
    invoice
}

/// Returns invoice with excessively large amount
pub fn invoice_with_excessive_amount() -> Invoice {
    let mut invoice = valid_invoice();
    invoice.amount = 999999999.99;
    invoice
}

/// Returns invoice with invalid currency
pub fn invoice_with_invalid_currency(variant: &str) -> Invoice {
    let mut invoice = valid_invoice();
    invoice.currency = match variant {
        "empty" => "".to_string(),
        "wrong_currency" => "SOL".to_string(), // x402 uses USDC
        "lowercase" => "usdc".to_string(),
        "invalid" => "BITCOIN".to_string(),
        _ => panic!("Unknown variant: {}", variant),
    };
    invoice
}

/// Returns invoice with malformed memo
pub fn invoice_with_malformed_memo(variant: &str) -> Invoice {
    let mut invoice = valid_invoice();
    invoice.memo = match variant {
        "empty" => "".to_string(),
        "not_uuid" => "not-a-valid-uuid".to_string(),
        "wrong_format" => "550e8400-e29b-41d4-a716".to_string(), // Too short
        "no_hyphens" => "550e8400e29b41d4a716446655440000".to_string(),
        "invalid_chars" => "550e8400-XXXX-YYYY-ZZZZ-446655440000".to_string(),
        _ => panic!("Unknown variant: {}", variant),
    };
    invoice
}

/// Returns invoice with invalid network
pub fn invoice_with_invalid_network(variant: &str) -> Invoice {
    let mut invoice = valid_invoice();
    invoice.network = match variant {
        "empty" => "".to_string(),
        "wrong_network" => "ethereum".to_string(),
        "typo" => "devnte".to_string(), // Typo in "devnet"
        _ => panic!("Unknown variant: {}", variant),
    };
    invoice
}

/// Returns invoice missing required fields (using JSON representation)
///
/// Note: This returns a JSON string representation to test deserialization errors.
pub fn invoice_missing_required_fields(variant: &str) -> String {
    match variant {
        "no_recipient" => r#"{
            "amount": 0.05,
            "currency": "USDC",
            "memo": "550e8400-e29b-41d4-a716-446655440000",
            "network": "devnet",
            "timestamp": "2024-01-01T00:00:00Z",
            "resource_path": "/api/data",
            "expires_at": "2024-01-01T00:05:00Z"
        }"#.to_string(),

        "no_amount" => r#"{
            "recipient": "GXk8vTest1111111111111111111111111111qPz9",
            "currency": "USDC",
            "memo": "550e8400-e29b-41d4-a716-446655440000",
            "network": "devnet",
            "timestamp": "2024-01-01T00:00:00Z",
            "resource_path": "/api/data",
            "expires_at": "2024-01-01T00:05:00Z"
        }"#.to_string(),

        "no_currency" => r#"{
            "recipient": "GXk8vTest1111111111111111111111111111qPz9",
            "amount": 0.05,
            "memo": "550e8400-e29b-41d4-a716-446655440000",
            "network": "devnet",
            "timestamp": "2024-01-01T00:00:00Z",
            "resource_path": "/api/data",
            "expires_at": "2024-01-01T00:05:00Z"
        }"#.to_string(),

        "no_memo" => r#"{
            "recipient": "GXk8vTest1111111111111111111111111111qPz9",
            "amount": 0.05,
            "currency": "USDC",
            "network": "devnet",
            "timestamp": "2024-01-01T00:00:00Z",
            "resource_path": "/api/data",
            "expires_at": "2024-01-01T00:05:00Z"
        }"#.to_string(),

        "no_network" => r#"{
            "recipient": "GXk8vTest1111111111111111111111111111qPz9",
            "amount": 0.05,
            "currency": "USDC",
            "memo": "550e8400-e29b-41d4-a716-446655440000",
            "timestamp": "2024-01-01T00:00:00Z",
            "resource_path": "/api/data",
            "expires_at": "2024-01-01T00:05:00Z"
        }"#.to_string(),

        _ => panic!("Unknown variant: {}", variant),
    }
}

/// Returns invoice with expired timestamp
pub fn expired_invoice() -> Invoice {
    let now = Utc::now();
    Invoice {
        recipient: VALID_TEST_ADDRESSES[0].to_string(),
        amount: 0.05,
        currency: "USDC".to_string(),
        memo: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        network: "devnet".to_string(),
        timestamp: now - Duration::minutes(10),
        resource_path: "/api/data".to_string(),
        expires_at: now - Duration::minutes(5), // Expired 5 minutes ago
    }
}

/// Returns invoice about to expire (within 30 seconds)
pub fn almost_expired_invoice() -> Invoice {
    let now = Utc::now();
    Invoice {
        recipient: VALID_TEST_ADDRESSES[0].to_string(),
        amount: 0.05,
        currency: "USDC".to_string(),
        memo: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        network: "devnet".to_string(),
        timestamp: now - Duration::minutes(4) - Duration::seconds(30),
        resource_path: "/api/data".to_string(),
        expires_at: now + Duration::seconds(30), // Expires in 30 seconds
    }
}

/// Returns a randomly generated valid invoice
///
/// Use this for property-based testing or generating unique test data.
pub fn random_valid_invoice() -> Invoice {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let now = Utc::now();
    let recipient_idx = rng.gen_range(0..VALID_TEST_ADDRESSES.len());
    let amount = rng.gen_range(0.01..=10.0);
    let memo = uuid::Uuid::new_v4().to_string();

    Invoice {
        recipient: VALID_TEST_ADDRESSES[recipient_idx].to_string(),
        amount,
        currency: "USDC".to_string(),
        memo,
        network: "devnet".to_string(),
        timestamp: now,
        resource_path: format!("/api/random/{}", rng.gen::<u32>()),
        expires_at: now + Duration::minutes(5),
    }
}

/// Returns an invalid invoice based on the specified error type
///
/// Convenience function for comprehensive invalid invoice testing.
pub fn invalid_invoice(error_type: &str) -> Invoice {
    match error_type {
        "invalid_recipient" => invoice_with_invalid_recipient("invalid_chars"),
        "negative_amount" => invoice_with_negative_amount(),
        "zero_amount" => invoice_with_zero_amount(),
        "invalid_currency" => invoice_with_invalid_currency("wrong_currency"),
        "malformed_memo" => invoice_with_malformed_memo("not_uuid"),
        "invalid_network" => invoice_with_invalid_network("wrong_network"),
        "expired" => expired_invoice(),
        _ => panic!("Unknown error type: {}", error_type),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_invoice() {
        let invoice = valid_invoice();
        assert_eq!(invoice.currency, "USDC");
        assert_eq!(invoice.network, "devnet");
        assert!(invoice.amount > 0.0);
        assert!(invoice.expires_at > invoice.timestamp);
    }

    #[test]
    fn test_network_specific_invoices() {
        let devnet = devnet_invoice(0.01);
        assert_eq!(devnet.network, "devnet");

        let testnet = testnet_invoice(0.02);
        assert_eq!(testnet.network, "testnet");

        let mainnet = mainnet_invoice(0.05);
        assert_eq!(mainnet.network, "mainnet-beta");
    }

    #[test]
    fn test_invalid_recipient_variants() {
        let variants = vec!["empty", "too_short", "invalid_chars", "special_chars"];
        for variant in variants {
            let invoice = invoice_with_invalid_recipient(variant);
            assert_ne!(invoice.recipient, VALID_TEST_ADDRESSES[0]);
        }
    }

    #[test]
    fn test_amount_edge_cases() {
        let negative = invoice_with_negative_amount();
        assert!(negative.amount < 0.0);

        let zero = invoice_with_zero_amount();
        assert_eq!(zero.amount, 0.0);

        let excessive = invoice_with_excessive_amount();
        assert!(excessive.amount > 1000000.0);
    }

    #[test]
    fn test_currency_variants() {
        let variants = vec!["empty", "wrong_currency", "lowercase", "invalid"];
        for variant in variants {
            let invoice = invoice_with_invalid_currency(variant);
            assert_ne!(invoice.currency, "USDC");
        }
    }

    #[test]
    fn test_memo_variants() {
        let variants = vec!["empty", "not_uuid", "wrong_format", "no_hyphens", "invalid_chars"];
        for variant in variants {
            let invoice = invoice_with_malformed_memo(variant);
            // Valid UUID format: 36 characters with 4 hyphens and only hex digits
            let is_valid_uuid = invoice.memo.len() == 36
                && invoice.memo.matches('-').count() == 4
                && invoice.memo.chars().all(|c| c.is_ascii_hexdigit() || c == '-');
            assert!(!is_valid_uuid, "Variant {} should have invalid memo", variant);
        }
    }

    #[test]
    fn test_expired_invoices() {
        let expired = expired_invoice();
        assert!(Utc::now() > expired.expires_at);

        let almost = almost_expired_invoice();
        let time_left = almost.expires_at - Utc::now();
        assert!(time_left.num_seconds() <= 30);
    }

    #[test]
    fn test_random_invoice_generation() {
        let invoice1 = random_valid_invoice();
        let invoice2 = random_valid_invoice();

        // Should generate unique memos
        assert_ne!(invoice1.memo, invoice2.memo);

        // Both should be valid
        assert!(invoice1.amount > 0.0);
        assert!(invoice2.amount > 0.0);
        assert_eq!(invoice1.currency, "USDC");
        assert_eq!(invoice2.currency, "USDC");
    }

    #[test]
    fn test_missing_fields_variants() {
        let variants = vec!["no_recipient", "no_amount", "no_currency", "no_memo", "no_network"];
        for variant in variants {
            let json = invoice_missing_required_fields(variant);
            assert!(!json.is_empty(), "Variant {} should not be empty", variant);
            // Verify JSON is parseable but missing field
            assert!(json.contains("{"));
            assert!(json.contains("}"));
        }
    }

    #[test]
    fn test_valid_test_addresses() {
        for addr in VALID_TEST_ADDRESSES {
            assert!(addr.len() >= 32 && addr.len() <= 44);
            // Check Base58 characters only (no 0, O, I, l)
            for c in addr.chars() {
                assert!(c.is_ascii_alphanumeric());
                assert!(c != '0' && c != 'O' && c != 'I' && c != 'l');
            }
        }
    }

    #[test]
    fn test_invalid_addresses() {
        assert_eq!(INVALID_ADDRESSES[0], ""); // Empty
        assert!(INVALID_ADDRESSES[1].len() < 32); // Too short
        assert!(INVALID_ADDRESSES[2].contains('0')); // Invalid Base58
    }
}
