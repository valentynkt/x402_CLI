# x402-dev - Architecture Document

**Author:** Valik
**Date:** 2025-11-09
**Version:** 1.0
**Project Level:** Level 2 (Medium Complexity)

---

## Executive Summary

x402-dev is a **pure Rust CLI toolkit** for x402 protocol development on Solana. The architecture emphasizes:

- **Simplicity** (KISS principle - single language, minimal dependencies)
- **Performance** (Rust type safety, compiled binary, small footprint)
- **Single binary distribution** (~2-3MB pure Rust)

**Key Differentiation:** First comprehensive x402 dev tool built with pure Rust for maximum simplicity and performance.

**Performance Targets:**
- Binary size: 2-3MB (pure Rust, no embedded runtime)
- Command execution: <1 second (excluding network calls)
- Mock server startup: <2 seconds
- Build time: ~15 seconds (vs ~45 seconds with TypeScript bundling)

---

## Project Initialization

**Workspace Structure:**
```bash
# Initialize workspace with 3 crates
cargo new --bin x402-dev
cd x402-dev
mkdir -p crates/{x402-cli,x402-core,xtask} ts

# Create workspace Cargo.toml
# See "Complete Project Structure" section below
```

**First Implementation Story:** Epic 1, Story 1.1 - Project scaffolding with Rust workspace (pure Rust, no TypeScript)

---

## Decision Summary

| Category | Decision | Version | Affects Epics | Rationale |
|----------|----------|---------|---------------|-----------|
| **Language** | Rust | 1.75+ | All | Performance, type safety, small binary (KISS) |
| **CLI Framework** | Clap | 4.5 | Epic 1, 2-6 | Industry standard, derive macros, excellent DX |
| **Async Runtime** | tokio | 1.48 (`rt-multi-thread`) | All | Full async capabilities, no V8 constraints |
| **Error Handling** | anyhow | 1.0 | All | Ergonomic error propagation with context |
| **HTTP Server** | actix-web | 4.9 | Epic 2 | Pure Rust, mature framework, simple integration |
| **Configuration** | serde_yaml + directories | - | Epic 1 | Multi-tier config (CLI > env > file > defaults) |
| **HTTP Client** | reqwest | 0.12 | Epic 4 | Async HTTP for verification |
| **Solana SDK** | solana-client | 2.0 | Epic 4 | RPC queries (optional feature) |
| **x402 Protocol** | Manual implementation | - | Epic 2 | Simple invoice generation, full control |
| **Build Tool** | cargo | - | All | Standard Rust build system |
| **Distribution** | GitHub releases + cargo | - | Epic 6 | Binary downloads, cargo install |

---

## Technology Stack Details

### Core Technologies

**Rust Crates:**
```toml
[workspace.dependencies]
# CLI Framework
clap = { version = "4.5", features = ["derive", "color", "suggestions"] }
anyhow = "1.0"

# Async Runtime
tokio = { version = "1.48", features = ["rt-multi-thread", "macros"] }

# HTTP Server
actix-web = "4.9"
actix-cors = "0.7"

# HTTP Client
reqwest = { version = "0.12", features = ["json"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"

# Configuration
directories = "5.0"

# Solana (optional)
solana-client = "2.0"
bs58 = "0.5"  # Base58 encoding for addresses
```

### Build Configuration

**Cargo.toml (Release Profile):**
```toml
[profile.release]
opt-level = "z"       # Optimize for size (not speed)
lto = "fat"           # Link-time optimization
codegen-units = 1     # Single codegen unit for better optimization
strip = "symbols"     # Strip debug symbols
panic = "abort"       # No unwinding (smaller binary)
```

**Result:** ~2-3MB binary (pure Rust, optimized for size)

---

## Complete Project Structure

```
x402-dev/
├── Cargo.toml                            # Workspace manifest
├── Cargo.lock                            # Lockfile (committed)
├── .cargo/
│   └── config.toml                       # Build config
│
├── crates/
│   ├── x402-cli/                         # BINARY CRATE
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs                   # Entry point
│   │       ├── cli.rs                    # Clap CLI definition
│   │       └── commands/                 # Epic 1-6 commands
│   │           ├── mod.rs
│   │           ├── init.rs               # Epic 1
│   │           ├── version.rs            # Epic 1
│   │           ├── mock.rs               # Epic 2
│   │           ├── test.rs               # Epic 3
│   │           ├── verify.rs             # Epic 4
│   │           ├── check.rs              # Epic 4
│   │           ├── monitor.rs            # Epic 4
│   │           ├── policy.rs             # Epic 5
│   │           ├── examples.rs           # Epic 6
│   │           └── doctor.rs             # Epic 6
│   │
│   ├── x402-core/                        # LIBRARY CRATE
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── errors.rs
│   │       ├── config/                   # Epic 1: Configuration
│   │       ├── protocol/                 # Epic 2, 4: x402 protocol
│   │       ├── server/                   # Epic 2: Mock server (actix-web)
│   │       ├── test/                     # Epic 3: Test runner
│   │       ├── verify/                   # Epic 4: Validation
│   │       ├── monitor/                  # Epic 4: Monitoring
│   │       ├── policy/                   # Epic 5: Policy engine
│   │       │   ├── rules/
│   │       │   └── codegen/
│   │       ├── examples/                 # Epic 6: Example library
│   │       ├── doctor/                   # Epic 6: Diagnostics
│   │       └── solana/                   # Epic 4: Solana (optional)
│   │
│   └── xtask/                            # BUILD AUTOMATION (optional)
│       └── src/main.rs                   # CI tasks, release
│
├── tests/                                # Integration tests
├── examples/                             # User-facing examples (Epic 6)
│   ├── mcp-server-starter/
│   ├── ai-agent-policies/
│   └── cicd-testing/
│
└── docs/
    ├── architecture.md                   # This document
    ├── PRD.md
    └── epics.md
```

---

## Epic to Architecture Mapping

**Note:** Epics align with PRD structure (FR-1 through FR-11)

| Epic | PRD FRs | Rust Modules | Key Files |
|------|---------|--------------|-----------|
| **1: Mock Server Infrastructure** | FR-1 | `x402-core/src/{server, protocol}` | `server/manager.rs`, `server/routes.rs` |
|  |  | `x402-cli/src/commands/mock.rs` | CLI command |
| **2: Automated Testing Framework** | FR-2 | `x402-core/src/test/` | `test/runner.rs`, `test/assertions.rs` |
|  |  | `x402-cli/src/commands/test.rs` | CLI command |
| **3: Validation & Verification Tools** | FR-3, FR-4 | `x402-core/src/{verify, monitor, solana}` | `verify/headers.rs`, `monitor/tail.rs` |
|  |  | `x402-cli/src/commands/{verify, check, monitor}.rs` | CLI commands |
| **4: Policy Enforcement Engine** | FR-5 | `x402-core/src/policy/` | `policy/engine.rs`, `policy/rules.rs` |
|  |  | `x402-cli/src/commands/policy.rs` | CLI command |
| **5: Middleware Generation** | FR-6 | `x402-core/src/policy/codegen/` | `codegen/rust_middleware.rs` |
|  |  | `x402-cli/src/commands/policy.rs` | `policy generate` subcommand |
| **6: CLI Infrastructure & DX** | FR-7, FR-8, FR-9, FR-10, FR-11 | `x402-cli/src/{main, cli}` | `main.rs`, `cli.rs` |
|  |  | `x402-core/src/{config, examples, doctor}` | `config/loader.rs`, `doctor/checks.rs` |

---

## Implementation Patterns (Pure Rust)

### 1. x402 Invoice Generation

```rust
// crates/x402-core/src/protocol/invoice.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Invoice {
    pub recipient: String,  // Solana address (Base58)
    pub amount: String,     // USDC amount ("0.01")
    pub currency: String,   // "USDC"
    pub memo: String,       // "req_abc123_resource_data"
    pub network: String,    // "devnet" | "testnet" | "mainnet"
}

impl Invoice {
    pub fn new(amount: f64, recipient: String) -> Self {
        Self {
            recipient,
            amount: format!("{:.2}", amount),
            currency: "USDC".to_string(),
            memo: format!("req_{}", uuid::Uuid::new_v4()),
            network: "devnet".to_string(),
        }
    }

    pub fn to_www_authenticate_header(&self) -> String {
        format!(
            "x402-solana recipient={} amount={} currency={} memo={} network={}",
            self.recipient, self.amount, self.currency, self.memo, self.network
        )
    }
}
```

### 2. actix-web Mock Server

```rust
// crates/x402-core/src/server/routes.rs
use actix_web::{web, HttpResponse, Result};
use crate::protocol::Invoice;

pub async fn handle_payment_required() -> Result<HttpResponse> {
    let invoice = Invoice::new(0.01, "GXk8v...qPz9".to_string());

    Ok(HttpResponse::PaymentRequired()
        .insert_header(("WWW-Authenticate", invoice.to_www_authenticate_header()))
        .json(invoice))
}
```

### 3. Async Patterns (tokio multi-thread)

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Mock(args) => mock::run(args).await?,
        Commands::Test(args) => test::run(args).await?,
        // ... other commands
    }

    Ok(())
}
```

---

## Implementation Patterns

### Naming Conventions

- **Types:** `PascalCase` (e.g., `MockServerConfig`)
- **Functions:** `snake_case` (e.g., `create_invoice`)
- **Constants:** `SCREAMING_SNAKE_CASE` (e.g., `DEFAULT_PORT`)
- **Modules:** `snake_case` (e.g., `mod mock_server`)
- **Traits:** `PascalCase` (e.g., `InvoiceGenerator`)

### Module Structure

```rust
// crates/x402-core/src/policy/mod.rs
mod engine;
mod parser;
mod evaluator;

// Public API
pub use engine::PolicyEngine;
pub use parser::parse_policy;
pub use evaluator::{evaluate, PolicyDecision};
```

### Error Propagation

```rust
// Use ? operator for clean error handling
pub fn load_config(path: &Path) -> Result<Config, CoreError> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}

// Add context to errors
.map_err(|e| CoreError::Config(format!("Invalid config: {}", e)))?
```

### Async Patterns (tokio current_thread)

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let runtime = X402Runtime::new()?;
    runtime.execute().await?;
    Ok(())
}

// Use spawn_blocking for CPU-intensive work
let result = tokio::task::spawn_blocking(|| {
    heavy_computation()
}).await?;
```

---

## Consistency Rules

### Error Handling

- **Library code (x402-core):** Return `Result<T, CoreError>` with thiserror
- **Binary code (x402-cli):** Convert to `miette::Report` for beautiful diagnostics
- **Always provide:** Actionable fix suggestion + documentation link

### Logging

- **Use tracing:** `info!`, `warn!`, `error!`, `debug!`
- **Structured fields:** `info!(port = 3402, "Server started")`
- **Levels:** error (failures), warn (potential issues), info (key operations), debug (detailed flow)

### Date/Time

- **Store internally:** `chrono::DateTime<Utc>`
- **External format:** ISO 8601 (RFC3339)
- **Never:** Use local timezone internally

### CLI Output

- **Colors:** Green (success), Red (error), Yellow (warning), Blue (info)
- **Emoji:** Use with ASCII fallback (`Emoji("✅ ", "OK ")`)
- **Progress bars:** For operations >2 seconds (indicatif)

---

## Data Architecture

### Configuration Schema

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct X402Config {
    pub mock_server: MockServerConfig,
    pub test: TestConfig,
    pub solana: SolanaConfig,
    pub policy: PolicyConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MockServerConfig {
    pub port: u16,
    pub pricing: HashMap<String, f64>,
}
```

**Configuration Priority:**
1. CLI flags (highest)
2. Environment variables (`X402_DEV_*`)
3. Project config (`.x402dev.yaml`)
4. Global config (`~/.x402dev/config.yaml`)
5. Built-in defaults (lowest)

### Policy Schema

```yaml
# .x402dev-policy.yaml
policies:
  - type: allowlist
    field: agent_id
    values: ["agent-1", "agent-2"]

  - type: rate_limit
    max_requests: 100
    window_seconds: 3600

  - type: spending_cap
    max_amount: 10.00
    currency: USDC
    window_seconds: 86400
```

---

## API Contracts

### x402 Protocol Response

```http
HTTP/1.1 402 Payment Required
WWW-Authenticate: x402-solana recipient=GXk8v...qPz9 amount=0.01 currency=USDC memo=req_abc123 network=devnet

{
  "error": "Payment Required",
  "invoice": {
    "recipient": "GXk8v...qPz9",
    "amount": "0.01",
    "currency": "USDC",
    "memo": "req_abc123_resource_data",
    "network": "devnet"
  }
}
```

---

## Security Architecture

### Configuration Security

- **Environment variables:** For secrets (Solana RPC auth tokens)
- **No plaintext secrets:** In config files (warn user if detected)
- **File permissions:** Validate config file permissions (warn if world-readable)

### Policy Audit Trail

```rust
// All policy decisions logged
info!(
    agent_id = request.agent_id,
    decision = ?decision,
    rule = rule.name,
    "Policy evaluation"
);
```

---

## Performance Considerations

### Binary Size Optimization

- **opt-level="z":** Size-focused optimization
- **LTO="fat":** Link-time optimization
- **strip="symbols":** Remove debug symbols
- **Result:** ~8-15MB binary (V8 runtime + Rust + bundled JavaScript)

### Runtime Performance

- **Policy evaluation:** High performance (Rust CEL engine, actual throughput depends on policy complexity)
- **Command execution:** <1 second (excluding network)
- **Mock server startup:** <2 seconds
- **Test runner:** 100 tests in <10 seconds

### Memory Footprint

- **CLI commands:** <100MB
- **Mock server:** <200MB
- **No memory leaks:** Rust ownership prevents leaks

---

## Deployment Architecture

### Distribution

```bash
# Cargo install
cargo install x402-dev

# Direct download from GitHub releases
curl -fsSL https://github.com/yourusername/x402-dev/releases/latest/download/x402-dev-$(uname -s)-$(uname -m) -o x402-dev
chmod +x x402-dev

# Install script (future)
curl -fsSL https://x402.dev/install.sh | sh
```

### Cross-Compilation Targets

- macOS (Intel + Apple Silicon)
- Linux (Ubuntu 20.04+, Debian, RHEL)
- Windows (native + WSL2)

### CI/CD

```yaml
# .github/workflows/release.yml
- Cross-compile for all platforms
- Strip symbols, optimize size
- Create GitHub release with binaries
- Publish to crates.io (cargo publish)
```

---

## Development Environment

### Prerequisites

```bash
# Rust 1.75+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Development tools (optional)
cargo install cargo-watch cargo-nextest
```

### Setup Commands

```bash
# Clone and build
git clone https://github.com/yourusername/x402-dev
cd x402-dev

# Build
cargo build --release

# Run
./target/release/x402-dev --version
```

### Development Workflow

```bash
# Watch mode (auto-rebuild on changes)
cargo watch -x check -x test

# Integration tests
cargo nextest run --all

# Run mock server locally
cargo run -- mock --port 3402
```

---

## Architecture Decision Records (ADRs)

### ADR-001: Pure Rust Implementation (KISS Principle)

**Date:** 2025-11-10 (Supersedes original hybrid approach)

**Context:**
- Original plan assumed $5,000 Corbits Project prize for using Corbits SDK (TypeScript/Node.js)
- Research revealed **NO such prize exists** - false premise invalidates hybrid architecture
- deno_core integration estimated at 8+ hours for zero tangible benefit
- Binary bloat: 8-15MB (hybrid) vs 2-3MB (pure Rust) - 80% larger for no prize reward

**Decision:** Pure Rust implementation using actix-web for HTTP server, manual x402 protocol implementation.

**Rationale:**
1. **Simplicity:** Single language eliminates Rust↔JS boundary complexity
2. **Performance:** actix-web > Express.js (native async, no marshalling overhead)
3. **Binary Size:** 2-3MB vs 8-15MB (67-80% smaller)
4. **Build Time:** ~15 sec vs ~45 sec (66% faster)
5. **Implementation Speed:** 2-3 hours vs 8+ hours for Epic 2
6. **No Prize Trade-off:** Zero opportunity cost (Corbits prize doesn't exist)

**Consequences:**
- ✅ **10 hours saved** vs original plan (20% of hackathon time)
- ✅ Simple debugging (single language, no runtime boundary issues)
- ✅ Smaller deployment artifact (faster downloads, lower hosting costs)
- ✅ Faster builds (no TypeScript bundling step)
- ✅ Full async capabilities (no V8 single-thread constraints)
- ⚠️ Cannot run Faremeter TypeScript SDK natively (mitigated: shell out to Node.js if needed post-hackathon)

**Reversibility:** HIGH - Can add deno_core in ~2 hours if requirements change post-hackathon

**Reference:** See KISS refactoring analysis (docs/KISS-refactoring-plan.md)

---

### ADR-002: tokio multi-thread Runtime

**Date:** 2025-11-10 (Supersedes current_thread decision)

**Context:** No deno_core V8 constraints in pure Rust architecture.

**Decision:** Use `tokio::runtime::Builder::new_multi_thread()` (default).

**Consequences:**
- ✅ Full async parallelism capabilities
- ✅ Can scale to multi-core workloads if needed
- ✅ No artificial runtime constraints
- ✅ Standard Rust async patterns

---

### ADR-003: Simplified Crate Structure (2-3 crates)

**Date:** 2025-11-05 (Unchanged)

**Context:** 7 crates may slow hackathon development.

**Decision:** Merge into 2-3 crates (x402-cli, x402-core, optional xtask).

**Consequences:**
- ✅ Faster iteration during hackathon
- ✅ Simpler dependency management
- ✅ Can split later if needed
- ⚠️ Less modular, but sufficient for MVP

---

### ADR-004: Manual x402 Protocol Implementation

**Date:** 2025-11-10 (New)

**Context:** Need to generate x402 payment invoices for mock server.

**Decision:** Manual protocol implementation (HTTP 402 status + WWW-Authenticate header formatting).

**Rationale:**
- x402 protocol is simple (HTTP headers + JSON structure)
- No SDK integration complexity (documentation gaps, version conflicts)
- Full control over invoice format
- Faster implementation (<2 hours vs 6+ hours SDK integration)

**Consequences:**
- ✅ Simple, predictable code
- ✅ No external SDK dependencies
- ✅ Complete control over invoice generation
- ⚠️ Manual protocol updates if spec changes (mitigated: spec is stable)

---

## Next Steps

**After Architecture Approval:**
1. ✅ Run solutioning-gate-check workflow
2. ✅ Validate PRD + Architecture alignment
3. ✅ Begin Epic 1: Foundation & CLI Infrastructure
4. ✅ Set up project scaffolding with approved structure

**First Story:** Epic 1, Story 1.1 - Project Scaffolding & Build System

---

_Generated by BMAD Decision Architecture Workflow v1.3.2_
_Date: 2025-11-09_
_For: Valik_
