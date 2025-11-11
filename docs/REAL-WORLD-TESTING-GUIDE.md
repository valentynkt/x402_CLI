# x402-cli Real-World Testing Guide

## 🎯 What's Actually Implemented (Epic 1 + Epic 2)

### Available Commands

```bash
x402-dev --help                    # Show all commands
x402-dev init                      # Interactive project setup
x402-dev config show               # Display configuration
x402-dev mock [--port PORT]        # Start mock server (default: 3402)
x402-dev mock stop                 # Stop running server
x402-dev mock status               # Check server status
x402-dev mock restart              # Restart server
```

**Note**: Invoice generation happens automatically in the `WWW-Authenticate` header when the server returns 402 responses.

## 🚀 Quick Test (2 Minutes)

### Build & Run

```bash
# 1. Build the project
cargo build --release

# 2. Create test directory and initialize
mkdir test-x402 && cd test-x402
../target/release/x402-dev init
# Answer prompts:
# - Port: 8402
# - Network: devnet
# - Log level: info

# 3. Start mock server (Terminal 1)
../target/release/x402-dev mock --port 8402

# 4. Test 402 response (Terminal 2)
curl -i http://127.0.0.1:8402/api/test

# 5. Stop server (Terminal 1 - press Ctrl+C)
```

## 📋 Detailed Testing Scenarios

### Scenario 1: Interactive Initialization

```bash
# Start in clean directory
mkdir my-x402-project
cd my-x402-project

# Run interactive init
../target/release/x402-dev init
```

**You'll be prompted for:**
```
Mock server port [8402]:
Solana network:
  > devnet
    testnet
    mainnet-beta
Log level:
    error
    warn
  > info
    debug
    trace
```

**Result**: Creates `.x402dev.yaml`:
```yaml
port: 8402
solana_rpc: https://api.devnet.solana.com
log_level: info
```

### Scenario 2: View Configuration

```bash
# Show current config (reads .x402dev.yaml or defaults)
./target/release/x402-dev config show

# Override with CLI flags
./target/release/x402-dev config show --port 9000

# Override with environment variables
X402_DEV_PORT=9999 ./target/release/x402-dev config show
```

**Expected Output:**
```
Configuration Overview
======================

Sources (priority order):
  1. CLI flags:              --port, --solana-rpc, --log-level
  2. Environment variables:  X402_DEV_PORT, X402_DEV_SOLANA_RPC, X402_DEV_LOG_LEVEL
  3. Project config:         ./.x402dev.yaml
  4. Global config:          ~/.x402dev/config.yaml
  5. Built-in defaults

Current Configuration:
  Port:        8402 (from project config)
  Solana RPC:  https://api.devnet.solana.com (from project config)
  Log Level:   info (from project config)
```

### Scenario 3: Mock Server - 402 Payment Required

**Terminal 1 - Start Server:**
```bash
cd my-x402-project
../target/release/x402-dev mock --port 8402
```

**Expected Output:**
```
🚀 Starting x402 mock payment server...
📝 PID file: /Users/[you]/.x402dev/mock-server.pid
🌐 Server running at: http://127.0.0.1:8402
💰 Test wallet: Dev1234567890abcdefghijklmnopqrstuvwxyzABCDEF
💵 Test pricing: 100 USDC per request
⏱️  Timeout delay: 5000ms (5.0s)

Press Ctrl+C to stop the server...
```

**Terminal 2 - Test 402 Response:**
```bash
curl -i http://127.0.0.1:8402/api/test
```

**Expected Response:**
```http
HTTP/1.1 402 Payment Required
www-authenticate: x402-solana recipient=Dev1234567890abcdefghijklmnopqrstuvwxyzABCDEF amount=100 currency=USDC memo=req-a1b2c3d4-5678-90ab-cdef-1234567890ab network=devnet timestamp=2025-11-11T20:30:00Z resource=/api/test expires=2025-11-11T20:35:00Z
content-type: application/json
access-control-allow-origin: *
content-length: 82

{"error":"Payment required","message":"Please submit payment to access this resource"}
```

### Scenario 4: Payment Simulation

**Test with Authorization Header:**
```bash
curl -i -H "Authorization: payment-tx-123456" http://127.0.0.1:8402/api/test
```

**Server logs:**
```
⏳ Simulating payment verification (5000ms)...
[wait 5 seconds]
✅ Payment verified successfully
```

**Response after 5s:**
```http
HTTP/1.1 200 OK
content-type: application/json
access-control-allow-origin: *

{"status":"success","message":"Payment verified","resource":"Protected resource content"}
```

### Scenario 5: Multiple Unique Invoices

Each request without authorization generates a unique memo:

```bash
# Request 1
curl -s http://127.0.0.1:8402/api/endpoint1 | grep memo
# Output: memo=req-12345678-...

# Request 2
curl -s http://127.0.0.1:8402/api/endpoint2 | grep memo
# Output: memo=req-87654321-...  (different UUID)

# Request 3
curl -s http://127.0.0.1:8402/api/endpoint3 | grep memo
# Output: memo=req-abcdef12-...  (different UUID)
```

### Scenario 6: Graceful Shutdown

**Stop Server with Ctrl+C:**
```bash
# In server terminal, press Ctrl+C
^C
```

**Expected Output:**
```
🛑 Received interrupt signal, shutting down gracefully...
✅ Server stopped successfully
```

**Verify Clean Shutdown:**
```bash
# PID file should be removed
ls ~/.x402dev/mock-server.pid
# Expected: No such file or directory

# Port should be free
lsof -i :8402
# Expected: (no output)
```

### Scenario 7: Prevent Multiple Servers (PID Locking)

**Terminal 1:**
```bash
./target/release/x402-dev mock --port 8402
# Server starts successfully
```

**Terminal 2:**
```bash
./target/release/x402-dev mock --port 8402
```

**Expected Output:**
```
❌ Error: Server already running (cannot acquire PID file lock)
💡 Fix: Stop the existing server or use a different port
Exit code: 1
```

**Terminal 2 (Try Different Port):**
```bash
./target/release/x402-dev mock --port 8403
```

**If port 8402 still listening:**
```
❌ Error: Port 8402 is already in use
💡 Fix: Stop the process using this port or choose a different port
Exit code: 2
```

## 🧪 Automated Testing

### Run Unit Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_invoice_creation

# With output
cargo test -- --nocapture

# Show test names
cargo test -- --list
```

**Expected: 15 tests passing**

### Run Integration Tests

```bash
# Full Epic 2 integration suite
./tests/integration_test_epic2.sh
```

**Expected Output:**
```
═══════════════════════════════════════════════════════════
x402-cli Epic 2 Integration Tests
═══════════════════════════════════════════════════════════

Building project... ✅
Test 1: Mock Server Startup ......................... ✅ PASS
Test 2: 402 Payment Required Response ............... ✅ PASS
Test 3: WWW-Authenticate Header Format .............. ✅ PASS
Test 4: Payment Verification Simulation ............. ✅ PASS
Test 5: Server Lifecycle Management ................. ✅ PASS
Test 6: Multiple Endpoints .......................... ✅ PASS

═══════════════════════════════════════════════════════════
Test Results: 6/6 passed (100%)
═══════════════════════════════════════════════════════════
```

## 📊 Protocol Verification

### Check WWW-Authenticate Header Format

```bash
# Start server
./target/release/x402-dev mock --port 8402 &

# Extract and verify header
curl -s -I http://127.0.0.1:8402/api/test | grep www-authenticate
```

**Expected Format:**
```
www-authenticate: x402-solana recipient=<address> amount=<num> currency=USDC memo=req-<uuid> network=devnet timestamp=<iso8601> resource=<path> expires=<iso8601>
```

**Verify Fields:**
- ✅ Protocol: `x402-solana`
- ✅ Recipient: 44-character Base58 address
- ✅ Amount: Numeric value
- ✅ Currency: `USDC`
- ✅ Memo: `req-<uuid>` format (hyphen, not underscore)
- ✅ Network: `devnet`
- ✅ Timestamp: ISO8601 format
- ✅ Resource: Requested path
- ✅ Expires: ISO8601 format (5 minutes from timestamp)

## 🎬 Complete Demo Script

```bash
#!/bin/bash
# Complete x402-cli demo

echo "════════════════════════════════════════════════"
echo "x402-cli Complete Demo (Epic 1 + Epic 2)"
echo "════════════════════════════════════════════════"

# 1. Build
echo "📦 Building..."
cargo build --release

# 2. Initialize project
echo "📋 Initializing project..."
mkdir demo-x402 && cd demo-x402
echo -e "8402\n0\n2\n" | ../target/release/x402-dev init

# 3. Show config
echo "📝 Configuration:"
../target/release/x402-dev config show

# 4. Start server in background
echo "🚀 Starting mock server..."
../target/release/x402-dev mock --port 8402 > /dev/null 2>&1 &
SERVER_PID=$!
sleep 2

# 5. Test 402 response
echo "🧪 Testing 402 response:"
curl -i http://127.0.0.1:8402/api/demo

# 6. Test payment simulation (5 second wait)
echo ""
echo "💰 Testing payment verification (5s wait):"
time curl -s -H "Authorization: test-payment" http://127.0.0.1:8402/api/demo | jq .

# 7. Stop server
echo "🛑 Stopping server..."
kill -SIGTERM $SERVER_PID
wait $SERVER_PID 2>/dev/null

# 8. Verify cleanup
echo "✅ Verification:"
[ ! -f ~/.x402dev/mock-server.pid ] && echo "  ✓ PID file removed" || echo "  ✗ PID file still exists"

# 9. Cleanup
cd .. && rm -rf demo-x402
echo ""
echo "✅ Demo complete!"
```

## 🔍 Troubleshooting

### Issue: Server won't start

```bash
# Check if PID file is stale
ls ~/.x402dev/mock-server.pid

# Remove if server is not actually running
rm ~/.x402dev/mock-server.pid

# Check if port is in use
lsof -i :8402
```

### Issue: Config not found

```bash
# init creates .x402dev.yaml in CURRENT directory
ls .x402dev.yaml

# If missing, run init again
./target/release/x402-dev init
```

### Issue: Connection refused

```bash
# Verify server is actually running
ps aux | grep x402-dev

# Check server logs
tail -f ~/.x402dev/server.log  # If logs are enabled
```

### Issue: Tests failing

```bash
# Clean build
cargo clean
cargo build --release

# Run tests with output
cargo test -- --nocapture

# Check test file directly
cat crates/x402-cli/src/commands/invoice.rs | grep -A 5 "test_"
```

## ✅ Success Checklist

After running all tests, verify:

- ✅ Binary builds successfully (~2.6MB)
- ✅ `init` command creates `.x402dev.yaml`
- ✅ `config show` displays configuration
- ✅ Mock server starts on specified port
- ✅ Returns 402 with correct WWW-Authenticate header
- ✅ Memo format uses hyphen: `req-{uuid}`
- ✅ Payment simulation waits 5 seconds (default)
- ✅ Graceful shutdown with Ctrl+C
- ✅ PID file prevents multiple servers
- ✅ Exit code 2 for port-in-use
- ✅ All 15 unit tests pass
- ✅ All 6 integration tests pass

## 🎯 What's Working (Epic 1 + Epic 2)

### ✅ Epic 1: Project Setup & Configuration
- Story 1.3: Version command
- Story 1.4: Config management (show command)
- Story 1.7: Init command (interactive setup)

### ✅ Epic 2: Mock Payment Server
- Story 2.1: HTTP server with 402 responses
- Story 2.2: Configurable pricing rules
- Story 2.3: Payment verification simulation
- Story 2.4: Invoice generation (WWW-Authenticate headers)
- Story 2.5: Zero blockchain dependency
- Story 2.6: Lifecycle management (start/stop/signals)

## 📈 Performance Metrics

```bash
# Binary size
ls -lh target/release/x402-dev
# Expected: ~2.6MB

# Startup time
time ./target/release/x402-dev mock --port 8402 &
# Expected: <100ms

# 402 response time
time curl -s http://127.0.0.1:8402/api/test > /dev/null
# Expected: <10ms

# Payment simulation time
time curl -s -H "Authorization: test" http://127.0.0.1:8402/api/test > /dev/null
# Expected: ~5 seconds (default timeout_delay_ms)
```

## 🚀 Next Steps (Epic 3+)

- Epic 3: Actual Solana blockchain integration
- Epic 4: Health checks and diagnostics
- Epic 5: Transaction monitoring and policies
- Epic 6: Examples and documentation

---

**Platform**: Unix/Linux/macOS only (Windows: use WSL2)
**Docs**: See [PLATFORM-REQUIREMENTS.md](./PLATFORM-REQUIREMENTS.md)
