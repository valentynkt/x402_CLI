# Test Infrastructure Refactoring Plan
## x402 Protocol - Comprehensive Test Analysis & Recommendations

**Date**: 2025-11-12
**Status**: 42 passing / 3 failing / 0 integration tests
**Coverage**: Est. ~60-70% (unit tests only)

---

## Executive Summary

The x402 Protocol test suite shows good unit test coverage but suffers from:
1. **3 Failing Tests** - Logic bugs in sliding window calculations
2. **0 Integration Tests** - Mock server (490 lines) has no real integration coverage
3. **Test Organization Issues** - Inline tests mixed with implementation
4. **Missing Test Infrastructure** - No test helpers, builders, or fixtures
5. **No End-to-End Tests** - Full payment flow untested

### Critical Issues Priority
1. **🔴 CRITICAL**: Fix 3 failing tests (data integrity bugs)
2. **🟠 HIGH**: Add mock server integration tests (490 lines, 0% coverage)
3. **🟡 MEDIUM**: Extract test helpers and builders
4. **🟢 LOW**: Property-based test improvements

---

## 1. Current Test Failures - Root Cause Analysis

### 1.1 Test Failure: `test_policy_priority_order`

**Location**: `crates/x402-core/src/policy/engine.rs:387-418`
**Failure**: `assertion failed: decision.is_allowed()`

#### Root Cause
The test expects priority-based evaluation where **higher priority wins**, but the current implementation evaluates in sorted order and uses **fail-fast on deny**. The test has:

```rust
Policy {
    priority: 1,    // Lower priority DENY
    action: Deny,
}
Policy {
    priority: 10,   // Higher priority ALLOW
    action: Allow,
}
```

After sorting by priority (highest first):
1. Priority 10 (Allow) evaluated first → matches → marks `allow_matched = true`
2. Priority 1 (Deny) evaluated second → matches → **DENIES** (fail-fast)

**The Bug**: Once an Allow policy matches, the engine should not continue evaluating lower priority policies if they would deny. This violates the expected "highest priority wins" semantics.

#### Fix Recommendation
**Option A**: Return immediately after first policy match (any priority)
```rust
match &policy.action {
    PolicyAction::Allow => {
        self.update_state(policy, request, now)?;
        return Ok(PolicyDecision::Allow { policy_id: policy.id.clone() });
    }
    PolicyAction::Deny(reason) => {
        return Ok(PolicyDecision::Deny {
            reason: reason.clone(),
            policy_id: policy.id.clone()
        });
    }
}
```

**Option B**: Clarify test expectations to match "deny overrides allow" semantics
```rust
// Update test to expect DENY because fail-fast on deny is security feature
assert!(decision.is_denied());
```

**Recommendation**: Choose **Option A** - First match wins by priority is more intuitive and secure.

---

### 1.2 Test Failure: `test_rate_limit_expiration`

**Location**: `crates/x402-core/src/policy/state.rs:199-217`
**Failure**: `assertion left == right failed: left: 3, right: 1`

#### Root Cause
Test expects sliding window to count **only requests within 60s window** from `base_time`:

```rust
state.add_request(base_time);                           // t=0
state.add_request(base_time + Duration::from_secs(30)); // t=30
state.add_request(base_time + Duration::from_secs(70)); // t=70 (outside window)

// From base_time perspective (t=0), window is [0-60]
let count = state.count_in_window(window, base_time);
assert_eq!(count, 1); // Expected: only request at t=0
```

**The Bug**: The test expects `count = 1` but gets `count = 3`. The sliding window logic in `count_in_window` is:

```rust
let window_start = now.checked_sub(window).unwrap_or(now); // now - 60s
self.request_times
    .iter()
    .filter(|&&time| time >= window_start) // Count all >= window_start
    .count()
```

For `now = base_time` (t=0), `window_start = 0 - 60 = 0` (saturates to 0).
All three requests (t=0, t=30, t=70) have `time >= 0`, so all 3 are counted.

**The Real Issue**: The test is checking from `base_time` BEFORE later requests were added, but the implementation counts ALL requests in state regardless of when they were added relative to the check time.

#### Fix Recommendation
**Option A**: Fix the sliding window logic (CORRECT)
```rust
pub fn count_in_window(&self, window: Duration, now: SystemTime) -> usize {
    let window_start = now.checked_sub(window).unwrap_or(now);
    self.request_times
        .iter()
        .filter(|&&time| time >= window_start && time <= now) // Add upper bound
        .count()
}
```

**Option B**: Fix the test expectations (INCORRECT - masks bug)

**Recommendation**: **Option A** - The sliding window must exclude future requests.

---

### 1.3 Test Failure: `test_spending_window_expiration`

**Location**: `crates/x402-core/src/policy/state.rs:240-256`
**Failure**: `assertion left == right failed: left: 600, right: 100`

#### Root Cause
Same root cause as `test_rate_limit_expiration`:

```rust
state.add_spending(base_time, 100);                     // t=0,  amount=100
state.add_spending(base_time + Duration::from_secs(30), 200); // t=30, amount=200
state.add_spending(base_time + Duration::from_secs(70), 300); // t=70, amount=300

// From base_time (t=0), window = 60s, so window is [0, 60]
assert_eq!(state.total_in_window(window, base_time), 100); // Expected
```

Gets `600` (sum of 100+200+300) instead of `100` because `total_in_window` doesn't exclude future spending.

#### Fix Recommendation
```rust
pub fn total_in_window(&self, window: Duration, now: SystemTime) -> u64 {
    let window_start = now.checked_sub(window).unwrap_or(now);

    self.spending_records
        .iter()
        .filter(|(time, _)| *time >= window_start && *time <= now) // Add upper bound
        .map(|(_, amount)| amount)
        .sum()
}
```

---

## 2. Test Organization Issues

### 2.1 Inline Tests vs Test Modules

**Current State**: 90% of tests are inline `#[cfg(test)] mod tests` blocks within implementation files.

**Problems**:
- Hard to find and navigate tests
- Test code increases cognitive load when reading implementation
- Difficult to share test fixtures
- Can't easily run tests by domain

**Recommendation**: Create dedicated test module structure:

```
crates/x402-core/
├── src/
│   ├── policy/
│   │   ├── engine.rs          (impl only, no tests)
│   │   ├── state.rs           (impl only, no tests)
│   │   └── ...
│   └── lib.rs
├── tests/
│   ├── unit/
│   │   ├── policy_engine_tests.rs
│   │   ├── policy_state_tests.rs
│   │   ├── pattern_matching_tests.rs
│   │   └── mod.rs
│   ├── integration/
│   │   ├── mock_server_tests.rs
│   │   ├── policy_evaluation_tests.rs
│   │   ├── payment_flow_tests.rs
│   │   └── mod.rs
│   ├── fixtures/
│   │   ├── policies.rs
│   │   ├── requests.rs
│   │   └── mod.rs
│   └── helpers/
│       ├── builders.rs
│       ├── assertions.rs
│       └── mod.rs
```

**Migration Strategy**:
1. Keep existing inline tests (don't break)
2. Create new test organization in parallel
3. Gradually migrate tests (25% per sprint)
4. Remove inline tests once external tests have same coverage

---

### 2.2 Test Code Duplication

**Identified Duplications**:

1. **Test Request Creation** (appears 15+ times):
```rust
// Duplicated everywhere
fn create_test_request(agent_id: &str, amount: u64, endpoint: &str) -> Request {
    Request {
        agent_id: agent_id.to_string(),
        wallet_address: None,
        ip_address: None,
        amount: amount as f64,
        endpoint: endpoint.to_string(),
        timestamp: SystemTime::now(),
    }
}
```

2. **Test Policy Creation** (appears 10+ times):
```rust
// Duplicated in engine.rs tests
fn create_allow_policy(id: &str, agent_patterns: Vec<String>) -> Policy { ... }
fn create_deny_policy(id: &str, agent_patterns: Vec<String>) -> Policy { ... }
```

3. **Pattern Matching Helper** (duplicated in property_tests.rs):
```rust
// Appears in both property_tests.rs and should use engine's impl
fn matches_pattern(pattern: &str, value: &str) -> bool { ... }
```

**Recommendation**: Create test builders module:

```rust
// tests/helpers/builders.rs
pub struct RequestBuilder {
    agent_id: String,
    wallet_address: Option<String>,
    amount: f64,
    endpoint: String,
    timestamp: SystemTime,
}

impl RequestBuilder {
    pub fn new(agent_id: &str) -> Self { ... }
    pub fn with_amount(mut self, amount: f64) -> Self { ... }
    pub fn with_endpoint(mut self, endpoint: &str) -> Self { ... }
    pub fn with_timestamp(mut self, time: SystemTime) -> Self { ... }
    pub fn build(self) -> Request { ... }
}

pub struct PolicyBuilder {
    id: String,
    priority: i32,
    action: PolicyAction,
    agent_patterns: Vec<String>,
    endpoint_patterns: Vec<String>,
    rate_limit: Option<RateLimitConfig>,
    spending_cap: Option<SpendingCapConfig>,
}

impl PolicyBuilder {
    pub fn allow(id: &str) -> Self { ... }
    pub fn deny(id: &str, reason: &str) -> Self { ... }
    pub fn with_priority(mut self, priority: i32) -> Self { ... }
    pub fn with_rate_limit(mut self, max: u32, window: Duration) -> Self { ... }
    pub fn build(self) -> Policy { ... }
}
```

**Usage**:
```rust
let request = RequestBuilder::new("agent-123")
    .with_amount(100.0)
    .with_endpoint("/api/test")
    .build();

let policy = PolicyBuilder::allow("allow-all")
    .with_priority(10)
    .with_rate_limit(100, Duration::from_secs(3600))
    .build();
```

---

## 3. Missing Integration Tests

### 3.1 Mock Server Integration Tests (HIGH PRIORITY)

**Current State**: Mock server has **490 lines** of code, **0% integration test coverage**.

**Gap Analysis**:

The existing `mock_server_integration.rs` is actually **unit tests** using `actix_web::test`, not real server integration tests. Real integration tests should:

1. Start actual HTTP server on real port
2. Make real HTTP requests (not `test::TestRequest`)
3. Test full request/response cycle
4. Test concurrent connections
5. Test server lifecycle (start, stop, restart)

**Required Tests**:

```rust
// tests/integration/mock_server_real.rs

#[tokio::test]
async fn test_server_starts_and_responds() {
    // Start real server
    let server = start_mock_server(8402).await;

    // Make real HTTP request
    let client = reqwest::Client::new();
    let resp = client.get("http://localhost:8402/api/test").send().await.unwrap();

    assert_eq!(resp.status(), StatusCode::PAYMENT_REQUIRED);
    assert!(resp.headers().contains_key("www-authenticate"));

    // Cleanup
    server.stop().await;
}

#[tokio::test]
async fn test_full_payment_flow() {
    let server = start_mock_server(8403).await;
    let client = reqwest::Client::new();

    // 1. Request without payment -> 402
    let resp = client.get("http://localhost:8403/api/premium").send().await.unwrap();
    assert_eq!(resp.status(), 402);

    // 2. Extract invoice from WWW-Authenticate header
    let invoice = resp.headers()
        .get("www-authenticate")
        .unwrap()
        .to_str()
        .unwrap();

    // 3. Simulate payment and get proof
    let payment_proof = simulate_payment(&invoice).await;

    // 4. Retry request with payment proof -> 200
    let resp = client.get("http://localhost:8403/api/premium")
        .header("x-payment-proof", payment_proof)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    server.stop().await;
}

#[tokio::test]
async fn test_concurrent_requests_real_server() {
    let server = start_mock_server(8404).await;
    let client = Arc::new(reqwest::Client::new());

    // Spawn 100 concurrent requests
    let mut handles = vec![];
    for i in 0..100 {
        let client = Arc::clone(&client);
        let handle = tokio::spawn(async move {
            client.get(&format!("http://localhost:8404/api/test?req={}", i))
                .send()
                .await
        });
        handles.push(handle);
    }

    // All should complete
    for handle in handles {
        let resp = handle.await.unwrap().unwrap();
        assert_eq!(resp.status(), 402);
    }

    server.stop().await;
}

#[tokio::test]
async fn test_policy_enforcement_integration() {
    // Start server with specific policy
    let policy = PolicyBuilder::allow("test-agents")
        .matching_agents(&["agent-allowed"])
        .with_rate_limit(3, Duration::from_secs(60))
        .build();

    let server = start_mock_server_with_policy(8405, policy).await;
    let client = reqwest::Client::new();

    // Test rate limiting actually works end-to-end
    for i in 0..3 {
        let resp = client.get("http://localhost:8405/api/test")
            .header("x-agent-id", "agent-allowed")
            .send()
            .await
            .unwrap();

        assert_eq!(resp.status(), 402); // Payment required but policy allows
    }

    // 4th request should be rate limited
    let resp = client.get("http://localhost:8405/api/test")
        .header("x-agent-id", "agent-allowed")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 429); // Too Many Requests

    server.stop().await;
}
```

**Test Infrastructure Needed**:

```rust
// tests/helpers/mock_server_helper.rs

pub struct TestServer {
    handle: ServerHandle,
    port: u16,
}

impl TestServer {
    pub async fn start(port: u16) -> Self { ... }
    pub async fn start_with_policy(port: u16, policy: PolicyConfig) -> Self { ... }
    pub async fn stop(self) { ... }
    pub fn url(&self) -> String { format!("http://localhost:{}", self.port) }
}

pub async fn start_mock_server(port: u16) -> TestServer { ... }
pub async fn simulate_payment(invoice: &str) -> String { ... }
```

---

### 3.2 Policy Evaluation Integration Tests

**Gap**: Tests verify individual components (parser, validator, engine) but not the **full pipeline**:

```
YAML File → Parser → Validator → Engine → Code Generator → Running Middleware
```

**Required Tests**:

```rust
// tests/integration/policy_pipeline_tests.rs

#[test]
fn test_full_policy_pipeline_allowlist() {
    // 1. Create YAML
    let yaml = r#"
policies:
  - type: allowlist
    field: agent_id
    values: ["agent-good"]
"#;

    // 2. Parse
    let config = PolicyParser::parse_yaml(yaml).unwrap();

    // 3. Validate
    let report = validate_policies(&config);
    assert!(report.is_valid());

    // 4. Create engine
    let runtime_policies = convert_to_runtime_policies(&config);
    let engine = PolicyEngine::new(runtime_policies);

    // 5. Evaluate requests
    let good_request = Request::new("agent-good", "/api/test");
    let bad_request = Request::new("agent-bad", "/api/test");

    assert!(engine.evaluate(&good_request).unwrap().is_allowed());
    assert!(engine.evaluate(&bad_request).unwrap().is_denied());
}

#[test]
fn test_full_pipeline_with_code_generation() {
    let yaml = load_test_policy("complex_policy.yaml");

    // Parse & validate
    let config = PolicyParser::parse_yaml(&yaml).unwrap();
    let report = validate_policies(&config);
    assert!(report.is_valid());

    // Generate Express middleware
    let express_code = generate_express_middleware(&config).unwrap();

    // Verify generated code contains expected patterns
    assert!(express_code.contains("x402Middleware"));
    assert!(express_code.contains("allowlist"));

    // Generate Fastify plugin
    let fastify_code = generate_fastify_plugin(&config).unwrap();
    assert!(fastify_code.contains("fastify-plugin"));

    // Could even run Node.js and test generated code (advanced)
}
```

---

## 4. Test Quality Issues

### 4.1 Tests That Test Implementation vs Behavior

**Problem**: Some tests are tightly coupled to implementation details rather than testing behavior.

**Example**:
```rust
// BAD: Tests implementation detail (sorting order)
#[test]
fn test_policies_sorted_by_priority() {
    let policies = vec![
        Policy { priority: 1, ... },
        Policy { priority: 10, ... },
    ];
    let engine = PolicyEngine::new(policies);

    // Checking internal field
    assert_eq!(engine.policies()[0].priority, 10);
    assert_eq!(engine.policies()[1].priority, 1);
}

// GOOD: Tests behavior (evaluation order)
#[test]
fn test_high_priority_policy_evaluated_first() {
    let policies = vec![
        Policy { priority: 1, action: Allow, ... },
        Policy { priority: 10, action: Deny, ... },
    ];
    let engine = PolicyEngine::new(policies);
    let request = create_test_request();

    // Test behavior, not implementation
    assert!(engine.evaluate(&request).is_denied());
}
```

**Refactoring**:
1. Identify tests that access private fields or internal state
2. Rewrite to test observable behavior via public API
3. Use property-based testing for invariants

---

### 4.2 Missing Edge Cases

**Identified Gaps**:

1. **Empty/Null Cases**:
   - Empty agent_id: `""`
   - Empty endpoint: `""`
   - Zero amount: `0.0`
   - Negative amount: `-100.0`

2. **Boundary Values**:
   - Max u32 for rate limit: `u32::MAX`
   - Max f64 for spending: `f64::MAX`
   - Very long strings (DoS test): `"a".repeat(10_000)`

3. **Time Edge Cases**:
   - SystemTime overflow/underflow
   - Duration zero: `Duration::from_secs(0)`
   - Window exactly at boundary

4. **Concurrent Edge Cases**:
   - Race condition between read/update
   - Simultaneous cleanup operations
   - Deadlock scenarios

**Recommendation**: Add edge case test suite:

```rust
// tests/unit/edge_cases.rs

#[test]
fn test_empty_agent_id() {
    let request = Request::new("", "/api/test");
    let policy = PolicyBuilder::allow("test")
        .matching_agents(&["agent-*"])
        .build();

    let engine = PolicyEngine::new(vec![policy]);
    let decision = engine.evaluate(&request).unwrap();

    // Should deny empty agent_id (doesn't match pattern)
    assert!(decision.is_denied());
}

#[test]
fn test_negative_amount() {
    let mut request = Request::new("agent-1", "/api/test");
    request.amount = -100.0;

    let policy = PolicyBuilder::allow("test")
        .with_spending_cap(1000.0, Duration::from_secs(3600))
        .build();

    let engine = PolicyEngine::new(vec![policy]);
    let decision = engine.evaluate(&request).unwrap();

    // Should handle gracefully (treat as 0 or deny?)
    // This test will expose the actual behavior
}

#[test]
fn test_max_rate_limit() {
    let policy = PolicyBuilder::allow("test")
        .with_rate_limit(u32::MAX, Duration::from_secs(1))
        .build();

    // Should not panic or overflow
    let engine = PolicyEngine::new(vec![policy]);
}

#[test]
fn test_zero_window_duration() {
    // Should this be allowed or return error?
    let result = PolicyBuilder::allow("test")
        .with_rate_limit(100, Duration::from_secs(0))
        .build();

    // Validate at build time or runtime?
}
```

---

### 4.3 Brittle Tests

**Problem**: Tests that break when implementation changes even though behavior is correct.

**Example**:
```rust
// BRITTLE: Exact string matching
#[test]
fn test_error_message() {
    let result = validate_policy(&invalid_policy);
    assert_eq!(result.unwrap_err(), "RateLimit must have 'max_requests' specified");
}
```

If error message changes slightly ("max_requests" → "maxRequests"), test breaks.

**Recommendation**: Use partial matching:
```rust
// ROBUST: Semantic matching
#[test]
fn test_error_message() {
    let result = validate_policy(&invalid_policy);
    let err = result.unwrap_err();
    assert!(err.contains("RateLimit"));
    assert!(err.contains("max_requests"));
}
```

---

## 5. Property-Based Testing Improvements

### 5.1 Current Property Tests

The existing `property_tests.rs` is excellent but has compilation issues:

**Problems**:
1. Uses non-existent methods: `is_rate_limited()` doesn't exist in `RateLimitState`
2. Missing imports: `Request`, `Policy` types incomplete
3. Test never runs (compilation errors)

**Fix Strategy**:

```rust
// crates/x402-core/src/policy/state.rs
impl RateLimitState {
    // Add missing method
    pub fn is_rate_limited(&self, config: &RateLimitConfig, now: SystemTime) -> bool {
        !self.check_limit(config.window, config.max_requests, now)
    }
}
```

### 5.2 Additional Property Tests Needed

```rust
// Property: Policies are deterministic
proptest! {
    #[test]
    fn evaluate_same_request_twice_gives_same_result(
        agent_id in "[a-z]{5,10}",
        endpoint in "/api/[a-z]{3,8}"
    ) {
        let policy = PolicyBuilder::allow("test")
            .matching_agents(&[format!("{}*", &agent_id[..3])])
            .build();

        let engine = PolicyEngine::new(vec![policy]);

        let request = Request::new(&agent_id, &endpoint);
        let result1 = engine.evaluate(&request).unwrap();
        let result2 = engine.evaluate(&request).unwrap();

        // Must be deterministic
        assert_eq!(result1.is_allowed(), result2.is_allowed());
    }
}

// Property: Rate limit is monotonic (never increases within window)
proptest! {
    #[test]
    fn rate_limit_count_never_decreases_within_window(
        num_requests in 1usize..50
    ) {
        let mut state = RateLimitState::new();
        let now = SystemTime::now();
        let window = Duration::from_secs(60);

        for _ in 0..num_requests {
            let before_count = state.count_in_window(window, now);
            state.add_request(now);
            let after_count = state.count_in_window(window, now);

            // Count should only increase or stay same
            assert!(after_count >= before_count);
        }
    }
}
```

---

## 6. Test Infrastructure Components Needed

### 6.1 Test Fixtures

Create reusable policy configurations:

```rust
// tests/fixtures/policies.rs

pub fn allowlist_policy_yaml() -> &'static str {
    r#"
policies:
  - type: allowlist
    field: agent_id
    values: ["agent-test"]
"#
}

pub fn complex_policy_yaml() -> &'static str {
    include_str!("../data/complex_policy.yaml")
}

pub fn rate_limit_policy() -> Policy {
    PolicyBuilder::allow("rate-test")
        .with_rate_limit(100, Duration::from_secs(3600))
        .build()
}
```

### 6.2 Custom Assertions

```rust
// tests/helpers/assertions.rs

pub trait PolicyDecisionAssertions {
    fn assert_allowed_by(&self, policy_id: &str);
    fn assert_denied_for_reason(&self, reason: &str);
}

impl PolicyDecisionAssertions for PolicyDecision {
    fn assert_allowed_by(&self, policy_id: &str) {
        match self {
            PolicyDecision::Allow { policy_id: id } => {
                assert_eq!(id, policy_id);
            }
            PolicyDecision::Deny { reason, .. } => {
                panic!("Expected allow by {}, got deny: {}", policy_id, reason);
            }
        }
    }

    fn assert_denied_for_reason(&self, expected_reason: &str) {
        match self {
            PolicyDecision::Deny { reason, .. } => {
                assert!(reason.contains(expected_reason),
                    "Expected reason containing '{}', got '{}'", expected_reason, reason);
            }
            PolicyDecision::Allow { .. } => {
                panic!("Expected deny, got allow");
            }
        }
    }
}

// Usage:
decision.assert_allowed_by("allow-all");
decision.assert_denied_for_reason("rate limit");
```

### 6.3 Test Data Generators

```rust
// tests/helpers/generators.rs

pub struct PolicyGenerator;

impl PolicyGenerator {
    pub fn random_allowlist() -> Policy { ... }
    pub fn random_rate_limit() -> Policy { ... }
    pub fn random_spending_cap() -> Policy { ... }

    pub fn conflicting_pair() -> (Policy, Policy) {
        let agent = format!("agent-{}", rand::random::<u32>());
        (
            PolicyBuilder::allow("allow").matching_agents(&[&agent]).build(),
            PolicyBuilder::deny("deny", "test").matching_agents(&[&agent]).build(),
        )
    }
}
```

---

## 7. Performance & Benchmark Tests

### 7.1 Missing Benchmarks

**Current State**: No performance benchmarks exist.

**Recommendation**: Add criterion benchmarks:

```rust
// benches/policy_evaluation.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_policy_evaluation(c: &mut Criterion) {
    let policies = vec![
        PolicyBuilder::allow("test").matching_agents(&["agent-*"]).build(),
    ];
    let engine = PolicyEngine::new(policies);
    let request = Request::new("agent-123", "/api/test");

    c.bench_function("evaluate_single_policy", |b| {
        b.iter(|| {
            engine.evaluate(black_box(&request))
        });
    });
}

fn benchmark_pattern_matching(c: &mut Criterion) {
    let engine = PolicyEngine::new(vec![]);

    c.bench_function("wildcard_pattern_match", |b| {
        b.iter(|| {
            engine.matches_pattern(black_box("agent-*"), black_box("agent-abc-123"))
        });
    });
}

criterion_group!(benches, benchmark_policy_evaluation, benchmark_pattern_matching);
criterion_main!(benches);
```

Add to `Cargo.toml`:
```toml
[[bench]]
name = "policy_evaluation"
harness = false
```

---

## 8. Test Coverage Gaps by Module

### 8.1 Coverage Analysis

Based on file sizes and test presence:

| Module | Lines | Unit Tests | Integration | Coverage Est. |
|--------|-------|-----------|-------------|---------------|
| `engine.rs` | 585 | ✅ 18 tests | ❌ None | ~85% |
| `state.rs` | 310 | ✅ 8 tests | ❌ None | ~70% |
| `parser.rs` | 339 | ✅ 15 tests | ❌ None | ~90% |
| `validator.rs` | 548 | ✅ 8 tests | ❌ None | ~80% |
| `codegen/express.rs` | ~400 | ✅ 7 tests | ❌ None | ~60% |
| `codegen/fastify.rs` | ~300 | ✅ 5 tests | ❌ None | ~60% |
| **Mock Server** | **490** | **❌ 0 real tests** | **❌ None** | **~0%** |
| CLI commands | ~800 | ⚠️ Stub tests | ❌ None | ~20% |

**Critical Gaps**:
1. Mock server (490 lines, 0% coverage) - **HIGHEST PRIORITY**
2. CLI integration (800 lines, stub tests only)
3. Code generator edge cases
4. Error handling paths

---

## 9. Recommended Test Organization

### 9.1 New Directory Structure

```
crates/x402-core/
├── src/
│   └── (implementation only, minimal inline tests)
├── tests/
│   ├── unit/
│   │   ├── mod.rs
│   │   ├── policy_engine_tests.rs       (moved from engine.rs)
│   │   ├── policy_state_tests.rs        (moved from state.rs)
│   │   ├── pattern_matching_tests.rs
│   │   ├── rate_limiting_tests.rs
│   │   ├── spending_cap_tests.rs
│   │   └── validator_tests.rs
│   ├── integration/
│   │   ├── mod.rs
│   │   ├── mock_server_real.rs          (NEW - real HTTP server tests)
│   │   ├── policy_pipeline_tests.rs     (NEW - full YAML→Engine flow)
│   │   ├── code_generation_tests.rs     (NEW - validate generated code)
│   │   └── concurrent_load_tests.rs     (NEW - stress tests)
│   ├── property/
│   │   ├── mod.rs
│   │   ├── pattern_properties.rs        (from property_tests.rs)
│   │   ├── rate_limit_properties.rs
│   │   ├── spending_properties.rs
│   │   └── engine_properties.rs
│   ├── fixtures/
│   │   ├── mod.rs
│   │   ├── policies.rs
│   │   ├── requests.rs
│   │   └── data/
│   │       ├── simple_policy.yaml
│   │       ├── complex_policy.yaml
│   │       └── conflicting_policies.yaml
│   ├── helpers/
│   │   ├── mod.rs
│   │   ├── builders.rs                  (RequestBuilder, PolicyBuilder)
│   │   ├── assertions.rs                (Custom assert helpers)
│   │   ├── mock_server_helper.rs        (TestServer wrapper)
│   │   └── generators.rs                (Random test data)
│   └── edge_cases/
│       ├── mod.rs
│       ├── boundary_values_tests.rs
│       ├── empty_null_tests.rs
│       ├── overflow_tests.rs
│       └── time_edge_cases_tests.rs
└── benches/
    ├── policy_evaluation.rs
    └── pattern_matching.rs

crates/x402-cli/
├── tests/
│   ├── integration/
│   │   ├── cli_commands_real.rs         (real subprocess tests)
│   │   └── full_workflow_tests.rs       (init → generate → validate)
│   └── helpers/
│       └── cli_runner.rs
```

---

## 10. Implementation Roadmap

### Phase 1: Fix Critical Bugs (Week 1)
**Priority**: 🔴 CRITICAL

- [ ] Fix `test_policy_priority_order` - First match wins semantics
- [ ] Fix `test_rate_limit_expiration` - Add upper bound to sliding window
- [ ] Fix `test_spending_window_expiration` - Add upper bound check
- [ ] Add regression tests for these fixes
- [ ] Document priority evaluation behavior

**Acceptance Criteria**: All 45 tests passing

---

### Phase 2: Add Mock Server Integration Tests (Week 2)
**Priority**: 🟠 HIGH (490 lines, 0% coverage)

- [ ] Create `tests/helpers/mock_server_helper.rs` with TestServer wrapper
- [ ] Write `test_server_starts_and_responds` (basic lifecycle)
- [ ] Write `test_full_payment_flow` (402 → payment → 200)
- [ ] Write `test_concurrent_requests_real_server` (100 concurrent)
- [ ] Write `test_policy_enforcement_integration` (rate limit works end-to-end)
- [ ] Add graceful shutdown tests

**Acceptance Criteria**: 10+ real integration tests, mock server coverage >60%

---

### Phase 3: Create Test Infrastructure (Week 3)
**Priority**: 🟡 MEDIUM

- [ ] Create `tests/helpers/builders.rs` with RequestBuilder & PolicyBuilder
- [ ] Create `tests/fixtures/policies.rs` with reusable configs
- [ ] Create `tests/helpers/assertions.rs` with custom assertions
- [ ] Refactor existing tests to use builders (50% migration)
- [ ] Create `tests/fixtures/data/` with YAML test files

**Acceptance Criteria**: All new tests use builders, 50% old tests migrated

---

### Phase 4: Add Edge Case Tests (Week 4)
**Priority**: 🟡 MEDIUM

- [ ] Create `tests/edge_cases/` directory
- [ ] Add boundary value tests (max/min values)
- [ ] Add empty/null string tests
- [ ] Add time edge case tests (overflow, underflow)
- [ ] Add concurrent edge case tests
- [ ] Add DoS prevention tests (very long strings)

**Acceptance Criteria**: 20+ edge case tests, no panics or undefined behavior

---

### Phase 5: Fix Property Tests & Add More (Week 5)
**Priority**: 🟢 LOW

- [ ] Fix `property_tests.rs` compilation errors
- [ ] Add `is_rate_limited()` method to RateLimitState
- [ ] Add determinism property tests
- [ ] Add monotonicity property tests
- [ ] Add commutativity tests (where applicable)

**Acceptance Criteria**: All property tests compile and pass

---

### Phase 6: Add CLI Integration Tests (Week 6)
**Priority**: 🟡 MEDIUM

- [ ] Create real subprocess CLI tests (not mock)
- [ ] Test full workflow: `init` → `generate` → `validate`
- [ ] Test error cases with invalid files
- [ ] Test output file creation
- [ ] Add CLI help text tests

**Acceptance Criteria**: CLI coverage >70%, all commands tested

---

### Phase 7: Add Benchmarks (Week 7)
**Priority**: 🟢 LOW

- [ ] Set up Criterion benchmark suite
- [ ] Benchmark policy evaluation (single, multiple policies)
- [ ] Benchmark pattern matching (exact, wildcard)
- [ ] Benchmark rate limiting (hit, miss)
- [ ] Benchmark spending tracking
- [ ] Add CI integration for performance regression

**Acceptance Criteria**: Baseline benchmarks established, no regressions

---

### Phase 8: Reorganize Tests (Week 8)
**Priority**: 🟢 LOW (but high impact on maintainability)

- [ ] Move 25% of inline tests to `tests/unit/`
- [ ] Keep inline tests for now (parallel migration)
- [ ] Add `#[deprecated]` to inline test modules
- [ ] Update documentation to point to new test location
- [ ] Remove inline tests in follow-up phase

**Acceptance Criteria**: Test organization matches new structure, no coverage loss

---

## 11. Success Metrics

### Coverage Goals
- **Unit Test Coverage**: 85% → 90%
- **Integration Test Coverage**: 0% → 70%
- **Mock Server Coverage**: 0% → 80%
- **CLI Coverage**: 20% → 70%

### Quality Goals
- **All Tests Passing**: 42/45 → 100% passing
- **Property Tests**: 0 running → 20+ running
- **Edge Cases**: ~10 → 50+ edge cases tested
- **Test Run Time**: <5s for unit, <30s for integration

### Maintainability Goals
- **Test Code Duplication**: High → Low (builders)
- **Test Discoverability**: Hard → Easy (organized structure)
- **Test Readability**: Medium → High (custom assertions)
- **Test Isolation**: Medium → High (fixtures, helpers)

---

## 12. Appendix: Test Examples

### A. RequestBuilder Example
```rust
// tests/helpers/builders.rs
pub struct RequestBuilder {
    agent_id: String,
    wallet_address: Option<String>,
    ip_address: Option<String>,
    amount: f64,
    endpoint: String,
    timestamp: SystemTime,
}

impl RequestBuilder {
    pub fn new(agent_id: &str) -> Self {
        Self {
            agent_id: agent_id.to_string(),
            wallet_address: None,
            ip_address: None,
            amount: 0.0,
            endpoint: "/".to_string(),
            timestamp: SystemTime::now(),
        }
    }

    pub fn with_wallet(mut self, address: &str) -> Self {
        self.wallet_address = Some(address.to_string());
        self
    }

    pub fn with_amount(mut self, amount: f64) -> Self {
        self.amount = amount;
        self
    }

    pub fn with_endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = endpoint.to_string();
        self
    }

    pub fn at_time(mut self, time: SystemTime) -> Self {
        self.timestamp = time;
        self
    }

    pub fn build(self) -> Request {
        Request {
            agent_id: self.agent_id,
            wallet_address: self.wallet_address,
            ip_address: self.ip_address,
            amount: self.amount,
            endpoint: self.endpoint,
            timestamp: self.timestamp,
        }
    }
}
```

### B. PolicyBuilder Example
```rust
pub struct PolicyBuilder {
    id: String,
    priority: i32,
    description: String,
    action: PolicyAction,
    agent_patterns: Vec<String>,
    endpoint_patterns: Vec<String>,
    rate_limit: Option<RateLimitConfig>,
    spending_cap: Option<SpendingCapConfig>,
}

impl PolicyBuilder {
    pub fn allow(id: &str) -> Self {
        Self {
            id: id.to_string(),
            priority: 0,
            description: "Test policy".to_string(),
            action: PolicyAction::Allow,
            agent_patterns: vec![],
            endpoint_patterns: vec![],
            rate_limit: None,
            spending_cap: None,
        }
    }

    pub fn deny(id: &str, reason: &str) -> Self {
        Self {
            action: PolicyAction::Deny(reason.to_string()),
            ..Self::allow(id)
        }
    }

    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    pub fn matching_agents(mut self, patterns: &[&str]) -> Self {
        self.agent_patterns = patterns.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn matching_endpoints(mut self, patterns: &[&str]) -> Self {
        self.endpoint_patterns = patterns.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_rate_limit(mut self, max_requests: u32, window: Duration) -> Self {
        self.rate_limit = Some(RateLimitConfig {
            max_requests,
            window,
        });
        self
    }

    pub fn with_spending_cap(mut self, max_amount: f64, window: Duration) -> Self {
        self.spending_cap = Some(SpendingCapConfig {
            max_amount,
            currency: "USDC".to_string(),
            window,
        });
        self
    }

    pub fn build(self) -> Policy {
        Policy {
            id: self.id,
            description: self.description,
            action: self.action,
            priority: self.priority,
            agent_patterns: self.agent_patterns,
            endpoint_patterns: self.endpoint_patterns,
            rate_limit: self.rate_limit,
            spending_cap: self.spending_cap,
        }
    }
}
```

---

## Summary

The x402 Protocol test suite requires focused attention on:

1. **Immediate**: Fix 3 failing tests (sliding window bugs)
2. **High Priority**: Add 490 lines of untested mock server integration tests
3. **Medium Priority**: Extract test helpers and builders to reduce duplication
4. **Long-term**: Reorganize tests into proper structure with clear separation

The roadmap provides an 8-week plan to achieve 90%+ coverage with high-quality, maintainable tests that focus on behavior over implementation, proper edge case handling, and comprehensive integration testing.
