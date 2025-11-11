# Story 2.5: Zero Blockchain Dependency

Status: pending

## Story

As a developer,
I want the mock server to work completely offline,
So that I can test without Solana RPC access or network connectivity.

## Acceptance Criteria

1. **Given** the mock server is running
   **When** I disconnect from internet
   **Then** server continues functioning normally

2. **And** no Solana RPC calls are made

3. **And** payment verification is simulated without on-chain checks

4. **And** server works completely offline with zero network calls

5. **And** no real blockchain transactions are required

## Tasks / Subtasks

- [ ] Task 1: Remove solana-client dependency (AC: #2, #4)
  - [ ] Audit crates/x402-mock-server/Cargo.toml for solana-client imports
  - [ ] Remove any solana-client references from mock server crate
  - [ ] Verify mock server builds without blockchain libraries
  - [ ] Document "Mock server uses test addresses only" in README

- [ ] Task 2: Implement offline payment verification (AC: #3, #5)
  - [ ] Create payment verification module without RPC calls
  - [ ] Accept payment header from request
  - [ ] Validate payment header format (base58 or hex string)
  - [ ] Return 200 OK for valid format (no blockchain check)
  - [ ] Return 400 Bad Request for invalid format

- [ ] Task 3: In-memory state tracking (AC: #1, #4)
  - [ ] Create in-memory payment state store (HashMap or similar)
  - [ ] Track request count per payment ID
  - [ ] Track payment status (unused/active/expired)
  - [ ] No external storage or network calls

- [ ] Task 4: Offline integration testing (AC: #1, #2, #4)
  - [ ] Write test that disables network access (if possible)
  - [ ] Start mock server, make requests, verify responses
  - [ ] Verify no network calls via system monitoring
  - [ ] Test payment verification without internet
  - [ ] Verify all endpoints work offline

- [ ] Task 5: Documentation and verification (AC: #2, #4, #5)
  - [ ] Update README: "Mock server operates completely offline"
  - [ ] Document test addresses usage (no real blockchain)
  - [ ] Add warning: "Not suitable for production use"
  - [ ] Verify zero dependencies on solana-client
  - [ ] Run `cargo tree` to confirm no hidden blockchain deps

## Dev Notes

### Architecture Constraints

- **Pure Rust Implementation** (ADR-001): No JavaScript/npm dependencies
- **Zero Network Calls**: Mock server must function completely offline
- **NO Blockchain Libraries**: Do not import solana-client crate
- **In-Memory State**: Use HashMap or similar for payment tracking
- **Format Validation Only**: Accept payment header, validate format, no on-chain checks

### Project Structure

```
crates/x402-mock-server/src/
├── main.rs              # Server entry point
├── payment/
│   ├── mod.rs          # Payment module
│   ├── verification.rs # NEW: Offline payment verification
│   └── state.rs        # NEW: In-memory payment state
└── routes/
    └── checkout.rs     # Payment endpoint
```

### Key Implementation Details

**Offline Payment Verification:**
```rust
// NO RPC calls, format validation only
pub fn verify_payment_offline(payment_id: &str) -> Result<bool, PaymentError> {
    // Validate format (base58 or hex)
    if is_valid_format(payment_id) {
        Ok(true) // Simulated success
    } else {
        Err(PaymentError::InvalidFormat)
    }
}

fn is_valid_format(payment_id: &str) -> bool {
    // Check if base58 or hex string (32-88 chars typical)
    payment_id.len() >= 32
        && payment_id.len() <= 88
        && payment_id.chars().all(|c| c.is_alphanumeric())
}
```

**In-Memory State:**
```rust
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct PaymentState {
    pub request_count: u32,
    pub status: PaymentStatus,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub enum PaymentStatus {
    Unused,
    Active,
    Expired,
}

pub struct PaymentStore {
    payments: Mutex<HashMap<String, PaymentState>>,
}

impl PaymentStore {
    pub fn new() -> Self {
        Self {
            payments: Mutex::new(HashMap::new()),
        }
    }

    pub fn track_payment(&self, payment_id: String) {
        let mut store = self.payments.lock().unwrap();
        store.entry(payment_id).or_insert(PaymentState {
            request_count: 0,
            status: PaymentStatus::Unused,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }).request_count += 1;
    }
}
```

**Checkout Route (No RPC):**
```rust
#[post("/checkout")]
async fn checkout(
    payment_store: web::Data<PaymentStore>,
    req: HttpRequest,
) -> impl Responder {
    // Extract payment header
    let payment_id = match req.headers().get("X-Payment-ID") {
        Some(value) => value.to_str().unwrap_or(""),
        None => return HttpResponse::BadRequest().body("Missing payment header"),
    };

    // Offline verification (format only)
    match verify_payment_offline(payment_id) {
        Ok(_) => {
            payment_store.track_payment(payment_id.to_string());
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "payment_id": payment_id,
                "message": "Payment accepted (mock)"
            }))
        }
        Err(_) => HttpResponse::BadRequest().body("Invalid payment format"),
    }
}
```

**NO Blockchain Dependencies:**
```toml
# crates/x402-mock-server/Cargo.toml

[dependencies]
actix-web = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
tokio = { workspace = true }
# DO NOT ADD: solana-client, solana-sdk, or any blockchain crates
```

### Dependencies

**Existing (no new dependencies needed):**
- actix-web - HTTP server
- serde/serde_json - JSON serialization
- tokio - Async runtime

**DO NOT ADD:**
- ❌ solana-client
- ❌ solana-sdk
- ❌ solana-program
- ❌ Any blockchain libraries

### Testing Standards

**Offline Testing:**
```rust
#[tokio::test]
async fn test_payment_verification_offline() {
    // Valid format (base58-like string)
    assert!(verify_payment_offline("4sGjMW1sUnHzSxGspuhpqLDx6wiyjNtZ").is_ok());

    // Invalid format (too short)
    assert!(verify_payment_offline("abc").is_err());

    // Invalid format (special characters)
    assert!(verify_payment_offline("abc@#$%").is_err());
}

#[tokio::test]
async fn test_server_runs_offline() {
    let payment_store = web::Data::new(PaymentStore::new());
    let app = test::init_service(
        App::new()
            .app_data(payment_store.clone())
            .service(checkout)
    ).await;

    let req = test::TestRequest::post()
        .uri("/checkout")
        .insert_header(("X-Payment-ID", "4sGjMW1sUnHzSxGspuhpqLDx6wiyjNtZ"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    // Verify no network calls made (manual verification in test logs)
}
```

**Dependency Verification:**
```bash
# Verify no blockchain dependencies
cargo tree -p x402-mock-server | grep -i solana
# Should return empty

# Verify builds without network
cargo build --offline -p x402-mock-server
# Should succeed (after initial deps cached)
```

### Learnings from Previous Stories

**From Story 2.3 (Payment Simulation):**
- Payment header format: `X-Payment-ID: <transaction_signature>`
- Mock server accepts payment header and validates format
- No real blockchain interaction

**From Story 2.4 (Invoice Generation):**
- Invoice contains test wallet addresses
- Clear documentation: "Test addresses only"
- No real on-chain transactions

**Key Integration Points:**
- Payment verification uses format validation only
- Invoice generation provides test addresses
- All operations are in-memory, no external calls

### References

- [Source: docs/epics.md#Story-2.5] - Story requirements
- [ADR-001: Pure Rust Implementation](docs/architecture.md)
- [Story 2.3: Payment Simulation](docs/stories/2-3-payment-simulation.md)
- [Story 2.4: Invoice Generation](docs/stories/2-4-invoice-generation.md)

## Dev Agent Record

### Context Reference

- Mock server must operate completely offline
- No solana-client or blockchain libraries allowed
- Payment verification is format validation only (no RPC)
- In-memory state tracking with HashMap

### Agent Model Used

_To be filled during implementation_

### Debug Log References

_To be filled during implementation_

### Completion Notes List

_To be filled during implementation_

### File List

**New Files:**
- crates/x402-mock-server/src/payment/verification.rs
- crates/x402-mock-server/src/payment/state.rs
- crates/x402-mock-server/tests/offline_tests.rs

**Modified Files:**
- crates/x402-mock-server/Cargo.toml (verify no blockchain deps)
- crates/x402-mock-server/src/payment/mod.rs (add verification, state modules)
- crates/x402-mock-server/src/routes/checkout.rs (integrate offline verification)
- README.md (document offline operation)

## Change Log

**2025-11-11** - Story 2.5 created
- Initial story definition for zero blockchain dependency
- Prerequisites: Story 2.3 (Payment simulation), Story 2.4 (Invoice generation)
- Status: pending

---

## Senior Developer Review (AI)

_To be filled after implementation_
