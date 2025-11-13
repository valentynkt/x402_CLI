#!/bin/bash
# Test x402__policy_validate tool with full output

# Create a test policy file
cat > /tmp/test_policy.yaml <<'EOF'
policies:
  - name: test-policy
    pricing: 0.001
    unit: request
EOF

{
  # Initialize
  echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0.0"}}}'
  sleep 0.2

  # Initialized notification
  echo '{"jsonrpc":"2.0","method":"notifications/initialized"}'
  sleep 0.2

  # Call policy_validate tool
  cat <<EOF
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"x402__policy_validate","arguments":{"policy_file":"/tmp/test_policy.yaml"}}}
EOF
  sleep 1
} | ../../target/release/x402-mcp-server 2>&1 | tail -10

rm -f /tmp/test_policy.yaml
