# x402-dev - Architecture Document

**Author:** Valik
**Date:** 2025-11-09
**Version:** 1.0
**Project Level:** Level 2 (Medium Complexity)

---

## Executive Summary

x402-dev is a **hybrid Rust + TypeScript CLI toolkit** for x402 protocol development on Solana. The architecture combines:

- **Rust core** (performance, type safety, small binary size)
- **TypeScript runtime** (Corbits SDK integration via deno_core)
- **Single binary distribution** (~3-5MB with embedded JavaScript)

**Key Differentiation:** First comprehensive x402 dev tool using embedded V8 runtime for seamless npm package integration while maintaining Rust performance benefits.

**Performance Targets:**
- Binary size: 3-5MB (vs 8-10MB pure TypeScript)
- Command execution: <1 second (excluding network calls)
- Policy evaluation: ~1M requests/second (Rust CEL engine)
- Mock server startup: <2 seconds

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

**First Implementation Story:** Epic 1, Story 1.1 - Project scaffolding with tsup + Rust workspace

---

## Decision Summary

| Category | Decision | Version | Affects Epics | Rationale |
|----------|----------|---------|---------------|-----------|
| **Language** | Rust | 1.75+ | All | Performance, type safety, small binary |
| **CLI Framework** | Clap | 4.5 | Epic 1, 2-6 | Industry standard, derive macros, excellent DX |
| **Async Runtime** | tokio | 1.40 (`current_thread`) | All | **CRITICAL:** deno_core requires single-threaded |
| **JS Runtime** | deno_core | 0.311 | Epic 2, 5, 6 | Embed TypeScript for Corbits SDK integration |
| **Error Handling (Lib)** | thiserror | 1.0 | All (x402-core) | Typed errors for library code |
| **Error Handling (CLI)** | miette | 7.0 | All (x402-cli) | Beautiful diagnostics with code snippets |
| **Error Handling (Util)** | anyhow | 1.0 | All (xtask) | Convenience for automation scripts |
| **Policy Engine** | cel-interpreter | 0.8 | Epic 5 | Pure Rust CEL, compatible with current_thread |
| **x402 SDK** | Corbits/Faremeter | latest | Epic 2, 5 | $5k bonus, Solana-first, open-source |
| **Mock Server** | Express.js | 4.x | Epic 2 | TypeScript (via deno_core), simple integration |
| **Configuration** | serde_yaml + cosmiconfig pattern | - | Epic 1 | Multi-tier config (CLI > env > file > defaults) |
| **Logging** | tracing + tracing-subscriber | 0.1 / 0.3 | All | Structured logging, async-aware |
| **HTTP Client** | reqwest | 0.12 | Epic 4 | Async HTTP for verification |
| **Solana SDK** | @solana/web3.js | 1.x | Epic 4 | RPC queries (optional feature) |
| **Build Tool** | tsup / esbuild | - | Epic 1 | TypeScript → single .js bundle |
| **Package Manager** | npm | - | All | TypeScript dependencies |
| **Distribution** | npm (global install) | - | Epic 7 | `npm install -g x402-dev` |

---

## Technology Stack Details

### Core Technologies

**Rust Crates:**
```toml
[workspace.dependencies]
# CLI & Terminal
clap = { version = "4.5", features = ["derive", "env", "wrap_help"] }
miette = { version = "7.0", features = ["fancy"] }
console = "0.15"
indicatif = "0.17"

# Error Handling
thiserror = "1.0"
anyhow = "1.0"

# Async Runtime (CRITICAL: current_thread for deno_core)
tokio = { version = "1.40", features = ["rt", "macros", "process", "fs", "io-util"] }
futures = "0.3"

# deno_core Integration
deno_core = "0.311"
serde_v8 = "0.227"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"

# Configuration
config = "0.14"
dotenvy = "0.15"

# CEL Policy Engine (Pure Rust)
cel-interpreter = "0.8"

# HTTP Client
reqwest = { version = "0.12", features = ["json"] }

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
```

**TypeScript Dependencies:**
```json
{
  "dependencies": {
    "@faremeter/fetch": "^latest",
    "@faremeter/middleware": "^latest",
    "express": "^4.18.0",
    "cors": "^2.8.5"
  },
  "devDependencies": {
    "typescript": "^5.2.0",
    "tsup": "^7.2.0",
    "@types/node": "^20.0.0",
    "@types/express": "^4.17.0"
  }
}
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

**Result:** ~3-5MB binary (vs 8-10MB pure TypeScript/Node.js)

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
│   │       ├── context.rs                # RuntimeContext
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
│   ├── x402-core/                        # LIBRARY CRATE (MERGED)
│   │   ├── Cargo.toml
│   │   ├── build.rs                      # Bundle TypeScript at build time
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── errors.rs
│   │       ├── config/                   # Epic 1: Configuration
│   │       ├── runtime/                  # Epic 1-6: deno_core runtime
│   │       │   ├── ops/                  # Rust ↔ JS ops
│   │       ├── protocol/                 # Epic 2, 4: x402 protocol
│   │       ├── server/                   # Epic 2: Mock server
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
│   └── xtask/                            # BUILD AUTOMATION
│       └── src/main.rs                   # CI tasks, release
│
├── ts/                                   # TYPESCRIPT (BUNDLED)
│   ├── package.json
│   ├── tsconfig.json
│   ├── build.ts
│   └── src/
│       ├── runtime.ts
│       ├── corbits/                      # Epic 2, 5: Corbits SDK
│       ├── server/                       # Epic 2: Express server
│       └── utils/
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

| Epic | Rust Modules | TypeScript Modules | Key Files |
|------|--------------|-------------------|-----------|
| **1: Foundation** | `x402-cli/src/{main, cli, context}` | - | `main.rs`, `cli.rs` |
|  | `x402-core/src/{config, runtime}` | `ts/src/runtime.ts` | `config/loader.rs`, `runtime/js_runtime.rs` |
| **2: Mock Server** | `x402-core/src/{server, protocol}` | `ts/src/{server, corbits}` | `server/manager.rs`, `server/app.ts` |
|  | `x402-cli/src/commands/mock.rs` | - | CLI command |
| **3: Test Runner** | `x402-core/src/test/` | - | `test/runner.rs`, `test/assertions.rs` |
|  | `x402-cli/src/commands/test.rs` | - | CLI command |
| **4: Validation** | `x402-core/src/{verify, monitor, solana}` | - | `verify/headers.rs`, `monitor/tail.rs` |
|  | `x402-cli/src/commands/{verify, check, monitor}.rs` | - | CLI commands |
| **5: Policy Engine** | `x402-core/src/policy/` | `ts/src/server/policy_middleware.ts` | `policy/engine.rs`, `policy/codegen/` |
|  | `x402-cli/src/commands/policy.rs` | - | CLI command |
| **6: Dev Experience** | `x402-core/src/{examples, doctor}` | - | `examples/scaffolder.rs`, `doctor/checks.rs` |
|  | `x402-cli/src/commands/{examples, doctor}.rs` | - | CLI commands |
| **7: Launch Prep** | `.github/workflows/` | - | `ci.yml`, `release.yml` |
|  | `docs/`, `README.md` | - | Documentation |

---

## Integration Points

### 1. Rust → TypeScript (deno_core ops)

```rust
// Rust side: Define op
#[op]
async fn op_create_invoice(amount: f64, recipient: String) -> Result<String, AnyError> {
    // TypeScript will call Corbits SDK
    Ok(serde_json::to_string(&json!({ "amount": amount, "recipient": recipient }))?)
}
```

```typescript
// TypeScript side: Call op
const invoice = await Deno.core.ops.op_create_invoice(0.01, "GXk8v...qPz9");
```

### 2. TypeScript → Corbits SDK

```typescript
// ts/src/corbits/client.ts
import { generateInvoice } from '@faremeter/middleware';

export async function createInvoice(params: InvoiceParams): Promise<Invoice> {
    return await generateInvoice({
        amount: params.amount,
        recipient: params.recipient,
        currency: 'USDC',
        network: 'solana-devnet'
    });
}
```

### 3. Build-Time TypeScript Bundling

```rust
// crates/x402-core/build.rs
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../../ts/src");

    // Compile TypeScript
    let status = Command::new("npm")
        .args(&["run", "build"])
        .current_dir("../../ts")
        .status()
        .expect("Failed to build TypeScript");

    assert!(status.success());
}
```

```rust
// crates/x402-core/src/runtime/js_runtime.rs
// Embed bundled JavaScript at compile time
let js_code = include_str!("../../../ts/dist/runtime.js");
js_runtime.execute_script("<runtime>", js_code)?;
```

---

## Novel Pattern: Hybrid Rust + TypeScript Architecture

**Problem:** Need to integrate Corbits SDK (TypeScript/Node.js) while maintaining Rust CLI benefits.

**Solution:** Embed deno_core V8 runtime in Rust binary.

**Benefits:**
- ✅ Single binary distribution (no Node.js dependency)
- ✅ Rust performance for CLI/policy engine
- ✅ TypeScript ecosystem access (Corbits SDK, Express)
- ✅ Type-safe FFI via deno_core ops
- ✅ Qualifies for $5k Corbits Project bonus

**Trade-offs:**
- ⚠️ Increased complexity (Rust ↔ JS boundary)
- ⚠️ tokio runtime must be `current_thread` (V8 constraint)
- ⚠️ Larger binary than pure Rust (~3-5MB vs ~2MB)

**Alternative Rejected:** Pure Rust with HTTP calls to Corbits API (loses SDK benefits, risks protocol drift)

---

## Implementation Patterns

### Naming Conventions

- **Rust types:** `PascalCase` (e.g., `MockServerConfig`)
- **Rust functions:** `snake_case` (e.g., `create_invoice`)
- **Rust constants:** `SCREAMING_SNAKE_CASE` (e.g., `DEFAULT_PORT`)
- **Rust modules:** `snake_case` (e.g., `mod mock_server`)
- **TypeScript interfaces:** `PascalCase` (e.g., `interface InvoiceParams`)
- **TypeScript functions:** `camelCase` (e.g., `function createInvoice`)
- **TypeScript constants:** `SCREAMING_SNAKE_CASE` (e.g., `const DEFAULT_AMOUNT`)

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

### deno_core Ops Interface

```rust
// Rust → TypeScript
#[op]
async fn op_create_invoice(amount: f64, recipient: String) -> Result<String, AnyError>;

#[op]
fn op_get_config(key: String) -> Result<String, AnyError>;

// TypeScript calls
const invoice = await Deno.core.ops.op_create_invoice(0.01, "GXk8v...qPz9");
const port = Deno.core.ops.op_get_config("mock_server.port");
```

---

## Security Architecture

### deno_core Security Model

- **Secure by default:** No file/network/env access unless explicitly granted
- **Permission model:** Rust code controls what JavaScript can access
- **No eval:** TypeScript code is bundled at build time (no runtime eval)

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
- **Result:** ~3-5MB binary

### Runtime Performance

- **Policy evaluation:** Rust CEL engine (~1M req/sec)
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
# npm global install
npm install -g x402-dev

# npx (no install)
npx x402-dev <command>

# Direct download (future)
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
- Create GitHub release
- Publish to npm registry
```

---

## Development Environment

### Prerequisites

```bash
# Rust 1.75+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js 18+ LTS
nvm install 20

# Development tools
cargo install cargo-watch cargo-nextest
npm install -g tsx
```

### Setup Commands

```bash
# Clone and build
git clone https://github.com/yourusername/x402-dev
cd x402-dev

# Install TypeScript dependencies
cd ts && npm install && cd ..

# Build Rust (includes TypeScript bundling via build.rs)
cargo build --release

# Run
./target/release/x402-dev --version
```

### Development Workflow

```bash
# Watch mode (Rust only)
cargo watch -x check -x test

# TypeScript rebuild
cd ts && npm run build && cd ..

# Integration tests
cargo nextest run --all
```

---

## Architecture Decision Records (ADRs)

### ADR-001: Hybrid Rust + TypeScript Architecture

**Context:** Need Corbits SDK (TypeScript) integration while maintaining Rust CLI benefits.

**Decision:** Embed deno_core V8 runtime in Rust binary.

**Consequences:**
- ✅ Single binary distribution
- ✅ $5k Corbits Project bonus
- ⚠️ Requires current_thread tokio runtime
- ⚠️ Increased binary size (~3-5MB vs ~2MB pure Rust)

### ADR-002: tokio current_thread Runtime

**Context:** deno_core requires single-threaded runtime (V8 constraint).

**Decision:** Use `tokio::runtime::Builder::new_current_thread()`.

**Consequences:**
- ✅ deno_core works out-of-box
- ⚠️ No parallel test execution in MVP
- ✅ Can add worker pool post-hackathon if needed

### ADR-003: Simplified Crate Structure (3 crates)

**Context:** 7 crates may slow hackathon development.

**Decision:** Merge into 3 crates (x402-cli, x402-core, xtask).

**Consequences:**
- ✅ Faster iteration during hackathon
- ✅ Simpler dependency management
- ✅ Can split later if needed
- ⚠️ Less modular, but sufficient for MVP

### ADR-004: cel-interpreter (Pure Rust CEL)

**Context:** Need CEL policy engine compatible with current_thread runtime.

**Decision:** Use cel-interpreter (pure Rust) instead of cel-cxx (FFI to C++).

**Consequences:**
- ✅ Compatible with current_thread tokio
- ✅ No C++ toolchain dependency
- ✅ Production usage validated
- ⚠️ Version 0.8.x (pre-1.0, but active development)

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
