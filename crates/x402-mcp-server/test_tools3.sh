#!/bin/bash
# Proper MCP protocol handshake test

{
  # Step 1: Initialize request
  echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0.0"}}}'
  sleep 0.2

  # Step 2: Initialized notification (no id, no response expected)
  echo '{"jsonrpc":"2.0","method":"notifications/initialized"}'
  sleep 0.2

  # Step 3: Now we can request tools list
  echo '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}'
  sleep 0.5
} | ../../target/release/x402-mcp-server 2>/dev/null | {
  # Read responses
  read -r init_response
  read -r tools_response

  echo "=== Initialization Response ==="
  echo "$init_response" | jq -r '.result.serverInfo.name, .result.instructions'

  echo ""
  echo "=== Tool Count ==="
  echo "$tools_response" | jq -r '.result.tools | length'

  echo ""
  echo "=== All Tools ==="
  echo "$tools_response" | jq -r '.result.tools[] | "- \(.name): \(.description)"'
}
