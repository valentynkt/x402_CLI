# x402-dev - x402 Protocol Standard Toolkit

> **Transform x402 development from hours to seconds**

[![Tests](https://img.shields.io/badge/tests-155%2B%20passing-brightgreen)]()
[![Binary Size](https://img.shields.io/badge/binary-2.5MB-blue)]()
[![Build](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Security](https://img.shields.io/badge/security-verified-green)]()

## Overview

x402-dev is the **first comprehensive CLI toolkit** for x402 protocol development on Solana. Test payment flows locally in **3 seconds** instead of **30 minutes**, with zero blockchain dependencies.

Built for the Solana x402 AI Hackathon.

## 🎯 What Problem Does This Solve?

**Before x402-dev:**
- ⏱️ **30 minutes** to test a single payment flow
- 🌐 Requires testnet deployment
- 🐛 Manual debugging with blockchain explorers
- 📝 100+ lines of custom security code per project

**After x402-dev:**
- ⚡ **3 seconds** to test payment flows
- 💻 Works completely offline
- 🔍 Clear error messages with fix suggestions
- 📋 **10-line YAML → 224-line middleware** (Epic 5 ✅)

**600x faster iteration speed** 🚀

## ✨ Key Features

- **Pure Rust Architecture** - KISS principle, no TypeScript complexity
- **Mock Facilitator Server** - Local x402 server for testing (actix-web)
- **Zero Blockchain Dependency** - Works completely offline
- **Invoice Generation** - Automatic x402-compliant invoices
- **Payment Simulation** - Success, failure, timeout modes
- **Configuration Management** - Multi-tier config (CLI > ENV > file > defaults)
- **Interactive Setup** - 2-minute project initialization
- **Professional UX** - Colored output, clear errors, helpful suggestions
- **🆕 Policy Engine** - Generate production middleware from YAML (Epic 5 ✅)
  - **8x code multiplier**: 29 lines YAML → 224 lines Express.js
  - **2 frameworks**: Express + Fastify plugins
  - **4 policy types**: Allowlist, Denylist, Rate Limit, Spending Cap
  - **Conflict detection**: Validates policies before generation

## 🚀 Quick Start (3 Minutes)

### Installation

```bash
# Clone and install
git clone <repository-url>
cd x402-dev
cargo install --path crates/x402-cli

# Verify installation
x402-dev --version
```

### 3-Minute Demo

```bash
# 1. Initialize project (30 seconds)
mkdir my-x402-project && cd my-x402-project
x402-dev init
# Follow prompts: port=8402, network=devnet, log_level=info

# 2. Start mock server (2 seconds)
x402-dev mock --port 8402

# 3. Test payment flow (1 second) - in another terminal
curl -i http://127.0.0.1:8402/api/test
```

**Expected output:**
```http
HTTP/1.1 402 Payment Required
www-authenticate: x402-solana recipient=Dev123... amount=100 currency=USDC memo=req-abc123... network=devnet
```

**That's it!** You just tested an x402 payment flow in **3 seconds** with **zero blockchain dependencies**.

### 🆕 Epic 5 Demo: Policy Engine (30 seconds)

```bash
# Create a simple policy file
cat > policy.yaml << 'EOF'
policies:
  - type: allowlist
    field: agent_id
    values: ["agent-gpt4", "agent-claude"]
  - type: rate_limit
    max_requests: 100
    window_seconds: 3600
  - type: spending_cap
    max_amount: 10.00
    currency: USDC
    window_seconds: 86400
EOF

# Validate the policy
x402-dev policy validate policy.yaml

# Generate Express middleware (224 lines from 29-line YAML!)
x402-dev policy generate policy.yaml --framework express -o middleware.js

# See the magic: 8x code multiplication
wc -l policy.yaml middleware.js
#   29 policy.yaml
#  224 middleware.js
```

**That's 224 lines of production-ready middleware from a 29-line YAML file!** 🚀

## Requirements

- **Rust**: >= 1.75.0 (for building from source)
- **Cargo**: Latest stable

### 📊 Current Status

**Completed:** Epic 1 (Foundation) + Epic 2 (Mock Server) + Epic 5 (Policy Engine) + **Wave 1 Refactoring**
- **Stories:** 13/13 complete (100%)
- **Tests:** 155+ passing (100% - includes unit, integration, security, and property tests)
- **Binary Size:** 2.5MB (clean build with optimizations)
- **Demo Checkpoint:** ✅ 3 seconds vs 30 minutes (achieved)
- **🆕 Policy Engine:** ✅ 10 lines YAML → 224 lines middleware (Epic 5 complete!)
- **✨ Wave 1 Refactoring:** ✅ Clean architecture, type safety, zero security vulnerabilities

## 📋 Available Commands

```bash
# Server Management (✅ WORKING)
x402-dev mock                      # Start mock server (default port: 8402)
x402-dev mock --port 9000          # Custom port
x402-dev mock stop                 # Stop running server
x402-dev mock status               # Check server status

# Configuration (✅ WORKING)
x402-dev init                      # Interactive project setup
x402-dev config show               # Display current configuration
x402-dev config show --port 9000   # Override with CLI flags

# Version & Help (✅ WORKING)
x402-dev version                   # Show version and updates
x402-dev --help                    # Show all commands
x402-dev mock --help               # Command-specific help

# Policy Engine (✅ Epic 5 - WORKING)
x402-dev policy validate policy.yaml                    # Validate YAML policies
x402-dev policy generate policy.yaml --framework express  # Generate Express middleware
x402-dev policy generate policy.yaml --framework fastify  # Generate Fastify plugin

# Coming in Future Epics (🚧 Placeholders)
x402-dev test                      # Epic 3: Test suites
x402-dev verify                    # Epic 3: Protocol verification
x402-dev check                     # Epic 4: Health checks
x402-dev doctor                    # Epic 4: Diagnostics
x402-dev monitor                   # Epic 5: Transaction monitoring
x402-dev examples                  # Epic 6: Example code
```

## 🏗️ Architecture

### Clean Crate Architecture (Post-Wave 1 Refactoring)

```
┌─────────────┐
│  Developer  │
└──────┬──────┘
       │ x402-dev CLI (x402-cli)
       ▼
┌──────────────────────────────────────────┐
│            x402-server                   │
│      Mock Facilitator Server             │
│      (Pure Rust - actix-web)             │
│                                          │
│  ✓ 402 Payment Required                 │
│  ✓ WWW-Authenticate headers             │
│  ✓ Invoice generation                   │
│  ✓ Payment simulation                   │
│  ✓ Zero blockchain calls                │
└─────────────┬────────────────────────────┘
              │
              ▼
       ┌──────────────┐
       │  x402-core   │ ◄──┐
       │              │    │ Uses
       │ Policy       │    │
       │ Engine       │    │
       └──────┬───────┘    │
              │            │
              ▼            │
       ┌──────────────┐    │
       │ x402-domain  │────┘
       │              │
       │ Type-Safe    │
       │ Newtypes     │
       └──────────────┘

Dependency Flow: CLI → Server → Core → Domain
                 (clean, no circular dependencies)
```

**Key Design Decisions:**
- **Pure Rust** (KISS principle) - No TypeScript/npm complexity
- **Clean Architecture** - Separated concerns across 5 crates
- **Type Safety** - Domain-driven design with validated newtypes
- **Zero blockchain dependency** - Complete offline testing
- **Security First** - All vulnerabilities patched, 9 security tests

## Project Structure

```
x402-dev/
├── crates/
│   ├── x402-cli/         # CLI binary (main executable)
│   ├── x402-server/      # 🆕 Mock HTTP server (extracted from CLI)
│   ├── x402-core/        # Policy engine & core logic
│   ├── x402-domain/      # 🆕 Shared types & newtypes (type safety)
│   └── xtask/            # Build automation
├── docs/                 # Documentation
│   ├── wave1-validation-report.md     # 🆕 Wave 1 refactoring details
│   └── [epic summaries, guides, etc.]
├── tests/                # Integration tests
└── examples/             # Example projects
    └── policies/         # Policy engine examples
```

**Crate Responsibilities:**
- **x402-cli**: Command-line interface, user interaction
- **x402-server**: HTTP mock server, request handling (768 lines)
- **x402-core**: Policy engine, evaluation, code generation
- **x402-domain**: Type-safe domain models (8 newtypes with validation)
- **xtask**: Build scripts and automation

## 🧪 Testing

```bash
# Run all tests (155+ comprehensive tests)
cargo test --workspace

# Expected: 155+ tests passing
# Test breakdown:
# - x402-core:   80 tests (unit, integration, property, security)
# - x402-domain: 55 tests (newtypes, validation, doc tests)
# - x402-cli:    20 tests (CLI integration)
# - x402-server:  8 tests (HTTP handlers)

# Run specific test suites
cargo test -p x402-core      # Core policy engine tests
cargo test -p x402-domain    # Type safety tests
cargo test --test cli_integration  # CLI end-to-end tests

# Security tests specifically
cargo test -p x402-core security  # 9 security-focused tests
```

## 📖 Documentation

- **[QUICK-START.md](./QUICK-START.md)** - Detailed quick start guide
- **[REAL-WORLD-TESTING-GUIDE.md](./docs/REAL-WORLD-TESTING-GUIDE.md)** - Comprehensive testing scenarios
- **[CLI-TESTING-GUIDE.md](./docs/CLI-TESTING-GUIDE.md)** - CLI usage examples
- **[EPIC-1-COMPLETION-SUMMARY.md](./docs/EPIC-1-COMPLETION-SUMMARY.md)** - Foundation details
- **[EPIC-2-COMPLETION-SUMMARY.md](./docs/EPIC-2-COMPLETION-SUMMARY.md)** - Mock server details
- **🆕 [EPIC-5-COMPLETION-SUMMARY.md](./docs/EPIC_5_COMPLETION_SUMMARY.md)** - Policy engine details
- **[examples/policies/README.md](./examples/policies/README.md)** - Policy engine usage guide
- **[PRD.md](./docs/PRD.md)** - Complete product requirements
- **[epics.md](./docs/epics.md)** - Epic and story breakdown

## Technology Stack

- **Language**: Pure Rust (KISS principle)
- **CLI Framework**: Clap 4.5 (derive API)
- **HTTP Server**: actix-web 4.9 (async)
- **Protocol**: x402 (HTTP 402 + USDC)
- **Blockchain**: Solana (devnet/testnet/mainnet - future)
- **Build Tools**: Cargo (workspace)
- **Binary Size**: 2.5MB (with all crates, LTO, opt-level="z")
- **Type Safety**: 8 validated newtypes (AgentId, Amount, Currency, etc.)

## 🔮 Roadmap

### Completed ✅
- **Epic 1:** Foundation & CLI Infrastructure (7/7 stories)
- **Epic 2:** Mock Facilitator Server (6/6 stories)
- **Epic 5:** Policy Engine & Security (10/10 requirements) - **29 lines YAML → 224 lines middleware!**
- **✨ Wave 1 Refactoring:** Clean architecture, type safety, security hardening
  - Extracted mock server to separate crate (x402-server)
  - Created domain types library (x402-domain) with 8 newtypes
  - Fixed critical security vulnerability (future timestamp bypass)
  - Eliminated all production unwrap() calls
  - 155+ comprehensive tests (unit, integration, security, property)

### Coming Soon 🚧
- **Epic 3:** Automated Test Runner - YAML test suites for CI/CD
- **Epic 4:** Validation Tools - Protocol compliance checking
- **Epic 6:** Developer Examples - 2-minute onboarding templates
- **Epic 7:** Launch Preparation - Demo video & polish

## 🤝 Contributing

This project follows the **KISS (Keep It Simple, Stupid)** and **YAGNI (You Aren't Gonna Need It)** principles:

- Pure Rust implementation
- No premature optimization
- Clear, tested code
- Comprehensive documentation

This is a hackathon project created for the Solana x402 AI Hackathon (October 28 - November 11, 2025).

## 📄 License

MIT License - See [LICENSE](./LICENSE) for details

## 🏆 Hackathon Submission

**Target:** Track 4 - Best x402 Dev Tool ($10,000 prize)

**Value Proposition:**
- **First comprehensive CLI toolkit** for x402 protocol
- **600x faster** developer iteration (3s vs 30min)
- **Zero blockchain dependencies** for testing
- **Production-ready** foundation (14/14 tests passing)
- **🆕 8x code multiplier** - 29 lines YAML → 224 lines middleware (Epic 5)

**Demo:** See [QUICK-START.md](./QUICK-START.md) for 3-minute demo flow

## Resources

- [x402 Protocol Documentation](https://docs.x402.org)
- [Solana Developer Docs](https://docs.solana.com)
- [Corbits Documentation](https://docs.corbits.ai)

## Acknowledgments

Built for the Solana x402 AI Hackathon with support from:
- Solana Foundation
- Visa TAP
- Coinbase CDP
- Switchboard
- Gradient Network

---

**Built with 🦀 Rust | x402 Protocol | Solana**

⭐ Star this repo if you find it useful!
