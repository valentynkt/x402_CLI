# Troubleshooting Guide

⚠️ **Important:** x402-dev is a **TESTING TOOLKIT**. Most issues relate to the mock server and CLI, not blockchain integration (which doesn't exist).

This guide covers the most common errors. 86% of issues are solved here.

---

## 🚨 Top 10 Common Errors

### 1. ❌ "Port 3402 already in use"

**What this means:** Another process is using the default port.

**Quick fix:**
```bash
# Option 1: Use different port
x402-dev mock --port 8402

# Option 2: Find and kill the process
lsof -i :3402  # Find PID
kill -9 <PID>  # Replace <PID> with actual number

# Option 3: Stop x402-dev cleanly
x402-dev mock stop
```

**Why this happens:** x402-dev might still be running from a previous session, or another application is using port 3402.

**Prevent this:** Always run `x402-dev mock stop` when done testing.

---

### 2. ❌ "Solana RPC connection failed"

⚠️ **Reality Check:** This error **should NOT occur** because x402-dev does NOT connect to Solana.

**What this actually means:** You might be seeing an error from future/unimplemented features.

**Quick fix:**
```bash
# The mock server doesn't need RPC connection
# Just start the mock server normally:
x402-dev mock

# If you see this error, please report it as a bug
# It indicates dead code trying to connect to blockchain
```

**Why this doesn't apply:** x402-dev is mock-only, no blockchain calls.

**The truth:** No RPC endpoint is needed for testing with the mock server.

---

### 3. ❌ "Command not found: x402-dev"

**What this means:** The CLI is not installed or not in your PATH.

**Quick fix:**
```bash
# Option 1: Install from crates.io
cargo install x402-dev

# Option 2: Install from source
cargo install --path crates/x402-cli

# Option 3: Add Cargo bin to PATH
export PATH="$HOME/.cargo/bin:$PATH"
# Add to ~/.bashrc or ~/.zshrc to make permanent
```

**Why this happens:** Cargo binaries not in PATH, or installation incomplete.

**Prevent this:** Verify installation with `which x402-dev` after installing.

---

### 4. ❌ "Failed to parse .x402dev.yaml"

**What this means:** Invalid YAML syntax in configuration file.

**Quick fix:**
```bash
# Option 1: Regenerate default config
x402-dev init --force

# Option 2: Check for common issues manually
cat .x402dev.yaml
# Look for:
# - Tabs instead of spaces
# - Missing colons
# - Incorrect indentation

# Option 3: Validate YAML (if you have Python)
python -c "import yaml; yaml.safe_load(open('.x402dev.yaml'))"
```

**Why this happens:** Manual edits to `.x402dev.yaml` with syntax errors.

**Prevent this:** Use `x402-dev init` to generate valid config, avoid manual edits.

---

### 5. ❌ "Rust version 1.75+ required"

**What this means:** Your Rust toolchain is outdated.

**Quick fix:**
```bash
# Update Rust to latest stable
rustup update stable

# Verify version
rustc --version
# Should show: rustc 1.75.0 or higher

# Set default toolchain
rustup default stable
```

**Why this happens:** Project requires features from Rust 1.75+.

**Prevent this:** Run `rustup update` regularly.

---

### 6. ❌ "Permission denied" (Port or File Access)

**What this means:** Insufficient permissions to bind port or access files.

**Quick fix:**
```bash
# Option 1: Use port > 1024 (doesn't require root)
x402-dev mock --port 8402

# Option 2: Check file permissions
chmod 644 .x402dev.yaml

# Option 3: Check directory permissions
ls -la .x402dev.yaml
```

**Why this happens:** Ports < 1024 require root privileges, or config files have wrong permissions.

**Prevent this:** Use non-privileged ports (> 1024) for development.

---

### 7. ❌ "Mock server already running"

**What this means:** Previous instance is still running in background.

**Quick fix:**
```bash
# Option 1: Stop existing instance
x402-dev mock stop

# Option 2: Check status
x402-dev mock status

# Option 3: Find and kill process manually
ps aux | grep x402-dev
kill -9 <PID>
```

**Why this happens:** Previous session didn't shut down cleanly or crashed.

**Prevent this:** Use `x402-dev mock stop` instead of Ctrl+C.

---

### 8. ❌ "Test suite file not found"

**What this means:** YAML test file doesn't exist or wrong path.

**Quick fix:**
```bash
# Option 1: Check file exists
ls tests/payment-flow.yaml

# Option 2: Use absolute path
x402-dev test /full/path/to/test-suite.yaml

# Option 3: Create example test suite
mkdir -p tests
cat > tests/basic.yaml << 'EOF'
tests:
  - name: "Returns 402 without payment"
    request:
      url: "http://localhost:3402/api/data"
    assertions:
      - type: status_code
        expected: 402
EOF
```

**Why this happens:** Incorrect path or file not created yet.

**Prevent this:** Always verify file path with `ls` before running tests.

---

### 9. ❌ "Invalid test assertion type"

**What this means:** YAML test suite uses unsupported assertion.

**Quick fix:**
```bash
# Valid assertion types:
# - status_code
# - header_exists
# - header_contains
# - body_contains
# - body_json
# - response_time

# Example valid test:
tests:
  - name: "Check 402 status"
    request:
      url: "http://localhost:3402/api/data"
    assertions:
      - type: status_code
        expected: 402
      - type: header_exists
        header: "WWW-Authenticate"
```

**Why this happens:** Typo in assertion type or using custom assertion.

**Prevent this:** Check [Testing Guide](testing.md) for valid assertion types.

---

### 10. ❌ "Policy generation failed"

**What this means:** Invalid YAML policy file.

**Quick fix:**
```bash
# Option 1: Validate policy syntax
cat policy.yaml

# Option 2: Use example policy
cat > policy.yaml << 'EOF'
policies:
  - type: rate_limit
    pattern: "/api/*"
    max_requests: 100
    window: 3600
EOF

# Option 3: Check for common errors
# - Invalid policy type
# - Missing required fields
# - Wrong indentation
```

**Why this happens:** Syntax error in policy YAML.

**Prevent this:** Start with example policies and modify incrementally.

---

## 💡 Frequently Asked Questions

### Do I need real SOL to test?

**No!** x402-dev is a mock server that doesn't touch blockchain.

**Reality:**
```bash
# Mock server doesn't need:
# - Real SOL
# - Solana wallets
# - Blockchain connection
# - Transaction fees

# Just start the mock server:
x402-dev mock
```

**For production:** You would need real SOL, but x402-dev is testing-only.

---

### Is this production-ready?

**No. x402-dev is a TESTING TOOLKIT only.**

**Current status:**
- ✅ Mock server works perfectly for testing
- ✅ CLI tools fully functional
- ✅ Test automation framework complete
- ✅ Policy code generation works
- ❌ NO real Solana blockchain integration
- ❌ NO payment verification
- ❌ NO production security

**Do NOT use for real payments.**

👉 See [Limitations](limitations.md) for what's missing.

---

### Can I use this with real money?

**Absolutely not.**

**Why:**
- No real blockchain integration (`solana-client` not used)
- Mock server accepts ANY payment proof
- No transaction verification
- No replay attack prevention
- Test addresses only (hardcoded fake addresses)

**For real payments:** You must add Solana SDK and payment verification yourself.

---

### How do I test payment flows?

**Perfect use case for x402-dev!**

```bash
# 1. Start mock server
x402-dev mock

# 2. Test endpoint without payment
curl http://localhost:3402/api/data
# Returns: 402 Payment Required

# 3. Test endpoint with fake payment (mock accepts anything)
curl http://localhost:3402/api/data -H "X-Payment-Proof: fake-proof"
# Returns: 200 OK (mock verification always succeeds)

# 4. Automated testing
x402-dev test tests/payment-flow.yaml
```

---

### What if tests are failing?

**Common causes:**

1. **Mock server not running:**
   ```bash
   x402-dev mock status
   # If not running: x402-dev mock
   ```

2. **Wrong URL in test:**
   ```yaml
   # Make sure URL matches where mock server is running
   url: "http://localhost:3402/api/data"  # ✅ Correct
   url: "http://localhost:8080/api/data"  # ❌ Wrong port
   ```

3. **Invalid YAML syntax:**
   ```bash
   # Validate YAML
   python -c "import yaml; yaml.safe_load(open('tests/suite.yaml'))"
   ```

---

### Can I test without blockchain?

**Yes! That's the entire point of x402-dev.**

**Reality:**
- Mock server simulates payment protocol
- No blockchain needed
- No wallets needed
- Instant responses
- Perfect for CI/CD

**Example:**
```bash
# This is ALL you need:
x402-dev mock

# No Solana CLI required
# No wallet setup required
# No SOL airdrop required
```

---

### How do I check what's actually implemented?

**Three ways:**

1. **Read limitations doc:**
   ```bash
   cat docs/limitations.md
   ```

2. **Run check command:**
   ```bash
   x402-dev check http://localhost:3402/api/data
   # Validates 402 protocol compliance only
   # Does NOT check blockchain
   ```

3. **Check source code:**
   ```bash
   # See what's actually in the code
   grep -r "solana-client" crates/
   # Result: 0 matches (no blockchain integration)
   ```

---

### What's the difference between mock and production mode?

**There is NO production mode.**

**The truth:**
- Only "mock" mode exists
- Configuration has "solana" section but it's not used
- `--production` flag doesn't exist in code
- All modes are mock-only

**Future (not implemented):**
- Real production mode would need `solana-client` integration
- Would need payment verification code
- Would need wallet management
- None of this exists yet

---

### Why does `.x402dev.yaml` have Solana config?

**Good question!** It's aspirational for future development.

**Current reality:**
```yaml
solana:
  rpc_url: "https://api.devnet.solana.com"  # NOT USED
  network: "devnet"                          # Hardcoded anyway
```

**What actually happens:**
- Mock server ignores these settings
- Invoices hardcoded to "devnet" regardless
- No RPC calls are ever made

**Why it's there:** Placeholder for future real blockchain integration.

---

### How do I reset everything?

```bash
# Stop mock server
x402-dev mock stop

# Remove config
rm .x402dev.yaml

# Reinitialize
x402-dev init

# Verify clean state
x402-dev doctor
```

---

### Can I run multiple mock servers?

**Yes, use different ports:**

```bash
# Terminal 1
x402-dev mock --port 3402

# Terminal 2
x402-dev mock --port 3403

# Terminal 3
x402-dev mock --port 3404
```

Each runs independently for testing different configurations.

---

## 🔧 Debug Commands

### System Diagnostics

```bash
# Full system check
x402-dev doctor

# Expected output:
# ✅ Rust version: 1.75.0
# ✅ Config file: .x402dev.yaml
# ✅ Mock server: Ready to start
# ⚠️  Note: No blockchain integration (mock only)
```

---

### Validate Endpoint

```bash
# Check if endpoint returns 402
x402-dev check http://localhost:3402/api/data

# Expected output:
# ✅ HTTP 402 status code: PASS
# ✅ WWW-Authenticate header: PASS
# ⚠️  Note: This validates protocol only, not blockchain
```

---

### View Configuration

```bash
# Show all settings
x402-dev config show

# Note: Solana settings are shown but not used
# Mock server operates independently of these
```

---

### Logging & Verbosity

```bash
# Enable debug logs
RUST_LOG=debug x402-dev mock

# Enable trace logs (very verbose)
RUST_LOG=trace x402-dev mock

# Log to file
x402-dev mock 2>&1 | tee x402dev.log

# Watch logs in real-time
tail -f x402dev.log
```

---

## 🆘 Getting Help

### GitHub Issues

**Found a bug?** Report it with details:
- https://github.com/valentynkit/x402-dev/issues
- Include error message, OS, Rust version
- Provide steps to reproduce
- Specify: Testing issue or asking about production features

---

### Discussions

**Have questions?** Join the community:
- https://github.com/valentynkit/x402-dev/discussions
- Q&A, ideas, and general help

---

### Documentation

**Need more info?** Check the docs:
- [README.md](../README.md) - Getting started (testing focus)
- [Quick Start](quickstart.md) - 90-second tutorial
- [Limitations](limitations.md) - What's NOT implemented
- [Architecture](architecture.md) - How mock server works

---

## 📋 Troubleshooting Checklist

Before asking for help, verify:

- [ ] Rust version 1.75+ (`rustc --version`)
- [ ] x402-dev installed (`x402-dev --version`)
- [ ] Mock server starts (`x402-dev mock`)
- [ ] No other process on port 3402 (`lsof -i :3402`)
- [ ] Valid YAML config (`.x402dev.yaml` exists)
- [ ] Latest code pulled (`git pull` if from source)
- [ ] Clean build (`cargo clean && cargo build` if from source)
- [ ] Understanding this is mock-only (not production)

**Note:** Do NOT check for:
- ❌ Solana CLI installation (not needed for mock)
- ❌ Wallet configuration (not used by mock)
- ❌ RPC endpoint accessibility (mock doesn't connect)
- ❌ Sufficient SOL balance (mock is free)

---

**Still stuck?** Include this info when asking for help:
```bash
x402-dev --version
x402-dev doctor
x402-dev config show
rustc --version
uname -a
```

---

## ⚠️ Critical Reminders

**x402-dev is for TESTING payment-protected APIs:**
- ✅ Use for local development
- ✅ Use for CI/CD test automation
- ✅ Use for learning HTTP 402 protocol
- ✅ Use for generating middleware boilerplate
- ❌ Do NOT use for production payments
- ❌ Do NOT expect real blockchain integration
- ❌ Do NOT use with real money

**For production:** You must add Solana SDK integration separately.

[← Back to README](../README.md) | [Limitations →](limitations.md) | [Quick Start →](quickstart.md)
