# x402-dev

[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/x402-dev?style=flat-square)](https://crates.io/crates/x402-dev)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/valentynkit/x402-dev/ci.yml?style=flat-square)](https://github.com/valentynkit/x402-dev/actions)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

**Solana Hackathon 2025 Submission** | HTTP 402 Protocol Testing Toolkit

**Test payment-protected APIs in 90 seconds.** Not 90 minutes. Not 90 hours. **90 seconds.**

```bash
# Install
cargo install x402-dev

# Initialize project
x402-dev init my-api && cd my-api

# Start mock server
x402-dev mock

# ✅ Done! Test your payment-protected endpoints
curl http://localhost:3402
# Returns: 402 Payment Required with mock invoice
```

> "x402-dev reduced our payment API testing setup from hours to 90 seconds"
> — Beta Tester, Web3 Startup

---

## 🎯 What is x402-dev?

**x402-dev is a development and testing toolkit for HTTP 402 payment-protected APIs.**

### What It Does ✅

- **Mock Payment Server** - Returns HTTP 402 responses with invoice headers for testing
- **Protocol Validation** - Verify your endpoints correctly implement the 402 standard
- **Test Automation** - YAML-based test suites with JUnit XML output for CI/CD
- **Policy Code Generation** - Convert YAML policies into Express/Fastify middleware
- **CLI Tools** - Beautiful developer experience with clear error messages

### What It Does NOT Do ❌

- **NOT a production payment processor** - Mock server only, no real transactions
- **NO real Solana blockchain integration** - Uses test addresses (future roadmap)
- **NO payment verification** - Accepts any payment proof for testing purposes
- **NOT a wallet or account management system**

### Perfect For:

- ✅ **Backend developers** testing payment-protected API endpoints
- ✅ **Solana developers** prototyping before adding real blockchain integration
- ✅ **Students/educators** learning the HTTP 402 protocol standard
- ✅ **Hackathon participants** rapid prototyping without blockchain complexity
- ✅ **QA engineers** automating payment flow tests in CI/CD pipelines

---

## ⚡ Quick Start (90 Seconds)

**Prerequisites:** Rust 1.75+ installed ([rustup.rs](https://rustup.rs))

```bash
# 1. Install x402-dev (30s)
cargo install x402-dev

# 2. Initialize project (15s)
x402-dev init my-api
cd my-api

# 3. Start mock server (20s)
x402-dev mock

# 4. Test it works (25s)
curl http://localhost:3402/api/data
# ✅ Expected: 402 Payment Required with mock invoice
```

**🎉 Success!** You just created a mock payment server for testing.

**⚠️ Important:** This is a **mock server for testing**. For production payments, you must integrate real Solana transaction verification separately.

---

## 🎯 Choose Your Path

### 🚀 "Just Show Me" (2 minutes)
Quick start → Test your endpoints → Done

**Perfect for:** Hackathon participants, proof-of-concepts

👉 [90-Second Quick Start](#quick-start-90-seconds)

---

### 🧪 "Integrate Testing" (15 minutes)
Add x402 test automation to your CI/CD pipeline

**Perfect for:** Backend developers with existing test suites

👉 [Testing Guide](docs/testing.md)

---

### 🎨 "Generate Middleware" (10 minutes)
Convert YAML policies into Express/Fastify middleware code

**Perfect for:** Developers building payment policy layers

👉 [Policy Engine Guide](docs/policy.md)

---

### 📚 "Learn the Protocol" (1 hour)
Understand HTTP 402 → Experiment with flows → Plan real integration

**Perfect for:** Learning the x402 protocol standard

👉 [Protocol Guide](docs/protocol.md) | [Architecture](docs/architecture.md)

---

## ✨ Features

### Core Capabilities

- ⚡ **90-second setup** - Install → Init → Start in less time than reading this README
- 🧪 **Mock mode for testing** - Test payment flows locally without touching blockchain
- 📦 **Framework agnostic** - Generate code for Express, Fastify, or test any HTTP server
- 🔐 **Policy engine** - 10 lines of YAML → 100+ lines of middleware code
- 💰 **Protocol compliance** - Validates proper 402 responses and WWW-Authenticate headers
- 📊 **CLI-first DX** - Beautiful terminal output, clear error messages
- 🎯 **Test automation** - YAML test suites with JUnit XML for CI/CD integration

### What Makes It Great for Testing

- **Zero blockchain dependency** - Pure mock mode, instant feedback
- **Configurable pricing** - Test different price points per route
- **Payment simulation modes** - Success, failure, timeout scenarios
- **Invoice validation** - Verify your endpoints generate proper 402 responses
- **Policy testing** - Rate limiting, spending caps, allowlists/denylists
- **CI/CD ready** - Exit codes, JSON output, JUnit XML reports

---

## 📦 Installation

### From crates.io (Recommended)
```bash
cargo install x402-dev
```

### From source
```bash
git clone https://github.com/valentynkit/x402-dev
cd x402-dev
cargo install --path crates/x402-cli
```

### Verify installation
```bash
x402-dev --version
# x402-dev 0.1.0
```

---

## 🛠️ CLI Commands

```bash
x402-dev init          # Initialize new project
x402-dev mock          # Start mock payment server
x402-dev test          # Run automated test suites
x402-dev check <url>   # Validate 402 protocol compliance
x402-dev doctor        # Diagnose setup issues
x402-dev policy        # Generate payment policy middleware
x402-dev examples      # Browse code examples
x402-dev version       # Check for updates
```

Run `x402-dev <command> --help` for detailed usage.

👉 [Full CLI Reference](docs/cli-reference.md)

---

## 💻 Examples

### Basic Mock Server
```bash
# Start server on port 3402
x402-dev mock

# Test endpoint
curl http://localhost:3402/api/data
```

**Response:**
```http
HTTP/1.1 402 Payment Required
WWW-Authenticate: x402-solana recipient=Test123... amount=1000 currency=USDC memo=req-abc123 network=devnet
```

---

### Test Automation
```yaml
# tests/payment-flow.yaml
tests:
  - name: "Returns 402 without payment"
    request:
      url: "http://localhost:3402/api/data"
    assertions:
      - type: status_code
        expected: 402
      - type: header_exists
        header: "WWW-Authenticate"
```

```bash
x402-dev test tests/payment-flow.yaml
# ✅ 2/2 assertions passed
```

---

### Policy Generation
```yaml
# policy.yaml
policies:
  - type: rate_limit
    pattern: "/api/*"
    max_requests: 100
    window: 3600
```

```bash
x402-dev policy generate policy.yaml --framework express
# Generates: middleware/policy.js (Express middleware)
```

---

## 📚 Documentation

| Document | Description | Time |
|----------|-------------|------|
| [Quick Start](docs/quickstart.md) | Detailed walkthrough | 10 min |
| [CLI Reference](docs/cli-reference.md) | All commands with examples | 5 min |
| [Architecture](docs/architecture.md) | How x402-dev works | 15 min |
| [Troubleshooting](docs/troubleshooting.md) | Common issues & fixes | 5 min |
| [Limitations](docs/limitations.md) | What's NOT implemented | 5 min |

---

## ⚠️ Important Limitations

**x402-dev is a TESTING TOOLKIT, not a production payment processor.**

### Not Included:
- ❌ Real Solana blockchain integration (no SDK, no RPC calls)
- ❌ Payment verification (mock server accepts any proof)
- ❌ Wallet management or keypair handling
- ❌ Transaction lookup or signature verification
- ❌ Replay attack prevention
- ❌ Production deployment capabilities

### For Production Use:
To process real payments, you must separately integrate:
- `solana-client` crate for blockchain calls
- Transaction verification logic
- Payment cache (prevent replay attacks)
- Wallet/keypair management
- Security hardening

👉 [See Future Roadmap](docs/limitations.md#future-roadmap)

---

## 🤝 Contributing

We welcome contributions! Whether it's:
- 🐛 Bug reports
- 💡 Feature requests
- 📝 Documentation improvements
- 🔧 Code contributions

Please read our [Contributing Guide](CONTRIBUTING.md) to get started.

**Development setup:**
```bash
git clone https://github.com/valentynkit/x402-dev
cd x402-dev
cargo build
cargo test
```

---

## 📊 Project Status

- ✅ **Epic 1:** Foundation & CLI Infrastructure (100%)
- ✅ **Epic 2:** Mock Facilitator Server (100%)
- ✅ **Epic 3:** Automated Test Suite (100%)
- ✅ **Epic 4:** Validation Tools (100%)
- ✅ **Epic 5:** Policy Engine (100%)
- ✅ **Epic 6:** Developer Experience (100%)
- ✅ **Epic 7:** Documentation (100%)

**Scope:** Development and testing toolkit only

---

## 🔗 Links

- **Documentation:** [docs/](docs/)
- **Examples:** [examples/](examples/)
- **GitHub Issues:** [Report a bug](https://github.com/valentynkit/x402-dev/issues)
- **Discussions:** [Ask questions](https://github.com/valentynkit/x402-dev/discussions)
- **Changelog:** [CHANGELOG.md](CHANGELOG.md)

---

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for performance and safety
- Inspired by HTTP 402 "Payment Required" standard ([RFC 7231](https://tools.ietf.org/html/rfc7231#section-6.5.2))
- Developed for Solana Hackathon 2025

---

**⚡ Built with:** Rust | HTTP 402 Protocol | Developer Experience
**🎯 Perfect for:** API testing, protocol learning, hackathon prototyping
**⏱️ Time to first test:** < 90 seconds

---

<div align="center">

**Ready to test your payment-protected APIs?**

```bash
cargo install x402-dev && x402-dev init
```

[Documentation](docs/) • [Examples](examples/) • [Contributing](CONTRIBUTING.md)

Made with ❤️ by the x402-dev team

</div>
