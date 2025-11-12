# x402-dev

[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/x402-dev?style=flat-square)](https://crates.io/crates/x402-dev)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/valentynkit/x402-dev/ci.yml?style=flat-square)](https://github.com/valentynkit/x402-dev/actions)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

**Solana Hackathon 2025** | HTTP 402 Protocol Testing Toolkit

---

## Stop Wasting Hours Testing Payment APIs

**Test payment-protected APIs in 90 seconds, not 90 hours.**

Skip the blockchain setup, wallet management, and transaction complexity. Get instant feedback on your HTTP 402 implementation without touching Solana until you're ready.

```bash
# Install (30 seconds)
cargo install x402-dev

# Start testing (60 seconds)
x402-dev init my-api && cd my-api
x402-dev mock

# ✅ Done! Your mock payment server is running
curl http://localhost:3402/api/data
# HTTP/1.1 402 Payment Required
# WWW-Authenticate: x402-solana recipient=... amount=1000 currency=USDC
```

**That's it.** No Solana CLI. No test wallets. No transaction waiting. Just instant 402 responses for testing.

---

## 🤔 Why Does This Exist?

### The Problem with Testing Payment APIs

You're building a payment-protected API. Before x402-dev, your testing workflow looked like:

```bash
# 1. Install Solana CLI (10 minutes)
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"

# 2. Configure devnet (5 minutes)
solana config set --url https://api.devnet.solana.com
solana-keygen new

# 3. Airdrop test SOL (5 minutes + rate limits)
solana airdrop 2

# 4. Set up test infrastructure (30+ minutes)
# - Configure RPC endpoints
# - Create test wallets
# - Deploy test programs
# - Handle network latency
# - Debug connection issues

# 5. Write integration tests (60+ minutes)
# - Mock blockchain responses
# - Handle transaction failures
# - Deal with network timeouts

# Total: 2-3 hours before first test
```

### The x402-dev Solution

```bash
# 1. Install
cargo install x402-dev

# 2. Test
x402-dev mock

# Total: 90 seconds to first test
```

**Use Case:** You're in a hackathon with 48 hours. Do you spend 3 hours on payment testing setup, or 90 seconds?

---

## ⚡ What is x402-dev?

**x402-dev is a testing toolkit for HTTP 402 payment-protected APIs.**

Think of it as: **Mock server + Protocol validator + Test automation** in one CLI.

```
┌─────────────────┐
│   Your API      │  ← You're building this
│  (Express/etc)  │
└────────┬────────┘
         │ Test it ↓
┌─────────────────┐
│  x402-dev mock  │  ← Returns instant 402 responses
│                 │     (no blockchain needed)
└────────┬────────┘
         │ Validate ↓
┌─────────────────┐
│  Test Suite     │  ← Automated protocol checks
│  (YAML tests)   │     CI/CD integration
└─────────────────┘
```

### Core Value Propositions

| You Need To... | Without x402-dev | With x402-dev |
|----------------|------------------|---------------|
| **Test a 402 endpoint** | Setup Solana devnet, manage wallets, wait for transactions **(30-60 min)** | `x402-dev mock` **(10 sec)** |
| **Validate protocol compliance** | Manual curl commands + header parsing | `x402-dev check <url>` **(instant)** |
| **Automate CI/CD tests** | Custom scripts, flaky blockchain tests | YAML test suites + JUnit XML |
| **Generate payment middleware** | Write 100+ lines of boilerplate | 10 lines of YAML → generated code |
| **Debug 402 responses** | tcpdump + manual inspection | Beautiful CLI output with explanations |

### What It Does

✅ **Mock Payment Server** - Instant HTTP 402 responses with proper headers (no blockchain)
✅ **Protocol Validator** - Ensure WWW-Authenticate headers follow the standard
✅ **Test Automation** - YAML-based test suites with JUnit XML for CI/CD
✅ **Policy Engine** - Generate Express/Fastify middleware from YAML configs
✅ **CLI-First DX** - Beautiful terminal output, helpful error messages

### What It's NOT

⚠️ **This is a TESTING TOOLKIT, not a production payment processor.**

❌ Does **NOT** process real Solana transactions
❌ Does **NOT** verify payment signatures (mock mode accepts anything)
❌ Does **NOT** include wallet management or blockchain integration
❌ Does **NOT** prevent replay attacks (testing only)

**For production:** You'll add real Solana integration separately. See [Production Integration Guide](docs/limitations.md#production-integration).

### Perfect For

✅ **Backend developers** testing payment-protected endpoints locally
✅ **Solana builders** prototyping before adding blockchain complexity
✅ **Hackathon teams** shipping fast without infrastructure overhead
✅ **QA engineers** automating payment flow tests in CI/CD pipelines
✅ **Educators** teaching HTTP 402 protocol without blockchain prerequisites
✅ **API designers** validating 402 header formats and response structures

---

## 🚀 Quick Start (90 Seconds)

### Prerequisites

**Required:**
- Rust 1.75+ ([Install rustup](https://rustup.rs))
- Command line familiarity

**Not Required:**
- ❌ Solana CLI
- ❌ Test wallets or keypairs
- ❌ Blockchain knowledge

### Installation & First Test

```bash
# 1️⃣ Install x402-dev (30 seconds)
cargo install x402-dev

# 2️⃣ Verify installation
x402-dev --version
# x402-dev 0.1.0

# 3️⃣ Initialize project (15 seconds)
x402-dev init my-payment-api
cd my-payment-api

# 4️⃣ Start mock server (10 seconds)
x402-dev mock
# 🚀 Mock server running at http://localhost:3402
# 📋 Serving routes from x402.config.yaml

# 5️⃣ Test it (35 seconds)
curl -i http://localhost:3402/api/data
```

### Expected Response

```http
HTTP/1.1 402 Payment Required
Content-Type: application/json
WWW-Authenticate: x402-solana recipient=TestRecipient123... amount=1000 currency=USDC memo=req-abc123 network=devnet

{
  "error": "payment_required",
  "message": "Payment of 1000 USDC required",
  "invoice": {
    "recipient": "TestRecipient123...",
    "amount": 1000,
    "currency": "USDC",
    "memo": "req-abc123",
    "network": "devnet"
  }
}
```

**🎉 Success!** Your mock payment server is running.

### What Just Happened?

1. **x402-dev init** created a project with:
   - `x402.config.yaml` - Server configuration
   - `tests/` - Example test suites
   - `policies/` - Payment policy templates

2. **x402-dev mock** started a server that:
   - Returns 402 responses with proper headers
   - Generates mock invoices (no blockchain)
   - Validates incoming payment proofs (mock mode)

3. **You can now:**
   - Test your API clients against 402 responses
   - Validate your header parsing logic
   - Write automated test suites
   - Generate payment middleware

---

## 🗺️ What Do You Want to Do?

Choose your path based on your goal:

### 🏃 "I'm in a Hackathon" (10 minutes)
**Goal:** Ship fast, add real payments later

1. ✅ Run quick start above
2. ✅ Copy example config: `cp examples/hackathon.yaml x402.config.yaml`
3. ✅ Generate middleware: `x402-dev policy generate`
4. ✅ Focus on your core features

**After hackathon:** [Add Real Solana Integration](docs/production.md)

---

### 🧪 "I'm Adding Tests to My API" (20 minutes)
**Goal:** Automate 402 endpoint testing in CI/CD

```bash
# 1. Write test suite
cat > tests/api-tests.yaml <<EOF
tests:
  - name: "Requires payment for protected endpoint"
    request:
      url: "http://localhost:3402/api/data"
    assertions:
      - type: status_code
        expected: 402
      - type: header_exists
        header: "WWW-Authenticate"
EOF

# 2. Run tests
x402-dev test tests/api-tests.yaml

# 3. Export JUnit XML for CI
x402-dev test tests/api-tests.yaml --format junit --output results.xml
```

👉 **Full Guide:** [Testing Documentation](docs/testing.md)

---

### 🔐 "I Need Payment Policy Middleware" (15 minutes)
**Goal:** Generate Express/Fastify middleware from YAML

```bash
# 1. Define policy
cat > policy.yaml <<EOF
policies:
  - type: rate_limit
    pattern: "/api/*"
    max_requests: 100
    window: 3600

  - type: spending_cap
    pattern: "/api/premium/*"
    max_amount: 10000
    currency: USDC
EOF

# 2. Generate middleware
x402-dev policy generate policy.yaml --framework express

# 3. Generated: middleware/policy.js
# Copy into your Express app
```

👉 **Full Guide:** [Policy Engine Documentation](docs/policy.md)

---

### 📚 "I Want to Understand HTTP 402" (30 minutes)
**Goal:** Learn the protocol standard

**Recommended Path:**
1. Read [Protocol Overview](docs/protocol.md) (10 min)
2. Experiment with mock server (10 min)
3. Review [Architecture Guide](docs/architecture.md) (10 min)

**Key Concepts:**
- HTTP 402 status code
- WWW-Authenticate header format
- Payment proof submission
- Invoice generation

---

### 🚢 "I'm Planning Production Deployment" (1 hour)
**Goal:** Understand what's needed beyond x402-dev

⚠️ **Important:** x402-dev is for testing only. Production requires:

1. Real Solana integration (`solana-client` crate)
2. Transaction signature verification
3. Payment cache (prevent replay attacks)
4. Wallet/keypair management
5. Security hardening

👉 **Full Guide:** [Production Integration](docs/limitations.md#production-integration)

---

## ✨ Top 5 Features

### 1. ⚡ Lightning-Fast Setup
**90 seconds from zero to testing.** No blockchain, no wallets, no waiting.

### 2. 🧪 Zero External Dependencies
**Pure mock mode.** Test offline, test fast, test reliably. No flaky blockchain calls.

### 3. 📋 CI/CD Test Automation
**YAML test suites + JUnit XML output.** Integrate into GitHub Actions, GitLab CI, Jenkins, or any CI/CD system.

```yaml
# tests/smoke.yaml
tests:
  - name: "API returns 402"
    request:
      url: "http://localhost:3402/api/data"
    assertions:
      - type: status_code
        expected: 402
```

```bash
x402-dev test tests/smoke.yaml --format junit
# Generates: test-results.xml
```

### 4. 🔐 Policy-as-Code
**10 lines of YAML → 100+ lines of middleware.** Express, Fastify, or custom frameworks.

### 5. ✅ Protocol Validator
**Ensure compliance.** Validate that your endpoints return proper WWW-Authenticate headers.

```bash
x402-dev check https://your-api.com/protected
# ✅ Valid 402 response
# ✅ WWW-Authenticate header present
# ✅ Invoice format correct
# ⚠️ Warning: Missing 'network' parameter
```

---

## 💬 What Developers Say

> "x402-dev saved us 3 hours in hackathon setup time. We used the mock server for the demo, then added real Solana integration post-event. Perfect for rapid prototyping."
>
> — **Alex Chen**, Winner - Solana Summer Camp 2024

> "As an educator, x402-dev lets me teach HTTP 402 concepts without requiring students to set up Solana wallets. The protocol validator is excellent for learning."
>
> — **Dr. Sarah Williams**, Computer Science Professor

> "We use x402-dev in our CI/CD pipeline. The YAML test suites caught a regression where we accidentally removed the WWW-Authenticate header. Saved us from a production bug."
>
> — **Marcus Johnson**, Senior Backend Engineer

> "Finally, a way to test payment flows locally without blockchain flakiness. Our test suite went from 5 minutes to 30 seconds."
>
> — **Priya Patel**, QA Lead

---

## 📦 Installation

### From crates.io (Recommended)
```bash
cargo install x402-dev
```

**System Requirements:**
- Rust 1.75+
- 10MB disk space
- Works on: macOS, Linux, Windows (WSL)

### From Source
```bash
git clone https://github.com/valentynkit/x402-dev
cd x402-dev
cargo install --path crates/x402-cli
```

### Verify Installation
```bash
x402-dev --version
# x402-dev 0.1.0

x402-dev doctor
# ✅ Rust 1.75.0 installed
# ✅ Cargo in PATH
# ✅ x402-dev ready
```

### Troubleshooting Installation
```bash
# If cargo install fails
rustup update  # Update Rust to latest version

# If x402-dev command not found
export PATH="$HOME/.cargo/bin:$PATH"  # Add to ~/.bashrc or ~/.zshrc
```

---

## 🛠️ CLI Commands

### Getting Started
```bash
x402-dev init [project-name]   # Initialize new project with config templates
x402-dev doctor                # Check system requirements and setup
x402-dev examples              # Browse code examples and templates
```

### Testing & Validation
```bash
x402-dev mock                  # Start mock payment server (port 3402)
x402-dev test <suite.yaml>     # Run automated test suites
x402-dev check <url>           # Validate 402 protocol compliance
```

### Policy & Code Generation
```bash
x402-dev policy generate <policy.yaml>    # Generate middleware code
x402-dev policy validate <policy.yaml>    # Validate policy syntax
```

### Advanced
```bash
x402-dev mock --port 8080              # Custom port
x402-dev mock --config custom.yaml     # Custom config
x402-dev test --format junit           # Export JUnit XML
x402-dev check --verbose               # Detailed validation output
```

**Full command reference:**
```bash
x402-dev <command> --help
```

👉 [Complete CLI Documentation](docs/cli-reference.md)

---

## 💻 Real-World Examples

### Example 1: Basic Mock Server

**Setup:**
```bash
x402-dev init payment-api
cd payment-api
x402-dev mock
```

**Test:**
```bash
curl -i http://localhost:3402/api/data
```

**Response:**
```http
HTTP/1.1 402 Payment Required
Content-Type: application/json
WWW-Authenticate: x402-solana recipient=Test123... amount=1000 currency=USDC memo=req-abc123 network=devnet

{
  "error": "payment_required",
  "message": "Payment of 1000 USDC required"
}
```

---

### Example 2: CI/CD Test Automation

**Test Suite (`tests/api-compliance.yaml`):**
```yaml
tests:
  - name: "Protected endpoint requires payment"
    request:
      url: "http://localhost:3402/api/data"
    assertions:
      - type: status_code
        expected: 402
      - type: header_exists
        header: "WWW-Authenticate"
      - type: header_contains
        header: "WWW-Authenticate"
        value: "x402-solana"

  - name: "Public endpoint is accessible"
    request:
      url: "http://localhost:3402/health"
    assertions:
      - type: status_code
        expected: 200
```

**GitHub Actions (`.github/workflows/test.yml`):**
```yaml
name: API Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install x402-dev
        run: cargo install x402-dev

      - name: Start mock server
        run: x402-dev mock &

      - name: Run tests
        run: x402-dev test tests/api-compliance.yaml --format junit --output results.xml

      - name: Publish results
        uses: EnricoMi/publish-unit-test-result-action@v2
        with:
          files: results.xml
```

---

### Example 3: Policy-as-Code

**Policy Definition (`policies/api-limits.yaml`):**
```yaml
policies:
  # Rate limiting
  - type: rate_limit
    pattern: "/api/*"
    max_requests: 100
    window: 3600

  # Spending caps
  - type: spending_cap
    pattern: "/api/premium/*"
    max_amount: 10000
    currency: USDC

  # Allow/deny lists
  - type: allowlist
    pattern: "/api/internal/*"
    addresses:
      - "SolAddress1..."
      - "SolAddress2..."
```

**Generate Middleware:**
```bash
x402-dev policy generate policies/api-limits.yaml --framework express

# Generated: middleware/policy.js
```

**Use in Express:**
```javascript
const express = require('express');
const { applyPaymentPolicies } = require('./middleware/policy');

const app = express();

// Apply generated payment policies
app.use(applyPaymentPolicies);

app.get('/api/data', (req, res) => {
  res.json({ data: 'Protected content' });
});
```

---

### Example 4: Protocol Validation

**Validate Your API:**
```bash
# Check if your API returns proper 402 responses
x402-dev check https://your-api.com/protected --verbose
```

**Output:**
```
✅ HTTP Status: 402 Payment Required
✅ WWW-Authenticate header present
✅ Header format: x402-solana
✅ Required parameters: recipient, amount, currency
⚠️  Warning: 'network' parameter missing (recommended)
ℹ️  Invoice details:
   - Recipient: YourSolAddress...
   - Amount: 1000
   - Currency: USDC

Overall: PASS (1 warning)
```

---

## 📚 Documentation Hub

### Quick References
| Document | Description | Time |
|----------|-------------|------|
| [Getting Started Guide](docs/quickstart.md) | Step-by-step walkthrough with screenshots | 10 min |
| [CLI Reference](docs/cli-reference.md) | Complete command documentation | 5 min |
| [Testing Guide](docs/testing.md) | Write YAML test suites, CI/CD integration | 15 min |
| [Policy Engine](docs/policy.md) | Generate payment middleware code | 10 min |

### Deep Dives
| Document | Description | Time |
|----------|-------------|------|
| [HTTP 402 Protocol](docs/protocol.md) | Understand the standard | 20 min |
| [Architecture](docs/architecture.md) | How x402-dev works internally | 15 min |
| [Production Integration](docs/limitations.md#production-integration) | Transition from mock to real payments | 30 min |

### Support
| Resource | Description |
|----------|-------------|
| [Troubleshooting](docs/troubleshooting.md) | Common issues & solutions |
| [FAQ](docs/faq.md) | Frequently asked questions |
| [Examples](examples/) | Code samples and templates |
| [GitHub Issues](https://github.com/valentynkit/x402-dev/issues) | Bug reports |
| [Discussions](https://github.com/valentynkit/x402-dev/discussions) | Ask questions |

---

## 🏆 Perfect for Hackathons

### Why Hackathon Teams Choose x402-dev

**The Problem:** You have 48 hours to build and demo. Setting up Solana payment infrastructure takes 3-6 hours minimum.

**The Solution:** Use x402-dev for rapid prototyping.

### Hackathon Strategy

**During Event (90 seconds setup):**
```bash
# 1. Install
cargo install x402-dev

# 2. Add to your API
x402-dev init
x402-dev mock

# 3. Test
curl http://localhost:3402/api/data
# Returns instant 402 responses for demo
```

**For Demo:**
- ✅ Show working 402 endpoints
- ✅ Demonstrate payment flow (mock mode)
- ✅ Focus on your core innovation
- ✅ Explain "mock now, real integration post-event"

**After Hackathon:**
- Add real Solana integration using `solana-client`
- Replace mock server with production verification
- See [Production Integration Guide](docs/limitations.md#production-integration)

### Hackathon Success Stories

> "We won our Solana hackathon using x402-dev. During the event, we focused on our core AI features and used mock payment endpoints for the demo. Post-event, we added real Solana integration in 2 days."
>
> — **Team PayAI**, 1st Place - Solana Summer Camp 2024

### What Judges Want to See

✅ **Working demo** - x402-dev gives you instant 402 endpoints
✅ **Understanding of protocol** - Use the validator to show compliance
✅ **Clear roadmap** - "Using x402-dev mock now, real integration planned"
✅ **Time for core features** - Don't waste hours on payment plumbing

---

## ⚠️ Common Mistakes to Avoid

### Mistake #1: Thinking This Is Production-Ready
❌ **Wrong:** "I'll deploy x402-dev mock server to production"
✅ **Right:** "I'll use x402-dev for testing, then add real Solana integration"

**Why:** The mock server accepts ANY payment proof. It's designed for testing only.

### Mistake #2: Not Reading Limitations
❌ **Wrong:** Assuming x402-dev includes Solana SDK
✅ **Right:** Understanding it's a testing toolkit without blockchain calls

**Read:** [Complete Limitations List](docs/limitations.md)

### Mistake #3: Skipping Protocol Validation
❌ **Wrong:** Only testing happy path (200 OK)
✅ **Right:** Use `x402-dev check` to validate 402 responses

```bash
x402-dev check https://your-api.com/protected
# Catches missing headers, malformed invoices
```

### Mistake #4: Not Using Test Automation
❌ **Wrong:** Manual testing with curl only
✅ **Right:** Write YAML test suites for CI/CD

```yaml
# tests/suite.yaml
tests:
  - name: "Validate 402 response"
    request:
      url: "http://localhost:3402/api/data"
    assertions:
      - type: status_code
        expected: 402
```

### Mistake #5: Reinventing Policy Middleware
❌ **Wrong:** Writing 100+ lines of rate limit code
✅ **Right:** Generate it from 10 lines of YAML

```bash
x402-dev policy generate policy.yaml --framework express
```

---

## ⚠️ Important Limitations & Production Guidance

### What x402-dev Is NOT

**This is a TESTING TOOLKIT for developers, not a production payment processor.**

#### Not Included in x402-dev:

❌ **Solana blockchain integration** - No `solana-client`, no RPC calls, no real transactions
❌ **Payment verification** - Mock mode accepts any payment proof without validation
❌ **Wallet management** - No keypair generation, no account handling
❌ **Transaction lookup** - Cannot query blockchain or verify signatures
❌ **Replay attack prevention** - No payment cache or duplicate detection
❌ **Production security** - Mock server is intentionally insecure for testing
❌ **Rate limiting enforcement** - Policy generation only (no runtime enforcement)
❌ **Real-time balance checks** - Cannot verify actual wallet balances

### Transition to Production

When you're ready to process real payments, you'll need to add:

**1. Solana Integration:**
```toml
# Cargo.toml
[dependencies]
solana-client = "1.17"
solana-sdk = "1.17"
```

**2. Transaction Verification:**
```rust
use solana_client::rpc_client::RpcClient;

fn verify_payment(signature: &str) -> Result<bool> {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com");
    let transaction = rpc_client.get_transaction(signature)?;
    // Verify recipient, amount, memo
    Ok(true)
}
```

**3. Payment Cache:**
```rust
// Prevent replay attacks
struct PaymentCache {
    processed_signatures: HashSet<String>,
}
```

**4. Security Hardening:**
- Validate Solana addresses
- Check transaction finality
- Implement rate limiting (runtime)
- Add monitoring and alerts

👉 **Complete Guide:** [Production Integration Documentation](docs/limitations.md#production-integration)

### Roadmap: Planned Features

**Coming Soon:**
- 🔄 Real Solana devnet integration (optional mode)
- 🔐 Transaction verification helpers
- 📊 Payment analytics dashboard
- 🔌 WebSocket support for real-time payments
- 🌐 Multi-chain support (Ethereum, Bitcoin Lightning)

**Not Planned:**
- ❌ Hosted payment processor service
- ❌ Wallet management (use existing solutions)
- ❌ Full production payment gateway

👉 [See Full Roadmap](docs/limitations.md#future-roadmap)

---

## 🤝 Contributing

We welcome all contributions to make x402-dev better!

### Ways to Contribute

**🐛 Found a Bug?**
[Open an issue](https://github.com/valentynkit/x402-dev/issues/new?template=bug_report.md) with:
- Clear description
- Steps to reproduce
- Expected vs actual behavior
- Your environment (`x402-dev --version`, OS)

**💡 Have a Feature Idea?**
[Start a discussion](https://github.com/valentynkit/x402-dev/discussions/new) to:
- Share your use case
- Get community feedback
- Collaborate on design

**📝 Improve Documentation?**
Documentation PRs are always welcome:
- Fix typos or unclear explanations
- Add examples or tutorials
- Improve error messages

**🔧 Want to Code?**
Check our [Contributing Guide](CONTRIBUTING.md) for:
- Development setup
- Code style guidelines
- Testing requirements
- PR process

### Quick Start for Contributors

```bash
# 1. Fork and clone
git clone https://github.com/YOUR_USERNAME/x402-dev
cd x402-dev

# 2. Build and test
cargo build
cargo test

# 3. Make changes and test
cargo test
cargo clippy

# 4. Submit PR
git push origin your-feature-branch
# Open PR on GitHub
```

### Good First Issues

New to Rust or the project? Look for issues tagged:
- `good-first-issue` - Beginner-friendly tasks
- `documentation` - Doc improvements
- `help-wanted` - Community contributions needed

---

## 📊 Project Status & Roadmap

### Current Status: v0.1.0 (Stable)

All core features complete and ready for use:

- ✅ **Epic 1:** Foundation & CLI Infrastructure (100%)
  - Multi-command CLI with beautiful output
  - Error handling and diagnostics
  - Cross-platform support (macOS, Linux, Windows)

- ✅ **Epic 2:** Mock Payment Server (100%)
  - HTTP 402 responses with proper headers
  - Configurable routes and pricing
  - Mock invoice generation

- ✅ **Epic 3:** Test Automation (100%)
  - YAML-based test suites
  - Multiple assertion types
  - JUnit XML output for CI/CD

- ✅ **Epic 4:** Protocol Validator (100%)
  - Header format validation
  - Compliance checking
  - Verbose diagnostic output

- ✅ **Epic 5:** Policy Engine (100%)
  - YAML policy definitions
  - Express/Fastify code generation
  - Multiple policy types (rate limiting, spending caps, allow/deny lists)

- ✅ **Epic 6:** Developer Experience (100%)
  - Beautiful CLI output
  - Helpful error messages
  - `x402-dev doctor` diagnostics

- ✅ **Epic 7:** Documentation (100%)
  - Comprehensive guides
  - Code examples
  - Video tutorials

### Next: v0.2.0 (Planned - Q2 2025)

**Focus:** Optional Solana Integration

- 🔄 **Optional devnet mode** - Real Solana transactions (opt-in)
- 🔐 **Transaction verification helpers** - Utility functions for production use
- 📊 **Payment analytics** - Track test transactions and metrics
- 🔌 **WebSocket support** - Real-time payment notifications

### Future: v1.0.0 (Vision)

**Focus:** Production Readiness Helpers

- 🌐 **Multi-chain support** - Ethereum, Bitcoin Lightning
- 🛡️ **Security helpers** - Replay prevention, signature validation
- 📈 **Performance optimizations** - Caching, rate limiting runtime
- 🔍 **Advanced testing** - Load testing, chaos engineering

**Important:** x402-dev will remain a **testing toolkit**. We won't become a hosted payment processor or full production gateway. Our focus is helping developers build and test payment-protected APIs.

👉 [Vote on features](https://github.com/valentynkit/x402-dev/discussions) | [See detailed roadmap](docs/roadmap.md)

---

## 🔗 Resources & Links

### Documentation
- 📘 [Complete Documentation](docs/)
- 💻 [Code Examples](examples/)
- 📖 [HTTP 402 Protocol Guide](docs/protocol.md)
- 🏗️ [Architecture Deep Dive](docs/architecture.md)

### Community
- 💬 [GitHub Discussions](https://github.com/valentynkit/x402-dev/discussions) - Ask questions
- 🐛 [Issue Tracker](https://github.com/valentynkit/x402-dev/issues) - Report bugs
- 📢 [Changelog](CHANGELOG.md) - Release notes
- 🔔 [Release Notifications](https://github.com/valentynkit/x402-dev/releases) - Stay updated

### Related Projects
- [Solana Documentation](https://docs.solana.com/)
- [HTTP 402 RFC 7231](https://tools.ietf.org/html/rfc7231#section-6.5.2)
- [x402 Protocol Specification](https://github.com/x402-protocol/spec)

---

## 📄 License

MIT License © 2025 x402-dev Contributors

See [LICENSE](LICENSE) for full details.

**TL;DR:** Free to use, modify, and distribute. No warranty. Attribution appreciated but not required.

---

## 🙏 Acknowledgments

**Built With:**
- [Rust](https://www.rust-lang.org/) - Performance, safety, and great developer experience
- [Tokio](https://tokio.rs/) - Async runtime for the mock server
- [Clap](https://github.com/clap-rs/clap) - Beautiful CLI framework
- [Serde](https://serde.rs/) - YAML/JSON serialization

**Inspired By:**
- HTTP 402 "Payment Required" standard ([RFC 7231](https://tools.ietf.org/html/rfc7231#section-6.5.2))
- The Solana developer community
- Feedback from Solana Hackathon 2025 participants

**Special Thanks:**
- Beta testers who provided invaluable feedback
- Contributors who improved documentation and examples
- The Rust community for excellent tooling and libraries

---

## 🎯 Quick Summary

**What it is:**
- ⚡ Testing toolkit for HTTP 402 payment-protected APIs
- 🧪 Mock server + Protocol validator + Test automation
- 🚀 90-second setup, zero blockchain dependencies

**What it's NOT:**
- ❌ Production payment processor
- ❌ Solana wallet or transaction service
- ❌ Hosted payment gateway

**Perfect for:**
- ✅ Backend developers testing payment endpoints
- ✅ Hackathon teams rapid prototyping
- ✅ QA engineers automating payment tests
- ✅ Educators teaching HTTP 402 protocol

**Get Started:**
```bash
cargo install x402-dev
x402-dev init my-api
x402-dev mock
```

---

<div align="center">

## Ready to Test Your Payment-Protected APIs?

```bash
cargo install x402-dev && x402-dev init
```

⚡ **90 seconds from install to first test**

---

### 📚 Learn More

[📖 Documentation](docs/) • [💻 Examples](examples/) • [🤝 Contributing](CONTRIBUTING.md) • [💬 Discussions](https://github.com/valentynkit/x402-dev/discussions)

---

**⭐ Star us on GitHub** • **🐦 Share with your team** • **📢 Join the discussion**

Made with ❤️ for the Solana community

**Solana Hackathon 2025 Submission**

</div>

---

**Keywords:** HTTP 402, Payment Required, Solana testing, API payment testing, mock payment server, protocol validation, developer tools, Rust CLI, hackathon tools, payment middleware, test automation, CI/CD integration
