# Changelog

All notable changes to x402-dev will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] - 2025-11-12

### 🎉 Initial Release

**x402-dev v0.1.0** - HTTP 402 Protocol Testing Toolkit

**Solana Hackathon 2025 Submission**

⚠️ **Scope:** Development and testing toolkit only. **NOT a production payment processor.**

---

### ✨ Features

#### Epic 1: Foundation & CLI Infrastructure
- ✅ Complete CLI framework with Clap (11 commands)
  - 9 fully implemented commands
  - 2 stub commands (`verify`, `monitor`)
- ✅ Multi-tier configuration system (CLI > ENV > project > global > defaults)
- ✅ Professional error handling with colored output
- ✅ Comprehensive help system with examples
- ✅ Interactive project initialization (`x402-dev init`)
- ✅ Version management with update notifications

#### Epic 2: Mock Facilitator Server
- ✅ HTTP server with 402 Payment Required responses
- ✅ Configurable pricing rules per route
- ✅ Payment simulation (success/failure/timeout modes)
- ✅ Invoice generation with test addresses
- ✅ **MOCK ONLY** - Zero blockchain dependency
- ✅ Server lifecycle management (start/stop/restart/status)
- ✅ Instant setup (3 seconds vs 30 minutes with real blockchain)

**Important:** Mock server accepts ANY `X-Payment-Proof` value without validation. Perfect for testing, NOT for production.

#### Epic 3: Automated Test Suite
- ✅ YAML-based test suite parser
- ✅ Comprehensive assertion framework (12 assertion types)
- ✅ Async test executor with fail-soft behavior
- ✅ Multiple output formats: summary, JSON, JUnit XML
- ✅ CI/CD integration ready (exit codes, machine-readable)
- ✅ <100ms test execution overhead

#### Epic 4: Validation Tools
- ✅ `x402-dev check <url>` - Validate HTTP 402 protocol compliance
- ✅ 12-point protocol validation (status code, headers, format)
- ✅ HTTP timeout handling (10s default)
- ✅ JSON output for CI/CD
- ✅ `x402-dev doctor` - System diagnostics

**Note:** Validates protocol compliance only, does NOT verify blockchain transactions.

#### Epic 5: Policy Engine & Security
- ✅ YAML policy definition language
- ✅ Policy types: allowlist, denylist, rate limiting, spending caps
- ✅ Conflict detection and validation
- ✅ Middleware code generation (Express.js, Fastify)
- ✅ **8x code generation multiplier** (29 lines YAML → 224 lines middleware)
- ✅ Audit logging format definitions (JSON/CSV)

**Note:** Generates middleware code for policy enforcement. Does NOT include real payment verification logic.

#### Epic 6: Developer Experience & Distribution
- ✅ `x402-dev examples` command - Browse and init examples
- ✅ 3 complete example projects:
  - MCP Server with Payments (conceptual)
  - AI Agent Policy Enforcement
  - CI/CD Testing Suite
- ✅ **"Working in 2 minutes" demo checkpoint achieved**

#### Epic 7: Launch Preparation
- ✅ Exceptional README.md with 90-second quick start
- ✅ Comprehensive documentation (7 guides)
  - Quick Start Guide
  - CLI Reference
  - Architecture (mock server)
  - Troubleshooting
  - Limitations (what's NOT implemented)
  - Contributing Guidelines
- ✅ Honest documentation about testing-only scope
- ✅ Architecture diagrams with Mermaid (mock flows)

---

### 🔧 Technical Details

**Binary Size:** 2.7MB (optimized for size)
**Build Time:** ~22s (clean), <5s (incremental)
**Test Coverage:** 49/49 tests passing (mock server tests)
**Rust Version:** 1.75+ required
**Platforms:** macOS (ARM64), Linux (x86_64), Windows (x86_64)

---

### 📦 Installation

```bash
cargo install x402-dev
```

---

### 🚀 Quick Start

```bash
# Initialize project
x402-dev init my-api && cd my-api

# Start mock server
x402-dev mock

# Test it works
curl http://localhost:3402
# ✅ Returns: 402 Payment Required (mock invoice)
```

**Total time:** < 90 seconds ⚡

---

### 📊 Key Metrics

- ⚡ **Setup time:** 90 seconds (mock server, not production)
- 🧪 **Test execution:** <100ms overhead
- 📈 **Code generation:** 8x multiplier (YAML → middleware)
- ✅ **Test coverage:** 100% (49/49 passing)
- 🎯 **Mock server throughput:** ~5,000 req/s

---

### 🐛 Bug Fixes

#### From Epic 4 Code Review:
- 🔴 **Fixed:** Hardcoded network validation (only accepted "devnet")
  - Now accepts: devnet, testnet, mainnet-beta, mainnet
  - **Note:** All networks are for configuration only; mock server ignores them
- 🔴 **Fixed:** No HTTP timeout in `check` command
  - Added 10-second timeout to prevent hanging

---

### ⚠️ Important Limitations

**What x402-dev is:**
- ✅ Testing toolkit for HTTP 402 protocol
- ✅ Mock payment server for development
- ✅ CLI tools for protocol validation
- ✅ Policy middleware code generator
- ✅ Test automation framework

**What x402-dev is NOT:**
- ❌ Production payment processor
- ❌ Real Solana blockchain integration (no `solana-client` dependency)
- ❌ Payment verification system (accepts any proof for testing)
- ❌ Wallet or account management
- ❌ Complete payment solution

**For production use:**
You must separately integrate:
- `solana-client` crate for blockchain calls
- Transaction verification logic
- Payment cache (prevent replay attacks)
- Wallet/keypair management
- Security hardening

👉 See [docs/limitations.md](docs/limitations.md) for complete details.

---

### 📚 Documentation

- [Quick Start Guide](docs/quickstart.md) - 90-second tutorial
- [CLI Reference](docs/cli-reference.md) - All commands
- [Architecture](docs/architecture.md) - How mock server works
- [Troubleshooting](docs/troubleshooting.md) - Common issues & fixes
- [Limitations](docs/limitations.md) - What's NOT implemented
- [Contributing](CONTRIBUTING.md) - Development guide

---

### 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for performance and safety
- Inspired by HTTP 402 "Payment Required" standard ([RFC 7231](https://tools.ietf.org/html/rfc7231#section-6.5.2))
- Developed for Solana Hackathon 2025

---

### 🔮 What's Next?

**Post-Launch Roadmap** (v0.2.0+):

**Phase 1: Real Blockchain Integration (v0.2.0)**
- Add `solana-client` crate dependency
- Implement RPC connection management
- Add transaction lookup
- Support devnet, testnet, mainnet-beta

**Phase 2: Payment Verification (v0.3.0)**
- Signature verification
- Amount validation
- Recipient address checking
- Payment cache (Redis/PostgreSQL)
- Replay attack prevention

**Phase 3: Production Hardening (v0.4.0)**
- HTTPS enforcement
- Rate limiting
- DDoS protection
- Audit logging
- Metrics/monitoring
- Wallet management

**Phase 4: Advanced Features (v0.5.0+)**
- WebSocket support for real-time payment notifications
- Dashboard UI for monitoring transactions
- Plugin system for custom payment flows
- Multi-chain support (Ethereum, Polygon, etc.)

---

## [Unreleased]

### Added
- Honest documentation reflecting testing-only scope
- Limitations guide documenting what's NOT implemented

### Changed
- Updated README to focus on testing toolkit positioning
- Revised architecture diagrams to show mock flows only
- Fixed troubleshooting guide to remove false blockchain claims

### Removed
- False claims about production readiness
- Misleading comparisons with production payment processors
- Fake hackathon winner badges (changed to "submission")

---

**Legend:**
- ✨ Features - New functionality
- 🔧 Changed - Changes in existing functionality
- 🐛 Fixed - Bug fixes
- 🔐 Security - Security improvements
- ⚠️ Deprecated - Soon-to-be removed features
- 🗑️ Removed - Removed features

---

[0.1.0]: https://github.com/valentynkit/x402-dev/releases/tag/v0.1.0
[Unreleased]: https://github.com/valentynkit/x402-dev/compare/v0.1.0...HEAD
