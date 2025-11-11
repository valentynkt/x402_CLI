#!/bin/bash

# x402-cli Quick Demo Script
# Demonstrates all Epic 1 + Epic 2 features in 2 minutes

set -e

echo "════════════════════════════════════════════════════════"
echo "x402-cli Quick Demo - Epic 1 + Epic 2 Features"
echo "════════════════════════════════════════════════════════"
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Build project
echo -e "${BLUE}📦 Building project...${NC}"
cargo build --release 2>&1 | tail -n 1
echo ""

# Epic 1: Init and Config
echo -e "${BLUE}📋 Epic 1: Project Initialization${NC}"
echo "Running: ./target/release/x402 init demo-x402"
./target/release/x402 init demo-x402
echo ""

echo -e "${BLUE}📝 Viewing configuration...${NC}"
echo "Running: ./target/release/x402 config show --config demo-x402/.x402/config.toml"
./target/release/x402 config show --config demo-x402/.x402/config.toml
echo ""

# Epic 2: Mock Server
echo -e "${BLUE}🚀 Epic 2: Starting Mock Server${NC}"
echo "Running: ./target/release/x402 mock start --port 8402"
./target/release/x402 mock start --port 8402 > /tmp/x402-server.log 2>&1 &
SERVER_PID=$!
echo "Server PID: $SERVER_PID"
sleep 2
echo ""

# Test 402 Response
echo -e "${BLUE}🧪 Testing 402 Payment Required Response${NC}"
echo "Running: curl -i http://127.0.0.1:8402/api/test"
curl -i http://127.0.0.1:8402/api/test
echo ""

# Generate Invoice
echo -e "${BLUE}💰 Generating Payment Invoice${NC}"
echo "Running: ./target/release/x402 invoice generate"
./target/release/x402 invoice generate \
  --recipient Dev1234567890abcdefghijklmnopqrstuvwxyzABCDEF \
  --amount 100 \
  --resource /api/test
echo ""

# Test WWW-Authenticate Header Format
echo -e "${BLUE}📄 Generating WWW-Authenticate Header${NC}"
./target/release/x402 invoice generate \
  --recipient Dev1234567890abcdefghijklmnopqrstuvwxyzABCDEF \
  --amount 250 \
  --resource /api/premium \
  --header
echo ""

# Shutdown
echo -e "${BLUE}🛑 Stopping Server (SIGTERM)${NC}"
kill -SIGTERM $SERVER_PID
sleep 2
echo ""

# Verify cleanup
echo -e "${BLUE}✅ Verifying Clean Shutdown${NC}"
if [ ! -f ~/.x402dev/mock-server.pid ]; then
    echo -e "${GREEN}✅ PID file removed successfully${NC}"
else
    echo -e "${YELLOW}⚠️  PID file still exists${NC}"
fi
echo ""

# Cleanup
echo -e "${BLUE}🧹 Cleaning up demo files...${NC}"
rm -rf demo-x402
echo ""

echo "════════════════════════════════════════════════════════"
echo -e "${GREEN}✅ Demo Complete!${NC}"
echo "════════════════════════════════════════════════════════"
echo ""
echo "Features Demonstrated:"
echo "  ✅ Project initialization (Epic 1)"
echo "  ✅ Configuration management (Epic 1)"
echo "  ✅ Mock server startup (Epic 2)"
echo "  ✅ 402 Payment Required responses (Epic 2)"
echo "  ✅ Invoice generation (Epic 2)"
echo "  ✅ WWW-Authenticate header format (Epic 2)"
echo "  ✅ Graceful shutdown (Epic 2)"
echo ""
echo "Next steps:"
echo "  • Run full test suite: cargo test"
echo "  • Run integration tests: ./tests/integration_test_epic2.sh"
echo "  • Read testing guide: docs/CLI-TESTING-GUIDE.md"
echo ""
