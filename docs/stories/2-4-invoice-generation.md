# Story 2.4: Invoice Generation

Status: pending

## Story

As a developer,
I want valid x402-compliant invoices generated automatically,
So that I can test invoice parsing and validation.

## Acceptance Criteria

1. **Given** a request to the mock server
   **When** the server generates an invoice
   **Then** invoice includes Solana-format test address (Base58, 32-44 chars)

2. **And** invoice includes version, recipient, amount, currency, memo, network fields

3. **And** invoice follows x402 protocol specification

4. **And** unique memo is generated per request (for tracking)

5. **And** generated invoices pass `x402-dev verify invoice` validation

6. **And** test addresses are clearly marked (not real blockchain addresses)

## Tasks / Subtasks

- [ ] Task 1: Add invoice generation dependencies (AC: #1, #4)
  - [ ] Add `uuid = "1.10"` to workspace dependencies in Cargo.toml
  - [ ] Add `chrono = "0.4"` to workspace dependencies in Cargo.toml
  - [ ] Add both dependencies to crates/x402-mock/Cargo.toml

- [ ] Task 2: Create test Solana address pool (AC: #1, #6)
  - [ ] Create `crates/x402-mock/src/addresses.rs`
  - [ ] Define pool of 10-20 Base58-encoded test addresses (32-44 chars)
  - [ ] Add TEST_ADDRESSES constant with clear documentation
  - [ ] Implement address rotation function (round-robin or random)
  - [ ] Add validation function to ensure addresses match Base58 format

- [ ] Task 3: Define x402 invoice structure (AC: #2, #3)
  - [ ] Create `crates/x402-mock/src/invoice.rs`
  - [ ] Define Invoice struct with fields: version, recipient, amount, currency, memo, network, expires_at
  - [ ] Implement Serialize for Invoice (serde_json)
  - [ ] Add Invoice::new() constructor with validation
  - [ ] Add Invoice::format_www_authenticate() method for space-separated header format

- [ ] Task 4: Implement invoice generation logic (AC: #1-4)
  - [ ] Create InvoiceGenerator struct in invoice.rs
  - [ ] Implement generate() method that:
    - [ ] Selects address from TEST_ADDRESSES pool
    - [ ] Generates unique memo using uuid (format: "req-{uuid}")
    - [ ] Sets amount from pricing rules (Story 2.2)
    - [ ] Sets currency to "USDC"
    - [ ] Sets version to "1.0"
    - [ ] Sets network to "devnet"
    - [ ] Sets expires_at to current time + 5 minutes (ISO8601 format)

- [ ] Task 5: Implement WWW-Authenticate header formatting (AC: #3)
  - [ ] Add format_www_authenticate() method to Invoice
  - [ ] Format header as space-separated key-value pairs
  - [ ] Header format: `WWW-Authenticate: x402-solana recipient=<addr> amount=<val> currency=USDC memo=<id> network=devnet`
  - [ ] Add tests for header format validation

- [ ] Task 6: Integrate invoice generation with HTTP server (AC: all)
  - [ ] Update request handler in server.rs to use InvoiceGenerator
  - [ ] Generate invoice for each 402 response
  - [ ] Add invoice to response headers (WWW-Authenticate)
  - [ ] Log invoice details (recipient, amount, memo) for debugging
  - [ ] Ensure invoice generation errors return 500 Internal Server Error

- [ ] Task 7: Test invoice generation (AC: #5, #6)
  - [ ] Create tests/invoice_generation_test.rs
  - [ ] Test: Invoice contains required fields (recipient, amount, memo, network, etc.)
  - [ ] Test: Recipient address is Base58 format (32-44 chars)
  - [ ] Test: Memo is unique per request (generate 100, check uniqueness)
  - [ ] Test: expires_at is in future (5 minutes)
  - [ ] Test: WWW-Authenticate header format matches x402 spec (space-separated format)
  - [ ] Test: Invoice passes x402-dev verify invoice validation (integration test)
  - [ ] Test: Test addresses are clearly marked (check for "TEST" in address pool documentation)
  - [ ] Test: Network field is set to "devnet"

## Dev Notes

### Architecture Constraints

- **Pure Rust Implementation** (ADR-001): Use rust-base58 or bs58 for Base58 encoding
- **x402 Protocol Compliance**: Follow x402 invoice specification exactly
- **Test-Only Addresses**: Use fixed pool of test addresses, never generate real blockchain addresses
- **UUID-Based Tracking**: Each invoice gets unique memo for request tracking
- **Time Format**: ISO8601 timestamps (chrono::DateTime<Utc>)
- **Error Handling**: Invoice generation errors should not crash server

### Project Structure

```
crates/x402-mock/src/
├── server.rs         # HTTP server (Story 2.1)
├── pricing.rs        # Pricing rules (Story 2.2)
├── invoice.rs        # NEW: Invoice generation logic
├── addresses.rs      # NEW: Test Solana address pool
└── lib.rs            # Module exports
```

### Key Implementation Details

**Test Address Pool:**
```rust
// addresses.rs
pub const TEST_ADDRESSES: &[&str] = &[
    "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr", // TEST ADDRESS 1
    "2wmVCSfPxGPjrnMMn7rchp4uaeoTqN39mXFC2zhPdri9", // TEST ADDRESS 2
    "FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH", // TEST ADDRESS 3
    // ... 7-17 more test addresses
];

pub fn get_test_address(index: usize) -> &'static str {
    TEST_ADDRESSES[index % TEST_ADDRESSES.len()]
}
```

**Invoice Structure:**
```rust
// invoice.rs
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub struct Invoice {
    pub version: String,
    pub recipient: String,
    pub amount: f64,
    pub currency: String,
    pub memo: String,
    pub network: String,
    pub expires_at: String, // ISO8601
}

impl Invoice {
    pub fn new(recipient: String, amount: f64) -> Self {
        let memo = format!("req-{}", Uuid::new_v4());
        let expires_at = (Utc::now() + chrono::Duration::minutes(5))
            .to_rfc3339();

        Self {
            version: "1.0".to_string(),
            recipient,
            amount,
            currency: "USDC".to_string(),
            memo,
            network: "devnet".to_string(),
            expires_at,
        }
    }

    pub fn format_www_authenticate(&self) -> String {
        format!(
            "x402-solana recipient={} amount={} currency={} memo={} network={}",
            self.recipient, self.amount, self.currency, self.memo, self.network
        )
    }
}
```

**Invoice Generator:**
```rust
pub struct InvoiceGenerator {
    address_index: std::sync::atomic::AtomicUsize,
}

impl InvoiceGenerator {
    pub fn new() -> Self {
        Self {
            address_index: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    pub fn generate(&self, amount: f64) -> Invoice {
        let idx = self.address_index.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let recipient = addresses::get_test_address(idx).to_string();
        Invoice::new(recipient, amount)
    }
}
```

**Integration with HTTP Server:**
```rust
// server.rs (modified)
use crate::invoice::InvoiceGenerator;

async fn handle_request(
    req: Request<Body>,
    generator: Arc<InvoiceGenerator>,
) -> Result<Response<Body>, hyper::Error> {
    // ... check pricing rules ...

    let amount = pricing::get_price(&path); // From Story 2.2
    let invoice = generator.generate(amount);
    let header_value = invoice.format_www_authenticate();

    Response::builder()
        .status(StatusCode::PAYMENT_REQUIRED)
        .header("WWW-Authenticate", header_value)
        .body(Body::from("Payment required"))
        .unwrap()
}
```

### Dependencies Added

- `uuid = "1.10"` - Unique memo generation
- `chrono = "0.4"` - ISO8601 timestamp formatting

### Testing Standards

- **Unit Tests**: Invoice struct, address pool, memo uniqueness
- **Header Format Tests**: WWW-Authenticate space-separated format validation
- **Integration Tests**: Full HTTP server with invoice generation
- **Validation Tests**: Run `x402-dev verify invoice` on generated invoices (requires verify command from Story 2.5)
- **Uniqueness Tests**: Generate 100 invoices, verify all memos unique
- **Address Format Tests**: Validate Base58 format (32-44 chars)
- **Network Tests**: Verify network field is set to "devnet"

### Learnings from Previous Stories

**From Story 2.1 (HTTP Server - Status: pending)**

- HTTP server structure available for integration
- Request handler can be extended with invoice generation
- Response headers can include WWW-Authenticate

**From Story 2.2 (Pricing Rules - Status: pending)**

- Pricing logic provides amount for invoice
- Amount from pricing::get_price() used in invoice generation

**From Story 1.5 (Error Handling - Status: done)**

- Use anyhow::Result for invoice generation errors
- Context messages for debugging
- Error handling patterns available

**Key Interfaces to Use:**
- `pricing::get_price(path: &str) -> f64` - Get amount for invoice
- `Invoice::new(recipient, amount)` - Create invoice
- `Invoice::format_www_authenticate()` - Generate space-separated header format

### References

- [Source: docs/epics.md#Story-2.4] - Story requirements
- [x402 Protocol Specification](https://github.com/x402-protocol/spec) - Invoice format
- [Solana Address Format](https://docs.solana.com/terminology#account) - Base58 encoding
- [RFC 7235 - WWW-Authenticate Header](https://tools.ietf.org/html/rfc7235#section-4.1)
- [uuid Documentation](https://docs.rs/uuid/1.10/uuid/)
- [chrono Documentation](https://docs.rs/chrono/0.4/chrono/)

## Dev Agent Record

### Context Reference

- Implements x402 protocol invoice generation
- Uses test address pool (not real blockchain addresses)
- UUID-based memo for request tracking
- ISO8601 timestamps via chrono
- Space-separated key-value format in WWW-Authenticate header (NOT base64-encoded JSON)
- Network field set to "devnet" for all invoices

### Agent Model Used

(To be filled after implementation)

### Debug Log References

(To be filled after implementation)

### Completion Notes List

(To be filled after implementation)

### File List

**New Files:**
(To be filled after implementation)

**Modified Files:**
(To be filled after implementation)

## Change Log

(To be filled after implementation)

---

## Senior Developer Review (AI)

(To be filled after code review)
