# Story 2.3: Payment Verification Simulation

Status: pending

## Story

As a developer,
I want to simulate payment verification outcomes (success, failure, timeout),
So that I can test error handling in my client code without actual payment processing.

## Acceptance Criteria

1. **Given** client requests any resource without payment proof
   **When** request made
   **Then** server ALWAYS returns 402 Payment Required with invoice (Story 2.4)

2. **And** client submits payment with X-Payment-Proof header
   **When** simulation mode is "success"
   **Then** server returns 200 OK with requested resource

3. **And** when simulation mode is "failure"
   **Then** server returns 402 Payment Required with error message "Payment rejected"

4. **And** when simulation mode is "timeout"
   **Then** server delays response by configured duration (default: 5000ms)
   **Then** server returns 408 Request Timeout

5. **And** simulation mode is configurable globally in configuration file

6. **And** simulation mode can be overridden per-request via X-Simulation-Mode header

## Tasks / Subtasks

- [ ] Task 1: Separate initial 402 response from payment verification (AC: #1, #2)
  - [ ] Implement payment proof detection (X-Payment-Proof header)
  - [ ] Route to invoice generation if no payment proof
  - [ ] Route to payment verification if payment proof exists
  - [ ] Add clear separation in request handler logic

- [ ] Task 2: Implement payment proof parsing (AC: #2)
  - [ ] Parse X-Payment-Proof header value
  - [ ] Extract transaction_id from header (format: "X-Payment-Proof: <transaction_id>")
  - [ ] Validate header format (non-empty string)
  - [ ] Log payment proof submission attempts

- [ ] Task 3: Add simulation mode configuration (AC: #5, #6)
  - [ ] Extend MockServerConfig with simulation_mode field (success/failure/timeout)
  - [ ] Add timeout_delay_ms field for timeout simulation (default: 5000ms)
  - [ ] Update config validation to accept simulation mode values
  - [ ] Add configuration to .x402dev.yaml template

- [ ] Task 4: Implement header-based mode override (AC: #6)
  - [ ] Parse X-Simulation-Mode header from HTTP requests
  - [ ] Map header value to simulation mode enum
  - [ ] Prioritize header override over global config
  - [ ] Validate header value (success/failure/timeout)

- [ ] Task 5: Implement success verification handler (AC: #2)
  - [ ] Create verify_payment_success() function
  - [ ] Accept payment proof and return 200 OK
  - [ ] Include payment confirmation in response body
  - [ ] Log successful payment verification

- [ ] Task 6: Implement failure verification handler (AC: #3)
  - [ ] Create verify_payment_failure() function
  - [ ] Return 402 Payment Required with rejection reason
  - [ ] Include error message: "Payment rejected"
  - [ ] Log payment rejection with reason

- [ ] Task 7: Implement timeout verification handler (AC: #4)
  - [ ] Create verify_payment_timeout() function
  - [ ] Use tokio::time::sleep() for delay
  - [ ] Read timeout_delay_ms from config
  - [ ] Return 408 Request Timeout after delay
  - [ ] Log timeout simulation with delay duration

- [ ] Task 8: Integration testing (AC: #1-6)
  - [ ] Test flow: Request without payment → 402 with invoice
  - [ ] Test flow: Request with payment (success mode) → 200 OK
  - [ ] Test flow: Request with payment (failure mode) → 402 rejected
  - [ ] Test flow: Request with payment (timeout mode) → 408 timeout
  - [ ] Test header override: X-Simulation-Mode works
  - [ ] Test global config mode precedence

## Dev Notes

### Architecture Constraints

- **Pure Rust Implementation** (ADR-001): Use tokio::time::sleep for timeout simulation
- **Configuration Integration**: Extend MockServerConfig from Story 1.4
- **Async/Await**: All handlers must be async for tokio compatibility
- **Error Handling**: Use anyhow::Result with context messages
- **Two-Phase Flow**: Separate initial 402 response from payment verification

### Project Structure

```
crates/x402-mock/src/
├── config.rs         # Extend with SimulationMode enum
├── server/
│   ├── mod.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── payment_proof.rs     # NEW: Payment proof parsing
│   │   ├── verify_success.rs    # NEW: Success verification
│   │   ├── verify_failure.rs    # NEW: Failure verification
│   │   └── verify_timeout.rs    # NEW: Timeout verification
│   └── routes.rs     # Update with two-phase routing
```

### Key Implementation Details

**x402 Two-Phase Flow:**
```
PHASE 1: Initial Request (No Payment Proof)
  Client → GET /resource
  Server → 402 Payment Required + Invoice (Story 2.4)

PHASE 2: Payment Verification (Has Payment Proof)
  Client → GET /resource + X-Payment-Proof: <tx_id>
  Server → Simulate verification:
    - Success mode → 200 OK + resource
    - Failure mode → 402 Payment Required + error
    - Timeout mode → (delay) → 408 Request Timeout
```

**Configuration Structure:**
```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SimulationMode {
    Success,
    Failure,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockServerConfig {
    pub port: u16,
    pub simulation_mode: SimulationMode,
    pub timeout_delay_ms: u64,
}
```

**Payment Proof Detection:**
```rust
fn has_payment_proof(headers: &HeaderMap) -> bool {
    headers
        .get("X-Payment-Proof")
        .and_then(|v| v.to_str().ok())
        .map(|s| !s.is_empty())
        .unwrap_or(false)
}

fn extract_payment_proof(headers: &HeaderMap) -> Option<String> {
    headers
        .get("X-Payment-Proof")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}
```

**Header Override:**
```rust
fn get_simulation_mode(
    headers: &HeaderMap,
    config: &MockServerConfig,
) -> SimulationMode {
    headers
        .get("X-Simulation-Mode")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| match s.to_lowercase().as_str() {
            "success" => Some(SimulationMode::Success),
            "failure" => Some(SimulationMode::Failure),
            "timeout" => Some(SimulationMode::Timeout),
            _ => None,
        })
        .unwrap_or(config.simulation_mode)
}
```

**Success Verification Handler:**
```rust
pub async fn verify_payment_success(
    payment_proof: String,
    resource_path: String,
) -> Result<impl IntoResponse> {
    info!("Payment verification SUCCESS for proof: {}", payment_proof);

    let response = json!({
        "status": "success",
        "payment_proof": payment_proof,
        "message": "Payment accepted",
        "resource": format!("Content for {}", resource_path)
    });

    Ok((StatusCode::OK, Json(response)))
}
```

**Failure Verification Handler:**
```rust
pub async fn verify_payment_failure(
    payment_proof: String,
) -> Result<impl IntoResponse> {
    warn!("Payment verification FAILURE for proof: {}", payment_proof);

    let response = json!({
        "status": "failure",
        "payment_proof": payment_proof,
        "error": "Payment rejected",
        "message": "Payment verification failed - invalid or expired proof"
    });

    Ok((StatusCode::PAYMENT_REQUIRED, Json(response)))
}
```

**Timeout Verification Handler:**
```rust
pub async fn verify_payment_timeout(
    payment_proof: String,
    delay_ms: u64,
) -> Result<impl IntoResponse> {
    info!(
        "Payment verification TIMEOUT for proof {} with delay {}ms",
        payment_proof, delay_ms
    );

    tokio::time::sleep(Duration::from_millis(delay_ms)).await;

    let response = json!({
        "status": "timeout",
        "payment_proof": payment_proof,
        "error": "Request timeout",
        "message": "Payment verification timed out"
    });

    Ok((StatusCode::REQUEST_TIMEOUT, Json(response)))
}
```

**Two-Phase Routing Integration:**
```rust
pub async fn handle_resource_request(
    headers: HeaderMap,
    path: String,
    config: MockServerConfig,
) -> Result<impl IntoResponse> {
    // PHASE 1: Check for payment proof
    if !has_payment_proof(&headers) {
        // No payment proof → Return 402 with invoice (Story 2.4)
        return generate_invoice_response(path, config).await;
    }

    // PHASE 2: Verify payment proof
    let payment_proof = extract_payment_proof(&headers)
        .ok_or_else(|| anyhow!("Invalid payment proof"))?;

    let mode = get_simulation_mode(&headers, &config);

    match mode {
        SimulationMode::Success => {
            verify_payment_success(payment_proof, path).await
        }
        SimulationMode::Failure => {
            verify_payment_failure(payment_proof).await
        }
        SimulationMode::Timeout => {
            verify_payment_timeout(payment_proof, config.timeout_delay_ms).await
        }
    }
}
```

### Dependencies Added

- No new dependencies required (tokio::time already available)

### Testing Standards

**Unit Tests:**
- Test payment proof detection (has/missing X-Payment-Proof header)
- Test payment proof extraction
- Test each verification handler in isolation (success, failure, timeout)
- Verify correct status codes (200, 402, 408)
- Verify response body structure
- Test timeout delay with tokio::time::advance

**Integration Tests:**
- **Two-Phase Flow Tests:**
  - Test: Request without payment proof → 402 with invoice
  - Test: Request with payment proof (success) → 200 OK
  - Test: Request with payment proof (failure) → 402 rejected
  - Test: Request with payment proof (timeout) → 408 timeout
- **Configuration Tests:**
  - Test: Header override (X-Simulation-Mode: failure)
  - Test: Global config mode
  - Test: Precedence (header > config)
- **Invoice Integration (Story 2.4):**
  - Test: Initial 402 response includes valid invoice
  - Test: Payment proof references invoice ID

**Test Coverage:**
- Target: >90% coverage for payment verification handlers
- Include edge cases: invalid header values, missing headers, malformed payment proofs

### Learnings from Previous Stories

**From Story 1.4 (Configuration Management - Status: done)**
- Reuse Config validation pattern for simulation mode
- Extend existing configuration system
- Use serde for YAML serialization/deserialization

**From Story 2.1 (HTTP Server - Status: done)**
- Reuse HTTP server infrastructure from Story 2.1
- Integrate with existing request handling
- Use axum for routing and response handling

**From Story 2.4 (Invoice Generation - Status: done)**
- **CRITICAL DEPENDENCY**: This story requires Story 2.4 to be completed first
- Integration with invoice generation for Phase 1 (initial 402 response)
- Payment proof should reference invoice ID for verification
- Coordinate payment simulation with invoice workflow

### Protocol Compliance

**x402 Protocol Requirements:**
1. ✅ Server ALWAYS returns 402 first (no payment proof path)
2. ✅ Client submits payment proof via X-Payment-Proof header
3. ✅ Server simulates payment verification (Phase 2)
4. ✅ Verification can succeed (200), fail (402), or timeout (408)

**Payment Proof Format (Placeholder):**
- Header: `X-Payment-Proof: <transaction_id>`
- No actual blockchain verification in mock server
- Transaction ID is opaque string for simulation purposes

### References

- [Story 2.1: HTTP Server](./2-1-http-server-402-responses.md)
- [Story 2.4: Invoice Generation](./2-4-invoice-generation.md) ← **DEPENDENCY**
- [tokio::time Documentation](https://docs.rs/tokio/latest/tokio/time/)
- [axum Documentation](https://docs.rs/axum/latest/axum/)

## Dev Agent Record

### Prerequisites

**REQUIRED (Must be completed first):**
- Story 2.1: HTTP Server (Status: done)
- Story 2.4: Invoice Generation (Status: done) ← **MUST BE DONE FIRST**

**RELATED:**
- Story 1.4: Configuration Management (Status: done)

### Context Reference

- Implementation extends MockServerConfig from Story 1.4
- Integrates with HTTP server infrastructure from Story 2.1
- **DEPENDS ON** invoice generation from Story 2.4 for Phase 1 responses
- Uses tokio::time for async timeout simulation
- Implements x402 two-phase payment flow

### Agent Model Used

(To be filled during implementation)

### Debug Log References

(To be filled during implementation)

### Completion Notes List

(To be filled during implementation)

### File List

**New Files:**
- crates/x402-mock/src/server/handlers/payment_proof.rs (payment proof parsing)
- crates/x402-mock/src/server/handlers/verify_success.rs (success verification)
- crates/x402-mock/src/server/handlers/verify_failure.rs (failure verification)
- crates/x402-mock/src/server/handlers/verify_timeout.rs (timeout verification)
- crates/x402-mock/tests/payment_verification.rs (integration tests)

**Modified Files:**
- crates/x402-mock/src/config.rs (add SimulationMode enum, extend MockServerConfig)
- crates/x402-mock/src/server/handlers/mod.rs (export new handlers)
- crates/x402-mock/src/server/routes.rs (add two-phase routing)
- .x402dev.yaml (add simulation_mode and timeout_delay_ms fields)

## Change Log

(To be filled during implementation)

---

## Senior Developer Review (AI)

(To be filled after implementation)
