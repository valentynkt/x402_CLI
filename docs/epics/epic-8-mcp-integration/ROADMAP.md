# Epic 8: Implementation Roadmap & Task Coordination Plan

**Status:** Ready for Execution
**Created:** 2025-11-12
**Duration:** 6 days (26 hours)
**Target:** x402-mcp-server v0.1.0 → crates.io

---

## Executive Summary

This roadmap coordinates the hive mind swarm to build a Rust MCP server with direct library integration for x402-dev testing toolkit. The implementation uses parallel workstreams, explicit dependencies, and validation gates to ensure high-quality delivery.

**Key Metrics:**
- 7 workflow tools (mock server, testing, policy)
- <1ms tool execution latency (P95)
- 80%+ test coverage
- Zero critical security vulnerabilities
- 6-day delivery timeline

---

## Hive Mind Agent Roles

### Primary Agents

| Agent | Primary Responsibility | Skills Required |
|-------|----------------------|-----------------|
| **Architect** | System design, Rust architecture, integration patterns | Rust, async/await, rmcp SDK |
| **Coder** | Core implementation, library integration, tool development | Rust, x402-core, tokio |
| **Tester** | Test suite creation, integration testing, benchmarking | cargo test, criterion |
| **Reviewer** | Code quality, security audit, performance validation | Rust best practices, security |
| **Researcher** | CLI analysis, API discovery, documentation | x402-dev internals |
| **Documenter** | README, API docs, examples, troubleshooting guides | Technical writing |

### Supporting Agents

| Agent | Support Role | Activation Trigger |
|-------|-------------|-------------------|
| **Performance Analyzer** | Benchmark analysis, optimization recommendations | When latency > 1ms |
| **Security Auditor** | Vulnerability scanning, dependency audits | Phase 4 (pre-publication) |
| **DevOps** | CI/CD setup, crates.io publication | Day 6 |

---

## Phase Breakdown

### Day 0: Pre-Implementation Refactoring (2 hours)

**CRITICAL BLOCKER:** Must complete before Phase 1 begins

#### Task D0.1: Refactor Test Command (2 hours)
**Owner:** Coder
**Dependencies:** None
**Urgency:** CRITICAL

**Problem:** `x402-cli/src/commands/test.rs:60` calls `std::process::exit()` directly, preventing library integration.

**Solution:**
1. Create new function `execute_with_result()` that returns `Result<TestResult>`
2. Update existing `execute()` to call new function and handle exit
3. Add unit tests for new function
4. Verify all call sites still work

**Success Gate:**
- ✅ Test command callable as library function (returns Result)
- ✅ All existing CLI tests pass
- ✅ No breaking changes to CLI interface

**Deliverable:** `x402-cli` ready for MCP integration

**Coordination:**
```bash
npx claude-flow@alpha hooks pre-task --description "Refactor test command for library integration"
# ... implementation work ...
npx claude-flow@alpha hooks post-task --task-id "D0.1"
```

---

### Phase 1: Foundation (Days 1-2, 8 hours)

**Goal:** Working Rust MCP server + 3 simple tools

#### Parallel Workstreams

**Workstream A: Rust Project Setup (Day 1 Morning, 2 hours)**

##### Task P1.A1: Initialize Rust Project
**Owner:** Architect
**Dependencies:** Day 0 complete
**Priority:** CRITICAL

**Actions:**
1. Create Rust project: `cargo new x402-mcp-server`
2. Configure Cargo.toml with dependencies:
   - rmcp = "0.8.5"
   - tokio = { version = "1.35", features = ["full"] }
   - serde, serde_json
   - x402-core, x402-server, x402-cli (path dependencies)
3. Create directory structure:
   ```
   src/
   ├── main.rs
   ├── tools/
   │   ├── mod.rs
   │   ├── mock.rs
   │   ├── testing.rs
   │   ├── policy.rs
   │   └── config.rs
   ├── error.rs
   └── lib.rs (optional)
   tests/
   benches/
   ```

**Success Gate:**
- ✅ `cargo build` succeeds
- ✅ `cargo test` runs (no tests yet)
- ✅ All dependencies resolve

##### Task P1.A2: Implement stdio Transport
**Owner:** Coder
**Dependencies:** P1.A1
**Priority:** CRITICAL

**Implementation:**
```rust
// src/main.rs
use rmcp::{Server, ServerBuilder};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!("x402-mcp-server starting...");

    let server = ServerBuilder::new("x402-dev-mcp")
        .version("0.1.0")
        .description("MCP server for x402-dev protocol testing")
        .build();

    tools::register_all(&server).await?;
    server.serve_stdio().await?;
    Ok(())
}
```

**Success Gate:**
- ✅ stdio transport responds to MCP handshake
- ✅ Server doesn't pollute stdout (logs to stderr only)

---

**Workstream B: First Tools Implementation (Day 1 Afternoon, 3 hours)**

##### Task P1.B1: Implement server_mock_start
**Owner:** Coder
**Dependencies:** P1.A2
**Priority:** HIGH

**Implementation Pattern:**
```rust
// src/tools/mock.rs
use rmcp::{tool, Server, CallToolResult, TextContent};
use x402_server::{start_server, MockServerConfig};

#[tool(
    name = "x402__server_mock_start",
    description = "Start mock payment server for testing",
)]
pub async fn server_mock_start(
    #[arg(default = 3402)] port: u16,
    #[arg(default = 0.01)] pricing: f64,
) -> anyhow::Result<CallToolResult> {
    // Direct library call (0ms overhead)
    let config = MockServerConfig { port, pricing, ..Default::default() };
    start_server(config).await?;

    Ok(CallToolResult {
        isError: false,
        content: vec![TextContent {
            type_: "text".to_string(),
            text: json!({
                "status": "started",
                "pid": std::process::id(),
                "port": port,
                "server_url": format!("http://localhost:{}", port)
            }).to_string()
        }]
    })
}
```

**Success Gate:**
- ✅ Tool callable from Claude Code
- ✅ Server starts successfully
- ✅ Sub-millisecond latency confirmed

##### Task P1.B2: Implement policy_validate
**Owner:** Coder
**Dependencies:** P1.A2
**Priority:** HIGH

**Success Gate:**
- ✅ YAML validation works (in-memory, no temp files)
- ✅ Error translation to MCP format

##### Task P1.B3: Implement config_show
**Owner:** Coder
**Dependencies:** P1.A2
**Priority:** MEDIUM

**Success Gate:**
- ✅ Configuration merging works
- ✅ Structured JSON response

---

**Workstream C: Error Translation Layer (Day 2 Morning, 2 hours)**

##### Task P1.C1: Create Error Translation
**Owner:** Architect + Coder
**Dependencies:** P1.B1-P1.B3
**Priority:** HIGH

**Implementation:**
```rust
// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum McpError {
    #[error("Port {port} is already in use")]
    PortInUse { port: u16 },

    #[error("Invalid port number: {port}")]
    InvalidPort { port: u16 },

    // ... more error types
}

impl From<McpError> for CallToolResult {
    fn from(err: McpError) -> Self {
        // Map to structured MCP errors with codes, suggestions, docs links
    }
}
```

**Success Gate:**
- ✅ All x402-core errors translate to MCP format
- ✅ Error codes match API-REFERENCE.md (E3001-E5003)
- ✅ Actionable suggestions included

---

**Workstream D: Testing & Validation (Day 2 Afternoon, 1 hour)**

##### Task P1.D1: Integration Test Suite
**Owner:** Tester
**Dependencies:** P1.C1
**Priority:** HIGH

**Test Coverage:**
1. stdio transport handshake
2. First 3 tools callable end-to-end
3. Error handling paths
4. Parameter validation

**Success Gate:**
- ✅ 50%+ test coverage
- ✅ All 3 tools pass integration tests
- ✅ Error paths tested

##### Task P1.D2: Performance Benchmark
**Owner:** Performance Analyzer
**Dependencies:** P1.B1-P1.B3
**Priority:** MEDIUM

**Benchmark Tools:**
- `criterion` benchmarks
- Latency P50, P95, P99 measurements

**Success Gate:**
- ✅ P95 latency < 1ms for all 3 tools
- ✅ Baseline metrics documented

---

### Phase 1 Success Gate (End of Day 2)

**Validation Checklist:**
- [ ] 3 tools working (mock_start, policy_validate, config_show)
- [ ] stdio transport stable
- [ ] <1ms latency (P95)
- [ ] 50%+ test coverage
- [ ] Direct library integration confirmed (no subprocess)

**Decision Point:**
- ✅ All gates passed → **GO** to Phase 2
- ❌ Critical blocker → Extend by 0.5 day or reevaluate approach

---

### Phase 2: Core Tools (Days 3-4, 8 hours)

**Goal:** All 7 tools functional with comprehensive error handling

#### Parallel Workstreams

**Workstream E: Remaining Mock Tools (Day 3 Morning, 2 hours)**

##### Task P2.E1: Implement server_mock_stop
**Owner:** Coder
**Dependencies:** Phase 1 complete
**Priority:** HIGH

**Success Gate:**
- ✅ Graceful shutdown works
- ✅ Error handling for "not running" case

##### Task P2.E2: Implement server_mock_status
**Owner:** Coder
**Dependencies:** Phase 1 complete
**Priority:** MEDIUM

**Success Gate:**
- ✅ Status check works for running/not-running states
- ✅ Uptime and config info included

---

**Workstream F: Testing Tools (Day 3 Afternoon, 3 hours)**

##### Task P2.F1: Implement testing_run_suite
**Owner:** Coder
**Dependencies:** Phase 1 complete
**Priority:** HIGH

**Key Features:**
- In-memory YAML parsing (no temp files)
- Direct call to `x402_core::testing::execute_test_suite`
- Structured JSON output (not text parsing)

**Success Gate:**
- ✅ Test suite execution works
- ✅ JUnit XML output format supported
- ✅ Test result aggregation correct

##### Task P2.F2: Implement testing_check_compliance
**Owner:** Coder
**Dependencies:** Phase 1 complete
**Priority:** HIGH

**Success Gate:**
- ✅ HTTP 402 validation works
- ✅ Timeout enforcement (default: 10s)
- ✅ Compliance checks structured

---

**Workstream G: Policy Tools (Day 4 Morning, 2 hours)**

##### Task P2.G1: Implement policy_generate_express
**Owner:** Coder
**Dependencies:** P1.B2 (policy_validate)
**Priority:** MEDIUM

**Implementation:**
```rust
// Validate policy first, then generate code
let policy = Policy::from_yaml_str(&policy_yaml)?;
let validation = validate_policies(&policy_yaml)?;

if !validation.is_valid {
    return error_response("E5002", "Cannot generate from invalid policy");
}

let code = generate_middleware(&policy, Framework::Express, filename)?;
```

**Success Gate:**
- ✅ Express middleware generation works
- ✅ Generated code is valid JavaScript
- ✅ Pre-validation prevents bad code generation

---

**Workstream H: Integration Testing (Day 4 Afternoon, 1 hour)**

##### Task P2.H1: End-to-End Workflow Tests
**Owner:** Tester
**Dependencies:** All 7 tools implemented
**Priority:** CRITICAL

**Test Scenarios:**
1. Complete workflow: start → status → validate → test → stop
2. Error path: start → start again (port in use)
3. Error path: stop when not running
4. Error path: invalid YAML validation

**Success Gate:**
- ✅ 60%+ test coverage
- ✅ All 7 tools covered
- ✅ Error paths tested

##### Task P2.H2: Performance Benchmarking
**Owner:** Performance Analyzer
**Dependencies:** All 7 tools implemented
**Priority:** HIGH

**Benchmarks:**
- All 7 tools latency measurements
- Memory usage profiling
- Comparison vs TypeScript approach (should be 10-1000x faster)

**Success Gate:**
- ✅ All tools < 1ms P95 latency
- ✅ No memory leaks detected
- ✅ Performance report generated

---

### Phase 2 Success Gate (End of Day 4)

**Validation Checklist:**
- [ ] All 7 tools functional
- [ ] 60%+ test coverage
- [ ] <1ms P95 latency for all tools
- [ ] Integration tests pass
- [ ] No critical bugs

**Decision Point:**
- ✅ All gates passed → **GO** to Phase 3
- ❌ Quality below threshold → Extend by 0.5 day or descope to 5 tools

---

### Phase 3: Testing & Documentation (Day 5, 6 hours)

**Goal:** Production-ready quality with comprehensive tests and documentation

#### Parallel Workstreams

**Workstream I: Test Coverage & Quality (Day 5 Morning, 3 hours)**

##### Task P3.I1: Increase Test Coverage
**Owner:** Tester
**Dependencies:** Phase 2 complete
**Priority:** HIGH

**Coverage Targets:**
- Unit tests: 85%+
- Integration tests: 75%+
- Error paths: 90%+

**Test Types:**
1. Unit tests for error translation
2. Property-based tests (if applicable)
3. Edge case tests (invalid inputs, timeouts)

**Success Gate:**
- ✅ 80%+ overall test coverage
- ✅ All error codes tested
- ✅ `cargo tarpaulin` or `cargo-llvm-cov` report generated

##### Task P3.I2: Performance Optimization
**Owner:** Performance Analyzer
**Dependencies:** P2.H2
**Priority:** MEDIUM

**Optimization Targets:**
- Identify any tool > 1ms P95 latency
- Profile with `cargo flamegraph`
- Optimize hot paths

**Success Gate:**
- ✅ All tools meet <1ms target
- ✅ No unnecessary allocations
- ✅ Async operations optimized

---

**Workstream J: Documentation (Day 5 Afternoon, 3 hours)**

##### Task P3.J1: Create README.md
**Owner:** Documenter
**Dependencies:** Phase 2 complete
**Priority:** HIGH

**Sections:**
1. Installation (cargo install + claude mcp add)
2. Quick Start (90-second demo)
3. Features (7 tools, <1ms latency, zero subprocess overhead)
4. Architecture (direct library integration diagram)
5. Examples (3-5 common workflows)
6. Troubleshooting (link to IMPLEMENTATION-GUIDE.md)

**Success Gate:**
- ✅ New user can install and run first tool in 2 minutes
- ✅ README is clear and compelling

##### Task P3.J2: Generate API Documentation
**Owner:** Documenter
**Dependencies:** Phase 2 complete
**Priority:** MEDIUM

**Actions:**
1. Run `cargo doc --no-deps --open`
2. Add doc comments to all public functions
3. Add examples to doc comments
4. Verify docs.rs will build correctly

**Success Gate:**
- ✅ All public APIs documented
- ✅ Examples compile and run
- ✅ No broken links

##### Task P3.J3: Create Example Workflows
**Owner:** Documenter + Coder
**Dependencies:** Phase 2 complete
**Priority:** MEDIUM

**Examples:**
1. `examples/payment_workflow.rs` - Complete end-to-end workflow
2. `examples/policy_validation.rs` - Policy validation + code generation
3. `examples/ci_integration.rs` - CI/CD test suite execution

**Success Gate:**
- ✅ All examples run successfully
- ✅ Examples demonstrate key features
- ✅ Clear comments explain each step

---

### Phase 3 Success Gate (End of Day 5)

**Validation Checklist:**
- [ ] 80%+ test coverage achieved
- [ ] Performance benchmarks pass (<1ms P95)
- [ ] README complete and tested
- [ ] API documentation complete
- [ ] Example workflows functional

**Decision Point:**
- ✅ All gates passed → **GO** to Phase 4
- ❌ Critical gap → Extend by 0.5 day

---

### Phase 4: Production Release (Day 6, 4 hours)

**Goal:** Published to crates.io + MCP directory listed

#### Sequential Workflow (with gates)

##### Task P4.1: Security Audit
**Owner:** Security Auditor + Reviewer
**Dependencies:** Phase 3 complete
**Priority:** CRITICAL

**Security Checks:**
1. Run `cargo audit` (zero critical vulnerabilities)
2. Check for unsafe code: `rg "unsafe " src/`
3. Run clippy with pedantic lints: `cargo clippy -- -W clippy::all -W clippy::pedantic`
4. Verify no hardcoded secrets
5. Input validation audit (all tool parameters)
6. Error message security (no sensitive data leaks)

**Success Gate:**
- ✅ Zero critical vulnerabilities
- ✅ Zero unsafe blocks (unless justified)
- ✅ All clippy warnings resolved
- ✅ Security checklist complete

**Decision Point:**
- ✅ Security passed → **GO** to P4.2
- ❌ Critical issue → Fix immediately, delay publication

---

##### Task P4.2: Prepare for Publication
**Owner:** DevOps + Lead Developer
**Dependencies:** P4.1 passed
**Priority:** CRITICAL

**Actions:**
1. Update Cargo.toml metadata:
   ```toml
   [package]
   name = "x402-mcp-server"
   version = "0.1.0"
   edition = "2024"
   rust-version = "1.85.0"
   authors = ["x402-dev Team <dev@x402.io>"]
   license = "MIT OR Apache-2.0"
   description = "Rust MCP server for x402-dev payment protocol testing toolkit with direct library integration"
   repository = "https://github.com/x402-dev/x402-mcp-server"
   keywords = ["mcp", "x402", "testing", "solana", "payment"]
   categories = ["development-tools", "testing"]
   ```

2. Verify package contents: `cargo package --list`
3. Dry run: `cargo publish --dry-run`
4. Review generated package

**Success Gate:**
- ✅ Dry run succeeds
- ✅ Package size reasonable (<10MB)
- ✅ All necessary files included
- ✅ No unnecessary files (tests excluded from package)

---

##### Task P4.3: Publish to crates.io
**Owner:** DevOps
**Dependencies:** P4.2 passed
**Priority:** CRITICAL

**Actions:**
1. Actual publish: `cargo publish`
2. Verify on crates.io: https://crates.io/crates/x402-mcp-server
3. Wait for docs.rs build: https://docs.rs/x402-mcp-server
4. Test installation: `cargo install x402-mcp-server`

**Success Gate:**
- ✅ Crate published successfully
- ✅ docs.rs build succeeded
- ✅ Installation works from crates.io

---

##### Task P4.4: MCP Directory Submission
**Owner:** DevOps + Documenter
**Dependencies:** P4.3 passed
**Priority:** HIGH

**Actions:**
1. Create `mcp-directory.json`:
   ```json
   {
     "name": "x402-mcp",
     "vendor": "x402-dev",
     "description": "Rust MCP server for x402 payment protocol testing with <1ms latency",
     "version": "0.1.0",
     "install_command": "cargo install x402-mcp-server",
     "integration_command": "claude mcp add x402-mcp x402-mcp-server",
     "tools": [
       "x402__server_mock_start",
       "x402__server_mock_stop",
       "x402__server_mock_status",
       "x402__testing_run_suite",
       "x402__testing_check_compliance",
       "x402__policy_validate",
       "x402__policy_generate_express"
     ]
   }
   ```

2. Submit to MCP directory (mcpcat.io or official MCP registry)
3. Verify listing appears

**Success Gate:**
- ✅ MCP directory listing live
- ✅ Installation instructions correct

---

##### Task P4.5: Community Announcement
**Owner:** Documenter + DevRel
**Dependencies:** P4.3 + P4.4 passed
**Priority:** MEDIUM

**Announcement Channels:**
1. r/ClaudeAI (Reddit) - with demo video/GIF
2. r/solana (Reddit) - emphasize hackathon use case
3. x402-dev Discord/Slack - community announcement
4. Twitter/X - with screenshot and link
5. Solana Hackathon Discord (Sept-Oct 2025) - developer toolkit showcase

**Announcement Template:**
```
🚀 Introducing x402-mcp-server: Rust MCP Testing Toolkit

Built with Rust for 10-1000x faster performance (<1ms vs 50-200ms)

✅ 7 workflow tools for payment API testing
✅ Direct library integration (zero subprocess overhead)
✅ Type-safe with compile-time guarantees
✅ Perfect for AI-assisted development

Install: cargo install x402-mcp-server
Docs: https://docs.rs/x402-mcp-server
GitHub: https://github.com/x402-dev/x402-mcp-server

#Rust #MCP #Solana #x402 #AI
```

**Success Gate:**
- ✅ Announcements posted to all channels
- ✅ Community engagement started

---

### Phase 4 Success Gate (End of Day 6)

**Validation Checklist:**
- [ ] Security audit passed (0 critical vulnerabilities)
- [ ] Published to crates.io successfully
- [ ] MCP directory listing live
- [ ] Community announcements posted
- [ ] Documentation live on docs.rs

**Final Decision:**
- ✅ All gates passed → **PROJECT COMPLETE**
- ❌ Critical issue → Delay announcement until fixed

---

## Critical Path Analysis

### Dependencies Graph

```
Day 0 (Refactor)
    ↓
Day 1-2 (Phase 1: Foundation)
├── A1: Rust project setup
│   ↓
├── A2: stdio transport ──→ B1: server_mock_start
│   ↓                       ↓
├── B2: policy_validate ←───┘
│   ↓
├── B3: config_show
│   ↓
├── C1: Error translation layer
│   ↓
├── D1: Integration tests
│   ↓
└── D2: Performance benchmarks
    ↓
Day 3-4 (Phase 2: Core Tools)
├── E1: server_mock_stop
├── E2: server_mock_status
├── F1: testing_run_suite
├── F2: testing_check_compliance
├── G1: policy_generate_express
│   ↓
├── H1: End-to-end workflow tests
│   ↓
└── H2: Performance benchmarking
    ↓
Day 5 (Phase 3: Polish)
├── I1: Increase test coverage (parallel)
├── I2: Performance optimization (parallel)
├── J1: README.md (parallel)
├── J2: API documentation (parallel)
└── J3: Example workflows (parallel)
    ↓
Day 6 (Phase 4: Production)
├── P4.1: Security audit ──→ GATE
│   ↓
├── P4.2: Prepare for publication ──→ GATE
│   ↓
├── P4.3: Publish to crates.io ──→ GATE
│   ↓
├── P4.4: MCP directory submission
│   ↓
└── P4.5: Community announcement
```

### Critical Path Items (Sequential)

1. **Day 0:** Refactor test command (BLOCKER for Phase 1)
2. **Day 1:** Rust project + stdio transport (BLOCKER for all tools)
3. **Day 1-2:** First 3 tools + error translation (BLOCKER for Phase 2)
4. **Day 3-4:** Remaining 4 tools (BLOCKER for Phase 3)
5. **Day 5:** Tests + docs (BLOCKER for Phase 4)
6. **Day 6:** Security audit → Publication (sequential gates)

### Parallel Opportunities

**Day 1-2 (Phase 1):**
- B1, B2, B3 (3 tools) can be implemented in parallel after A2 completes
- D1 and D2 can run in parallel once tools are ready

**Day 3-4 (Phase 2):**
- E1, E2, F1, F2, G1 (5 tools) can be implemented in parallel
- H1 and H2 depend on all tools being complete

**Day 5 (Phase 3):**
- I1, I2, J1, J2, J3 can all run in parallel (5 parallel workstreams)

**Day 6 (Phase 4):**
- Sequential only (security gates)

---

## Resource Allocation

### Agent Assignment Matrix

| Phase | Task | Primary Agent | Support Agent | Estimated Hours |
|-------|------|--------------|---------------|-----------------|
| Day 0 | D0.1: Refactor test command | Coder | Reviewer | 2h |
| Day 1-2 | P1.A1: Rust project setup | Architect | - | 1h |
| Day 1-2 | P1.A2: stdio transport | Coder | Architect | 1h |
| Day 1-2 | P1.B1: server_mock_start | Coder | - | 1h |
| Day 1-2 | P1.B2: policy_validate | Coder | - | 1h |
| Day 1-2 | P1.B3: config_show | Coder | - | 0.5h |
| Day 1-2 | P1.C1: Error translation | Architect + Coder | - | 2h |
| Day 1-2 | P1.D1: Integration tests | Tester | Coder | 1h |
| Day 1-2 | P1.D2: Benchmarks | Performance Analyzer | - | 0.5h |
| Day 3-4 | P2.E1: server_mock_stop | Coder | - | 1h |
| Day 3-4 | P2.E2: server_mock_status | Coder | - | 0.5h |
| Day 3-4 | P2.F1: testing_run_suite | Coder | Researcher | 1.5h |
| Day 3-4 | P2.F2: testing_check_compliance | Coder | - | 1.5h |
| Day 3-4 | P2.G1: policy_generate_express | Coder | - | 1h |
| Day 3-4 | P2.H1: End-to-end tests | Tester | Coder | 1.5h |
| Day 3-4 | P2.H2: Performance benchmarking | Performance Analyzer | - | 1h |
| Day 5 | P3.I1: Increase test coverage | Tester | Coder | 2h |
| Day 5 | P3.I2: Performance optimization | Performance Analyzer | Coder | 1h |
| Day 5 | P3.J1: README.md | Documenter | - | 1.5h |
| Day 5 | P3.J2: API documentation | Documenter | - | 1h |
| Day 5 | P3.J3: Example workflows | Documenter + Coder | - | 1.5h |
| Day 6 | P4.1: Security audit | Security Auditor + Reviewer | - | 1.5h |
| Day 6 | P4.2: Prepare for publication | DevOps + Lead Developer | - | 0.5h |
| Day 6 | P4.3: Publish to crates.io | DevOps | - | 0.5h |
| Day 6 | P4.4: MCP directory | DevOps + Documenter | - | 1h |
| Day 6 | P4.5: Community announcement | Documenter + DevRel | - | 0.5h |

**Total Estimated Hours:** 28 hours (26 hours + 2 hours Day 0)

---

## Risk Buffers & Contingency

### Time Buffers

| Phase | Planned Duration | Buffer | Total |
|-------|-----------------|--------|-------|
| Day 0 | 2h | 0.5h | 2.5h |
| Phase 1 (Days 1-2) | 8h | 1h | 9h |
| Phase 2 (Days 3-4) | 8h | 1h | 9h |
| Phase 3 (Day 5) | 6h | 1h | 7h |
| Phase 4 (Day 6) | 4h | 0.5h | 4.5h |

**Total with Buffers:** 32 hours (vs 28 hours planned)

### Decision Points & Escalation

**Phase 1 Gate (End of Day 2):**
- ✅ Pass: Continue to Phase 2 on schedule
- ⚠️ Minor issues: Use 1h buffer to fix
- ❌ Critical blocker: Escalate to CTO, extend by 0.5 day

**Phase 2 Gate (End of Day 4):**
- ✅ Pass: Continue to Phase 3 on schedule
- ⚠️ Quality below 60% coverage: Use 1h buffer to improve tests
- ❌ Tool failures: Descope to 5 tools or extend by 0.5 day

**Phase 3 Gate (End of Day 5):**
- ✅ Pass: Continue to Phase 4 on schedule
- ⚠️ Coverage 70-79%: Acceptable with plan to improve post-launch
- ❌ Coverage <70% or docs incomplete: Extend by 0.5 day

**Phase 4 Gate (Security Audit):**
- ✅ Pass: Publish to crates.io
- ❌ Critical vulnerability: Fix immediately (1-4h delay), re-audit

---

## Coordination Protocol

### Pre-Task Setup
Every agent MUST run before starting work:
```bash
npx claude-flow@alpha hooks pre-task --description "[task description]"
npx claude-flow@alpha hooks session-restore --session-id "epic8-hive"
```

### During Work
Report progress:
```bash
npx claude-flow@alpha hooks notify --message "[progress update]"
npx claude-flow@alpha hooks post-edit --file "[file]" --memory-key "epic8/[agent]/[task]"
```

### Post-Task Completion
Mark task complete:
```bash
npx claude-flow@alpha hooks post-task --task-id "[task-id]"
npx claude-flow@alpha hooks session-end --export-metrics true
```

### Memory Coordination

**Store task results:**
```bash
npx claude-flow@alpha memory store epic8/[agent]/[task]/result "[result-json]"
```

**Retrieve dependencies:**
```bash
npx claude-flow@alpha memory get epic8/[agent]/[task]/result
```

**Memory Namespaces:**
- `epic8/planning/roadmap` - This roadmap document
- `epic8/coder/tool-[name]` - Tool implementation status
- `epic8/tester/coverage` - Test coverage metrics
- `epic8/performance/benchmarks` - Performance data
- `epic8/security/audit` - Security audit results

---

## Success Metrics & Validation

### Technical KPIs

| Metric | Target | Measurement | Validation Gate |
|--------|--------|-------------|-----------------|
| Latency (P95) | <1ms | criterion benchmarks | Phase 1, 2, 3 |
| Test Coverage | 80%+ | cargo tarpaulin | Phase 3 |
| Tool Success Rate | >95% | Integration tests | Phase 2, 3 |
| Security Vulnerabilities | 0 critical | cargo audit | Phase 4 |
| Build Time | <2 min | cargo build --release | Phase 3 |

### User Experience KPIs

| Metric | Target | Measurement |
|--------|--------|-------------|
| Installation Time | <2 min | Manual testing |
| First Tool Execution | <30 sec | From installation to first success |
| Error Rate | <5% | Integration test failure rate |
| Documentation Clarity | New user success in 5 min | User testing (post-launch) |

### Ecosystem KPIs

| Milestone | Target Date |
|-----------|-------------|
| crates.io Published | Day 6 |
| MCP Directory Listed | Day 6 |
| Solana Hackathon Showcase | Day 6 |
| GitHub Stars | 3+ in Week 1 |
| Downloads | 50+ in Week 1 |

---

## Communication Plan

### Daily Standup (Async)

**Format:** Post in coordination channel
- What did I complete yesterday?
- What am I working on today?
- Any blockers?

**Timing:** End of each work session

### Phase Gates (Synchronous)

**Format:** Quick sync (15 minutes)
- Review gate checklist
- Discuss any issues
- Go/No-Go decision

**Timing:** End of Phase 1, 2, 3, 4

### Escalation Path

1. **Blocker identified** → Agent notifies in coordination channel (0 hours)
2. **Can't resolve in 1 hour** → Escalate to Tech Lead (immediate)
3. **Impacts timeline** → Tech Lead → CTO (24 hours)
4. **Strategic decision needed** → CTO final call (48 hours)

---

## Next Steps for Hive Mind

### Immediate Actions (Today)

1. **Planner Agent (YOU):**
   - ✅ Share this roadmap with hive mind
   - [ ] Get stakeholder approval
   - [ ] Assign agents to tasks

2. **Architect Agent:**
   - [ ] Review Day 0 refactoring requirements
   - [ ] Prepare Rust project structure design
   - [ ] Research rmcp SDK patterns

3. **Coder Agent:**
   - [ ] Clone x402-dev repository
   - [ ] Set up Rust development environment (Rust 1.85.0+)
   - [ ] Review IMPLEMENTATION-GUIDE.md

4. **Tester Agent:**
   - [ ] Set up testing environment (cargo test, criterion)
   - [ ] Prepare test data fixtures
   - [ ] Review integration test patterns

5. **Researcher Agent:**
   - [ ] Analyze x402-cli commands for integration points
   - [ ] Document any edge cases or special handling needed
   - [ ] Prepare API mapping document

### Day 1 Kickoff (Tomorrow)

**Morning:**
- All agents: Read EPIC-8-OVERVIEW.md, API-REFERENCE.md, IMPLEMENTATION-GUIDE.md
- Architect: Start P1.A1 (Rust project setup)
- Coder: Prepare to implement P1.A2 (stdio transport)

**Afternoon:**
- Coder: Implement first 3 tools in parallel
- Tester: Start planning integration tests

**Evening:**
- Phase 1 progress check (50% complete expected)

---

## Appendix A: Tool Implementation Checklist

### For Each Tool:

- [ ] Define parameter schema (with defaults)
- [ ] Implement direct library integration (no subprocess)
- [ ] Add error translation (x402-core::Error → MCP JSON)
- [ ] Write unit tests (parameter validation, error paths)
- [ ] Write integration test (end-to-end)
- [ ] Add performance benchmark
- [ ] Document in rustdoc comments
- [ ] Update API-REFERENCE.md (if needed)

### Tool-Specific Notes:

**server_mock_start:**
- Special handling: Port validation (1024-65535)
- Error: E3001 (port in use), E3002 (invalid port)

**testing_run_suite:**
- Special handling: YAML parsing (in-memory)
- Error: E4001 (invalid YAML), E4002 (test execution failed)

**policy_generate_express:**
- Special handling: Pre-validate before code generation
- Error: E5002 (cannot generate from invalid policy)

---

## Appendix B: Coordination Timeline (Gantt Chart)

```
Day 0: Refactoring
[===================] D0.1: Refactor test command (2h)

Days 1-2: Phase 1 Foundation
[====] P1.A1: Rust project setup (1h)
  [====] P1.A2: stdio transport (1h)
    [====] P1.B1: server_mock_start (1h)
    [====] P1.B2: policy_validate (1h)
    [===] P1.B3: config_show (0.5h)
      [=========] P1.C1: Error translation (2h)
        [====] P1.D1: Integration tests (1h)
        [===] P1.D2: Benchmarks (0.5h)

Days 3-4: Phase 2 Core Tools
[====] P2.E1: server_mock_stop (1h)
[===] P2.E2: server_mock_status (0.5h)
[======] P2.F1: testing_run_suite (1.5h)
[======] P2.F2: testing_check_compliance (1.5h)
[====] P2.G1: policy_generate_express (1h)
  [======] P2.H1: End-to-end tests (1.5h)
  [====] P2.H2: Performance benchmarking (1h)

Day 5: Phase 3 Polish
[=========] P3.I1: Test coverage (2h)
[====] P3.I2: Performance optimization (1h)
[======] P3.J1: README.md (1.5h)
[====] P3.J2: API documentation (1h)
[======] P3.J3: Example workflows (1.5h)

Day 6: Phase 4 Production
[======] P4.1: Security audit (1.5h)
  [===] P4.2: Prepare publication (0.5h)
    [===] P4.3: Publish to crates.io (0.5h)
      [====] P4.4: MCP directory (1h)
        [===] P4.5: Community announcement (0.5h)
```

**Legend:**
- `[===]` = Work in progress
- Sequential tasks: Indented
- Parallel tasks: Same indentation level

---

**Document Status:** ✅ Ready for Execution
**Next Review:** After Phase 1 completion (End of Day 2)
**Owner:** Planner Agent → Tech Lead

For questions or feedback, coordinate through hive mind memory system or escalate to Tech Lead.
