use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use uuid::Uuid;

// ============================================================================
// Test Address Pool
// ============================================================================

/// Pool of test Solana addresses for invoice generation
///
/// IMPORTANT: These are TEST ADDRESSES ONLY - not real blockchain addresses!
/// Format: Base58-encoded, 32-44 characters (excludes 0, O, I, l)
///
/// These addresses are used for testing the x402 protocol implementation
/// and should never be used with actual blockchain transactions.
pub const TEST_ADDRESSES: &[&str] = &[
    "GXk8vTest1111111111111111111111111111qPz9", // TEST ADDRESS 1
    "HYn9xTest2222222222222222222222222222rAb3", // TEST ADDRESS 2
    "JZp4yTest3333333333333333333333333333sCd7", // TEST ADDRESS 3
    "KAq5zTest4444444444444444444444444444tDe8", // TEST ADDRESS 4
    "MBr6ATest5555555555555555555555555555uEf9", // TEST ADDRESS 5
    "NCs7BTest6666666666666666666666666666vFg1", // TEST ADDRESS 6
    "PDt8CTest7777777777777777777777777777wGh2", // TEST ADDRESS 7
    "QEu9DTest8888888888888888888888888888xHi3", // TEST ADDRESS 8
    "RFv1ETest9999999999999999999999999999yJk4", // TEST ADDRESS 9
    "SGw2FTestAAAAAAAAAAAAAAAAAAAAAAAAAAAAzKm5", // TEST ADDRESS 10
    "THx3GTestBBBBBBBBBBBBBBBBBBBBBBBBBBB1Mn6",  // TEST ADDRESS 11
    "UJy4HTestCCCCCCCCCCCCCCCCCCCCCCCCCCC2Np7",  // TEST ADDRESS 12
    "VKz5JTestDDDDDDDDDDDDDDDDDDDDDDDDDDD3Pq8",  // TEST ADDRESS 13
    "WMa6KTestEEEEEEEEEEEEEEEEEEEEEEEEEEE4Qr9",  // TEST ADDRESS 14
    "XNb7MTestFFFFFFFFFFFFFFFFFFFFFFFFFFFF5Rs1", // TEST ADDRESS 15
    "YPc8NTestGGGGGGGGGGGGGGGGGGGGGGGGGGG6St2",  // TEST ADDRESS 16
    "ZQd9PTestHHHHHHHHHHHHHHHHHHHHHHHHHHH7Tu3",  // TEST ADDRESS 17
    "ARe1QTestJJJJJJJJJJJJJJJJJJJJJJJJJJJ8Uv4",  // TEST ADDRESS 18
    "BSf2RTestKKKKKKKKKKKKKKKKKKKKKKKKKKK9Vw5",  // TEST ADDRESS 19
    "CTg3STestMMMMMMMMMMMMMMMMMMMMMMMMMMM1Wx6",  // TEST ADDRESS 20
];

// ============================================================================
// Invoice Structure
// ============================================================================

/// x402-compliant invoice for payment requests
///
/// This struct represents a payment request in the x402-solana protocol.
/// It includes all required fields for invoice generation and formatting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    /// Solana recipient address (Base58-encoded test address)
    pub recipient: String,

    /// Payment amount in USDC
    pub amount: f64,

    /// Currency (always "USDC" for x402-solana)
    pub currency: String,

    /// Unique memo/request ID for tracking (UUID-based)
    pub memo: String,

    /// Network identifier (always "devnet" for testing)
    pub network: String,

    /// Invoice creation timestamp (ISO8601)
    pub timestamp: DateTime<Utc>,

    /// Request path that triggered this invoice
    pub resource_path: String,

    /// Invoice expiration timestamp (ISO8601)
    pub expires_at: DateTime<Utc>,
}

impl Invoice {
    /// Create a new invoice with the specified amount and resource path
    ///
    /// # Arguments
    ///
    /// * `amount` - Payment amount in USDC
    /// * `resource_path` - Request path that triggered the payment requirement
    /// * `recipient` - Solana address for payment recipient
    ///
    /// # Returns
    ///
    /// A new Invoice instance with:
    /// - Unique UUID-based memo
    /// - Current timestamp
    /// - 5-minute expiration
    /// - USDC currency
    /// - devnet network
    ///
    /// Library API for programmatic invoice creation
    #[allow(dead_code)]
    pub fn new(amount: f64, resource_path: &str, recipient: String) -> Self {
        let now = Utc::now();
        let expires_at = now + Duration::minutes(5);

        Self {
            recipient,
            amount,
            currency: "USDC".to_string(),
            memo: format!("req-{}", Uuid::new_v4()),
            network: "devnet".to_string(),
            timestamp: now,
            resource_path: resource_path.to_string(),
            expires_at,
        }
    }

    /// Format invoice as WWW-Authenticate header per x402 protocol
    ///
    /// Returns space-separated key-value pairs format:
    /// `x402-solana recipient=<addr> amount=<val> currency=USDC memo=<id> network=devnet`
    ///
    /// CRITICAL: This is space-separated format (NOT base64-encoded JSON)
    /// Reference: PRD lines 83-86, x402 protocol specification
    ///
    /// Library API for programmatic invoice formatting
    #[allow(dead_code)]
    pub fn format_www_authenticate(&self) -> String {
        format!(
            "x402-solana recipient={} amount={} currency={} memo={} network={}",
            self.recipient, self.amount, self.currency, self.memo, self.network
        )
    }

    /// Check if invoice has expired
    ///
    /// Reserved for Epic 2: Payment verification will need to check invoice expiration
    /// before accepting payments (Story 2.1: Solana RPC Integration)
    #[allow(dead_code)]
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Get remaining time until expiration
    ///
    /// Reserved for Epic 2: Will be used in payment verification to display
    /// remaining time for users and in timeout handling (Story 2.3)
    #[allow(dead_code)]
    pub fn time_until_expiration(&self) -> Duration {
        self.expires_at - Utc::now()
    }
}

// ============================================================================
// Invoice Generator
// ============================================================================

/// Invoice generator with rotating test address pool
///
/// This generator creates x402-compliant invoices with rotating test addresses
/// from the TEST_ADDRESSES pool. Each invoice gets a unique memo and rotates
/// through available test addresses.
///
/// Library API for programmatic invoice generation
#[allow(dead_code)]
pub struct InvoiceGenerator {
    /// Current index in test address pool (atomic for thread-safety)
    #[allow(dead_code)]
    address_index: AtomicUsize,
}

impl InvoiceGenerator {
    /// Create a new invoice generator
    ///
    /// Library API for programmatic invoice generation
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            address_index: AtomicUsize::new(0),
        }
    }

    /// Generate a new invoice with specified amount and resource path
    ///
    /// # Arguments
    ///
    /// * `amount` - Payment amount in USDC
    /// * `resource_path` - Request path that triggered payment requirement
    ///
    /// # Returns
    ///
    /// A new Invoice with:
    /// - Rotated test address from pool
    /// - Unique UUID-based memo
    /// - Current timestamp
    /// - 5-minute expiration
    ///
    /// Library API for programmatic invoice generation
    #[allow(dead_code)]
    pub fn generate(&self, amount: f64, resource_path: &str) -> Invoice {
        // Get next address from pool (round-robin)
        let idx = self.address_index.fetch_add(1, Ordering::SeqCst);
        let recipient = TEST_ADDRESSES[idx % TEST_ADDRESSES.len()].to_string();

        Invoice::new(amount, resource_path, recipient)
    }

    /// Get a specific test address by index
    ///
    /// Reserved for Epic 2: Will be used in payment verification tests
    /// to generate predictable test addresses for validation scenarios
    #[allow(dead_code)]
    pub fn get_test_address(index: usize) -> &'static str {
        TEST_ADDRESSES[index % TEST_ADDRESSES.len()]
    }

    /// Get total number of test addresses
    ///
    /// Reserved for Epic 2: Used in test infrastructure to validate
    /// address pool rotation and ensure adequate test coverage
    #[cfg(test)]
    pub fn test_address_count() -> usize {
        TEST_ADDRESSES.len()
    }
}

impl Default for InvoiceGenerator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_invoice_creation() {
        let recipient = TEST_ADDRESSES[0].to_string();
        let invoice = Invoice::new(0.05, "/api/data", recipient.clone());

        // Verify all required fields
        assert_eq!(invoice.recipient, recipient);
        assert_eq!(invoice.amount, 0.05);
        assert_eq!(invoice.currency, "USDC");
        assert_eq!(invoice.network, "devnet");
        assert_eq!(invoice.resource_path, "/api/data");

        // Verify memo format
        assert!(invoice.memo.starts_with("req-"));
        assert!(invoice.memo.len() > 10); // UUID adds significant length

        // Verify timestamps
        assert!(invoice.expires_at > invoice.timestamp);
        let duration = invoice.expires_at - invoice.timestamp;
        assert!(duration.num_minutes() >= 4 && duration.num_minutes() <= 5);
    }

    #[test]
    fn test_www_authenticate_format() {
        let recipient = TEST_ADDRESSES[0].to_string();
        let invoice = Invoice::new(0.01, "/test", recipient);
        let header = invoice.format_www_authenticate();

        // Verify header starts with protocol identifier
        assert!(header.starts_with("x402-solana "));

        // Verify all required fields are present
        assert!(header.contains("recipient="));
        assert!(header.contains("amount="));
        assert!(header.contains("currency="));
        assert!(header.contains("memo="));
        assert!(header.contains("network="));

        // Verify specific values
        assert!(header.contains(&format!("recipient={}", TEST_ADDRESSES[0])));
        assert!(header.contains("amount=0.01"));
        assert!(header.contains("currency=USDC"));
        assert!(header.contains("network=devnet"));
        assert!(header.contains("memo=req-"));

        // Verify space-separated format (NOT base64)
        assert!(!header.contains("{"));
        assert!(!header.contains("}"));
        assert!(!header.contains("["));
        assert!(!header.contains("]"));
    }

    #[test]
    fn test_invoice_generator_rotation() {
        let generator = InvoiceGenerator::new();

        // Generate multiple invoices
        let invoice1 = generator.generate(0.05, "/api/endpoint1");
        let invoice2 = generator.generate(0.10, "/api/endpoint2");
        let invoice3 = generator.generate(0.15, "/api/endpoint3");

        // Verify addresses rotate
        assert_eq!(invoice1.recipient, TEST_ADDRESSES[0]);
        assert_eq!(invoice2.recipient, TEST_ADDRESSES[1]);
        assert_eq!(invoice3.recipient, TEST_ADDRESSES[2]);
    }

    #[test]
    fn test_unique_memo_generation() {
        let generator = InvoiceGenerator::new();
        let mut memos = HashSet::new();

        // Generate 100 invoices and verify memo uniqueness
        for i in 0..100 {
            let invoice = generator.generate(0.01, &format!("/test{}", i));
            assert!(
                memos.insert(invoice.memo.clone()),
                "Duplicate memo detected: {}",
                invoice.memo
            );
        }

        // All 100 memos should be unique
        assert_eq!(memos.len(), 100);
    }

    #[test]
    fn test_test_address_pool() {
        // Verify we have at least 10 test addresses
        assert!(TEST_ADDRESSES.len() >= 10);

        // Verify all addresses are Base58 format (32-44 chars)
        for addr in TEST_ADDRESSES {
            assert!(addr.len() >= 32 && addr.len() <= 44);
            // Verify Base58 characters only
            for c in addr.chars() {
                assert!(
                    c.is_ascii_alphanumeric() && c != '0' && c != 'O' && c != 'I' && c != 'l',
                    "Invalid Base58 character: {}",
                    c
                );
            }
        }

        // Verify all addresses contain "Test" marker
        for addr in TEST_ADDRESSES {
            assert!(
                addr.contains("Test"),
                "Test address missing 'Test' marker: {}",
                addr
            );
        }
    }

    #[test]
    fn test_invoice_expiration() {
        let recipient = TEST_ADDRESSES[0].to_string();
        let invoice = Invoice::new(0.05, "/test", recipient);

        // Invoice should not be expired immediately
        assert!(!invoice.is_expired());

        // Time until expiration should be approximately 5 minutes
        let time_left = invoice.time_until_expiration();
        assert!(time_left.num_minutes() >= 4 && time_left.num_minutes() <= 5);
    }

    #[test]
    fn test_www_authenticate_parsing() {
        let recipient = TEST_ADDRESSES[0].to_string();
        let invoice = Invoice::new(0.01, "/test", recipient);
        let header = invoice.format_www_authenticate();

        // Parse header into key-value pairs
        let parts: Vec<&str> = header.split_whitespace().collect();

        // First part should be protocol identifier
        assert_eq!(parts[0], "x402-solana");

        // Count required fields
        let mut fields = std::collections::HashMap::new();
        for part in &parts[1..] {
            if let Some((key, value)) = part.split_once('=') {
                fields.insert(key, value);
            }
        }

        // Verify all required fields present
        assert!(fields.contains_key("recipient"));
        assert!(fields.contains_key("amount"));
        assert!(fields.contains_key("currency"));
        assert!(fields.contains_key("memo"));
        assert!(fields.contains_key("network"));

        // Verify field values
        assert_eq!(fields.get("currency"), Some(&"USDC"));
        assert_eq!(fields.get("network"), Some(&"devnet"));
    }

    #[test]
    fn test_invoice_generator_wrap_around() {
        let generator = InvoiceGenerator::new();
        let pool_size = InvoiceGenerator::test_address_count();

        // Generate pool_size + 2 invoices to test wrap-around
        let mut invoices = Vec::new();
        for i in 0..(pool_size + 2) {
            invoices.push(generator.generate(0.01, &format!("/test{}", i)));
        }

        // Verify first and pool_size+1 invoices have same address (wrap around)
        assert_eq!(invoices[0].recipient, invoices[pool_size].recipient);
        assert_eq!(invoices[1].recipient, invoices[pool_size + 1].recipient);
    }
}
