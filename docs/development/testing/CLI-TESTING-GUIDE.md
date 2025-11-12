# x402-cli Testing & Demo Guide

## 🚀 Quick Start

### 1. Build the Project

```bash
# Development build (faster, includes debug symbols)
cargo build

# Production build (optimized, ~2.6MB binary)
cargo build --release

# The binary will be at:
# - Debug: ./target/debug/x402
# - Release: ./target/release/x402
```

### 2. Check Installation

```bash
# Run from project root (uses debug build)
./target/debug/x402 --version

# Or use cargo run
cargo run -- --version

# Expected output:
# x402 0.1.0
```

## 📋 Available Commands

### View All Commands

```bash
./target/debug/x402 --help
```

**Expected Output:**
```
x402-cli - Command-line interface for x402 protocol

Usage: x402 <COMMAND>

Commands:
  config   Configuration management
  init     Initialize a new x402 project
  mock     Start mock payment server for testing
  invoice  Generate payment invoices
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## 🧪 Testing Flow (Epic 1 + Epic 2)

### Scenario 1: First-Time Setup (Epic 1)

#### Step 1: Initialize Project

```bash
# Create a new project
./target/debug/x402 init my-x402-project

# Expected output:
# ✅ Created project directory: my-x402-project
# ✅ Created config file: my-x402-project/.x402/config.toml
# 📄 Configuration:
#    Port: 8402
#    Timeout: 30s
#    Payment verification: simulated
```

#### Step 2: View Generated Config

```bash
cat my-x402-project/.x402/config.toml
```

**Expected Content:**
```toml
[server]
port = 8402
timeout_delay_ms = 30000

[payment]
verification_mode = "simulated"
test_recipient = "Dev1234567890abcdefghijklmnopqrstuvwxyzABCDEF"
test_amount = 100
test_currency = "USDC"
```

#### Step 3: Validate Config

```bash
cd my-x402-project
../target/debug/x402 config validate

# Expected output:
# ✅ Configuration is valid
# Server port: 8402
# Timeout delay: 30000ms (30.0s)
# Verification mode: simulated
```

#### Step 4: View Config

```bash
../target/debug/x402 config show

# Expected output shows full config with all settings
```

### Scenario 2: Mock Server Testing (Epic 2)

#### Step 1: Start Mock Server (Terminal 1)

```bash
# From project root
./target/debug/x402 mock start --port 8402

# Expected output:
# 🚀 Starting x402 mock payment server...
# 📝 PID file: /Users/[you]/.x402dev/mock-server.pid
# 🌐 Server running at: http://127.0.0.1:8402
# 💰 Test wallet: Dev1234567890abcdefghijklmnopqrstuvwxyzABCDEF
# 💵 Test pricing: 100 USDC per request
# ⏱️  Timeout delay: 30000ms (30.0s)
#
# Press Ctrl+C to stop the server...
```

#### Step 2: Test 402 Response (Terminal 2)

```bash
# Request without payment - should return 402
curl -i http://127.0.0.1:8402/api/resource

# Expected output:
# HTTP/1.1 402 Payment Required
# www-authenticate: x402-solana recipient=Dev123... amount=100 currency=USDC memo=req-[uuid] network=devnet timestamp=[iso8601] resource=/api/resource expires=[iso8601]
# content-type: application/json
#
# {"error":"Payment required","message":"Please submit payment to access this resource"}
```

#### Step 3: Test Payment Simulation

```bash
# Simulate payment verification (waits 30 seconds by default)
curl -i -H "Authorization: payment-id-12345" http://127.0.0.1:8402/api/resource

# Expected behavior:
# 1. Server logs: "⏳ Simulating payment verification (30000ms)..."
# 2. Waits 30 seconds...
# 3. Returns: HTTP 200 OK
#
# Response after 30s:
# HTTP/1.1 200 OK
# content-type: application/json
#
# {"status":"success","message":"Payment verified","resource":"Protected resource content"}
```

#### Step 4: Test Different Endpoints

```bash
# Different resource paths
curl -i http://127.0.0.1:8402/api/users
curl -i http://127.0.0.1:8402/api/data
curl -i http://127.0.0.1:8402/custom/endpoint

# Each returns 402 with unique memo UUID
```

#### Step 5: Stop Server (Terminal 1)

```bash
# Press Ctrl+C in the server terminal

# Expected output:
# ^C
# 🛑 Received interrupt signal, shutting down gracefully...
# ✅ Server stopped successfully
```

#### Step 6: Verify Clean Shutdown

```bash
# Check that PID file is removed
ls ~/.x402dev/mock-server.pid

# Expected: File not found (should be deleted)
```

### Scenario 3: Invoice Generation (Epic 2)

#### Step 1: Generate Invoice

```bash
./target/debug/x402 invoice generate \
  --recipient Dev1234567890abcdefghijklmnopqrstuvwxyzABCDEF \
  --amount 100 \
  --resource /api/premium

# Expected output:
# 📄 Invoice Generated
# ══════════════════════════════════════════════════════════
# Recipient:     Dev1234567890abcdefghijklmnopqrstuvwxyzABCDEF
# Amount:        100 USDC
# Network:       devnet
# Memo:          req-[uuid-v4]
# Resource:      /api/premium
# Timestamp:     2025-11-11T20:30:00Z
# Expires:       2025-11-11T20:35:00Z (in 5 minutes)
# ══════════════════════════════════════════════════════════
```

#### Step 2: Generate WWW-Authenticate Header

```bash
./target/debug/x402 invoice generate \
  --recipient Dev1234567890abcdefghijklmnopqrstuvwxyzABCDEF \
  --amount 250 \
  --resource /api/data \
  --header

# Expected output:
# x402-solana recipient=Dev1234567890abcdefghijklmnopqrstuvwxyzABCDEF amount=250 currency=USDC memo=req-[uuid] network=devnet timestamp=[iso] resource=/api/data expires=[iso]
```

#### Step 3: Custom Amount Invoice

```bash
./target/debug/x402 invoice generate \
  --recipient Dev1234567890abcdefghijklmnopqrstuvwxyzABCDEF \
  --amount 500 \
  --resource /api/exclusive

# Each call generates unique memo with req-[uuid] format
```

## 🧪 Advanced Testing Scenarios

### Scenario 4: Multi-Server Prevention (Epic 2 - PID Locking)

#### Terminal 1:
```bash
./target/debug/x402 mock start --port 8402
# Server starts successfully
```

#### Terminal 2:
```bash
./target/debug/x402 mock start --port 8402

# Expected output:
# ❌ Error: Server already running (cannot acquire PID file lock)
# 💡 Fix: Stop the existing server or use a different port
# Exit code: 1
```

#### Terminal 2 (Different Port):
```bash
./target/debug/x402 mock start --port 8403

# Expected output:
# ❌ Error: Port 8402 is already in use
# 💡 Fix: Stop the process using this port or choose a different port
# Exit code: 2
```

### Scenario 5: Configuration Validation (Epic 1 + Epic 2)

```bash
# Test invalid timeout (too short)
cat > test-config.toml << 'EOF'
[server]
port = 8402
timeout_delay_ms = 50

[payment]
verification_mode = "simulated"
EOF

./target/debug/x402 config validate --config test-config.toml

# Expected output:
# ❌ Error: Invalid timeout delay: 50 ms. Must be between 100ms and 60000ms (1 minute).
# Fix: Set timeout_delay_ms to a reasonable value between 100 and 60000
```

```bash
# Test valid config
cat > test-config.toml << 'EOF'
[server]
port = 9000
timeout_delay_ms = 5000

[payment]
verification_mode = "simulated"
EOF

./target/debug/x402 config validate --config test-config.toml

# Expected output:
# ✅ Configuration is valid
```

### Scenario 6: Concurrent Requests (Epic 2)

```bash
# Terminal 1: Start server
./target/debug/x402 mock start --port 8402

# Terminal 2: Send multiple concurrent requests
for i in {1..5}; do
  curl -s http://127.0.0.1:8402/api/test$i &
done
wait

# All should return 402 with unique memos
```

## 📊 Integration Test Suite

### Run Full Integration Tests

```bash
# Run Epic 2 integration tests
./tests/integration_test_epic2.sh

# Expected output:
# ═══════════════════════════════════════════════════════════
# x402-cli Epic 2 Integration Tests
# ═══════════════════════════════════════════════════════════
#
# Test 1: Mock Server Startup ......................... ✅ PASS
# Test 2: 402 Payment Required Response ............... ✅ PASS
# Test 3: WWW-Authenticate Header Format .............. ✅ PASS
# Test 4: Payment Verification Simulation ............. ✅ PASS
# Test 5: Server Lifecycle Management ................. ✅ PASS
# Test 6: Multiple Endpoints .......................... ✅ PASS
#
# ═══════════════════════════════════════════════════════════
# Test Results: 6/6 passed (100%)
# ═══════════════════════════════════════════════════════════
```

### Run Unit Tests

```bash
# All unit tests
cargo test

# Expected output:
# running 15 tests
# test commands::config::tests::test_config_defaults ... ok
# test commands::config::tests::test_config_validation ... ok
# test commands::invoice::tests::test_invoice_creation ... ok
# test commands::invoice::tests::test_www_authenticate_format ... ok
# ... (11 more tests)
#
# test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured
```

### Run Specific Test

```bash
# Test invoice generation only
cargo test test_invoice_creation -- --nocapture

# Test memo format
cargo test test_www_authenticate_format -- --nocapture
```

## 🎬 Complete Demo Flow (5 Minutes)

```bash
# 1. Build project (30 seconds)
cargo build --release

# 2. Initialize project (5 seconds)
./target/release/x402 init demo-project
cd demo-project

# 3. View config (2 seconds)
../target/release/x402 config show

# 4. Start mock server in background (2 seconds)
../target/release/x402 mock start --port 8402 &
SERVER_PID=$!
sleep 2

# 5. Test 402 response (2 seconds)
curl -i http://127.0.0.1:8402/api/test

# 6. Generate invoice (2 seconds)
../target/release/x402 invoice generate \
  --recipient Dev1234567890abcdefghijklmnopqrstuvwxyzABCDEF \
  --amount 100 \
  --resource /api/test

# 7. Stop server (2 seconds)
kill -SIGTERM $SERVER_PID
sleep 2

# 8. Verify clean shutdown (1 second)
ls ~/.x402dev/mock-server.pid 2>&1 | grep "No such file"

echo "✅ Demo complete!"
```

## 🔍 Troubleshooting

### Server Won't Start

```bash
# Check if port is in use
lsof -i :8402

# Kill existing process
kill -9 $(lsof -t -i:8402)

# Remove stale PID file
rm ~/.x402dev/mock-server.pid
```

### Config Not Found

```bash
# Check current directory
pwd

# Config must be at ./.x402/config.toml
ls .x402/config.toml

# Or specify explicit path
./target/debug/x402 config show --config /path/to/config.toml
```

### Permission Denied

```bash
# Ensure binary is executable
chmod +x ./target/debug/x402

# Or use cargo run
cargo run -- mock start
```

## 📈 Performance Benchmarks

```bash
# Measure server startup time
time ./target/release/x402 mock start --port 8402 &
sleep 1
kill $!

# Typical: ~50-100ms startup time

# Measure 402 response time
time curl -s http://127.0.0.1:8402/api/test > /dev/null

# Typical: <10ms response time
```

## ✅ Success Criteria

After running these tests, you should see:

- ✅ All 15 unit tests passing
- ✅ All 6 integration tests passing
- ✅ Server starts and stops cleanly
- ✅ 402 responses return correct headers
- ✅ Invoice generation creates unique memos
- ✅ PID locking prevents multiple servers
- ✅ Config validation catches errors
- ✅ Binary size ~2.6MB (release build)
- ✅ Zero blockchain dependencies
- ✅ Clean shutdown with Ctrl+C

## 🎯 Next Steps

1. **Epic 3**: Implement actual Solana blockchain verification
2. **Production deployment**: Deploy mock server for testing
3. **Client integration**: Integrate x402-cli into your application
4. **Monitoring**: Add metrics and logging
5. **Documentation**: API documentation and examples

---

**Platform Support**: Unix/Linux/macOS only. Windows users should use WSL2.
See [PLATFORM-REQUIREMENTS.md](./PLATFORM-REQUIREMENTS.md) for details.
