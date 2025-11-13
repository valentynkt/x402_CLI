#!/bin/bash
# Debug MCP server responses

{
  cat <<'EOF'
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0.0"}}}
{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}
EOF
  sleep 1
} | ../../target/release/x402-mcp-server 2>&1 | cat -n
