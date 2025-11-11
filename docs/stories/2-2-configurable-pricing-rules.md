# Story 2.2: Configurable Pricing Rules

Status: pending

## Story

As a developer,
I want configurable pricing for different endpoints,
So that I can test various payment scenarios.

## Acceptance Criteria

1. **Given** pricing rules are configured
   **When** I request different endpoints
   **Then** each endpoint returns correct invoice amount

2. **And** per-request pricing works (e.g., $0.01 per call)

3. **And** per-resource pricing works (e.g., `/api/data` costs $0.05)

4. **And** config file or CLI flags set pricing

5. **And** different endpoints return different invoice amounts

## Tasks / Subtasks

- [ ] Task 1: Extend configuration schema for pricing (AC: #4)
  - [ ] Add `PricingConfig` struct to config.rs
  - [ ] Add `default_pricing` field (default: 0.01 SOL)
  - [ ] Add `per_resource_pricing` HashMap for endpoint-specific pricing
  - [ ] Update .x402dev.yaml schema documentation
  - [ ] Add serde defaults for pricing fields

- [ ] Task 2: Update mock command to accept pricing config (AC: #4)
  - [ ] Add --pricing CLI flag to MockArgs in cli.rs
  - [ ] Read pricing config from Config::load()
  - [ ] Pass pricing config to HTTP server handler
  - [ ] Support CLI override: `x402-dev mock --pricing 0.02`
  - [ ] Merge CLI pricing with config file pricing (CLI > config > default)

- [ ] Task 3: Implement route-matching logic (AC: #1, #3)
  - [ ] Create PricingMatcher struct with route matching logic
  - [ ] Implement exact path match (highest priority)
  - [ ] Implement prefix path match (e.g., `/api/` matches `/api/users`)
  - [ ] Implement wildcard match (e.g., `/api/*`)
  - [ ] Fallback to default pricing if no match
  - [ ] Add unit tests for route matching

- [ ] Task 4: Update invoice generation with dynamic pricing (AC: #1, #2, #3, #5)
  - [ ] Modify generate_invoice() to accept amount parameter
  - [ ] Extract request path from HttpRequest
  - [ ] Match request path against pricing rules
  - [ ] Generate invoice with matched amount
  - [ ] Include pricing breakdown in invoice metadata
  - [ ] Update invoice JSON structure to include pricing info

- [ ] Task 5: Add pricing configuration examples (AC: #4)
  - [ ] Document .x402dev.yaml pricing section in docs/
  - [ ] Add example config with multiple pricing tiers
  - [ ] Document CLI pricing override flag
  - [ ] Add pricing precedence documentation
  - [ ] Create example: default, per-resource, wildcard patterns

- [ ] Task 6: Test pricing configuration system (AC: #1-5)
  - [ ] Test default pricing (0.01 SOL) for unmatched routes
  - [ ] Test exact match: `/api/data` → 0.05 SOL
  - [ ] Test prefix match: `/api/premium/*` → 0.10 SOL
  - [ ] Test CLI override: `--pricing 0.02` changes default
  - [ ] Test config file pricing rules
  - [ ] Test multiple endpoints return different amounts
  - [ ] Verify invoice amounts in WWW-Authenticate header
  - [ ] Decode and validate invoice JSON with correct amounts

## Dev Notes

### Architecture Constraints

- **Pure Rust Implementation** (ADR-001): Use native HashMap for pricing rules
- **Configuration Integration**: Extend Story 1.4 Config system
- **Route Matching**: Implement efficient path matching (exact > prefix > default)
- **Error Handling**: Use anyhow::Result with context messages
- **Validation**: Validate pricing values (non-negative, reasonable range)

**Future Enhancements (Out of Scope for MVP):**
- Time-based pricing multipliers (e.g., peak hours 2x pricing)
- PRD FR-1.2 mentions time-based pricing as SHOULD have, deferred post-hackathon
- Can be added via `peak_hours` config with multiplier values

### Project Structure

From Story 1.4 and 2.1, configuration and mock server already exist:
```
crates/x402-cli/src/
├── config.rs         # Config struct - ADD PricingConfig (Story 1.4)
├── commands/
│   ├── mod.rs
│   ├── init.rs       # init command (Story 1.7)
│   ├── mock.rs       # MODIFY: Use pricing config (Story 2.1)
│   └── version.rs
├── cli.rs            # ADD --pricing flag to MockArgs
├── main.rs           # Main entry point
```

**Configuration File Structure:**
```yaml
# .x402dev.yaml
mock_server:
  port: 3402
  pricing:
    default: 0.01          # Default price for all requests
    per_resource:
      /api/data: 0.05      # Exact match
      /api/premium: 0.10   # Exact match
      /api/admin/*: 0.20   # Wildcard prefix match
```

### Key Implementation Details

**Pricing Configuration Schema:**
```rust
// crates/x402-cli/src/config.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub solana_rpc: String,
    pub log_level: String,

    #[serde(default)]
    pub pricing: PricingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingConfig {
    #[serde(default = "default_pricing_amount")]
    pub default: f64,

    #[serde(default)]
    pub per_resource: HashMap<String, f64>,
}

fn default_pricing_amount() -> f64 { 0.01 }

impl Default for PricingConfig {
    fn default() -> Self {
        PricingConfig {
            default: 0.01,
            per_resource: HashMap::new(),
        }
    }
}

impl PricingConfig {
    pub fn validate(&self) -> Result<()> {
        if self.default < 0.0 {
            bail!("Default pricing must be non-negative");
        }
        if self.default > 100.0 {
            bail!("Default pricing must be <= 100 SOL");
        }
        for (path, amount) in &self.per_resource {
            if *amount < 0.0 {
                bail!("Pricing for {} must be non-negative", path);
            }
            if *amount > 100.0 {
                bail!("Pricing for {} must be <= 100 SOL", path);
            }
        }
        Ok(())
    }
}
```

**Route Matching Logic:**
```rust
pub struct PricingMatcher {
    config: PricingConfig,
}

impl PricingMatcher {
    pub fn new(config: PricingConfig) -> Self {
        PricingMatcher { config }
    }

    pub fn get_price_for_path(&self, path: &str) -> f64 {
        // Priority 1: Exact match
        if let Some(&amount) = self.config.per_resource.get(path) {
            return amount;
        }

        // Priority 2: Prefix match (wildcard patterns)
        for (pattern, &amount) in &self.config.per_resource {
            if pattern.ends_with("/*") {
                let prefix = &pattern[..pattern.len() - 2];
                if path.starts_with(prefix) {
                    return amount;
                }
            }
        }

        // Priority 3: Default pricing
        self.config.default
    }
}
```

**Updated Invoice Generation:**
```rust
// crates/x402-cli/src/commands/mock.rs
async fn payment_required_handler(
    req: HttpRequest,
    pricing: web::Data<PricingMatcher>,
) -> HttpResponse {
    let path = req.path();
    let amount = pricing.get_price_for_path(path);
    let invoice = generate_invoice(amount, path);

    HttpResponse::PaymentRequired()
        .insert_header(("WWW-Authenticate", format!("Bearer invoice={}", invoice)))
        .insert_header(("Content-Type", "application/json"))
        .json(serde_json::json!({
            "error": "Payment Required",
            "message": "Please complete payment to access this resource",
            "invoice": invoice,
            "amount_sol": amount,
            "path": path,
        }))
}

fn generate_invoice(amount: f64, path: &str) -> String {
    use base64::{Engine as _, engine::general_purpose};

    let invoice_data = serde_json::json!({
        "amount": amount,
        "recipient": "DevnetTestWallet1111111111111111111111111",
        "memo": format!("x402-dev payment for {}", path),
        "timestamp": chrono::Utc::now().timestamp(),
        "currency": "SOL",
        "network": "devnet",
        "resource": path,
    });

    let json_str = serde_json::to_string(&invoice_data).unwrap();
    general_purpose::STANDARD.encode(json_str.as_bytes())
}
```

**CLI Flag Override:**
```rust
// crates/x402-cli/src/cli.rs
#[derive(Parser, Debug)]
pub struct MockArgs {
    /// Override default pricing (in SOL)
    #[arg(long, value_name = "AMOUNT")]
    pub pricing: Option<f64>,

    /// Override server port
    #[arg(long, short = 'p', value_name = "PORT")]
    pub port: Option<u16>,
}

// In mock command:
let mut config = Config::load()?;
if let Some(pricing) = args.pricing {
    config.pricing.default = pricing;
}
```

**Example Configuration:**
```yaml
# .x402dev.yaml
port: 3402
solana_rpc: "https://api.devnet.solana.com"
log_level: "info"

pricing:
  default: 0.01
  per_resource:
    /api/data: 0.05
    /api/premium: 0.10
    /api/admin/*: 0.20
    /free/status: 0.00
```

### Dependencies

All dependencies already available from previous stories:
- `serde = "1.0"` ✅ (Story 1.4)
- `serde_yaml = "0.9"` ✅ (Story 1.4)
- `anyhow = "1.0"` ✅ (Story 1.1)
- `actix-web = "4.9"` ✅ (Story 2.1)
- `base64 = "0.22"` ✅ (Story 2.1)
- `chrono = "0.4"` ✅ (Story 2.1)

No new dependencies required.

### Testing Standards

**Manual CLI Testing:**
1. Create .x402dev.yaml with pricing rules
2. Start server: `x402-dev mock`
3. Test default pricing: `curl -v http://localhost:3402/random`
4. Test exact match: `curl -v http://localhost:3402/api/data`
5. Test prefix match: `curl -v http://localhost:3402/api/admin/users`
6. Test CLI override: `x402-dev mock --pricing 0.02`, then test endpoints
7. Verify WWW-Authenticate header contains correct amounts
8. Decode invoice and verify amount matches pricing rule

**Automated Testing:**
- Unit test PricingMatcher::get_price_for_path() with various patterns
- Test exact match priority
- Test prefix match priority
- Test default fallback
- Test wildcard patterns
- Integration test: Start server, make requests, decode invoices, verify amounts

**Test Scenarios:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let mut per_resource = HashMap::new();
        per_resource.insert("/api/data".to_string(), 0.05);
        let config = PricingConfig { default: 0.01, per_resource };
        let matcher = PricingMatcher::new(config);

        assert_eq!(matcher.get_price_for_path("/api/data"), 0.05);
    }

    #[test]
    fn test_prefix_match() {
        let mut per_resource = HashMap::new();
        per_resource.insert("/api/admin/*".to_string(), 0.20);
        let config = PricingConfig { default: 0.01, per_resource };
        let matcher = PricingMatcher::new(config);

        assert_eq!(matcher.get_price_for_path("/api/admin/users"), 0.20);
        assert_eq!(matcher.get_price_for_path("/api/admin/settings"), 0.20);
    }

    #[test]
    fn test_default_fallback() {
        let config = PricingConfig::default();
        let matcher = PricingMatcher::new(config);

        assert_eq!(matcher.get_price_for_path("/random/path"), 0.01);
    }

    #[test]
    fn test_exact_over_prefix() {
        let mut per_resource = HashMap::new();
        per_resource.insert("/api/*".to_string(), 0.03);
        per_resource.insert("/api/data".to_string(), 0.05);
        let config = PricingConfig { default: 0.01, per_resource };
        let matcher = PricingMatcher::new(config);

        // Exact match should win
        assert_eq!(matcher.get_price_for_path("/api/data"), 0.05);
        // Prefix match for other paths
        assert_eq!(matcher.get_price_for_path("/api/users"), 0.03);
    }
}
```

### Learnings from Previous Stories

**From Story 1.4 (Configuration Management - Status: done)**
- Config struct with serde defaults available
- Config::load() reads .x402dev.yaml
- Config::validate() pattern for validation
- CLI flag override pattern established
- Use anyhow::Result with context for errors

**From Story 2.1 (HTTP Server - Status: pending)**
- MockArgs struct with --port flag (add --pricing flag)
- generate_invoice() function exists (modify to accept amount)
- payment_required_handler() returns 402 (modify to use pricing)
- actix-web Data wrapper for shared state (use for PricingMatcher)

**Key Interfaces to Reuse:**
- `Config::load()` for reading configuration
- `Config::validate()` pattern for PricingConfig validation
- actix-web's `web::Data<T>` for dependency injection
- actix-web's `HttpRequest` for path extraction

**From Story 1.7 (Init Command - Status: done)**
- Interactive prompts for configuration
- Consider adding pricing prompts in future enhancement
- Config file generation pattern established

### References

- [Source: docs/epics.md#Story-2.2] - Story requirements
- [Source: docs/stories/1-4-configuration-management-system.md] - Config system details
- [Source: docs/stories/2-1-http-server-402-responses.md] - Mock server implementation
- [actix-web Data Documentation](https://actix.rs/docs/application/#state)
- [Rust HashMap Documentation](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

## Dev Agent Record

### Context Reference

- Implementation builds on Story 1.4 (Configuration) and Story 2.1 (Mock Server)
- Route matching follows common HTTP routing patterns (exact > prefix > default)
- Configuration follows YAML best practices for nested structures

### Agent Model Used

_To be filled by dev agent_

### Debug Log References

_To be filled by dev agent during implementation_

**Implementation Approach:**
- _Dev agent will document key decisions here_

**Key Decisions:**
- _Dev agent will document architectural choices here_

**Build Results:**
- _Dev agent will document build output here_

### Completion Notes List

_To be filled by dev agent upon completion_

**Implementation Status:**
- _Dev agent will update this section_

**Code Quality:**
- _Dev agent will document quality metrics_

**Testing Notes:**
- _Dev agent will document test results_

**KISS/YAGNI Compliance:**
- _Dev agent will verify simplicity_

**Manual Testing Checklist:**
- [ ] Default pricing works (0.01 SOL)
- [ ] Exact match pricing works (/api/data → 0.05 SOL)
- [ ] Prefix match pricing works (/api/admin/* → 0.20 SOL)
- [ ] CLI override works (--pricing 0.02)
- [ ] Config file pricing loads correctly
- [ ] Different endpoints return different amounts
- [ ] Invoice amounts match pricing rules
- [ ] Decoded invoice JSON has correct amount field

Date: _To be filled_

### File List

**New Files:**
- _To be filled by dev agent_

**Modified Files:**
- crates/x402-cli/src/config.rs (add PricingConfig)
- crates/x402-cli/src/commands/mock.rs (use pricing config)
- crates/x402-cli/src/cli.rs (add --pricing flag)

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
