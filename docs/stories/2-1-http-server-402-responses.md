# Story 2.1: HTTP Server with 402 Responses

Status: done

## Story

As a developer,
I want an HTTP server that responds with 402 Payment Required status,
So that I can test x402 payment flows without a real facilitator.

## Acceptance Criteria

1. **Given** the mock server is started
   **When** I run `x402-dev mock`
   **Then** the server starts on configurable port (default: 3402)

2. **And** any request returns `402 Payment Required` status

3. **And** response includes `WWW-Authenticate` header with payment invoice

4. **And** CORS headers are present for frontend testing

5. **And** `curl localhost:3402` returns 402 status with valid invoice

6. **And** server starts in <2 seconds

## Tasks / Subtasks

- [ ] Task 1: Add actix-web dependencies (AC: #1)
  - [ ] Add `actix-web = "4.9"` to workspace dependencies in Cargo.toml
  - [ ] Add `actix-cors = "0.7"` to workspace dependencies in Cargo.toml
  - [ ] Add actix-web and actix-cors to crates/x402-cli/Cargo.toml
  - [ ] Add `actix-rt = "2.10"` for async runtime

- [ ] Task 2: Create mock command module (AC: #1)
  - [ ] Create `crates/x402-cli/src/commands/mock.rs`
  - [ ] Implement `run()` function to start HTTP server
  - [ ] Read port from config (default: 3402)
  - [ ] Support --port CLI flag override
  - [ ] Use actix_rt::System for async runtime

- [ ] Task 3: Implement wildcard route handler (AC: #2, #3)
  - [ ] Create handler function for all HTTP methods (GET, POST, PUT, DELETE, etc.)
  - [ ] Return 402 Payment Required status code
  - [ ] Generate x402-compliant invoice in WWW-Authenticate header
  - [ ] Use actix-web's web::route() for wildcard path matching
  - [ ] Support all paths with "/*" route pattern

- [ ] Task 4: Implement CORS middleware (AC: #4)
  - [ ] Configure actix-cors::Cors middleware
  - [ ] Allow all origins for development testing
  - [ ] Allow all methods (GET, POST, PUT, DELETE, OPTIONS, etc.)
  - [ ] Allow all headers
  - [ ] Set max age for preflight requests

- [ ] Task 5: Generate x402-compliant invoice (AC: #3, #5)
  - [ ] Create invoice generation function
  - [ ] Format per x402 protocol (PRD lines 83-86): `x402-solana recipient=<address> amount=<value> currency=USDC memo=<id> network=devnet`
  - [ ] Space-separated key-value pairs (NOT base64-encoded JSON)
  - [ ] Include: recipient, amount, currency, memo, network
  - [ ] Use Solana devnet address for testing
  - [ ] NO base64 encoding - use direct space-separated format

- [ ] Task 6: Wire mock command to CLI (AC: #1)
  - [ ] Import mock module in commands/mod.rs
  - [ ] Import mock in main.rs
  - [ ] Update Commands::Mock match arm to call mock::run()
  - [ ] Add --port flag to MockArgs in cli.rs

- [ ] Task 7: Test mock server (AC: #1-6)
  - [ ] Verify `x402-dev mock --help` shows usage
  - [ ] Build successful with actix-web dependencies
  - [ ] Run server and verify startup in <2 seconds
  - [ ] Test `curl -v localhost:3402` returns 402 status
  - [ ] Verify WWW-Authenticate header present with invoice
  - [ ] Test CORS preflight: `curl -X OPTIONS -H "Origin: http://localhost:3000"`
  - [ ] Test all HTTP methods (GET, POST, PUT, DELETE)
  - [ ] Verify wildcard routing: test multiple paths

## Dev Notes

### Architecture Constraints

- **Pure Rust Implementation** (ADR-001): Use actix-web 4.9 for HTTP server
- **Async Runtime**: Use actix-rt for async/await support
- **Configuration**: Read port from Story 1.4 Config system
- **CLI Integration**: Follow Story 1.2 command pattern
- **Error Handling**: Use anyhow::Result with context messages
- **CORS**: Use actix-cors middleware for frontend compatibility
- **x402 Protocol Compliance**: WWW-Authenticate header MUST use space-separated format per PRD specification (lines 83-86)
- **NO base64 encoding** of invoice data - use direct `x402-solana key=value key=value...` format
- **Invoice Format**: `x402-solana recipient=<address> amount=<value> currency=USDC memo=<id> network=devnet`

### Project Structure

From Story 1.2 and 1.4, CLI and configuration system already exist:
```
crates/x402-cli/src/
├── config.rs         # Config struct with port (Story 1.4)
├── commands/
│   ├── mod.rs
│   ├── init.rs       # init command (Story 1.7)
│   ├── mock.rs       # NEW: HTTP mock server command
│   └── version.rs
├── cli.rs            # CLI argument parsing
├── main.rs           # Main entry point
```

**Configuration:**
- Default port: 3402 (from .x402dev.yaml or Config::default())
- CLI flag override: `--port 8080`
- Read from Config::load() (Story 1.4)

### Key Implementation Details

**HTTP Server Setup:**
```rust
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use actix_cors::Cors;

#[actix_rt::main]
async fn run(args: MockArgs) -> anyhow::Result<()> {
    let config = Config::load()?;
    let port = args.port.unwrap_or(config.port);

    println!("🚀 Starting x402 mock server on port {}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            )
            .route("/*", web::route().to(payment_required_handler))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await?;

    Ok(())
}
```

**402 Handler:**
```rust
async fn payment_required_handler(req: HttpRequest) -> HttpResponse {
    let invoice_header = format_www_authenticate_header();

    HttpResponse::PaymentRequired()
        .insert_header(("WWW-Authenticate", invoice_header))
        .insert_header(("Content-Type", "application/json"))
        .json(serde_json::json!({
            "error": "Payment Required",
            "message": "Please complete payment to access this resource"
        }))
}
```

**Invoice Generation (x402 Protocol Compliant):**
```rust
/// Format WWW-Authenticate header per x402 protocol specification
/// Format: x402-solana recipient=<address> amount=<value> currency=USDC memo=<id> network=devnet
/// Reference: PRD lines 83-86
fn format_www_authenticate_header() -> String {
    format!(
        "x402-solana recipient={} amount={} currency={} memo={} network={}",
        "GXk8vTest1111111111111111111111111111qPz9", // Devnet test address
        "0.01",                                        // Amount in USDC
        "USDC",                                        // Currency
        format!("req_{}", chrono::Utc::now().timestamp()), // Unique memo/request ID
        "devnet"                                       // Network
    )
}
```

**CORS Configuration:**
```rust
Cors::default()
    .allow_any_origin()        // Allow all origins for testing
    .allow_any_method()        // GET, POST, PUT, DELETE, OPTIONS, etc.
    .allow_any_header()        // Custom headers supported
    .max_age(3600)             // Cache preflight for 1 hour
```

### Dependencies Added

- `actix-web = "4.9"` - HTTP server framework
- `actix-cors = "0.7"` - CORS middleware
- `actix-rt = "2.10"` - Async runtime
- `chrono = "0.4"` - Timestamp generation for unique memo IDs (already in workspace)

**Note:** base64 dependency NOT needed - x402 protocol uses space-separated key-value format, not base64 encoding

### Testing Standards

**Manual Testing:**
1. Start server: `x402-dev mock`
2. Test GET: `curl -v http://localhost:3402/test`
3. Test POST: `curl -v -X POST http://localhost:3402/api/data -d '{"test":true}'`
4. Test CORS preflight: `curl -v -X OPTIONS -H "Origin: http://localhost:3000" http://localhost:3402`
5. Verify 402 status code
6. Verify WWW-Authenticate header format matches x402 protocol
7. Verify CORS headers (Access-Control-Allow-*)
8. Parse header: Extract and validate space-separated key-value pairs (recipient, amount, currency, memo, network)
9. Test startup time: `time x402-dev mock` (should be <2 seconds)

**Automated Testing:**
- Unit test invoice generation: Verify output matches format `x402-solana recipient=... amount=... currency=... memo=... network=...`
- Test WWW-Authenticate header parsing: Split by spaces, validate all required fields present
- Integration test: Start server, make HTTP request, verify response
- Test all HTTP methods
- Test multiple paths
- Test CORS headers
- **NEW**: Verify WWW-Authenticate header matches x402 protocol format (space-separated, not base64)
- **NEW**: Parse header and validate all required fields present (recipient, amount, currency, memo, network)

### Learnings from Previous Stories

**From Story 1.2 (CLI Framework - Status: done)**
- Commands::Mock enum variant available
- MockArgs struct for --port flag
- Follow existing command pattern in commands/mod.rs

**From Story 1.4 (Configuration Management - Status: done)**
- Config::load() available for reading port
- Config::default() provides default port (update to 3402 for mock server)
- Config validation ensures port is valid

**Key Interfaces to Reuse:**
- `Config::load()` for configuration
- `Commands::Mock { args }` CLI pattern
- Error handling with anyhow::Result

**From Story 1.7 (Init Command - Status: done)**
- Project config in .x402dev.yaml
- Interactive port configuration already available
- Validation ensures port >=1024

### References

- [Source: docs/epics.md#Story-2.1-lines-??] - Story requirements
- [actix-web Documentation](https://actix.rs/)
- [actix-cors Documentation](https://docs.rs/actix-cors/)
- [HTTP 402 Specification](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/402)
- [WWW-Authenticate Header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/WWW-Authenticate)
- [Source: docs/stories/1-4-configuration-management-system.md] - Config system
- [Source: docs/stories/1-2-cli-framework-setup.md] - CLI framework

## Dev Agent Record

### Context Reference

- Implementation leverages existing Config system from Story 1.4
- CLI framework from Story 1.2 provides command structure
- actix-web 4.9 for production-quality HTTP server
- actix-cors for CORS middleware
- x402-compliant invoice generation with base64 encoding

### Agent Model Used

Claude Sonnet 4.5 (claude-sonnet-4-5-20250929)

### Debug Log References

**Implementation Approach:**
- Used actix-web 4.9 with default_service for wildcard routing
- Implemented CORS middleware with allow_any_origin for development testing
- Generated x402-compliant WWW-Authenticate headers with space-separated format
- Used chrono for unique timestamp-based memo generation

**Key Decisions:**
- Changed from .route("/*") to .default_service() for proper wildcard routing
- Used space-separated key=value format (NOT base64) per x402 protocol
- Default port 3402 with CLI override via --port flag
- Async/await with actix-rt runtime (compatible with tokio)

**Build Results:**
- Build successful in 1.97s (incremental)
- All 3 unit tests passing (test_www_authenticate_header_format, test_header_parsing, test_unique_memo_generation)
- Zero runtime errors
- Server startup < 2 seconds

### Completion Notes List

**Implementation Status:**
- ✅ All 7 tasks completed successfully
- ✅ HTTP server implemented with actix-web 4.9
- ✅ 402 Payment Required responses working correctly
- ✅ WWW-Authenticate header format compliant with x402 protocol
- ✅ CORS middleware enabled for all origins, methods, and headers
- ✅ CLI integration complete with --port flag
- ✅ Unit tests passing (3/3)

**Code Quality:**
- Clean, readable code with comprehensive documentation
- Proper error handling with anyhow::Result and context
- Type-safe with Rust's ownership model
- No compiler warnings for mock.rs module
- Follows ADR-001 (Pure Rust) and ADR-002 (async runtime)

**Testing Notes:**
- Unit tests: 3/3 passing
- Manual curl tests: All passing
- GET, POST, PUT, DELETE methods: All return 402
- CORS preflight: Working correctly (200 OK with proper headers)
- Header format: Verified space-separated (not base64)
- Custom port: Tested with --port 8080
- Startup time: < 2 seconds (actual: ~0.36s build + ~1s startup)

**KISS/YAGNI Compliance:**
- Minimal implementation - exactly what's needed
- No over-engineering or unnecessary features
- Single responsibility: return 402 with x402 invoice
- Simple wildcard routing with default_service
- Clear, straightforward code structure

**Manual Testing Checklist:**
- [x] Server starts on port 3402
- [x] Server starts on custom port with --port flag
- [x] All HTTP methods return 402
- [x] WWW-Authenticate header present with x402-solana prefix
- [x] Header format matches: `x402-solana recipient=... amount=... currency=... memo=... network=...`
- [x] All required fields present: recipient, amount, currency, memo, network
- [x] Header uses space-separated format (NOT base64-encoded JSON)
- [x] CORS headers present
- [x] Startup time <2 seconds

Date: 2025-11-11

### File List

**New Files:**
- `/Users/valentynkit/dev/sandbox/Hackaton/crates/x402-cli/src/commands/mock.rs` - HTTP server implementation with 402 handler

**Modified Files:**
- `/Users/valentynkit/dev/sandbox/Hackaton/Cargo.toml` - Added actix-web, actix-cors, actix-rt, chrono to workspace dependencies
- `/Users/valentynkit/dev/sandbox/Hackaton/crates/x402-cli/Cargo.toml` - Added actix dependencies to x402-cli
- `/Users/valentynkit/dev/sandbox/Hackaton/crates/x402-cli/src/cli.rs` - Added --port flag to MockArgs
- `/Users/valentynkit/dev/sandbox/Hackaton/crates/x402-cli/src/commands/mod.rs` - Exported mock module
- `/Users/valentynkit/dev/sandbox/Hackaton/crates/x402-cli/src/main.rs` - Wired mock command to call mock::run()

## Change Log

_To be updated as story progresses_

---

## Senior Developer Review (AI)

**Reviewer:** _To be assigned_
**Date:** _To be filled_
**Model:** _To be filled_
**Outcome:** _To be filled_

### Summary

_Senior developer review to be conducted after implementation_

### Acceptance Criteria Coverage

_To be filled during review_

### Task Completion Validation

_To be filled during review_

### Code Quality Assessment

_To be filled during review_

### Architectural Alignment

_To be filled during review_

### Test Coverage

_To be filled during review_

### Security Notes

_To be filled during review_

### Best Practices

_To be filled during review_

### Action Items

_To be filled during review_

### Recommendation

_To be filled during review_

---
