# x402-dev - Product Requirements Document

**Author:** Valik
**Date:** 2025-11-05
**Version:** 1.0

---

## Executive Summary

x402-dev is the **x402 Protocol Standard Toolkit**—essential infrastructure for Solana AI agent developers. It provides testing, validation, and security capabilities that reduce development cycles from weeks to days, enabling developers to test x402 payment flows locally, validate implementations, and enforce payment policies without writing custom code.

**Target Market:** x402 protocol developers building AI agent payment infrastructure on Solana

**Primary Goal:** Win Track 4 (Best x402 Dev Tool) - $10,000 prize + demonstrate technical excellence to Solana Foundation AI team

**Timeline:** 6 days (November 5-11, 2024)

**Value Proposition:** **"x402 development—done right, in seconds"**

### What Makes This Special

The magic of x402-dev lies in **transforming developer workflow from hours to seconds**:
- **30 minutes → 30 seconds:** Manual testing cycle becomes automated command
- **Zero blockchain dependencies:** Test payment flows locally without deploying
- **Confidence before deployment:** Validate x402 compliance before going live
- **Security made simple:** YAML policies eliminate 100+ lines of boilerplate code

This is the **first comprehensive CLI toolkit** for x402 development—filling 15+ documented gaps across testing, validation, and security that currently force developers to write custom code or use slow manual processes.

### Strategic Positioning

x402-dev positions itself as **essential ecosystem infrastructure**, not a vendor product:
- **Protocol-aligned**: Works with all x402 implementations (Corbits, PayAI, CDP)
- **Open source**: LGPL-3.0 license enables community contributions
- **Distribution-first**: Designed for discovery via examples, partnerships, and community
- **Network effects**: Policy library and examples create increasing value with adoption

**Ecosystem Impact:** Every x402 developer saved 60+ hours accelerates the Solana AI agent economy by reducing time-to-market for autonomous payment applications.

---

## Project Classification

**Technical Type:** CLI Tool / Developer Infrastructure
**Domain:** General Software Development Tools
**Complexity:** Medium

**Classification Details:**
- **Developer-facing tool:** Not end-user application
- **Multi-feature toolkit:** Three integrated feature sets (Testing, Validation, Security)
- **Ecosystem infrastructure:** Public goods positioning (developer productivity)
- **Pure Rust Implementation:** Single-language codebase with actix-web HTTP server (KISS principle)
- **Cross-platform:** macOS, Linux, Windows support via Rust (single binary distribution ~2-3MB)

---

## x402 Protocol Context

### Protocol Overview

x402 extends HTTP 402 "Payment Required" status code with crypto payment invoices for autonomous AI agent transactions:

**Payment Flow:**
1. **Client requests protected resource** → `GET /api/data`
2. **Server responds with 402 + invoice** → HTTP 402 + `WWW-Authenticate` header containing payment invoice
3. **Client constructs USDC transaction** → Solana blockchain transaction
4. **Payment verified via facilitator** → <2 seconds confirmation
5. **Resource delivered** → After payment confirmation, server provides requested data

### Invoice Structure (USDC on Solana)

```json
{
  "recipient": "GXk8v...qPz9" ,
  "amount": "0.01",
  "currency": "USDC",
  "memo": "req_abc123_resource_data",
  "network": "devnet"
}
```

**Invoice transmitted via WWW-Authenticate header:**
```http
HTTP/1.1 402 Payment Required
WWW-Authenticate: x402-solana recipient=GXk8v...qPz9 amount=0.01 currency=USDC memo=req_abc123 network=devnet
```

### Key Concepts

**Facilitator**: Payment verification service that handles transaction confirmation
- Examples: PayAI Network, Corbits/Faremeter
- Role: Verifies payment on-chain, notifies both parties
- Performance: <2 second confirmation time

**Agent**: AI service making autonomous payments
- Characterized by: agent_id, wallet_address, spending patterns
- Behavior: Reads 402 response → constructs transaction → submits to facilitator

**Policy**: Security rules governing payment authorization
- Controls: which agents allowed, rate limits, spending caps, allowlists/denylists
- Enforcement: Before generating 402 response OR in middleware

### Why x402 Matters for AI Agents

Traditional payment APIs require:
- API keys (security risk for autonomous agents)
- Credit cards (human intervention)
- Complex OAuth flows (not agent-friendly)

x402 protocol enables:
- **Autonomous payments** - No human in the loop
- **Micropayments** - Transactions as small as $0.0001
- **Instant settlement** - <2 second confirmation
- **No intermediaries** - Direct blockchain settlement

### Reference Documentation

For complete protocol specification see:
- **Local:** `hackathon-research/x402-protocol-specification.md`
- **Official:** https://docs.x402.org

---

## Success Criteria

### Hackathon Success (Primary - November 2024)

**Must Achieve:**
- ✅ **Win Track 4: Best x402 Dev Tool** ($10,000 prize)
  - Confidence level: 85% (calculated from strategic analysis - see Product Brief)
  - Differentiation: First comprehensive CLI toolkit for x402
  - Universal need: Every x402 developer needs testing/validation/security
  - Lowest competition: Track 4 less glamorous than agent apps (Track 1/3/5)

**Quality Indicators:**
- **Exceptional demo video** (3 minutes): Showcases time savings (30 min → 30 sec)
- **Complete documentation**: README, architecture diagrams, 10+ code examples
- **Functional completeness**: All 3 feature sets working (Testing, Validation, Security)
- **Zero blockchain dependencies**: Full testing without Solana devnet deployment

**Pre-Launch Validation:**
- **10+ beta users** - Developers testing pre-release during hackathon week
- **User testimonials** - At least 3 developers provide feedback for demo video
- **Discord/Telegram engagement** - Post in x402 communities, gather interest
- **GitHub stars day-1** - Target 20+ stars from beta users on launch

**Why this matters:** Traction before launch = judges see validation, not just promises.

**Secondary Opportunities:**
- Track 2: Best x402 API Integration ($10k) - if dogfooding demo impresses
- **Realistic prize range: $10-20k**

### Developer Adoption Success (Post-Hackathon)

**Month 1 Targets (Updated):**
- **100+ npm downloads** (increased from 50—higher bar reflects "essential infrastructure" positioning)
- **10+ GitHub stars** (social proof)
- **5+ real-world usage testimonials** (use case diversity)
- **Corbits docs mention** (partnership initiated)
- **3+ community policy contributions** (network effects starting)

**Month 3 Targets (Updated):**
- **500+ npm downloads** (unchanged—validating sustained adoption)
- **Corbits SDK integration** (doctor command detects Corbits, provides tailored recommendations)
- **Policy library: 20+ policies** (community engaged, network effects visible)
- **Referenced in 2+ blog posts** (organic content creation)
- **Solana Foundation grant application submitted** ($25k-$50k for production hardening)

### Long-Term Career Success (2026+)

**Strategic Goal: Join Solana Foundation AI Team**
- **Solana Foundation Grant**: $25k-$50k for production hardening (security/developer tools RFP)
- **Standard tool status**: Becomes to x402 what Postman is to REST APIs
- **Interview positioning**: "I build the infrastructure Solana AI agents need"
- **Public goods demonstration**: Developer productivity infrastructure aligns with SF priorities

### Measurable User Success

**Developer Time Savings:**
- **Before x402-dev**: 30 minutes per test cycle (manual)
- **After x402-dev**: 30 seconds per test cycle (automated)
- **Impact**: 60x faster iteration speed

**Confidence Before Deployment:**
- **Before**: Deploy to testnet → test → debug → redeploy (4-6 hours)
- **After**: Local validation → deploy once (30 minutes)
- **Impact**: Deploy with confidence, eliminate debug loops

**Security Code Elimination:**
- **Before**: 100+ lines of custom policy enforcement per project
- **After**: 10-line YAML policy file
- **Impact**: 90% code reduction, zero security logic duplication

### Business Metrics

**Not Revenue-Focused (Open Source Tool):**
- Primary value: Career positioning + ecosystem contribution
- No monetization required for success
- Future premium offering possible (cloud-hosted mock servers) but not necessary

---

## User Personas

### Primary User: x402 Protocol Developers

**Profile:**
- **Role:** Backend engineers building AI agent payment infrastructure
- **Tech Stack:** TypeScript/JavaScript (primary), Python, Go
- **Context:** Building APIs, MCP servers, or agent payment gateways
- **Experience Level:** Intermediate to advanced developers
- **Team Size:** Solo developers to small teams (2-5 people)

**Current Workflow (Without x402-dev):**
1. Write x402 integration code (2-4 hours)
2. Deploy to testnet for testing (30 min setup)
3. Manual testing via curl/Postman (30 min per test cycle)
4. Debug failures with blockchain explorers (1-2 hours)
5. Write custom security code (2-3 hours)
6. Deploy to production (hoping it works)

**Pain Points:**
- _"I spent 4 hours debugging a typo in x402 headers"_
- _"Manual testing is killing my iteration speed"_
- _"I'm copying security code from my last project"_
- _"Can't test payment flows in CI/CD"_
- _"PayAI Echo Merchant requires manual workflows - can't automate"_

**Validation Sources:**
- x402 Discord community feedback (Oct-Nov 2024)
- PayAI Echo Merchant user complaints (manual workflow limitations)
- Corbits SDK GitHub issues (#42, #58, #71 - testing pain points documented)
- Direct conversations with 3 x402 developers during research phase
- Hackathon organizers created Track 4 specifically for dev tooling gaps

**Quote from x402 Developer:**
> "I spent 4 hours debugging a header typo that would've taken 30 seconds with proper validation. We need better tooling or this protocol won't scale." - Developer, x402 Discord, Oct 28 2024

**What They Value Most:**
1. **Speed:** Local testing without blockchain wait times (30 min → 30 sec)
2. **Confidence:** Validate implementation before deploying
3. **Security:** Pre-built policy enforcement (eliminate 100+ lines boilerplate)
4. **Automation:** CI/CD integration for continuous testing

**Success Metrics (from their perspective):**
- ✅ Reduce testing time from 30 minutes to 30 seconds
- ✅ Deploy with confidence (validated locally first)
- ✅ Eliminate security code duplication across projects
- ✅ Run x402 tests in GitHub Actions workflow

**Jobs to be Done:**
- **Test x402 payment flows** without deploying to blockchain
- **Validate protocol compliance** before going live
- **Enforce payment policies** without writing custom code
- **Debug failed transactions** quickly with clear error messages
- **Integrate with CI/CD** for automated testing

### Secondary User: x402 API Consumers

**Profile:**
- **Role:** Frontend developers, AI agent developers, DevOps engineers
- **Context:** Integrating with x402 APIs or monitoring payment infrastructure
- **Needs:** Verify API correctness, debug failed payments, monitor health

**How x402-dev Helps Them:**
- `verify` commands check API implementation correctness
- `monitor` commands tail payment logs for debugging
- Test suites validate API behavior before integration

**Use Cases:**
- Frontend developer: "Does this x402 API comply with the protocol?"
- Agent developer: "Why did my payment fail?"
- DevOps engineer: "Is the payment gateway healthy?"

---

## Product Scope

### MVP - Minimum Viable Product (Hackathon Deadline: Nov 11, 2024)

**Core Value Delivery: Complete testing, validation, and security workflow**

**Feature Set A: Testing & Mocking** (8 hours)
1. **Mock Facilitator Server**
   - Responds with `402 Payment Required` + payment invoices
   - Configurable pricing rules (per-request, per-resource)
   - Success/failure simulation modes
   - No blockchain dependency required

2. **Automated Test Runner**
   - YAML test suite definitions
   - Happy path + error scenario coverage
   - Assertion framework for x402 responses
   - CI/CD compatible (exit codes, JSON output)
   - Example: `x402-dev test ./tests/payment-flow.yaml`

**Feature Set B: Validation & Debugging** (6 hours)
3. **Header Verification**
   - Check x402 protocol compliance
   - Validate invoice structure (amount, recipient, memo)
   - Suggest fixes for common errors
   - Example: `x402-dev verify headers https://api.example.com/resource`

4. **Transaction Monitoring** (optional for mock-only testing)
   - Query Solana RPC for real transaction status
   - Track transaction confirmation on devnet/testnet/mainnet
   - Request/response logging (structured JSON)
   - Example: `x402-dev monitor --url https://api.example.com`
   - Note: Mock server (Feature #1) works offline without this feature

**Feature Set C: Security & Policy** (6 hours)
5. **Policy Enforcement Engine**
   - YAML policy definitions (allowlists, denylists, rate limits, spending caps)
   - Rule evaluation logic
   - Policy validation (check for conflicts)
   - Example policy file:
     ```yaml
     policies:
       - type: allowlist
         agents: [agent-id-1, agent-id-2]
       - type: rate_limit
         max_requests: 100
         window: 3600
     ```

6. **Middleware Generation**
   - Express.js integration code
   - Fastify.js integration code
   - Automatic policy enforcement in API routes
   - Audit logging (CSV/JSON export)
   - Example: `x402-dev policy generate policy.yaml --framework express --output middleware.js`

**Feature Set D: Example Library & Quick Start** (3 hours)
7. **Example Scaffolding**
   - Pre-built example projects for common use cases
   - One-command project initialization
   - Includes: MCP server starter, AI agent with policies, CI/CD templates
   - Example: `x402-dev examples init mcp-server-starter`
   - **Distribution strategy**: Examples discoverable via search → drives x402-dev adoption

8. **Doctor Command**
   - System diagnostics and compatibility checks
   - Detects Corbits SDK, validates config, checks port availability
   - Provides actionable fix suggestions
   - Example: `x402-dev doctor`
   - **Partnership strategy**: Corbits users get tailored recommendations

**MVP Success Criteria:**
- ✅ All 8 core features functional (including examples + doctor command)
- ✅ Zero blockchain dependencies for testing
- ✅ <5 minute setup time (npm install + first command)
- ✅ <2 minute setup with examples (x402-dev examples init)
- ✅ Clear error messages with actionable suggestions
- ✅ Commands execute <1 second (excluding network calls)
- ✅ Color-coded terminal output with progress indicators
- ✅ README with quickstart + 10+ code examples
- ✅ At least 2 working example projects (MCP server, AI agent)
- ✅ Architecture diagram
- ✅ 3-minute demo video (featuring beta user testimonials)
- ✅ 10+ beta users with feedback before launch

**Explicitly Out of Scope for MVP:**
- ❌ Interactive mode (`x402-dev interactive`)
- ❌ Jest/Vitest plugins
- ❌ Multi-chain support (Base, Ethereum)
- ❌ Cloud-hosted mock server
- ❌ VS Code extension
- ❌ UI dashboard
- ❌ Load testing capabilities

### Growth Features (Post-MVP / Post-Hackathon)

**Phase 1: Developer Experience Enhancements** (Q1 2026)
- **Interactive wizard mode**: `x402-dev interactive` for guided setup
- **Test framework plugins**: Jest/Vitest matchers for x402 assertions
- **Watch mode**: Auto-rerun tests on file changes
- **Configuration templates**: Pre-built configs for common use cases
- **Replay functionality**: Save/replay failed payment flows
- **Enhanced debugging**: Transaction replay, state inspection, visual trace logs

**Phase 2: CI/CD Integration** (Q1 2026)
- **GitHub Actions integration**: Pre-built workflow templates
- **GitLab CI templates**: Pipeline configurations
- **Docker images**: Containerized mock server for CI environments
- **Notification hooks**: Slack/Discord alerts for test failures
- **Coverage reporting**: Track x402 payment flow coverage

**Phase 3: Multi-Chain Support** (Q2 2026)
- **Base network support**: Extend beyond Solana
- **Ethereum Layer 2s**: Optimism, Arbitrum, Polygon
- **Chain-agnostic testing**: Same test suite, multiple chains
- **Cross-chain payment scenarios**: Test multi-chain flows

### Vision (Future / Post-Grant Funding)

**Version 2.0: Advanced Testing & Security** ($25k-$50k Solana Foundation Grant)

**Advanced Testing:**
- **Load testing**: Simulate high-volume payment scenarios
- **Chaos engineering**: Inject failures (network errors, timeout simulation)
- **Performance profiling**: Identify bottlenecks in payment flows
- **Scenario builder**: Visual tool for complex test scenarios

**Security Enhancements:**
- **ML-based fraud detection**: Anomaly detection in payment patterns
- **Threat intelligence**: Blocklists from community-reported abuse
- **Compliance reporting**: Audit trail export for regulatory needs
- **Advanced policies**: Time-based rules, geo-restrictions, spending velocity

**Developer Experience:**
- **IDE plugins**: VS Code, IntelliJ extensions with inline diagnostics
- **Interactive debugging**: REPL mode for exploring x402 flows
- **Visual flow diagrams**: Generate sequence diagrams from test runs
- **Natural language queries**: "Show me failed payments in the last hour"

**Enterprise Features:**
- **Multi-tenant policy management**: Centralized policy control
- **SSO integration**: Enterprise authentication
- **Cloud service**: Hosted mock servers (SaaS offering)
- **Team collaboration**: Shared test suites, policy libraries

**Community & Ecosystem:**
- **Public policy library**: Community-curated security policies
- **Plugin ecosystem**: Third-party extensions
- **Official Solana AI docs integration**: Become standard tool

---

## Innovation & Novel Patterns

### Innovation Pattern: First-Mover Developer Tooling

**What's Novel:**
x402-dev is the **first comprehensive CLI toolkit** for the x402 payment protocol. While the protocol itself exists (x402), the developer tooling ecosystem is nascent:
- No automated testing frameworks exist
- No validation tools available
- No policy enforcement infrastructure
- No CI/CD integration capabilities

**Market Validation:**
- **15+ documented gaps** in existing tooling (PayAI Echo Merchant, x402scan, SDK docs)
- **10,000% transaction growth** in x402 protocol (1 month, October 2025)
- **500,000+ weekly transactions** demonstrating ecosystem momentum
- **Track 4 prize exists** because organizers recognize the tooling gap

**Innovation Type:** Infrastructure innovation (not product innovation)
- Building the "missing layer" of developer productivity tools
- Analogous to: Postman for REST APIs, Swagger for OpenAPI

### Validation Approach

**Assumption: Developers will adopt CLI tooling over manual testing**

**Validation Strategy:**
1. **Hackathon validation** (Nov 2024):
   - Track 4 win = judges validate the gap exists
   - 10+ developer adoptions during hackathon period
   - Demo video showcases time savings (quantitative proof)

2. **Post-hackathon validation** (Month 1):
   - 50+ npm downloads = organic adoption signal
   - GitHub issues/PRs = developers finding it useful enough to contribute
   - Integration mentions in SDK docs = ecosystem recognition

3. **Fallback if adoption is slow:**
   - Still valuable as career demonstration (Solana Foundation positioning)
   - Can pivot to SaaS offering (hosted mock servers) if CLI distribution fails
   - Learnings inform next tool (e.g., VS Code extension instead)

**Risk Mitigation:**
- **Low execution risk**: CLI tools are proven pattern (not novel tech)
- **Clear user need**: 30 min → 30 sec is quantifiable pain point
- **Multiple win conditions**: Prize, adoption, career positioning (not single point of failure)

---

## CLI Tool / Developer Infrastructure Specific Requirements

### Command Structure & Interface

**Primary Commands:**
```bash
# Server Management
x402-dev mock [start] [options]        # Start mock server (start is default)
x402-dev mock stop                     # Stop running mock server
x402-dev mock status                   # Check if mock server is running
x402-dev mock restart [options]        # Restart mock server

# Testing
x402-dev test <suite> [options]        # Run automated test suite

# Validation
x402-dev verify headers <url>          # Verify x402 header compliance
x402-dev verify invoice <file>         # Verify invoice structure
x402-dev check <url>                   # Comprehensive API validation (headers + invoice + status)

# Monitoring
x402-dev monitor [options]             # Monitor live payment flows (real-time tail)
x402-dev logs [options]                # Alias for monitor

# Policy Management
x402-dev policy validate <file>        # Validate policy file syntax and detect conflicts
x402-dev policy generate <file> [opts] # Generate middleware from policy
  --framework <express|fastify>        #   Target framework (required)
  --output <file>                      #   Output file path (optional, prints to stdout if omitted)

# Example Library (NEW)
x402-dev examples list                 # Show all available example projects
x402-dev examples init <name>          # Scaffold example project (MCP server, AI agent, CI/CD)
x402-dev examples info <name>          # Show detailed example information

# Diagnostics (NEW)
x402-dev doctor                        # Run system diagnostics and compatibility checks

# Setup & Configuration
x402-dev init [options]                # Initialize project configuration
x402-dev version                       # Show version info
x402-dev help [command]                # Show help
```

**Command Design Rationale:**

**Mock Server Lifecycle:**
- `mock` without arguments starts server (common default pattern)
- `mock stop` and `mock status` added for complete lifecycle management
- Avoids manual process killing (`ps aux | grep x402`)

**Simplified Verbosity:**
- `test <suite>` instead of `test run <suite>` - "run" is implied (matches `npm test`)
- `monitor` instead of `monitor tail` - "tail" is default behavior (tail -f is Unix-specific)

**Policy Commands:**
- `policy validate` - Pre-deployment syntax checking
- `policy generate` - Creates Express/Fastify middleware code
- `--output` flag added to avoid shell redirection (cross-platform compatible)
- Note: Policy enforcement happens via generated middleware at runtime, not via CLI command

**Validation Commands:**
- `verify headers/invoice` - Specific validation for targeted checks
- `check` - NEW composite command for comprehensive validation (all checks at once)

**Example Library Commands (NEW):**
- `examples list/init/info` - Pre-built project templates
- Distribution strategy: Developers discover x402-dev via examples, not docs
- SEO benefit: "x402 mcp server example" finds x402-dev repos

**Doctor Command (NEW):**
- System diagnostics with actionable fixes
- Detects Corbits SDK → provides tailored recommendations
- Partnership strategy: Symbiotic with SDK vendors (not competitive)

**Design Principles:**
- **UNIX philosophy**: Each command does one thing well
- **Composability**: Output can pipe to other CLI tools (`x402-dev test run | jq`)
- **Progressive disclosure**: Simple defaults, advanced flags for power users
- **Non-interactive by default**: CI/CD friendly (exit codes, JSON output)
- **Color-coded output**: Success (green), errors (red), warnings (yellow)

### Installation & Distribution

**npm Package Distribution:**
```bash
npm install -g x402-dev
# or
npx x402-dev <command>
```

**Requirements:**
- **Runtime:** No dependencies (self-contained Rust binary)
- Cross-platform: macOS, Linux, Windows
- Binary size: 2-3MB (pure Rust, no embedded runtime)

**Setup Time:**
- <5 minutes from `npm install` to first command
- Auto-detect configuration where possible
- Interactive `init` command for first-time setup

### Configuration Management

**Configuration File:** `.x402dev.yaml` (project root)
```yaml
# Generated by: x402-dev init
mock_server:
  port: 3402
  pricing:
    default: 0.01  # USDC per request

test:
  timeout: 30000
  retry: 3

policy:
  log_file: ./logs/x402-audit.log

solana:
  rpc_url: https://api.devnet.solana.com
  network: devnet
```

**Configuration Priority:**
1. CLI flags (highest priority)
2. Environment variables (`X402_DEV_*`)
3. Project config file (`.x402dev.yaml`)
4. Global config file (`~/.x402dev/config.yaml`)
5. Built-in defaults (lowest priority)

### Output Formats & Logging

**Human-Readable (default):**
- Color-coded terminal output
- Progress indicators for long operations
- ASCII tables for structured data
- Emoji indicators (✅ ❌ ⚠️)

**Machine-Readable (--json flag):**
```json
{
  "status": "success",
  "command": "test run",
  "results": {
    "total": 10,
    "passed": 9,
    "failed": 1,
    "duration_ms": 1234
  }
}
```

**Exit Codes (CI/CD integration):**
- `0` = Success
- `1` = Test failures
- `2` = Configuration errors
- `3` = Network/Solana RPC errors

### Error Handling & Developer Experience

**Clear Error Messages:**
```
❌ Error: Invalid x402 header format

Found: X-Payment-Required: true
Expected: HTTP 402 status + WWW-Authenticate header

Fix: Add WWW-Authenticate header with payment invoice
Documentation: https://docs.x402.org/protocol#headers
```

**Actionable Suggestions:**
- Always suggest next step or fix
- Link to relevant documentation
- Show examples of correct format

**Debugging Support:**
- `--verbose` flag for detailed logs
- `--debug` flag for stack traces
- `--dry-run` flag to preview actions

### CI/CD Integration Requirements

**GitHub Actions Example:**
```yaml
name: Test x402 Payment Flows
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - run: npm install -g x402-dev
      - run: x402-dev test ./tests/payment-flow.yaml --json
```

**Requirements:**
- Non-interactive execution (no prompts)
- Fast execution (<30 seconds for typical test suite)
- Structured output (JSON for parsing)
- Exit codes for pass/fail detection

### Documentation Requirements

**README.md Must Include:**
- Quickstart (5 minutes to first command)
- Installation instructions (npm, npx, alternatives)
- 10+ code examples covering all core commands
- Troubleshooting section (common errors)
- Architecture diagram (CLI → Mock Server → Solana)
- Video demo link (3 minutes)

**CLI Built-in Help:**
- `x402-dev help` shows all commands
- `x402-dev <command> --help` shows command-specific help
- Examples included in help text

---

## Technology Stack

### Core Dependencies

**Language & Runtime:**
- **Rust 1.75+** - Core language (performance, type safety, small binary ~2-3MB)

**CLI Framework:**
- **Clap 4.5** - Industry-standard Rust CLI framework (derive macros)
  - Subcommands, options parsing, help generation
  - Auto-generated help text with examples
  - Built-in "did you mean?" suggestions and color output
- **anyhow 1.0** - Ergonomic error handling with context

**HTTP Server:**
- **actix-web 4.9** - Pure Rust HTTP server framework
  - Native async/await patterns with tokio
  - Middleware architecture for policy enforcement
  - Mature, battle-tested framework

**Configuration & Data:**
- **serde_yaml 0.9** - YAML parser for test suites and policy files
- **serde/serde_json** - Serialization for config and JSON data
- **directories** - Platform-specific config directory discovery

### x402 Integration

**Implementation Approach: Pure Rust**
- **Invoice Generation:** Manual x402 protocol implementation
  - HTTP 402 status code
  - WWW-Authenticate header formatting
  - JSON invoice structure (recipient, amount, currency, memo)
- **Advantages:**
  - Simple, predictable implementation
  - No external SDK dependencies
  - Full control over invoice format
  - Faster implementation (<2 hours vs 6+ hours SDK integration)

**Solana Address Generation (for testing):**
- **Mock addresses:** Generate valid Base58 format test addresses (32-44 chars)
- **No real blockchain interaction required** for mock server
- Use `bs58` crate for Base58 encoding/validation

**Optional: SDK Integration (Post-Hackathon)**
- Can add Corbits/PayAI SDK integration in future versions if needed
- Focus on core value proposition first (testing/validation/security)

### HTTP & Networking

**Mock Server:**
- **actix-web 4.9** - Pure Rust HTTP server framework
  - Native async/await with tokio
  - Middleware architecture for policy enforcement
  - Built-in CORS support
  - Simple routing for configurable pricing

**HTTP Client:**
- **reqwest 0.12** - Async HTTP client for `verify` commands
  - Timeout handling, retry logic
  - JSON deserialization support

### Solana Integration

**Blockchain Client:**
- **solana-client 2.0** - Official Rust Solana SDK
  - RPC queries for transaction status (FR-4)
  - Address validation (Base58 format checking)
  - Network switching (devnet, testnet, mainnet)

**Networks:**
- **Devnet** (default) - Free, fast, no real funds
- **Testnet** - For pre-production validation
- **Mainnet-beta** - Production use (optional for monitoring)

**RPC Endpoints:**
- **Default:** https://api.devnet.solana.com (public, rate-limited)
- **Recommended:** QuickNode, Helius (dedicated, no rate limits)
- **Configurable:** Via `.x402dev.yaml` or `X402_DEV_SOLANA_RPC` env var

### Testing Infrastructure

**Test Framework:**
- **Rust built-in tests** - Unit tests with `#[cfg(test)]`
- **cargo-nextest** - Fast parallel test runner (optional)

**Assertions:**
- **assert!** macros - Rust standard library
- **Custom assertions** - For x402-specific validations (invoice format, header compliance)

**Coverage:**
- **cargo-tarpaulin** - Code coverage tool
- **Target:** 80%+ coverage (per NFR-M1)

**Dogfooding:**
- x402-dev tests itself using its own test runner
- Example test suites in `examples/` directory

### Code Quality Tools

**Linting:**
- **clippy** - Rust linter (built-in)
- **rustfmt** - Code formatter (built-in)
- **Goal:** 0 errors, 0 warnings before release (NFR-M1)

**Type Checking:**
- **Rust type system** - Compile-time type safety
- **No unsafe code** - Unless absolutely necessary (clearly documented)

### Development Tools

**Build:**
- **cargo** - Rust build system and package manager
- **Output:** Single binary (~2-3MB) pure Rust

**Package Management:**
- **Cargo.toml** - Rust dependencies with workspace structure
- **Lock files:** Cargo.lock (committed to Git)

**Release:**
- **cargo-release** - Rust release automation
- **GitHub releases** - Binary distribution for all platforms
- **Semantic versioning:** Enforced via Cargo.toml

### Dependencies Budget

**Binary Size Target:** 2-3MB (per NFR-D1)
- Rust binary: ~2-3MB
- Optimization: Release profile, strip symbols, LTO

**Security:**
- **cargo audit** - Run before every release (0 critical vulnerabilities required)
- **Dependency review** - All Rust crates vetted
- **Regular updates** - Monthly security patch review

### Architecture Decision Records

**Key Decisions Documented:**
1. **Pure Rust Implementation (KISS)** - Simplicity over complexity (see architecture.md ADR-001)
2. **Clap 4.5** - Industry-standard CLI framework with derive macros
3. **actix-web over Express.js** - Pure Rust HTTP server, no multi-language complexity
4. **anyhow for error handling** - Ergonomic error propagation with context
5. **tokio multi-thread runtime** - No V8 constraints, can use full async capabilities
6. **Manual x402 protocol** - Simple invoice generation, no external SDK dependencies

---

## Functional Requirements

### FR-1: Mock Facilitator Server

**Capability:** Local x402 payment server simulation without blockchain dependency

**Requirements:**

**FR-1.1: HTTP Server with 402 Responses**
- MUST start HTTP server on configurable port (default: 3402)
- MUST respond to any request with `402 Payment Required` status
- MUST include `WWW-Authenticate` header with payment invoice
- MUST support CORS for frontend testing
- Acceptance Criteria: `curl localhost:3402` returns 402 status with valid invoice in WWW-Authenticate header

**FR-1.2: Configurable Pricing Rules**
- MUST support per-request pricing (e.g., $0.01 per call)
- MUST support per-resource pricing (e.g., `/api/data` costs $0.05)
- MUST read pricing from config file or CLI flags
- SHOULD support time-based pricing (e.g., peak hours)
- Acceptance Criteria: Different endpoints return different invoice amounts based on pricing config

**FR-1.3: Payment Simulation Modes**
- MUST support success simulation (payment accepted immediately)
- MUST support failure simulation (payment rejected)
- MUST support timeout simulation (delayed response)
- SHOULD support partial payment scenarios
- Acceptance Criteria: Test suite can verify happy path (success) and sad path (failure/timeout) flows

**FR-1.4: Invoice Generation**
- MUST generate placeholder Solana-format addresses for testing (Base58, 32-44 chars)
- MUST include amount, recipient, memo fields in invoice structure
- MUST follow x402 protocol specification for invoice format
- MUST generate unique memo per request (for transaction tracking)
- Note: Mock server uses test addresses only - no real blockchain addresses required
- Acceptance Criteria: Generated invoices pass `x402-dev verify invoice` command validation

**FR-1.5: Zero Blockchain Dependency**
- MUST NOT require actual Solana transactions
- MUST NOT require RPC node connectivity for basic mock server operation
- MUST simulate payment verification without on-chain checks
- Acceptance Criteria: Mock server works completely offline with zero network calls to Solana

**FR-1.6: Server Lifecycle Management**
- MUST support starting server: `x402-dev mock` or `x402-dev mock start`
- MUST support stopping server: `x402-dev mock stop`
- MUST support checking server status: `x402-dev mock status`
- MUST support restarting server: `x402-dev mock restart`
- MUST track server PID for stop/restart operations
- MUST return appropriate exit codes (0 = success, 1 = server not found)
- Acceptance Criteria: Can start, stop, check status, and restart server without manual process killing

---

### FR-2: Automated Test Runner

**Capability:** YAML-defined test suites for x402 payment flows

**Requirements:**

**FR-2.1: YAML Test Suite Format**
- MUST parse YAML test definition files
- MUST support test structure: name, url, method, assertions
- MUST validate test file syntax before execution
- SHOULD provide schema validation for test files
- Acceptance: Example test suite runs successfully

Example test format:
```yaml
tests:
  - name: "Payment required on protected resource"
    url: "http://localhost:3402/api/data"
    method: GET
    expect:
      status: 402
      headers:
        - name: WWW-Authenticate
          exists: true
```

**FR-2.2: Assertion Framework**
- MUST support status code assertions
- MUST support header existence assertions
- MUST support header value assertions (exact, regex, contains)
- MUST support invoice amount assertions
- SHOULD support response time assertions
- Acceptance: 10+ assertion types working

**FR-2.3: Test Execution Engine**
- MUST execute tests sequentially in defined order
- MUST report pass/fail for each test
- MUST continue execution on test failure (fail-soft)
- MUST collect timing metrics for each test
- SHOULD support parallel execution (future)
- Acceptance: 100-test suite completes in <10 seconds

**FR-2.4: CI/CD Compatibility**
- MUST exit with code 0 on all tests passing
- MUST exit with code 1 if any test fails
- MUST support `--json` flag for machine-readable output
- MUST support `--quiet` flag to suppress verbose output
- Command format: `x402-dev test <suite>` (simplified from `test run <suite>`)
- Acceptance Criteria: GitHub Actions workflow passes/fails correctly with `x402-dev test tests.yaml`

**FR-2.5: Test Reporting**
- MUST show summary: total, passed, failed, duration
- MUST show individual test results with timing
- SHOULD support JUnit XML output format (for CI integration)
- SHOULD support HTML report generation
- Acceptance: Clear console output + JSON export

---

### FR-3: Header Verification

**Capability:** Validate x402 protocol compliance for HTTP responses

**Requirements:**

**FR-3.1: Protocol Compliance Checking**
- MUST verify HTTP 402 status code present
- MUST verify `WWW-Authenticate` header exists
- MUST parse and validate invoice structure
- MUST check invoice contains: amount, recipient, memo
- Acceptance: Detects all protocol violations

**FR-3.2: Invoice Structure Validation**
- MUST validate Solana address format (Base58, 32-44 chars)
- MUST validate amount is positive number
- MUST validate currency (USDC) is specified
- SHOULD validate memo format (URL-safe characters)
- Acceptance: Catches malformed invoices

**FR-3.3: Error Suggestions**
- MUST provide actionable error messages
- MUST suggest fixes for common mistakes
- MUST link to relevant x402 protocol documentation
- SHOULD show example of correct format
- Acceptance: New developers can self-correct errors

**FR-3.4: Remote URL Verification**
- MUST support verifying live APIs: `x402-dev verify headers https://api.example.com`
- MUST handle network errors gracefully (timeout, DNS failure)
- MUST report HTTP status codes
- SHOULD save verification results to file
- Acceptance Criteria: Can verify deployed production APIs

**FR-3.5: Comprehensive API Check (NEW)**
- MUST support single-command comprehensive validation: `x402-dev check <url>`
- MUST perform all validation checks: headers, invoice structure, protocol compliance
- MUST aggregate results and show summary (pass/fail per check)
- MUST exit with code 0 if all checks pass, code 1 if any fail
- SHOULD include transaction status check if URL returns real invoices
- Acceptance Criteria: `x402-dev check https://api.example.com` validates entire API in one command

---

### FR-4: Transaction Monitoring

**Capability:** Query and track Solana payment transactions

**Note:** Transaction monitoring is for verifying **real Solana transactions** on devnet/testnet/mainnet. This is separate from the mock server (FR-1), which operates without blockchain connectivity. Use FR-4 when validating actual x402 implementations deployed to Solana networks.

**Requirements:**

**FR-4.1: Solana RPC Integration**
- MUST query Solana RPC for transaction status
- MUST support devnet, testnet, mainnet networks
- MUST handle RPC errors gracefully (rate limits, downtime)
- SHOULD cache transaction results (5 min TTL)
- Acceptance: Retrieves real transaction data from Solana

**FR-4.2: Payment Status Tracking**
- MUST check if transaction is confirmed
- MUST retrieve transaction details (amount, sender, recipient)
- MUST verify payment matches invoice (amount, recipient)
- SHOULD calculate confirmation time
- Acceptance: Tracks payment from submission to confirmation

**FR-4.3: Request/Response Logging**
- MUST log all payment requests (timestamp, URL, invoice)
- MUST log all payment responses (status, transaction ID)
- MUST support structured logging (JSON format)
- SHOULD support log rotation (file size limit)
- Acceptance: Complete audit trail of all payment flows

**FR-4.4: Real-Time Monitoring**
- MUST support real-time log monitoring: `x402-dev monitor` (tail behavior is default)
- MUST support alias: `x402-dev logs` (alternative command name)
- MUST update display as new events occur (live streaming)
- MUST support filtering by status (success, failed, pending)
- SHOULD colorize output by event type (green=success, red=failed, yellow=pending)
- Command format simplified from `monitor tail` to just `monitor`
- Acceptance Criteria: Live updates visible in terminal with `x402-dev monitor`

---

### FR-5: Policy Enforcement Engine

**Capability:** YAML-based security policy definitions and enforcement

**Requirements:**

**FR-5.1: Policy Definition Format**
- MUST parse YAML policy files
- MUST support policy types: allowlist, denylist, rate_limit, spending_cap
- MUST validate policy syntax before enforcement
- SHOULD provide schema validation for policy files
- Acceptance: Example policy files load correctly

Example policy format:
```yaml
policies:
  - type: allowlist
    field: agent_id
    values:
      - "agent-abc-123"
      - "agent-xyz-789"

  - type: rate_limit
    max_requests: 100
    window_seconds: 3600

  - type: spending_cap
    max_amount: 10.00
    currency: USDC
    window_seconds: 86400
```

**FR-5.2: Policy Evaluation Logic**
- MUST evaluate policies in order defined
- MUST support allow (permit request) and deny (reject request) actions
- MUST short-circuit on first deny (fail-fast)
- MUST track policy state (e.g., request counts for rate limiting)
- Acceptance: Policies enforce correctly in test scenarios

**FR-5.3: Allowlist/Denylist Support**
- MUST support allowlist by agent_id, wallet_address, ip_address
- MUST support denylist (blocklist) with same fields
- MUST support wildcard patterns (e.g., `agent-*` matches all agents)
- SHOULD support regex patterns for advanced matching
- Acceptance: Blocks denied agents, allows permitted agents

**FR-5.4: Rate Limiting**
- MUST track request counts per time window
- MUST support sliding window algorithm (not fixed window)
- MUST reject requests exceeding rate limit
- SHOULD support different limits per agent or resource
- Acceptance: 100 req/hour limit enforced correctly

**FR-5.5: Spending Caps**
- MUST track cumulative spending per agent
- MUST reject payments exceeding cap
- MUST reset counters based on time window (hourly, daily)
- SHOULD support different caps per agent tier
- Acceptance: Blocks payment #11 if cap is 10 USDC

**FR-5.6: Policy Conflict Detection**
- MUST detect conflicting policies (e.g., allowlist + denylist same agent)
- MUST warn user about policy conflicts before code generation
- SHOULD suggest resolution strategies
- Acceptance Criteria: `x402-dev policy validate policy.yaml` detects and reports conflicts

---

### FR-6: Middleware Generation

**Capability:** Generate integration code for Express/Fastify frameworks

**Requirements:**

**FR-6.1: Express.js Middleware Generation**
- MUST generate valid Express middleware code from policy file
- MUST include policy enforcement logic (allowlists, rate limits, spending caps)
- MUST handle 402 response generation with invoices
- MUST include error handling and logging
- MUST support `--output <file>` flag to write to file (avoids shell redirection)
- MUST print to stdout if `--output` not provided
- Command: `x402-dev policy generate policy.yaml --framework express --output middleware.js`
- Acceptance Criteria: Generated code runs in Express app without modifications

Example output:
```javascript
// Generated by: x402-dev policy generate policy.yaml --framework express
// Policy file: policy.yaml
// Generated: 2024-11-05

const x402Middleware = (req, res, next) => {
  const agentId = req.headers['x-agent-id'];

  // Allowlist check (from policy)
  const allowedAgents = ['agent-abc-123', 'agent-xyz-789'];
  if (!allowedAgents.includes(agentId)) {
    return res.status(403).json({ error: 'Agent not allowed' });
  }

  // Rate limit check (from policy)
  if (rateLimitExceeded(agentId)) {
    return res.status(429).json({ error: 'Rate limit exceeded' });
  }

  // Generate 402 response with invoice
  const invoice = generateInvoice({
    amount: 0.01,
    currency: 'USDC',
    memo: `req_${Date.now()}`
  });

  res.status(402).set('WWW-Authenticate', invoice);
  logPaymentAttempt(agentId, 'payment_required');
};

module.exports = x402Middleware;
```

**FR-6.2: Fastify.js Plugin Generation**
- MUST generate valid Fastify plugin code
- MUST follow Fastify plugin conventions
- MUST include schema validation
- MUST support `--output <file>` flag (same as Express)
- Command: `x402-dev policy generate policy.yaml --framework fastify --output plugin.js`
- Acceptance Criteria: Generated code registers as Fastify plugin

**FR-6.3: Code Customization Options**
- MUST support `--framework <express|fastify>` flag (required)
- MUST support `--output <file>` flag (optional, prints to stdout if omitted)
- SHOULD support additional flags (e.g., `--auth-header`, `--language typescript`)
- SHOULD support code templates (user-defined)
- SHOULD include inline comments explaining logic
- Acceptance Criteria: Generated code matches user preferences and framework requirements

**FR-6.4: Audit Logging Integration**
- MUST include audit logging in generated middleware
- MUST log: timestamp, agent_id, resource, action, result
- MUST support CSV and JSON export formats
- SHOULD support custom log destinations (file, stdout, HTTP endpoint)
- Acceptance: All payment attempts logged with full context

**FR-6.5: Policy Hot-Reloading**
- SHOULD support reloading policy file without restart
- SHOULD validate new policy before applying
- SHOULD rollback to previous policy on validation failure
- Acceptance: `SIGHUP` signal reloads policy (future)

---

### FR-7: Configuration & Initialization

**Capability:** Easy project setup and configuration management

**Requirements:**

**FR-7.1: Interactive Initialization**
- MUST support `x402-dev init` command
- MUST prompt for: port, pricing, Solana network
- MUST generate `.x402dev.yaml` config file
- SHOULD detect existing config and offer to update
- Acceptance: New project setup in <2 minutes

**FR-7.2: Config File Management**
- MUST support project-level config (`.x402dev.yaml`)
- MUST support global config (`~/.x402dev/config.yaml`)
- MUST merge configs with correct priority (CLI > env > project > global)
- SHOULD validate config file syntax on load
- Acceptance: Multi-tier config precedence works

**FR-7.3: Environment Variable Support**
- MUST support `X402_DEV_PORT`, `X402_DEV_SOLANA_RPC`, etc.
- MUST document all supported env vars
- SHOULD support `.env` file loading
- Acceptance: Environment variables override config files

---

### FR-8: Documentation & Help System

**Capability:** Built-in help and comprehensive documentation

**Requirements:**

**FR-8.1: CLI Help System**
- MUST support `x402-dev help` (show all commands)
- MUST support `x402-dev <command> --help` (command-specific help)
- MUST include usage examples in help text
- MUST show available options and flags
- Acceptance: Every command has useful help text

**FR-8.2: Error Documentation**
- MUST link errors to online documentation
- MUST provide troubleshooting steps for common errors
- SHOULD include error codes for programmatic handling
- Acceptance: Error messages guide users to resolution

**FR-8.3: Code Examples Repository**
- SHOULD include `examples/` directory in npm package
- SHOULD provide 10+ working examples covering all features
- SHOULD include README for each example
- Acceptance: Developers can copy-paste examples

---

### FR-9: Version & Update Management

**Capability:** Version information and update notifications

**Requirements:**

**FR-9.1: Version Display**
- MUST support `x402-dev version` command
- MUST show x402-dev version, Rust version, platform (OS and architecture)
- SHOULD show embedded runtime version (deno_core/V8)
- Acceptance: Version info displayed correctly

**FR-9.2: Update Notifications**
- SHOULD check for newer versions on npm (weekly)
- SHOULD notify user if update available
- SHOULD support `--no-update-check` flag to disable
- Acceptance: Update notification shown once per week

---

### FR-10: Example Library & Quick Start

**Capability:** Pre-built example projects for rapid onboarding and distribution

**Requirements:**

**FR-10.1: Example Catalog**
- MUST include at least 3 example projects in MVP
- MUST cover common use cases: MCP server, AI agent, CI/CD integration
- SHOULD include README, architecture diagram, and usage instructions per example
- Example structure: `examples/mcp-server-starter/`, `examples/ai-agent-policies/`, `examples/cicd-testing/`
- Acceptance Criteria: Developer can scaffold working project in <2 minutes

**FR-10.2: Example Initialization Command**
- MUST support listing examples: `x402-dev examples list`
- MUST support scaffolding: `x402-dev examples init <name>`
- MUST support example info: `x402-dev examples info <name>`
- MUST copy example files to current directory with proper structure
- SHOULD detect conflicts and prompt before overwriting
- Acceptance Criteria: `x402-dev examples init mcp-server-starter` creates working project

**FR-10.3: Example Documentation**
- MUST include inline comments explaining key concepts
- MUST include step-by-step README (5-minute quickstart per example)
- SHOULD include video walkthrough link (post-hackathon)
- MUST include package.json with all dependencies
- Acceptance Criteria: Developer can run example without external docs

**FR-10.4: Distribution Strategy**
- SHOULD host examples as separate GitHub repos (SEO strategy)
- SHOULD tag examples with keywords: x402, solana, ai-agent, mcp-server
- Examples serve as discovery channel (find example → discover x402-dev)
- MUST include link to x402-dev in example README
- Acceptance Criteria: Google "x402 mcp server example" finds x402-dev-related repos

**Example Projects (MVP):**
1. **mcp-server-starter** - Basic MCP server with x402 payments (~50 lines)
2. **ai-agent-policies** - AI agent with spending limits and allowlists (~100 lines)
3. **cicd-testing** - GitHub Actions workflow for automated testing (YAML config)

**Strategic Value:**
- **Distribution**: 10x discovery potential (developers find examples → discover tool)
- **Onboarding**: <2 minute time-to-working-app reduces friction
- **SEO**: Multiple repos with keywords drive organic search traffic
- **Network effects**: Community can contribute examples (future)

---

### FR-11: System Diagnostics (Doctor Command)

**Capability:** Automated system diagnostics and compatibility checking

**Requirements:**

**FR-11.1: Environment Validation**
- MUST check Rust toolchain (1.75+ recommended for development builds)
- SHOULD check npm availability (optional, for development builds only)
- MUST detect x402-related packages (Corbits SDK, PayAI, CDP SDK via npm/package.json)
- SHOULD check port availability (3402 default for mock server)
- MUST use clear visual indicators: ✅ (pass), ❌ (fail), ⚠️ (warning)
- Acceptance Criteria: `x402-dev doctor` shows status for each check

**FR-11.2: Configuration Validation**
- MUST validate `.x402dev.yaml` syntax if present
- MUST check for conflicting configurations
- SHOULD suggest fixes for common errors
- MUST link to documentation for each error
- Acceptance Criteria: Invalid config → clear error + fix suggestion + docs link

**FR-11.3: SDK Integration Detection**
- MUST detect Corbits SDK installation
- SHOULD show version compatibility (Corbits 1.2.3+ recommended)
- SHOULD suggest x402-dev features based on detected SDK
- Example output: "✅ Corbits SDK v1.2.5 detected → Run: x402-dev test init"
- Acceptance Criteria: Corbits users see tailored recommendations

**FR-11.4: Actionable Diagnostics**
- MUST provide fix suggestions for every error
- MUST link to relevant documentation
- SHOULD offer auto-fix for common issues (future)
- Format: ❌ Issue → 💡 Fix → 📖 Docs link
- Acceptance Criteria: New users can self-service setup issues

**FR-11.5: Partnership Integration**
- MUST detect Corbits SDK → show Corbits-specific recommendations
- SHOULD detect PayAI packages → show PayAI-specific recommendations
- SHOULD detect CDP SDK → show CDP-specific recommendations
- Goal: Symbiotic positioning (enhance SDKs, not compete)
- Acceptance Criteria: SDK detection drives partnership conversations

**Strategic Value:**
- **Partnerships**: Doctor command creates "better together" story with SDK vendors
- **Support reduction**: Self-service diagnostics reduce support tickets
- **Onboarding**: Catches setup issues before developers get frustrated
- **Distribution**: SDK users discover x402-dev via doctor recommendations

---

## Non-Functional Requirements

### Performance

**NFR-P1: Command Execution Speed**
- MUST execute commands in <1 second (excluding network calls)
- MUST start mock server in <2 seconds
- MUST parse 100-test YAML suite in <500ms
- MUST run 100 tests in <10 seconds (local mock server)
- Rationale: Developer productivity depends on fast feedback loops
- Measurement: `time x402-dev <command>`

**NFR-P2: Memory Footprint**
- MUST run CLI commands with <100MB memory usage
- MUST run mock server with <200MB memory usage
- SHOULD release memory after command completion
- Rationale: Developers may run multiple tools simultaneously
- Measurement: `ps aux | grep x402-dev`

**NFR-P3: Startup Time**
- MUST cold-start CLI in <500ms (first command)
- SHOULD warm-start CLI in <100ms (subsequent commands)
- MUST NOT block on network checks at startup (async updates)
- Rationale: CLI responsiveness critical for developer experience
- Measurement: `time x402-dev version`

**NFR-P4: Test Suite Scalability**
- MUST support test suites with 1000+ tests
- MUST handle 100 concurrent mock server requests
- SHOULD support parallel test execution (future)
- Rationale: Large projects need comprehensive test coverage
- Measurement: Load test with 1000-test suite

---

### Security

**NFR-S1: Dependency Security**
- MUST use only npm packages with 0 critical vulnerabilities
- MUST run `npm audit` clean before each release
- MUST pin dependency versions (no `^` or `~` in package.json)
- SHOULD automate security scans in CI/CD
- Rationale: CLI tools run locally with developer credentials
- Measurement: `npm audit` report

**NFR-S2: Configuration Security**
- MUST NOT store sensitive data in config files
- MUST support environment variables for secrets (Solana RPC auth tokens)
- SHOULD warn if config file has world-readable permissions
- SHOULD support encrypted config sections (future)
- Rationale: Developers may commit config files to Git
- Measurement: Config file contains no plaintext secrets

**NFR-S3: Policy Enforcement Integrity**
- MUST NOT allow policy bypass via CLI flags
- MUST validate all policy rules before enforcement
- MUST log all policy decisions (allow/deny)
- SHOULD support policy signing for tamper detection (future)
- Rationale: Security policies protect production APIs
- Measurement: Attempt to bypass policy, verify logs

**NFR-S4: Audit Trail Completeness**
- MUST log all payment attempts (timestamp, agent, amount, result)
- MUST NOT log sensitive data (private keys, auth tokens)
- MUST support log integrity verification (checksums)
- SHOULD support remote log shipping (syslog, HTTP)
- Rationale: Audit trails support compliance and debugging
- Measurement: Verify all events logged correctly

---

### Reliability

**NFR-R1: Error Handling**
- MUST handle all errors gracefully (no unhandled exceptions)
- MUST provide actionable error messages
- MUST never crash without logging error details
- SHOULD support automatic crash reporting (opt-in)
- Rationale: CLI crashes frustrate developers
- Measurement: Inject errors, verify graceful handling

**NFR-R2: Network Resilience**
- MUST handle Solana RPC timeouts gracefully (5s timeout)
- MUST retry failed requests (3 attempts with exponential backoff)
- MUST work offline for mock server features
- SHOULD cache Solana RPC results (5 min TTL)
- Rationale: Network failures should not block local development
- Measurement: Disconnect network, verify mock server works

**NFR-R3: Data Integrity**
- MUST validate all user inputs (config files, test suites, policies)
- MUST prevent invalid data from corrupting state
- MUST support atomic config updates (all-or-nothing)
- SHOULD backup config before modification
- Rationale: Invalid config can break entire workflow
- Measurement: Inject invalid config, verify validation

**NFR-R4: Backward Compatibility**
- MUST maintain config file format compatibility across minor versions
- MUST provide migration tools for breaking changes
- SHOULD deprecate features before removal (1 major version)
- Rationale: Developers rely on stable tooling
- Measurement: Load v1.0 config in v1.5, verify works

---

### Usability

**NFR-U1: Developer Experience**
- MUST provide clear error messages with next steps
- MUST use color-coding for output (success, error, warning)
- MUST show progress indicators for long operations (>2s)
- SHOULD support `--help` for every command
- Rationale: Good DX drives adoption
- Measurement: User testing with 5 developers

**NFR-U2: Discoverability**
- MUST show available commands in `x402-dev help`
- MUST include examples in help text
- SHOULD suggest correct command on typos (did you mean?)
- SHOULD provide interactive mode (future)
- Rationale: Developers learn by exploring
- Measurement: New user can run first command in <5 min

**NFR-U3: Accessibility**
- SHOULD support `--no-color` flag for accessibility
- SHOULD work with screen readers (text-only output mode)
- MUST NOT rely on emoji for critical information
- Rationale: Inclusive design supports all developers
- Measurement: Test with screen reader software

**NFR-U4: Documentation Quality**
- MUST include quickstart guide in README (<5 min to first command)
- MUST provide 10+ code examples
- MUST include architecture diagram
- SHOULD provide video tutorial (3 min demo)
- Rationale: Poor docs kill adoption
- Measurement: User testing for time-to-first-success

---

### Compatibility

**NFR-C1: Platform Support**
- MUST work on macOS (Intel + Apple Silicon)
- MUST work on Linux (Ubuntu 20.04+, Debian, RHEL)
- MUST work on Windows (WSL2 + native)
- SHOULD work on CI/CD environments (GitHub Actions, GitLab CI)
- Rationale: Cross-platform CLI is standard expectation
- Measurement: Test on all platforms before release

**NFR-C2: Node.js Version Support**
- MUST support Node.js 18 LTS (minimum)
- MUST support Node.js 20 LTS (current)
- SHOULD support Node.js 22+ (future versions)
- MUST NOT use deprecated Node.js APIs
- Rationale: Developers use various Node versions
- Measurement: Test matrix with Node 18, 20, 22

**NFR-C3: Framework Integration**
- MUST generate compatible Express.js v4+ middleware
- MUST generate compatible Fastify v4+ plugins
- SHOULD support Hono, Koa, Nest.js (future)
- Rationale: Multiple frameworks in ecosystem
- Measurement: Test generated code in real apps

**NFR-C4: CI/CD Integration**
- MUST support GitHub Actions (Ubuntu, macOS, Windows runners)
- MUST support GitLab CI (Docker, shell executors)
- SHOULD support CircleCI, Travis CI, Jenkins
- SHOULD provide example workflow files
- Rationale: Automated testing is standard practice
- Measurement: Run in all major CI platforms

---

### Maintainability

**NFR-M1: Code Quality**
- MUST maintain 80%+ test coverage
- MUST pass ESLint with 0 errors
- MUST use TypeScript strict mode
- SHOULD follow Airbnb style guide
- Rationale: High-quality code is easier to maintain
- Measurement: Code coverage reports, linter output

**NFR-M2: Modularity**
- MUST separate concerns (CLI, core logic, integrations)
- MUST support plugin architecture (future)
- SHOULD use dependency injection for testability
- Rationale: Modular code supports extensibility
- Measurement: Code review for tight coupling

**NFR-M3: Testing Strategy**
- MUST include unit tests for all core logic
- MUST include integration tests for CLI commands
- SHOULD include end-to-end tests (mock server + test runner)
- SHOULD use dogfooding (x402-dev tests x402-dev)
- Rationale: Tests enable confident refactoring
- Measurement: Test suite passes consistently

**NFR-M4: Documentation Maintenance**
- MUST update docs with code changes (same PR)
- MUST include inline JSDoc comments
- SHOULD generate API docs from TypeScript types
- SHOULD version documentation (docs.x402.dev/v1.0)
- Rationale: Stale docs confuse users
- Measurement: Docs match current code

---

### Observability

**NFR-O1: Logging**
- MUST log all errors with stack traces
- MUST support log levels (debug, info, warn, error)
- MUST support structured logging (JSON format)
- SHOULD support log shipping (syslog, HTTP)
- Rationale: Logs enable debugging in production
- Measurement: Review logs for completeness

**NFR-O2: Metrics (Future)**
- SHOULD track command usage (telemetry, opt-in)
- SHOULD track error rates by command
- SHOULD track performance metrics (p50, p95, p99)
- Rationale: Metrics guide prioritization
- Measurement: Telemetry dashboard (future)

**NFR-O3: Debugging Support**
- MUST support `--verbose` flag for detailed logs
- MUST support `--debug` flag for stack traces
- SHOULD support `X402_DEV_DEBUG=*` env var
- SHOULD support remote debugging (future)
- Rationale: Debugging support reduces support burden
- Measurement: Debug obscure error successfully

---

### Deployment & Distribution

**NFR-D1: npm Package Quality**
- MUST publish to npm registry as public package
- MUST include executable bin script (`x402-dev`)
- MUST minimize bundle size (<10MB)
- MUST NOT bundle unnecessary files (tests, .github)
- Rationale: npm is standard distribution for Node tools
- Measurement: `npm pack` size, install time

**NFR-D2: Release Process**
- MUST use semantic versioning (semver)
- MUST publish release notes for each version
- MUST tag releases in Git
- SHOULD automate release via CI/CD (GitHub Actions)
- Rationale: Predictable releases build trust
- Measurement: Automated release succeeds

**NFR-D3: Installation Experience**
- MUST support global install: `npm install -g x402-dev`
- MUST support npx: `npx x402-dev <command>`
- SHOULD support Homebrew (future)
- SHOULD support Docker image (future)
- Rationale: Easy installation drives adoption
- Measurement: Installation success rate

**NFR-D4: Update Mechanism**
- SHOULD notify users of new versions (weekly check)
- SHOULD support `x402-dev update` command (future)
- MUST NOT force updates (user consent required)
- Rationale: Users need latest features and fixes
- Measurement: Update notification shown correctly

---

## Implementation Planning

### Epic Breakdown Required

This PRD contains:
- **9 major functional capabilities** (FR-1 through FR-9)
- **40+ detailed functional requirements** with acceptance criteria
- **30+ non-functional requirements** across 8 categories
- **6 core MVP features** to be delivered by November 11, 2024

Requirements must be decomposed into epics and bite-sized stories (200k context limit) for implementation.

**Project Track:** BMad Method (Level 2 - Medium complexity)

**Next Step:** Run `/bmad:bmm:workflows:create-epics-and-stories` to transform requirements into implementable stories.

### Epic Preview

Based on the functional requirements, the likely epic structure will be:

1. **Epic 1: Mock Server Infrastructure** (FR-1)
   - HTTP server with 402 responses
   - Configurable pricing rules
   - Payment simulation modes
   - Invoice generation
   - Server lifecycle management (start, stop, status, restart)

2. **Epic 2: Automated Testing Framework** (FR-2)
   - YAML test suite parser
   - Assertion framework
   - Test execution engine
   - CI/CD integration

3. **Epic 3: Validation & Verification Tools** (FR-3, FR-4)
   - Header verification (verify headers)
   - Invoice validation (verify invoice)
   - Comprehensive API check (check command - NEW)
   - Transaction monitoring (monitor/logs commands)
   - Solana RPC integration

4. **Epic 4: Policy Enforcement Engine** (FR-5)
   - Policy definition format
   - Evaluation logic
   - Allowlists, denylists, rate limiting, spending caps
   - Conflict detection

5. **Epic 5: Middleware Generation** (FR-6)
   - Express/Fastify code generation (with --output flag)
   - Policy integration in generated code
   - Audit logging in middleware

6. **Epic 6: CLI Infrastructure & DX** (FR-7, FR-8, FR-9)
   - Configuration management
   - Help system
   - Error handling
   - Installation & distribution

---

## References

### Primary Input Documents

- **Product Brief**: `/Users/valentynkit/dev/sandbox/Hackaton/docs/product-brief-x402-dev-2025-11-05.md`
  - Strategic analysis, market research, competitive landscape
  - MVP scope definition, timeline, and risk assessment
  - Hackathon strategy and success metrics

- **Brainstorming Session**: `/Users/valentynkit/dev/sandbox/Hackaton/docs/brainstorming-hybrid-variations-2025-11-04.md`
  - Innovation strategy analysis
  - Skill triangle analysis (OAuth + MCP + Solana)
  - Decision rationale for x402-dev

### External References

- **x402 Protocol Specification**: `hackathon-research/x402-protocol-specification.md`
- **Hackathon Rules & Tracks**: `hackathon-research/hackathon-rules-and-tracks.md`
- **Technical Stack Reference**: `hackathon-research/technical-stack-reference.md`
- **Ecosystem Tools**: `hackathon-research/ecosystem-tools-reference.md`

---

## Next Steps

### Immediate Next Steps (Phase 3: Solutioning)

1. **Architecture Decisions** (Required)
   - Run: `/bmad:bmm:workflows:architecture`
   - Key decisions: Corbits SDK vs PayAI API, middleware architecture, plugin system design
   - Output: Architecture document with technical decisions

2. **Epic & Story Breakdown** (Required)
   - Run: `/bmad:bmm:workflows:create-epics-and-stories`
   - Transform 9 functional capabilities into 6 epics with bite-sized stories
   - Output: Epic files with story backlog

3. **Epic Technical Context** (Required per Epic)
   - Run: `/bmad:bmm:workflows:epic-tech-context` for each epic
   - Generate technical specifications with acceptance criteria
   - Output: Technical context documents for development

4. **Solutioning Gate Check** (Required before implementation)
   - Run: `/bmad:bmm:workflows:solutioning-gate-check`
   - Validate PRD, architecture, and epics are aligned
   - Output: Go/no-go decision for Phase 4

### Phase 4: Implementation (Starting Nov 6-7)

5. **Sprint Planning**
   - Run: `/bmad:bmm:workflows:sprint-planning`
   - Generate sprint tracking file
   - Output: Sprint status with all epics and stories

6. **Story Development Loop** (Iterate until complete)
   - Run: `/bmad:bmm:workflows:create-story` (draft next story)
   - Run: `/bmad:bmm:workflows:story-context` (load dynamic context)
   - Run: `/bmad:bmm:workflows:dev-story` (implement with tasks/tests)
   - Run: `/bmad:bmm:workflows:code-review` (senior dev review)
   - Run: `/bmad:bmm:workflows:story-done` (mark complete)

### Timeline

- **Nov 5 (Today)**: PRD complete ✅
- **Nov 5-6**: Architecture + Epic breakdown
- **Nov 6-9**: Implementation (Feature Sets A, B, C)
- **Nov 10**: Documentation + polish
- **Nov 11**: Demo video + submission

---

## Product Magic Summary

**The magic of x402-dev is transforming developer workflow from hours to seconds.**

What currently takes 30 minutes of manual testing with PayAI Echo Merchant becomes a 30-second automated command. What requires 100+ lines of custom security code becomes a 10-line YAML policy file. What demands deployment to testnet before validation becomes local verification with zero blockchain dependency.

x402-dev is the **first comprehensive CLI toolkit** for x402 development—the missing infrastructure layer that turns the x402 payment protocol from "interesting but hard to use" into "simple and productive." It fills 15+ documented gaps in the ecosystem, making it possible for any developer to test, validate, and secure x402 payment flows with confidence.

**This is developer productivity infrastructure that enables the AI agent economy to scale.**

---

_This PRD captures the complete vision and requirements for x402-dev._

_It transforms the Product Brief into detailed functional and non-functional requirements, ready for architecture design and epic decomposition._

_Created through collaborative discovery between Valik and the BMad Method PRD workflow._

_Next: Architecture decisions, then epic breakdown, then implementation._

---

## Appendix A: Command Structure Evolution

### Command Improvements Summary

The command structure was refined based on CLI best practices and developer experience analysis. Key improvements:

#### 1. Server Lifecycle Management (FR-1.6)

**Added Commands:**
```bash
x402-dev mock stop       # Stop running server
x402-dev mock status     # Check server status
x402-dev mock restart    # Restart server
```

**Rationale:** Complete lifecycle management eliminates manual process killing. Industry standard pattern (docker stop, npm stop).

#### 2. Simplified Command Verbosity

**Before → After:**
```bash
x402-dev test run <suite>  →  x402-dev test <suite>
x402-dev monitor tail      →  x402-dev monitor
```

**Rationale:**
- "run" is implied for test command (matches `npm test`)
- "tail" is Unix-specific jargon, made default behavior
- Reduces typing, faster workflow
- Consistent with industry standards (git, docker, npm)

#### 3. Cross-Platform Output Flag

**Before → After:**
```bash
x402-dev policy generate policy.yaml > file.js  # Shell redirection
x402-dev policy generate policy.yaml --output file.js  # Explicit flag
```

**Rationale:**
- Shell redirection doesn't work consistently across Windows/PowerShell/bash
- `--output` flag is beginner-friendly and self-documenting
- Still supports stdout if `--output` omitted (for piping)

#### 4. Comprehensive Validation Command (NEW)

**Added Command:**
```bash
x402-dev check <url>  # Validates headers + invoice + protocol compliance
```

**Rationale:**
- Single command for complete API validation
- Reduces workflow from 2-3 commands to 1
- Better developer experience for common use case: "Is this API compliant?"

#### 5. Monitor Command Alias

**Added Alias:**
```bash
x402-dev logs  # Alias for "monitor"
```

**Rationale:**
- `logs` is more intuitive than `monitor` for some developers
- Matches Docker (`docker logs`) and Heroku (`heroku logs`)
- Provides flexibility in command naming

### Design Principles Applied

1. **UNIX Philosophy:** Each command does one thing well
2. **Progressive Disclosure:** Simple defaults, advanced flags for power users
3. **Industry Standards:** Matches patterns from git, docker, npm
4. **Cross-Platform:** Works consistently on Windows/Mac/Linux
5. **Beginner-Friendly:** Self-documenting names, clear help text

### Value Impact

**Time Savings:**
- Mock server lifecycle: Saves 2-3 minutes per restart (no manual PID lookup)
- Simplified commands: Saves ~10 characters per invocation (×100s of daily uses)
- Comprehensive check: Saves 30-60 seconds per validation workflow
- `--output` flag: Eliminates cross-platform debugging (saves hours for Windows users)

**Developer Experience:**
- Professional CLI consistent with ecosystem norms
- Lower cognitive load (fewer words to remember)
- Reduced error-prone operations (no shell redirection mistakes)
- Self-service server management (no manual process killing)

### Implementation Effort

**Total Effort:** ~4 hours for all improvements
- Mock lifecycle (stop, status, restart): 2 hours
- Command simplification (test, monitor): 1 hour
- `--output` flag for policy generate: 30 minutes
- `check` command (composite validation): 30 minutes (Phase 2)

**Priority:** All improvements (except `check`) are critical for MVP launch.
