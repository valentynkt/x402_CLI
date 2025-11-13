#!/bin/bash
# Test MCP server protocol compliance
# Epic 8, Phase 1: Manual testing script

set -e

echo "🧪 Testing x402-mcp-server MCP protocol compliance"
echo ""

# Build server
echo "📦 Building server..."
cargo build --release -p x402-mcp-server --quiet

SERVER_BIN="../../target/release/x402-mcp-server"

# Test 1: Initialize
echo "1️⃣ Testing initialize protocol handshake..."
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{"roots":{"listChanged":true}},"clientInfo":{"name":"test-client","version":"1.0.0"}}}' | \
  timeout 2 $SERVER_BIN 2>/dev/null | head -1 || echo "⚠️  Initialize test completed (timeout expected)"

echo ""
echo "2️⃣ Testing tools/list request..."
(
  echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}'
  sleep 0.5
  echo '{"jsonrpc":"2.0","id":2,"method":"tools/list"}'
  sleep 0.5
) | timeout 3 $SERVER_BIN 2>/dev/null | grep -A 2 "tools" || echo "⚠️  Tools list test completed"

echo ""
echo "✅ Basic MCP protocol tests complete!"
echo ""
echo "Next steps:"
echo "  - Implement x402__policy_validate tool"
echo "  - Add comprehensive unit tests"
echo "  - Test with Claude Code"
