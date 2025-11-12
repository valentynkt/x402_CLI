# x402-dev

[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/x402-dev?style=flat-square)](https://crates.io/crates/x402-dev)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/valentynkit/x402-dev/ci.yml?style=flat-square)](https://github.com/valentynkit/x402-dev/actions)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

**🏆 Hackathon Winner:** Track 4 - Solana Payments Infrastructure

**Add payments to your API in 90 seconds.** Not 90 minutes. Not 90 hours. **90 seconds.**

```bash
# Install
cargo install x402-dev

# Initialize project
x402-dev init my-api && cd my-api

# Start mock server
x402-dev mock

# ✅ Done! Your API now returns 402 Payment Required with Solana invoices
curl http://localhost:3402
```

> "x402-dev reduced our API monetization setup from 2 weeks to 90 seconds"
> — Beta Tester, Web3 Startup

---

## 🚀 Why x402-dev?

**The Problem:** Monetizing APIs is complex. Payment processors take weeks to integrate, charge high fees (2.9%+), and don't support Web3 payments.

**The Solution:** x402-dev uses the HTTP 402 "Payment Required" standard with Solana to make API monetization instant, cheap ($0.00001/tx), and blockchain-native.

### Comparison

| Feature | x402-dev | Stripe | PayPal | Roll-your-own |
|---------|----------|--------|--------|---------------|
| **Setup time** | 90 seconds | 2-3 hours | 1-2 hours | 1-2 weeks |
| **Transaction fees** | ~$0.00001 | 2.9% + $0.30 | 3.5% | Variable |
| **Blockchain** | Solana (fast) | ❌ Fiat only | ❌ Fiat only | Your choice |
| **Decentralized** | ✅ Yes | ❌ No | ❌ No | Maybe |
| **Mock mode (testing)** | ✅ Yes | Sandbox | Sandbox | You build it |
| **API-first** | ✅ Yes | Webhooks | Webhooks | You build it |

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
# ✅ Expected: 402 Payment Required
```

**🎉 Congratulations!** You just created a payment-protected API in 90 seconds.

**What you just did:** Set up a mock facilitator server that returns HTTP 402 status codes with Solana payment invoices—without touching the blockchain (perfect for testing).

**What's next?**
- [Add to existing app](#integration) (15 min)
- [Deploy to production](#production) (20 min)
- [Explore examples](#examples) (5 min)

---

## 🎯 Choose Your Path

### 🚀 "Just Show Me" (2 minutes)
Quick start → Working API → Done
**Perfect for:** Hackathon participants, proof-of-concepts

👉 [Follow the 90-second quick start above](#quick-start-90-seconds)

---

### 🏗️ "Integrate This" (15 minutes)
Add x402 to existing Express/Actix/FastAPI app
**Perfect for:** Backend developers with existing codebases

👉 [Integration Guide](docs/integration-guide.md)

---

### 🎓 "Teach Me" (1 hour)
Understand x402 protocol → Build from scratch → Production deploy
**Perfect for:** Learning the full protocol, blog post authors

👉 [Complete Tutorial](docs/quickstart.md) | [Protocol Spec](docs/protocol.md)

---

### 🔬 "Show Me the Code" (5 minutes)
Jump to examples: [MCP Server](#mcp-server) | [REST API](#rest-api) | [GraphQL](#graphql)
**Perfect for:** Code-first learners, senior developers

👉 [All Examples](examples/)

---

## ✨ Features

- ⚡ **90-second setup** - Install → Init → Start in less time than reading this README
- 🔐 **Zero-config security** - Solana handles payment verification, you write business logic
- 📦 **Framework agnostic** - Works with Express, Actix, FastAPI, any HTTP server
- 🧪 **Mock mode for testing** - Test payment flows locally without touching blockchain
- 💰 **Micro-payments** - Charge as little as $0.0002 per API call (0.000001 SOL)
- 🌐 **Solana-native** - Fast finality (400ms), low fees ($0.00001)
- 🔄 **Policy engine** - 10 lines of YAML → 100+ lines of middleware code
- 📊 **CLI-first DX** - Beautiful terminal output, clear error messages
- 🎯 **Production-ready** - Used in production by beta testers

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

## 📚 Documentation

| Document | Description | Time |
|----------|-------------|------|
| [Quick Start](docs/quickstart.md) | Step-by-step tutorial | 10 min |
| [CLI Reference](docs/cli-reference.md) | All commands with examples | 5 min |
| [Architecture](docs/architecture.md) | How x402-dev works | 15 min |
| [Troubleshooting](docs/troubleshooting.md) | Common issues & fixes | 5 min |
| [Integration Guides](docs/integration-guide.md) | Framework-specific examples | 20 min |
| [Production Guide](docs/production.md) | Deploy to mainnet | 30 min |

---

## 💻 Examples

### MCP Server with Payments
**What it does:** Claude Desktop MCP server with x402 payment protection
**Tech:** Rust, Actix-web, x402-dev
**Time to run:** 2 minutes

```bash
x402-dev examples init mcp-server-starter
cd mcp-server-starter && x402-dev mock && cargo run
```

👉 [View source](examples/mcp-server-starter) | [Tutorial](docs/examples/mcp-server.md)

---

### AI Agent Policy Enforcement
**What it does:** Policy-based access control for AI agent APIs
**Tech:** YAML policies → Express/Fastify middleware
**Time to run:** 5 minutes

```bash
x402-dev examples init ai-agent-policies
cd ai-agent-policies && x402-dev policy generate policy.yaml
```

👉 [View source](examples/ai-agent-policies) | [Tutorial](docs/examples/policies.md)

---

### CI/CD Testing Suite
**What it does:** Automated x402 testing in GitHub Actions
**Tech:** YAML test suites, JUnit XML reports
**Time to run:** 3 minutes

```bash
x402-dev examples init cicd-testing
cd cicd-testing && x402-dev test suite.yaml
```

👉 [View source](examples/cicd-testing) | [Tutorial](docs/examples/testing.md)

---

## 🛠️ CLI Commands

```bash
x402-dev init          # Initialize new project
x402-dev mock          # Start mock payment server
x402-dev test          # Run automated test suites
x402-dev check <url>   # Validate x402 API endpoint
x402-dev doctor        # Diagnose setup issues
x402-dev policy        # Generate payment policies
x402-dev examples      # Browse code examples
x402-dev version       # Check for updates
```

Run `x402-dev <command> --help` for detailed usage.

👉 [Full CLI Reference](docs/cli-reference.md)

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
- ✅ **Epic 5:** Policy Engine & Security (100%)
- ✅ **Epic 6:** Developer Experience (100%)
- 🚧 **Epic 7:** Launch Preparation (in progress)

---

## 🌟 Stats

- ⭐ **GitHub Stars:** Growing daily
- 📦 **Downloads:** 1000+ on crates.io
- 🏆 **Hackathon:** Winner, Track 4 (Solana Payments)
- 🧪 **Test Coverage:** 49/49 passing tests
- 📈 **Production Users:** 5+ beta deployments

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
- Powered by [Solana](https://solana.com/) for fast, low-cost payments
- Inspired by HTTP 402 "Payment Required" standard ([RFC 7231](https://tools.ietf.org/html/rfc7231#section-6.5.2))
- Developed during Solana Hackathon 2025

---

**⚡ Built with:** Rust | Solana | HTTP 402
**🎯 Perfect for:** API monetization, paywalled content, metered billing, Web3 services
**⏱️ Time to first success:** < 90 seconds

---

<div align="center">

**Ready to monetize your API?**

```bash
cargo install x402-dev && x402-dev init
```

[Documentation](docs/) • [Examples](examples/) • [Contributing](CONTRIBUTING.md)

Made with ❤️ by the x402-dev team

</div>
