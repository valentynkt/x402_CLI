#!/bin/bash
# Test MCP server tool registration

# Create a simple test that sends both messages and waits for responses
{
  cat <<'EOF'
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0.0"}}}
{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}
EOF
  sleep 0.5
} | ../../target/release/x402-mcp-server 2>/dev/null | {
  # Read both responses
  read -r init_response
  read -r tools_response

  echo "=== Tool Count ==="
  echo "$tools_response" | jq -r '.result.tools | length'

  echo ""
  echo "=== Tool Names ==="
  echo "$tools_response" | jq -r '.result.tools[].name'
}
