# x402-mcp-server

> MCP server for x402-dev payment protocol testing toolkit

**Status:** 🚧 In Development (Epic 8, Phase 1)

## Overview

x402-mcp-server enables AI agents (Claude Code, Cline, Continue.dev) to natively interact with x402-dev's payment protocol testing toolkit through simple, workflow-based tools.

**Key Features:**
- ✅ **Rust-native**: Direct library integration (no subprocess overhead)
- ✅ **<1ms latency**: 10-1000x faster than subprocess approach
- ✅ **Type-safe**: End-to-end type safety with Rust
- ✅ **Zero subprocess risks**: No command injection, no temp file vulnerabilities

## Architecture

```
AI Agent (Claude Code)
  ↓
MCP Protocol (JSON-RPC 2.0 stdio)
  ↓
rmcp procedural macros
  ↓
Direct function calls (0ms)
  ↓
x402-core / x402-server / x402-cli
```

## Installation

**Prerequisites:**
- Rust 1.85.0+ (with Edition 2024 support)
- x402-dev CLI installed

**Install from source:**
```bash
cargo install --path crates/x402-mcp-server
```

**Configure Claude Code:**
```bash
claude mcp add x402-mcp x402-mcp-server
```

## Available Tools (Phase 1-4)

### Phase 1 (Day 1-2) - Foundation + 3 Tools
- ⏳ `x402__server_mock_start` - Start mock payment server
- ⏳ `x402__server_mock_status` - Check server status
- ⏳ `x402__policy_validate` - Validate policy YAML

### Phase 2 (Day 3-4) - Core Tools
- ⏳ `x402__testing_run_suite` - Execute YAML test suite
- ⏳ `x402__testing_check_compliance` - Validate 402 endpoint
- ⏳ `x402__policy_generate_express` - Generate Express middleware
- ⏳ `x402__server_mock_stop` - Stop mock server

## Usage Example

```javascript
// AI agent workflow
await mcp.use_tool("x402__server_mock_start", {
  port: 3402,
  pricing: 0.02
});

await mcp.use_tool("x402__testing_check_compliance", {
  url: "http://localhost:3402/api"
});
```

## Development Roadmap

| Phase | Duration | Status | Key Deliverables |
|-------|----------|--------|------------------|
| **Phase 1** | Day 1-2 | 🚧 In Progress | stdio transport + 3 tools |
| **Phase 2** | Day 3-4 | ⏳ Planned | All 7 tools functional |
| **Phase 3** | Day 5 | ⏳ Planned | 80%+ coverage, docs |
| **Phase 4** | Day 6 | ⏳ Planned | crates.io publish |

## Performance Targets

- **P95 Latency:** <1ms (vs 50-200ms subprocess approach)
- **Test Coverage:** 80%+
- **Security:** 0 critical vulnerabilities
- **Binary Size:** <3MB (optimized release)

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for development guidelines.

## License

MIT License - see [LICENSE](../../LICENSE) for details.
