# Epic 8: MCP Server API Reference

**Version:** 0.1.0
**MCP Protocol:** v2025-06-18
**Last Updated:** 2025-11-12

---

## Tool Quick Reference

| Tool | Category | Purpose | Key Parameters | Avg Latency (Rust) | Error Codes |
|------|----------|---------|----------------|-------------------|-------------|
| `x402__server_mock_start` | Server | Start mock server | port, pricing | <1ms (direct lib call) | E3001-E3004 |
| `x402__server_mock_stop` | Server | Stop mock server | none | <1ms | E3004-E3005 |
| `x402__server_mock_status` | Server | Check server status | none | <0.5ms | none |
| `x402__testing_run_suite` | Testing | Run test suite | suite_yaml | <2ms + test exec time | E4001-E4003 |
| `x402__testing_check_compliance` | Testing | Validate endpoint | url | <2ms + HTTP request | E4003-E4005 |
| `x402__policy_validate` | Policy | Validate policy | policy_yaml | <1ms | E5001-E5003 |
| `x402__policy_generate_express` | Policy | Generate middleware | policy_yaml | <2ms | E5001-E5003 |

**Performance Note:** Rust direct integration eliminates 50-200ms subprocess overhead. Latency shown is MCP tool overhead only (not including I/O operations like HTTP requests or test execution).

---

## Common Types

### Tool Response Format

```typescript
interface ToolResponse {
  content: Array<{
    type: "text" | "image" | "resource";
    text?: string;
    data?: string;
    uri?: string;
  }>;
  isError?: boolean;
}
```

### Error Format

```typescript
interface McpError {
  code: string;           // E3001, E4002, etc.
  message: string;        // Human-readable description
  suggestion?: string;    // How to fix
  docs_link?: string;     // Documentation URL
  context?: {             // Additional context
    command?: string;
    args?: string[];
    exit_code?: number;
    [key: string]: any;
  };
}
```

---

## Tool 1: x402__server_mock_start

**Start x402-dev mock payment server for testing**

### Parameters

```typescript
{
  port?: number;              // Default: 3402, Range: 1024-65535
  pricing?: number;           // Default: 0.01, Min: 0
  simulation_mode?: string;   // "success" | "failure" | "timeout", Default: "success"
}
```

### Success Response

```json
{
  "status": "started",
  "pid": 12345,
  "port": 3402,
  "server_url": "http://localhost:3402",
  "config": {
    "pricing": 0.01,
    "simulation_mode": "success"
  },
  "started_at": "2025-11-12T13:42:01.123Z"
}
```

### Error Response

```json
{
  "error": {
    "code": "E3001",
    "message": "Port 3402 is already in use",
    "suggestion": "Stop existing server with x402__server_mock_stop or use different port",
    "context": { "port": 3402, "existing_pid": 12345 }
  }
}
```

### Error Codes

| Code | Description | Suggestion |
|------|-------------|------------|
| **E3001** | Port already in use | Stop existing server or use different port |
| **E3002** | Invalid port number | Use port between 1024-65535 |
| **E3003** | x402-dev CLI not found | Install: cargo install x402-dev |
| **E3004** | Server start failed | Check logs for details |

### Usage Example

```typescript
const result = await mcp.use_tool("x402__server_mock_start", {
  port: 3402,
  pricing: 0.02,
  simulation_mode: "success"
});

if (!result.error) {
  console.log(`Server started at ${result.server_url}`);
}
```

---

## Tool 2: x402__server_mock_stop

**Stop the running mock server gracefully**

### Parameters

None (finds running server via PID file)

### Success Response

```json
{
  "status": "stopped",
  "pid": 12345,
  "stopped_at": "2025-11-12T13:45:30.456Z"
}
```

### Error Response

```json
{
  "error": {
    "code": "E3004",
    "message": "No mock server is currently running",
    "suggestion": "Start server first with x402__server_mock_start"
  }
}
```

### Error Codes

| Code | Description | Suggestion |
|------|-------------|------------|
| **E3004** | Server not running | Start server first |
| **E3005** | Failed to stop server | May need manual kill |

---

## Tool 3: x402__server_mock_status

**Check mock server status**

### Parameters

None

### Response (Running)

```json
{
  "status": "running",
  "pid": 12345,
  "port": 3402,
  "server_url": "http://localhost:3402",
  "uptime_seconds": 120,
  "started_at": "2025-11-12T13:42:01.123Z",
  "config": {
    "pricing": 0.01,
    "simulation_mode": "success"
  }
}
```

### Response (Not Running)

```json
{
  "status": "not_running",
  "message": "No mock server is currently running"
}
```

---

## Tool 4: x402__testing_run_suite

**Execute YAML test suite to validate payment-protected endpoints**

### Parameters

```typescript
{
  suite_yaml: string;          // YAML test suite content (inline)
  output_format?: string;      // "json" | "junit" | "human", Default: "json"
  quiet?: boolean;             // Default: false
}
```

### Success Response

```json
{
  "summary": {
    "total_tests": 5,
    "passed": 4,
    "failed": 1,
    "skipped": 0,
    "duration_ms": 1250
  },
  "tests": [
    {
      "name": "Requires payment for protected endpoint",
      "status": "passed",
      "duration_ms": 150,
      "assertions": 2
    },
    {
      "name": "Invalid invoice rejection",
      "status": "failed",
      "duration_ms": 200,
      "error": "Expected status 402, got 500",
      "assertions_passed": 1,
      "assertions_failed": 1
    }
  ],
  "exit_code": 1
}
```

### Error Codes

| Code | Description | Suggestion |
|------|-------------|------------|
| **E4001** | Invalid test suite YAML | Check YAML syntax |
| **E4002** | Test execution failed | Check endpoint is running |
| **E4003** | Test suite validation failed | Missing required fields |

### Usage Example

```typescript
const suite_yaml = `
tests:
  - name: "Requires payment"
    request:
      url: "http://localhost:3402/api/data"
    assertions:
      - type: status_code
        expected: 402
      - type: header_exists
        header: "WWW-Authenticate"
`;

const result = await mcp.use_tool("x402__testing_run_suite", {
  suite_yaml,
  output_format: "json"
});
```

---

## Tool 5: x402__testing_check_compliance

**Validate HTTP 402 endpoint compliance with x402 protocol**

### Parameters

```typescript
{
  url: string;                 // Endpoint URL to check
  expected_recipient?: string; // Optional: Verify specific recipient
  expected_amount?: number;    // Optional: Verify specific amount
  timeout_seconds?: number;    // Default: 10
}
```

### Response (Compliant)

```json
{
  "compliant": true,
  "checks": [
    { "name": "HTTP 402 status code", "passed": true },
    { "name": "WWW-Authenticate header", "passed": true },
    { "name": "Recipient address format", "passed": true, "value": "TestRec123..." },
    { "name": "Amount validation", "passed": true, "value": "1000 USDC" },
    { "name": "Currency is USDC", "passed": true },
    { "name": "Memo format", "passed": true, "value": "req-abc123" },
    { "name": "Network specified", "passed": true, "value": "devnet" }
  ],
  "passed": 7,
  "total": 7,
  "warnings": []
}
```

### Response (Non-Compliant)

```json
{
  "compliant": false,
  "checks": [
    { "name": "HTTP 402 status code", "passed": true },
    { "name": "WWW-Authenticate header", "passed": true },
    { "name": "Amount validation", "passed": false, "error": "Missing 'amount' parameter" },
    { "name": "Network specified", "passed": false, "error": "Missing 'network' parameter" }
  ],
  "passed": 2,
  "total": 7,
  "warnings": ["Network parameter recommended for production"],
  "suggestions": [
    "Add 'amount=1000' to WWW-Authenticate header",
    "Add 'network=devnet' to WWW-Authenticate header"
  ]
}
```

### Error Codes

| Code | Description | Suggestion |
|------|-------------|------------|
| **E4003** | Endpoint unreachable | Check URL and network |
| **E4004** | Invalid HTTP response | Endpoint must return 402 status |
| **E4005** | Malformed WWW-Authenticate | Review header format |

---

## Tool 6: x402__policy_validate

**Validate payment policy YAML for syntax errors and logical conflicts**

### Parameters

```typescript
{
  policy_yaml: string;  // YAML policy content (inline)
}
```

### Response (Valid)

```json
{
  "valid": true,
  "has_errors": false,
  "has_warnings": true,
  "issues": [
    {
      "type": "warning",
      "message": "Overlapping rate limits detected",
      "details": "Policies #1 and #3 both match /api/*",
      "line": 12,
      "suggestions": [
        {
          "description": "Make patterns more specific",
          "action": "Change '/api/*' to '/api/v1/*' or '/api/v2/*'"
        }
      ]
    }
  ]
}
```

### Response (Invalid)

```json
{
  "valid": false,
  "has_errors": true,
  "has_warnings": false,
  "issues": [
    {
      "type": "error",
      "message": "Invalid policy type",
      "details": "Unknown type 'rate_limiter' (did you mean 'rate_limit'?)",
      "line": 8,
      "suggestions": [
        {
          "description": "Use valid policy type",
          "action": "Change 'rate_limiter' to 'rate_limit'"
        }
      ]
    }
  ]
}
```

### Error Codes

| Code | Description | Suggestion |
|------|-------------|------------|
| **E5001** | Invalid policy YAML | Check YAML syntax |
| **E5002** | Validation errors found | Fix logical conflicts |
| **E5003** | Missing required fields | Review policy structure |

---

## Tool 7: x402__policy_generate_express

**Generate Express.js middleware from payment policy YAML**

### Parameters

```typescript
{
  policy_yaml: string;
  filename?: string;  // Source filename for comments, Default: "policy.yaml"
}
```

### Success Response

```json
{
  "generated_code": "const express = require('express');\n\n// Generated from policy.yaml\nmodule.exports = function x402Middleware(options) {\n  return (req, res, next) => {\n    // Rate limiting logic...\n  };\n};\n",
  "lines": 150,
  "size_bytes": 4523,
  "framework": "express"
}
```

### Error Response

```json
{
  "error": {
    "code": "E5002",
    "message": "Cannot generate code from invalid policy",
    "suggestion": "Fix validation errors first using x402__policy_validate",
    "validation_errors": [
      { "line": 8, "message": "Invalid policy type" }
    ]
  }
}
```

### Error Codes

| Code | Description | Suggestion |
|------|-------------|------------|
| **E5001** | Invalid policy YAML | Check YAML syntax |
| **E5002** | Code generation failed | Fix validation errors first |
| **E5003** | Unsupported policy features | Review supported features |

### Usage Example

```typescript
const policy_yaml = `
policies:
  - type: rate_limit
    pattern: "/api/*"
    max_requests: 100
    window: 3600
`;

const result = await mcp.use_tool("x402__policy_generate_express", {
  policy_yaml
});

if (!result.error) {
  // Save generated_code to middleware/policy.js
  console.log(`Generated ${result.lines} lines`);
}
```

---

## Error Code Catalog

**Note:** All error responses include structured JSON with `error`, `message`, `suggestion`, and `docs_link` fields.

### Mock Server Errors (E3xxx)

| Code | Message | Suggestion | Docs Link |
|------|---------|------------|-----------|
| E3001 | Port already in use | Stop existing server or use different port | TBD (https://docs.x402-dev.com/errors/E3001) |
| E3002 | Invalid port number | Use port between 1024-65535 | TBD (https://docs.x402-dev.com/errors/E3002) |
| E3003 | x402-dev CLI not found | Install: cargo install x402-dev | TBD (https://docs.x402-dev.com/errors/E3003) |
| E3004 | Server not running / Failed to start | Start server first / Check logs | TBD (https://docs.x402-dev.com/errors/E3004) |
| E3005 | Failed to stop server | May need manual kill (see docs) | TBD (https://docs.x402-dev.com/errors/E3005) |

### Testing Errors (E4xxx)

| Code | Message | Suggestion | Docs Link |
|------|---------|------------|-----------|
| E4001 | Invalid test suite YAML | Check YAML syntax and structure | TBD (https://docs.x402-dev.com/errors/E4001) |
| E4002 | Test execution failed | Verify endpoint is running and accessible | TBD (https://docs.x402-dev.com/errors/E4002) |
| E4003 | Endpoint unreachable / Test suite validation failed | Check URL, network / Review required fields | TBD (https://docs.x402-dev.com/errors/E4003) |
| E4004 | Protocol non-compliant | Must return HTTP 402 status | TBD (https://docs.x402-dev.com/errors/E4004) |
| E4005 | Malformed WWW-Authenticate header | Review x402 protocol specification | TBD (https://docs.x402-dev.com/errors/E4005) |

### Policy Errors (E5xxx)

| Code | Message | Suggestion | Docs Link |
|------|---------|------------|-----------|
| E5001 | Invalid policy YAML | Check YAML syntax and structure | TBD (https://docs.x402-dev.com/errors/E5001) |
| E5002 | Validation errors found / Code generation failed | Fix logical conflicts / Fix validation errors first | TBD (https://docs.x402-dev.com/errors/E5002) |
| E5003 | Missing required fields / Unsupported features | Review policy structure / Check supported features | TBD (https://docs.x402-dev.com/errors/E5003) |

### General Errors (E9xxx)

| Code | Message | Suggestion |
|------|---------|------------|
| E9001 | Operation timeout | Check system performance, increase timeout |
| E9002 | Permission denied | Check file/process permissions |
| E9003 | Unknown error | Check logs for details |

---

## Library Integration Reference

**Architecture:** Rust MCP server with direct library integration (not subprocess)

| MCP Tool | x402-core/x402-server Function |
|----------|-------------------------------|
| `x402__server_mock_start` | `x402_server::start_server(MockServerConfig)` |
| `x402__server_mock_stop` | `x402_server::stop_server()` |
| `x402__server_mock_status` | `x402_server::server_status()` |
| `x402__testing_run_suite` | `x402_core::testing::execute_test_suite(&TestSuite)` |
| `x402__testing_check_compliance` | `x402_core::testing::check_compliance(&url)` |
| `x402__policy_validate` | `x402_core::policy::validate_policies(&policy_yaml)` |
| `x402__policy_generate_express` | `x402_core::policy::generate_middleware(&policy, Framework::Express)` |

**Key Advantages:**
- ✅ **Direct function calls** - No subprocess overhead (10-1000x faster)
- ✅ **No temp files** - Works with data structures in memory
- ✅ **Type-safe** - Rust → Rust compile-time guarantees
- ✅ **Zero security risks** - No command injection, no temp file vulnerabilities

**Note:** MCP tools accept inline YAML/content (not file paths) for better AI agent experience.

---

## Tool Naming Convention

**Pattern:** `x402__<domain>__<action>`

- **Namespace:** `x402__` (clear project namespace)
- **Domain:** `server`, `testing`, `policy` (logical grouping)
- **Action:** `mock_start`, `run_suite`, `validate` (verb + noun)

**Examples:**
- `x402__server_mock_start` → server domain, mock_start action
- `x402__testing_run_suite` → testing domain, run_suite action
- `x402__policy_validate` → policy domain, validate action

---

## Best Practices

### For AI Agents

1. **Always check tool results** for `error` field before proceeding
2. **Use structured data** instead of parsing text output
3. **Chain tools** for complex workflows (start server → run tests)
4. **Handle errors gracefully** using `suggestion` field

### For Developers

1. **Validate parameters** using Zod schemas before execution
2. **Return structured JSON** for all responses
3. **Include suggestions** in all error messages
4. **Test error paths** as thoroughly as success paths

### Performance Tips

1. **Cache idempotent operations** (status checks, validation)
2. **Use background processes** for long-running operations
3. **Implement timeouts** (default: 120s, max: 600s)
4. **Monitor latency** and optimize slow operations

---

**Total:** ~4,000 words | Complete API reference

For integration guides, see `EPIC-8-OVERVIEW.md` and `IMPLEMENTATION-GUIDE.md`.
