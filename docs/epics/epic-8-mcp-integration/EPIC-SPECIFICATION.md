# Epic 8: MCP Server Integration - Complete Specification

**Status:** Planning/Design
**Created:** 2025-11-12
**Author:** x402-dev Team
**Epic Duration:** 3-4 weeks
**Target Release:** x402-dev v0.3.0

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Requirements Specification](#2-requirements-specification)
3. [Scope & Boundaries](#3-scope--boundaries)
4. [Implementation Roadmap](#4-implementation-roadmap)
5. [API Specification](#5-api-specification)
6. [Risk Assessment](#6-risk-assessment)
7. [Success Metrics & KPIs](#7-success-metrics--kpis)
8. [Next Steps](#8-next-steps)

---

## 1. Executive Summary

### 1.1 What We're Building

**x402-mcp-server** is a Model Context Protocol (MCP) server that enables AI agents (Claude Code, Cline, Continue.dev, Cursor) to natively interact with x402-dev's payment protocol testing toolkit. Instead of AI agents needing to learn complex CLI commands, they can use simple, workflow-based tools with natural language descriptions.

**Example:**
```typescript
// AI agent can now do this:
await mcp.use_tool("x402__server_mock_start", { port: 3402 });
await mcp.use_tool("x402__testing_run_suite", { suite_yaml: "..." });
await mcp.use_tool("x402__testing_check_compliance", { url: "http://localhost:3402/api" });

// Instead of constructing complex bash commands:
// exec("x402-dev mock --port 3402 &");
// exec("x402-dev test tests/suite.yaml --format json");
```

### 1.2 Why This Matters

**Strategic Value:**
- **First-Mover Advantage:** First x402 protocol tool with native AI agent support
- **Solana Hackathon 2025:** Perfect timing for AI + blockchain hackathon track
- **Market Opportunity:** AI coding tool adoption growing 300%+ YoY
- **Ecosystem Expansion:** Opens x402-dev to entire AI developer ecosystem

**Technical Value:**
- **Lower Barrier to Entry:** AI agents can test payment APIs without learning CLI
- **Workflow Automation:** Enable complex multi-step payment testing workflows
- **Better Error Handling:** Structured errors with actionable suggestions for AI
- **Tool Composability:** AI agents can chain tools to build complex scenarios

**User Value:**
- **Faster Development:** "Start mock server and validate my API" → 10 seconds
- **Natural Language:** Describe what you want, AI figures out the tools
- **Reduced Errors:** AI validates parameters before execution
- **Learning Aid:** Developers learn x402-dev through AI interactions

### 1.3 How It Works

```
┌──────────────────────────────────────────────────────────────┐
│ Developer: "Start a mock payment server and test my API"     │
└────────────────────┬─────────────────────────────────────────┘
                     │
                     ↓
┌──────────────────────────────────────────────────────────────┐
│ AI Agent (Claude Code):                                       │
│   1. use_tool("x402__server_mock_start", {port: 3402})      │
│   2. use_tool("x402__testing_check_compliance", {...})      │
│   3. Report results to developer                             │
└────────────────────┬─────────────────────────────────────────┘
                     │
                     ↓
┌──────────────────────────────────────────────────────────────┐
│ x402-mcp-server (TypeScript):                                 │
│   - Validates parameters (Zod schemas)                        │
│   - Executes: x402-dev mock --port 3402                       │
│   - Parses output, formats as JSON                            │
│   - Returns structured response to AI                         │
└────────────────────┬─────────────────────────────────────────┘
                     │
                     ↓
┌──────────────────────────────────────────────────────────────┐
│ x402-dev CLI (Rust): Executes commands                        │
└──────────────────────────────────────────────────────────────┘
```

### 1.4 Timeline & Milestones

| Week | Phase | Deliverables |
|------|-------|--------------|
| **Week 1** | Foundation | Working MCP server with 1 tool, basic tests |
| **Week 2** | Core Tools | 5 functional workflow tools, 60%+ coverage |
| **Week 3** | Polish | 7 tools, complete docs, 80%+ coverage |
| **Week 4** | Production | NPM package, public release, community resources |

### 1.5 Success Criteria

**Must-Have (MVP):**
- ✅ All 7 core tools functional and tested
- ✅ <200ms average tool execution latency
- ✅ 80%+ test coverage (unit + integration)
- ✅ Claude Code integration working
- ✅ Published to NPM

**Should-Have (v1.0):**
- ✅ Listed in MCP directory
- ✅ Integration examples for 2+ AI tools
- ✅ 50+ NPM downloads in week 1
- ✅ Zero critical security vulnerabilities

**Nice-to-Have (Future):**
- Integration with Cline, Continue.dev, Cursor
- HTTP transport support
- Streaming for long-running operations
- Real Solana payment verification tools

---

## 2. Requirements Specification

### 2.1 Functional Requirements

#### FR-1: MCP Protocol Compliance
**Requirement:** Implement MCP protocol v2025-06-18 specification
**Priority:** P0 (Critical)
**Acceptance Criteria:**
- JSON-RPC 2.0 message format
- Proper tool listing via `tools/list` method
- Tool execution via `tools/call` method
- Error responses follow MCP error schema
- Protocol handshake completes successfully

#### FR-2: Seven Core Workflow Tools
**Requirement:** Implement 7 workflow-based tools covering mock server, testing, and policy operations
**Priority:** P0 (Critical)
**Tools:**

1. **`x402__server_mock_start`**
   - Start HTTP mock server for testing
   - Parameters: `port`, `pricing`, `simulation_mode`
   - Returns: Server URL, PID, configuration

2. **`x402__server_mock_stop`**
   - Stop running mock server
   - Parameters: None (finds running server via PID)
   - Returns: Success/failure status

3. **`x402__server_mock_status`**
   - Check mock server status
   - Parameters: None
   - Returns: Running status, uptime, configuration

4. **`x402__testing_run_suite`**
   - Execute YAML test suite
   - Parameters: `suite_yaml` (inline string), `output_format`
   - Returns: Test results in JSON/JUnit format

5. **`x402__testing_check_compliance`**
   - Validate HTTP 402 endpoint compliance
   - Parameters: `url`, `expected_recipient`, `expected_amount`
   - Returns: Compliance report with pass/fail checks

6. **`x402__policy_validate`**
   - Validate payment policy YAML
   - Parameters: `policy_yaml` (inline string)
   - Returns: Validation report with errors/warnings

7. **`x402__policy_generate_express`**
   - Generate Express.js middleware from policy
   - Parameters: `policy_yaml`, `filename`
   - Returns: Generated code as string

**Acceptance Criteria:**
- Each tool has complete JSON Schema definition
- Each tool has TypeScript type definitions
- Each tool handles all error scenarios
- Each tool has usage examples

#### FR-3: stdio Transport Implementation
**Requirement:** Use stdio (standard input/output) transport for Claude Code integration
**Priority:** P0 (Critical)
**Acceptance Criteria:**
- No stdout logging (corrupts JSON-RPC stream)
- All logging goes to stderr
- Clean JSON-RPC message formatting
- Graceful shutdown on SIGINT/SIGTERM
- Handles large messages (up to 10MB)

#### FR-4: Parameter Validation
**Requirement:** Validate all tool parameters using Zod schemas
**Priority:** P0 (Critical)
**Acceptance Criteria:**
- Every tool has Zod schema for parameters
- Invalid parameters return structured errors
- Type coercion for compatible types (string → number)
- Required vs optional parameters enforced
- Default values applied correctly

#### FR-5: Error Handling
**Requirement:** Translate CLI errors to structured MCP errors with actionable suggestions
**Priority:** P0 (Critical)
**Acceptance Criteria:**
- Structured error codes (E3001, E4001, etc.)
- Clear error messages (no technical jargon)
- Actionable suggestions ("Try: x402-dev mock stop")
- Documentation links for complex errors
- Context preservation (command, args, exit code)

#### FR-6: Claude Code Compatibility
**Requirement:** Full compatibility with Claude Code MCP client
**Priority:** P0 (Critical)
**Acceptance Criteria:**
- Installation via `claude mcp add` works
- Tools discoverable in Claude Code
- Tool descriptions render correctly
- Parameter prompts work
- Results display properly

#### FR-7: Tool Discovery & Listing
**Requirement:** AI agents can discover available tools and their schemas
**Priority:** P0 (Critical)
**Acceptance Criteria:**
- `tools/list` returns all 7 tools
- Each tool has name, description, inputSchema
- Schemas include parameter descriptions
- Examples included in descriptions

### 2.2 Non-Functional Requirements

#### NFR-1: Performance
**Requirement:** Tool execution latency <200ms (95th percentile)
**Priority:** P0 (Critical)
**Rationale:** AI agents timeout after 30s; fast tools feel responsive
**Metrics:**
- Average latency: <100ms
- P95 latency: <200ms
- P99 latency: <500ms
- Subprocess spawn: <50ms
- JSON parsing: <10ms

**Acceptance Criteria:**
- Benchmark tests pass
- No performance regressions in CI
- Load test handles 10+ concurrent tools

#### NFR-2: Reliability
**Requirement:** 99%+ uptime, graceful degradation
**Priority:** P0 (Critical)
**Acceptance Criteria:**
- Handles x402-dev CLI crashes gracefully
- Recovers from subprocess errors
- No memory leaks (run for 24h without OOM)
- Proper process cleanup on shutdown

#### NFR-3: Security
**Requirement:** Prevent injection attacks, DoS, data leaks
**Priority:** P0 (Critical)
**Security Controls:**
- Input validation (all user inputs sanitized)
- Command allowlisting (only `x402-dev` executable)
- Rate limiting (10 req/min per tool)
- Timeout enforcement (120s default, 600s max)
- No sensitive data in logs (redact payment proofs)

**Acceptance Criteria:**
- Security scan passes (npm audit, Snyk)
- Penetration testing finds no critical issues
- DoS test (1000 concurrent requests) doesn't crash

#### NFR-4: Maintainability
**Requirement:** 80%+ test coverage, clear code structure
**Priority:** P1 (High)
**Acceptance Criteria:**
- Unit test coverage: 80%+
- Integration test coverage: 60%+
- Code follows TypeScript best practices
- All functions have JSDoc comments
- README includes contribution guidelines

#### NFR-5: Usability
**Requirement:** Installation in <5 minutes, clear error messages
**Priority:** P1 (High)
**Acceptance Criteria:**
- NPM install completes in <60 seconds
- Claude Code setup documented (3 steps)
- First tool call works within 5 minutes
- Error messages include "how to fix" suggestions
- Troubleshooting guide covers 90% of issues

#### NFR-6: Documentation
**Requirement:** Complete API reference, integration guides, examples
**Priority:** P1 (High)
**Acceptance Criteria:**
- Every tool documented with examples
- Integration guide for Claude Code
- Troubleshooting guide with common issues
- Architecture documentation (this epic)
- Contributing guidelines

#### NFR-7: Compatibility
**Requirement:** Works on macOS, Linux, Windows (WSL2)
**Priority:** P1 (High)
**Acceptance Criteria:**
- CI tests pass on all 3 platforms
- Installation instructions for each platform
- Known issues documented (e.g., Windows native not supported)

#### NFR-8: Scalability
**Requirement:** Handle 10+ concurrent tool invocations
**Priority:** P2 (Medium)
**Acceptance Criteria:**
- Load test with 10 concurrent calls succeeds
- No resource exhaustion under load
- Proper queuing for expensive operations

### 2.3 Constraints

1. **Technical Constraints:**
   - Must use TypeScript (MCP SDK requirement)
   - Must work with existing x402-dev CLI (no source changes)
   - Must support Node.js 18+ (MCP SDK requirement)
   - stdio transport only (HTTP/WebSocket future)

2. **Resource Constraints:**
   - Single developer (initially)
   - 3-4 week timeline
   - No budget for infrastructure (local-only)

3. **Business Constraints:**
   - Must be ready for Solana Hackathon 2025 showcase
   - Must maintain x402-dev brand consistency
   - Must be open source (MIT license)

### 2.4 Assumptions

1. **User Assumptions:**
   - Users have x402-dev CLI installed and working
   - Users have Node.js 18+ installed
   - Users are familiar with AI coding tools (Claude Code, etc.)

2. **Technical Assumptions:**
   - x402-dev CLI API is stable (no breaking changes in 0.2.x)
   - MCP protocol remains stable (v2025-06-18)
   - stdio transport sufficient for local use cases

3. **Market Assumptions:**
   - AI coding tool adoption continues to grow
   - Developers prefer AI workflows over manual CLI
   - Payment protocol testing remains relevant

---

## 3. Scope & Boundaries

### 3.1 In Scope

**Core Functionality:**
- ✅ 7 workflow tools (mock server × 3, testing × 2, policy × 2)
- ✅ stdio transport implementation
- ✅ Parameter validation (Zod schemas)
- ✅ Error handling & translation
- ✅ Claude Code integration
- ✅ NPM package distribution

**Documentation:**
- ✅ API reference (all 7 tools)
- ✅ Integration guide (Claude Code)
- ✅ Architecture documentation (this epic)
- ✅ Troubleshooting guide
- ✅ Example workflows

**Testing:**
- ✅ Unit tests (80%+ coverage)
- ✅ Integration tests (60%+ coverage)
- ✅ Load tests (10+ concurrent)
- ✅ Security tests (injection, DoS)

**Quality:**
- ✅ TypeScript type safety
- ✅ Error handling for all edge cases
- ✅ Performance benchmarks
- ✅ Security audit

### 3.2 Out of Scope (Future Work)

**Advanced Transports:**
- ❌ HTTP transport (SSE/WebSockets)
- ❌ Remote MCP server deployment
- ❌ Multi-user authentication

**Advanced Features:**
- ❌ Real Solana payment verification (stays in CLI)
- ❌ Payment cache (Redis/PostgreSQL)
- ❌ Multi-language bindings (Python, Go, etc.)
- ❌ Streaming for long operations (progress updates)

**Extended Integrations:**
- ❌ Cline integration (community can add)
- ❌ Continue.dev integration (community)
- ❌ Cursor integration (community)
- ❌ VS Code extension

**Production Features:**
- ❌ Distributed tracing
- ❌ Metrics & monitoring (Prometheus)
- ❌ High availability / clustering
- ❌ Enterprise authentication

**Rationale for Scope:**
- Focus on MVP (7 tools, Claude Code)
- Ship fast for Solana Hackathon
- Community can extend based on demand
- Keep complexity low for v1.0

### 3.3 Dependencies

**External Dependencies:**
| Dependency | Version | Purpose | Risk |
|------------|---------|---------|------|
| **@modelcontextprotocol/sdk** | ^0.1.0 | MCP protocol implementation | LOW (official SDK) |
| **zod** | ^3.22.0 | Parameter validation | LOW (mature, stable) |
| **typescript** | ^5.3.0 | Type safety | LOW (standard) |
| **Node.js** | 18+ | Runtime | LOW (LTS) |
| **x402-dev CLI** | 0.1+ | Backend commands | MEDIUM (external) |

**Internal Dependencies:**
- x402-dev CLI stability (breaking changes would require updates)
- x402-dev CLI performance (subprocess overhead adds latency)

**Risk Mitigation:**
- Version pinning for x402-dev CLI (compatible range)
- Version detection + compatibility checks
- Fallback error messages if CLI not found

### 3.4 Success Criteria

**Phase 1 Success (Foundation - Week 1):**
- ✅ MCP server accepts connections
- ✅ 1 tool working end-to-end
- ✅ Basic test coverage (50%+)
- ✅ Documentation started

**Phase 2 Success (Core Tools - Week 2):**
- ✅ 5 tools functional
- ✅ Test coverage 60%+
- ✅ Integration tests passing
- ✅ Error handling implemented

**Phase 3 Success (Polish - Week 3):**
- ✅ All 7 tools complete
- ✅ Test coverage 80%+
- ✅ Complete documentation
- ✅ Security review passed

**Phase 4 Success (Production - Week 4):**
- ✅ NPM package published
- ✅ MCP directory listing live
- ✅ Solana Hackathon showcase ready
- ✅ Community resources available

**Epic Success (Overall):**
- ✅ All technical KPIs met (see Section 7)
- ✅ All user KPIs met (see Section 7)
- ✅ All ecosystem KPIs met (see Section 7)
- ✅ Positive community feedback
- ✅ Zero critical bugs in first week

---

## 4. Implementation Roadmap

### 4.1 Phase 1: Foundation (Week 1, Days 1-5)

**Goal:** Build MCP server infrastructure with 1 working tool

#### Day 1-2: Project Setup & MCP SDK Integration

**Tasks:**
1. Initialize TypeScript project (`npm init`, `tsconfig.json`)
2. Install dependencies (`@modelcontextprotocol/sdk`, `zod`, `typescript`)
3. Set up project structure:
   ```
   x402-mcp-server/
   ├── src/
   │   ├── index.ts           # Entry point
   │   ├── server.ts          # MCP protocol handler
   │   ├── tools/             # Tool implementations
   │   │   └── index.ts
   │   ├── types.ts           # TypeScript types
   │   └── utils/
   │       ├── subprocess.ts  # CLI executor
   │       └── errors.ts      # Error translation
   ├── tests/
   │   ├── unit/
   │   └── integration/
   ├── package.json
   ├── tsconfig.json
   └── README.md
   ```
4. Configure TypeScript compiler (strict mode, ES2022 target)
5. Set up Jest for testing
6. Initialize Git repository

**Deliverables:**
- ✅ Working TypeScript build
- ✅ Basic project structure
- ✅ Jest configured

**Time Estimate:** 8 hours

#### Day 3-4: stdio Transport & Tool Registry

**Tasks:**
1. Implement stdio transport (MCP SDK)
   ```typescript
   import { Server } from '@modelcontextprotocol/sdk/server/index.js';
   import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';

   const server = new Server({
     name: 'x402-dev-mcp',
     version: '0.1.0',
   }, {
     capabilities: { tools: {} }
   });

   const transport = new StdioServerTransport();
   await server.connect(transport);
   ```

2. Implement tool registry (registration, lookup)
3. Add logging (stderr only, structured format)
4. Implement graceful shutdown (SIGINT/SIGTERM)
5. Create subprocess executor utility:
   ```typescript
   async function execX402Dev(
     command: string,
     args: string[],
     options?: ExecOptions
   ): Promise<ExecResult> {
     // Spawn x402-dev CLI
     // Parse stdout/stderr
     // Handle errors
   }
   ```

**Deliverables:**
- ✅ stdio transport working
- ✅ Tool registration system
- ✅ Subprocess executor
- ✅ Logging infrastructure

**Time Estimate:** 12 hours

#### Day 5: First Tool (Proof of Concept)

**Tasks:**
1. Implement `x402__server_mock_start` tool:
   ```typescript
   server.setRequestHandler(CallToolRequestSchema, async (request) => {
     if (request.params.name === 'x402__server_mock_start') {
       const params = mockStartSchema.parse(request.params.arguments);
       const result = await execX402Dev('mock', [
         '--port', params.port.toString(),
         '--pricing', params.pricing.toString()
       ]);
       return {
         content: [{
           type: 'text',
           text: JSON.stringify({ status: 'started', ...result })
         }]
       };
     }
   });
   ```

2. Add Zod schema for parameters
3. Add unit tests (parameter validation)
4. Add integration test (real CLI execution)
5. Test with Claude Code locally

**Deliverables:**
- ✅ 1 working tool end-to-end
- ✅ Tests passing
- ✅ Verified with Claude Code

**Time Estimate:** 8 hours

**Phase 1 Total:** 28 hours (~3.5 days)

---

### 4.2 Phase 2: Core Tools (Week 2, Days 6-10)

**Goal:** Implement 5 workflow tools (mock server + testing)

#### Day 6-7: Mock Server Tools

**Tasks:**
1. Complete `x402__server_mock_start` (edge cases, errors)
2. Implement `x402__server_mock_stop`:
   ```typescript
   // Read PID file, send SIGTERM
   // If CLI has 'x402-dev mock stop', use that
   // Otherwise, manual kill
   ```
3. Implement `x402__server_mock_status`:
   ```typescript
   // Check PID file existence
   // Verify process running (kill -0)
   // Return status + uptime
   ```
4. Add error handling:
   - E3001: Port already in use
   - E3002: Invalid port number
   - E3003: x402-dev CLI not found
   - E3004: Server not running
5. Add integration tests (start → status → stop)

**Deliverables:**
- ✅ 3 mock server tools complete
- ✅ Tests passing (unit + integration)
- ✅ Error scenarios covered

**Time Estimate:** 12 hours

#### Day 8-9: Testing Tools

**Tasks:**
1. Implement `x402__testing_run_suite`:
   ```typescript
   // Accept inline YAML string (not file path)
   // Write to temp file
   // Execute: x402-dev test <tempfile> --format json
   // Parse result, return structured JSON
   ```
2. Implement `x402__testing_check_compliance`:
   ```typescript
   // Execute: x402-dev check <url> --format json
   // Parse compliance report
   // Return pass/fail with details
   ```
3. Add error handling:
   - E4001: Invalid test suite YAML
   - E4002: Test execution failed
   - E4003: Endpoint unreachable
   - E4004: Protocol non-compliant
4. Add integration tests (real test execution)
5. Test with various YAML formats

**Deliverables:**
- ✅ 2 testing tools complete
- ✅ YAML parsing robust
- ✅ Error handling comprehensive

**Time Estimate:** 12 hours

#### Day 10: Error Handling & Integration Tests

**Tasks:**
1. Implement error translator:
   ```typescript
   function translateCliError(cliError: CliError): McpError {
     return {
       code: mapErrorCode(cliError.code),
       message: cliError.message,
       suggestion: generateSuggestion(cliError),
       docs_link: `https://docs.x402-dev.com/errors/${cliError.code}`,
       context: { command, args, exitCode }
     };
   }
   ```
2. Add structured error codes (E3xxx, E4xxx)
3. Add actionable suggestions for each error
4. Create integration test suite (end-to-end workflows)
5. Achieve 60%+ test coverage

**Deliverables:**
- ✅ Error translation complete
- ✅ Integration tests passing
- ✅ 60%+ coverage achieved

**Time Estimate:** 8 hours

**Phase 2 Total:** 32 hours (~4 days)

---

### 4.3 Phase 3: Polish & Advanced (Week 3, Days 11-15)

**Goal:** Complete remaining tools, documentation, achieve 80%+ coverage

#### Day 11-12: Policy Tools

**Tasks:**
1. Implement `x402__policy_validate`:
   ```typescript
   // Accept inline policy YAML
   // Execute: x402-dev policy validate
   // Parse validation report
   // Return structured errors/warnings
   ```
2. Implement `x402__policy_generate_express`:
   ```typescript
   // Accept inline policy YAML
   // Execute: x402-dev policy generate --framework express
   // Return generated code as string
   ```
3. Add error handling:
   - E5001: Invalid policy YAML
   - E5002: Code generation failed
   - E5003: Validation errors found
4. Add integration tests

**Deliverables:**
- ✅ 2 policy tools complete
- ✅ All 7 tools functional

**Time Estimate:** 12 hours

#### Day 13: Error Handling Enhancement

**Tasks:**
1. Review all error scenarios
2. Add missing error codes
3. Improve error messages (clearer language)
4. Add suggestions for all errors
5. Add documentation links
6. Test all error paths

**Error Code Catalog:**
```typescript
const ERROR_CODES = {
  // Mock Server (E3xxx)
  E3001: { message: 'Port already in use', suggestion: 'Stop existing server or use different port' },
  E3002: { message: 'Invalid port number', suggestion: 'Use port between 1024-65535' },
  E3003: { message: 'x402-dev CLI not found', suggestion: 'Install: cargo install x402-dev' },
  E3004: { message: 'Server not running', suggestion: 'Start server first: x402__server_mock_start' },

  // Testing (E4xxx)
  E4001: { message: 'Invalid test suite YAML', suggestion: 'Check YAML syntax' },
  E4002: { message: 'Test execution failed', suggestion: 'Check endpoint is running' },
  E4003: { message: 'Endpoint unreachable', suggestion: 'Verify URL and network' },
  E4004: { message: 'Protocol non-compliant', suggestion: 'Review WWW-Authenticate header format' },

  // Policy (E5xxx)
  E5001: { message: 'Invalid policy YAML', suggestion: 'Check YAML syntax and policy structure' },
  E5002: { message: 'Code generation failed', suggestion: 'Review policy validation errors' },
  E5003: { message: 'Validation errors found', suggestion: 'Fix errors before generating code' },
};
```

**Deliverables:**
- ✅ All errors have codes
- ✅ All errors have suggestions
- ✅ Error tests passing

**Time Estimate:** 8 hours

#### Day 14: Documentation

**Tasks:**
1. Write API reference (README.md):
   - Installation instructions
   - Quick start guide
   - All 7 tools documented with examples
   - Parameter descriptions
   - Error codes reference
2. Write integration guide:
   - Claude Code setup (3 steps)
   - First tool invocation
   - Example workflows (3 scenarios)
   - Troubleshooting common issues
3. Write contributing guidelines
4. Add JSDoc comments to all functions

**Deliverables:**
- ✅ Complete README
- ✅ Integration guide
- ✅ Contributing guide
- ✅ Code comments

**Time Estimate:** 8 hours

#### Day 15: Example Workflows & Testing

**Tasks:**
1. Create example workflows:
   - **Workflow 1:** "Build Payment-Protected API"
     ```typescript
     // 1. Start mock server
     await mcp.use_tool("x402__server_mock_start", { port: 3402 });

     // 2. Validate policy
     await mcp.use_tool("x402__policy_validate", { policy_yaml: "..." });

     // 3. Generate middleware
     await mcp.use_tool("x402__policy_generate_express", { policy_yaml: "..." });

     // 4. Test endpoint
     await mcp.use_tool("x402__testing_check_compliance", { url: "http://localhost:8080/api" });
     ```

   - **Workflow 2:** "Validate Existing Endpoint"
   - **Workflow 3:** "Run Test Suite"

2. Add troubleshooting guide:
   - Issue: "x402-dev not found" → Solution: Install CLI
   - Issue: "Port already in use" → Solution: Change port or stop server
   - Issue: "Tests failing" → Solution: Check mock server running
   - [10+ common issues]

3. Achieve 80%+ test coverage:
   - Add missing unit tests
   - Add edge case tests
   - Add error scenario tests

**Deliverables:**
- ✅ 3 example workflows
- ✅ Troubleshooting guide
- ✅ 80%+ coverage

**Time Estimate:** 8 hours

**Phase 3 Total:** 36 hours (~4.5 days)

---

### 4.4 Phase 4: Production Readiness (Week 4, Days 16-20)

**Goal:** Security review, packaging, distribution, community resources

#### Day 16-17: Testing & Quality

**Tasks:**
1. **Load Testing:**
   ```typescript
   // Test: 10 concurrent tool invocations
   const promises = Array(10).fill(null).map(() =>
     mcp.use_tool("x402__server_mock_start", { port: 3402 + i })
   );
   await Promise.all(promises);
   // Verify: No errors, <200ms latency
   ```

2. **Security Testing:**
   - Injection test: Malicious parameters
   - DoS test: 1000 concurrent requests
   - Path traversal test: `../../etc/passwd`
   - Command injection test: `; rm -rf /`

3. **Performance Benchmarking:**
   - Measure latency for each tool
   - Measure subprocess overhead
   - Measure memory usage (24h run)
   - Identify bottlenecks

4. **Bug Fixes:**
   - Fix any issues found in testing
   - Optimize slow operations
   - Improve error messages based on testing

**Deliverables:**
- ✅ Load tests passing
- ✅ Security tests passing
- ✅ Performance benchmarks documented
- ✅ Critical bugs fixed

**Time Estimate:** 12 hours

#### Day 18: NPM Packaging & CI/CD

**Tasks:**
1. **NPM Package Configuration:**
   ```json
   {
     "name": "@x402-dev/mcp-server",
     "version": "0.1.0",
     "description": "MCP server for x402-dev payment protocol testing",
     "main": "dist/index.js",
     "bin": {
       "x402-mcp-server": "dist/index.js"
     },
     "keywords": ["mcp", "x402", "payment", "testing", "solana"],
     "repository": "https://github.com/x402-dev/mcp-server"
   }
   ```

2. **Build Process:**
   - Configure TypeScript build (`tsc`)
   - Add build script to package.json
   - Test build output

3. **CI/CD Setup (GitHub Actions):**
   ```yaml
   name: CI
   on: [push, pull_request]
   jobs:
     test:
       runs-on: ${{ matrix.os }}
       strategy:
         matrix:
           os: [ubuntu-latest, macos-latest]
           node: [18, 20]
       steps:
         - uses: actions/checkout@v3
         - uses: actions/setup-node@v3
         - run: npm ci
         - run: npm test
         - run: npm run build
   ```

4. **Pre-Release Checklist:**
   - [ ] All tests passing
   - [ ] Documentation complete
   - [ ] CHANGELOG.md written
   - [ ] Version bumped (0.1.0)
   - [ ] Git tags created

**Deliverables:**
- ✅ NPM package ready
- ✅ CI/CD pipeline working
- ✅ Pre-release checklist complete

**Time Estimate:** 8 hours

#### Day 19: GitHub Repository & Community Resources

**Tasks:**
1. **GitHub Repository Setup:**
   - Create public repo: `x402-dev/mcp-server`
   - Add LICENSE (MIT)
   - Add CODE_OF_CONDUCT.md
   - Add SECURITY.md
   - Configure issue templates
   - Add GitHub Actions workflows

2. **MCP Directory Submission:**
   - Submit to https://mcpcat.io
   - Provide description, keywords, examples
   - Link to documentation

3. **Community Resources:**
   - Write blog post announcement
   - Create demo video (5 minutes):
     - Installation
     - First tool call
     - Example workflow
   - Prepare Solana Hackathon showcase materials

**Deliverables:**
- ✅ GitHub repo live
- ✅ MCP directory submission
- ✅ Blog post drafted
- ✅ Demo video recorded

**Time Estimate:** 8 hours

#### Day 20: Public Release & Announcement

**Tasks:**
1. **Publish to NPM:**
   ```bash
   npm publish --access public
   ```

2. **Announcements:**
   - GitHub Discussions post
   - x402-dev Discord/Slack
   - Solana Hackathon channels
   - Reddit (r/rust, r/solana)
   - Twitter/X announcement

3. **Post-Release Monitoring:**
   - Monitor NPM downloads
   - Watch GitHub issues
   - Respond to community questions
   - Track error reports (if telemetry added)

4. **Gather Feedback:**
   - Create feedback form
   - Monitor social media mentions
   - Review first user experiences

**Deliverables:**
- ✅ Package published
- ✅ Announcements made
- ✅ Monitoring active
- ✅ Feedback collection started

**Time Estimate:** 4 hours

**Phase 4 Total:** 32 hours (~4 days)

---

### 4.5 Roadmap Summary

| Phase | Duration | Key Deliverables | Success Criteria |
|-------|----------|------------------|------------------|
| **1: Foundation** | Week 1 (28h) | MCP server + 1 tool | stdio working, tests 50%+ |
| **2: Core Tools** | Week 2 (32h) | 5 tools functional | Integration tests passing, 60%+ coverage |
| **3: Polish** | Week 3 (36h) | All 7 tools + docs | 80%+ coverage, complete docs |
| **4: Production** | Week 4 (32h) | NPM package + release | Published, community resources live |
| **TOTAL** | **128 hours** | **Production-ready MCP server** | **All KPIs met** |

**Critical Path:**
1. stdio transport (can't proceed without this)
2. First tool working (validates approach)
3. Error handling (required for UX)
4. All 7 tools (MVP scope)
5. Testing (quality gate)
6. Publication (go-live)

**Risk Buffer:**
- Each phase has 10-20% buffer built in
- Week 4 has lightest load (catch-up time)
- Can descope Workflow 3 or advanced testing if needed

---

## 5. API Specification

### 5.1 Tool Naming Convention

**Pattern:** `x402__<domain>__<action>`

**Examples:**
- `x402__server_mock_start` (domain: server, action: mock_start)
- `x402__testing_run_suite` (domain: testing, action: run_suite)
- `x402__policy_validate` (domain: policy, action: validate)

**Rationale:**
- Clear namespace (x402__)
- Domain grouping (easier discovery)
- Action-oriented (verb + noun)

### 5.2 Common Types

```typescript
// Base response type
interface ToolResponse {
  content: Array<{
    type: "text" | "image" | "resource";
    text?: string;
    data?: string;
    uri?: string;
  }>;
  isError?: boolean;
}

// Success response helper
function successResponse(data: any): ToolResponse {
  return {
    content: [{
      type: "text",
      text: JSON.stringify(data, null, 2)
    }]
  };
}

// Error response helper
function errorResponse(error: McpError): ToolResponse {
  return {
    content: [{
      type: "text",
      text: JSON.stringify({ error }, null, 2)
    }],
    isError: true
  };
}

// Error type
interface McpError {
  code: string;           // E3001, E4002, etc.
  message: string;        // Human-readable description
  suggestion?: string;    // How to fix
  docs_link?: string;     // Link to documentation
  context?: {             // Additional context
    command?: string;
    args?: string[];
    exit_code?: number;
    [key: string]: any;
  };
}
```

---

### 5.3 Tool 1: `x402__server_mock_start`

**Description:** Start x402-dev mock payment server for testing payment-protected APIs without blockchain dependencies.

**Parameters:**
```typescript
interface MockStartParams {
  port?: number;              // Server port (default: 3402, range: 1024-65535)
  pricing?: number;           // Default price per request (default: 0.01, min: 0)
  simulation_mode?: "success" | "failure" | "timeout";  // Payment simulation (default: "success")
}

// Zod schema
const mockStartSchema = z.object({
  port: z.number().int().min(1024).max(65535).default(3402),
  pricing: z.number().min(0).default(0.01),
  simulation_mode: z.enum(["success", "failure", "timeout"]).default("success")
});
```

**Response (Success):**
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

**Response (Error):**
```json
{
  "error": {
    "code": "E3001",
    "message": "Port 3402 is already in use",
    "suggestion": "Stop the existing server with x402__server_mock_stop or use a different port",
    "context": {
      "port": 3402,
      "existing_pid": 12345
    }
  }
}
```

**Error Codes:**
- **E3001:** Port already in use
- **E3002:** Invalid port number (must be 1024-65535)
- **E3003:** x402-dev CLI not found in PATH
- **E3004:** Failed to start server (check logs)

**Usage Example:**
```typescript
// AI agent workflow
const result = await mcp.use_tool("x402__server_mock_start", {
  port: 3402,
  pricing: 0.02,
  simulation_mode: "success"
});

if (!result.error) {
  console.log(`Server started at ${result.server_url}`);
}
```

**CLI Mapping:**
```bash
# This tool executes:
x402-dev mock --port 3402 --pricing 0.02
# (Background process via subprocess spawn)
```

---

### 5.4 Tool 2: `x402__server_mock_stop`

**Description:** Stop the running x402-dev mock payment server gracefully.

**Parameters:** None (finds running server via PID file)

**Response (Success):**
```json
{
  "status": "stopped",
  "pid": 12345,
  "stopped_at": "2025-11-12T13:45:30.456Z"
}
```

**Response (Error):**
```json
{
  "error": {
    "code": "E3004",
    "message": "No mock server is currently running",
    "suggestion": "Start a server first with x402__server_mock_start"
  }
}
```

**Error Codes:**
- **E3004:** Server not running
- **E3005:** Failed to stop server (may need manual kill)

**CLI Mapping:**
```bash
# If x402-dev CLI has 'mock stop':
x402-dev mock stop

# Otherwise, manual kill:
kill -TERM $(cat ~/.x402dev/mock-server.pid)
```

---

### 5.5 Tool 3: `x402__server_mock_status`

**Description:** Check the status of the x402-dev mock payment server.

**Parameters:** None

**Response (Running):**
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

**Response (Not Running):**
```json
{
  "status": "not_running",
  "message": "No mock server is currently running"
}
```

**CLI Mapping:**
```bash
# Check PID file and process status
if [ -f ~/.x402dev/mock-server.pid ]; then
  kill -0 $(cat ~/.x402dev/mock-server.pid) && echo "running"
fi
```

---

### 5.6 Tool 4: `x402__testing_run_suite`

**Description:** Execute a YAML test suite to validate payment-protected endpoints.

**Parameters:**
```typescript
interface RunSuiteParams {
  suite_yaml: string;          // YAML test suite content (inline, not file path)
  output_format?: "json" | "junit" | "human";  // Default: "json"
  quiet?: boolean;             // Suppress verbose output (default: false)
}

const runSuiteSchema = z.object({
  suite_yaml: z.string().min(1),
  output_format: z.enum(["json", "junit", "human"]).default("json"),
  quiet: z.boolean().default(false)
});
```

**Response (Success):**
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

**Error Codes:**
- **E4001:** Invalid test suite YAML (syntax error)
- **E4002:** Test execution failed (endpoint unreachable)
- **E4003:** Test suite validation failed (missing required fields)

**Usage Example:**
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

**CLI Mapping:**
```bash
# Write YAML to temp file, execute, parse output
echo "$suite_yaml" > /tmp/test-suite-${random}.yaml
x402-dev test /tmp/test-suite-${random}.yaml --format json
```

---

### 5.7 Tool 5: `x402__testing_check_compliance`

**Description:** Validate that an HTTP endpoint returns proper 402 Payment Required responses according to x402 protocol specification.

**Parameters:**
```typescript
interface CheckComplianceParams {
  url: string;                 // Endpoint URL to check
  expected_recipient?: string; // Optional: verify specific recipient address
  expected_amount?: number;    // Optional: verify specific amount
  timeout_seconds?: number;    // Request timeout (default: 10)
}

const checkComplianceSchema = z.object({
  url: z.string().url(),
  expected_recipient: z.string().optional(),
  expected_amount: z.number().positive().optional(),
  timeout_seconds: z.number().int().positive().default(10)
});
```

**Response (Compliant):**
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

**Response (Non-Compliant):**
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

**Error Codes:**
- **E4003:** Endpoint unreachable (network error, timeout)
- **E4004:** Invalid HTTP response (not 402)
- **E4005:** Malformed WWW-Authenticate header

**CLI Mapping:**
```bash
x402-dev check <url> --format json
```

---

### 5.8 Tool 6: `x402__policy_validate`

**Description:** Validate a payment policy YAML file for syntax errors and logical conflicts.

**Parameters:**
```typescript
interface PolicyValidateParams {
  policy_yaml: string;  // YAML policy content (inline)
}

const policyValidateSchema = z.object({
  policy_yaml: z.string().min(1)
});
```

**Response (Valid):**
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

**Response (Invalid):**
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

**Error Codes:**
- **E5001:** Invalid policy YAML (syntax error)
- **E5002:** Validation errors found (logical conflicts)
- **E5003:** Missing required fields

**CLI Mapping:**
```bash
echo "$policy_yaml" > /tmp/policy-${random}.yaml
x402-dev policy validate /tmp/policy-${random}.yaml --format json
```

---

### 5.9 Tool 7: `x402__policy_generate_express`

**Description:** Generate Express.js middleware code from a payment policy YAML specification.

**Parameters:**
```typescript
interface GenerateExpressParams {
  policy_yaml: string;
  filename?: string;  // Source filename for comments (default: "policy.yaml")
}

const generateExpressSchema = z.object({
  policy_yaml: z.string().min(1),
  filename: z.string().default("policy.yaml")
});
```

**Response (Success):**
```json
{
  "generated_code": "const express = require('express');\n\n// Generated from policy.yaml\nmodule.exports = function x402Middleware(options) {\n  return (req, res, next) => {\n    // Rate limiting logic...\n  };\n};\n",
  "lines": 150,
  "size_bytes": 4523,
  "framework": "express"
}
```

**Response (Error):**
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

**Error Codes:**
- **E5001:** Invalid policy YAML
- **E5002:** Code generation failed (validation errors)
- **E5003:** Unsupported policy features

**Usage Example:**
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
  console.log(`Generated ${result.lines} lines of middleware`);
}
```

**CLI Mapping:**
```bash
echo "$policy_yaml" > /tmp/policy-${random}.yaml
x402-dev policy generate /tmp/policy-${random}.yaml --framework express
```

---

### 5.10 Error Code Reference

| Code Range | Category | Examples |
|------------|----------|----------|
| **E3xxx** | Mock Server | E3001 (port in use), E3002 (invalid port), E3003 (CLI not found) |
| **E4xxx** | Testing | E4001 (invalid YAML), E4002 (execution failed), E4003 (unreachable) |
| **E5xxx** | Policy | E5001 (invalid YAML), E5002 (validation errors), E5003 (codegen failed) |
| **E9xxx** | General | E9001 (timeout), E9002 (permission denied), E9003 (unknown error) |

**Error Response Format (Standard):**
```typescript
interface McpError {
  code: string;           // E3001, E4002, etc.
  message: string;        // Human-readable description
  suggestion?: string;    // How to fix (actionable)
  docs_link?: string;     // Link to documentation
  context?: {             // Additional context
    command?: string;     // CLI command that failed
    args?: string[];      // Arguments passed
    exit_code?: number;   // Process exit code
    stderr?: string;      // Error output (truncated)
    [key: string]: any;   // Tool-specific context
  };
}
```

---

## 6. Risk Assessment

### 6.1 Risk Matrix

```
Impact
HIGH  │  [R1] [R3]           │
      │                      │
MED   │  [R2]    [R5]        │
      │                      │
LOW   │       [R4]     [R6]  │
      └──────────────────────┘
        LOW    MED     HIGH
           Likelihood
```

**Risk Scoring:**
- **Likelihood:** Low (1), Medium (2), High (3)
- **Impact:** Low (1), Medium (2), High (3)
- **Risk Score:** Likelihood × Impact (1-9)

---

### 6.2 Top Risks

#### Risk 1: CLI Breaking Changes
**ID:** R1
**Category:** Technical
**Likelihood:** Medium (2)
**Impact:** High (3)
**Risk Score:** 6 (HIGH)

**Description:**
x402-dev CLI introduces breaking changes (command renamed, output format changed, parameters removed) that break MCP server tool implementations.

**Scenarios:**
1. `x402-dev mock` renamed to `x402-dev serve`
2. JSON output format changes structure
3. Exit codes change meaning
4. New required parameters added

**Consequences:**
- Tools fail with cryptic errors
- Users can't use MCP server with latest CLI
- Emergency fix required (hotfix release)
- Reputation damage ("unreliable integration")

**Mitigation Strategies:**

**1. Version Pinning + Compatibility Range:**
```json
// package.json
{
  "peerDependencies": {
    "x402-dev": ">=0.1.0 <0.3.0"
  }
}
```

**2. CLI Version Detection:**
```typescript
async function checkCliVersion(): Promise<void> {
  const version = await execX402Dev('--version');
  if (!semver.satisfies(version, '>=0.1.0 <0.3.0')) {
    throw new Error(`Unsupported x402-dev version ${version}. Required: 0.1.x - 0.2.x`);
  }
}
```

**3. Output Format Validation:**
```typescript
// Validate CLI output structure
const schema = z.object({
  status: z.string(),
  port: z.number(),
  // ... expected fields
});

try {
  schema.parse(cliOutput);
} catch (err) {
  throw new Error('CLI output format changed. Please update MCP server.');
}
```

**4. Deprecation Warnings:**
```typescript
// Detect deprecated CLI patterns
if (stderr.includes('DEPRECATED')) {
  console.warn('⚠️  x402-dev CLI using deprecated features. Update recommended.');
}
```

**Contingency Plan:**
- Monitor x402-dev releases (GitHub watch)
- Test with pre-release versions
- Publish hotfix within 24h of breaking change
- Communicate issue in release notes

**Risk Owner:** MCP Server maintainer

---

#### Risk 2: Subprocess Overhead Too High
**ID:** R2
**Category:** Technical
**Likelihood:** Low (1)
**Impact:** Medium (2)
**Risk Score:** 2 (LOW)

**Description:**
Spawning x402-dev CLI process for each tool call adds 50-200ms latency. For complex workflows (10+ tool calls), this adds 1-2 seconds, making AI agent experience feel sluggish.

**Scenarios:**
1. Subprocess spawn takes 100ms on slow systems
2. CLI startup time increases (new dependencies)
3. AI agent calls 20+ tools in one workflow

**Consequences:**
- Users perceive MCP server as "slow"
- AI agents timeout on long workflows
- Negative reviews ("subprocess is bottleneck")

**Mitigation Strategies:**

**1. Early Benchmarking (Week 1):**
```typescript
// Measure subprocess overhead
console.time('subprocess');
await execX402Dev('mock', ['--version']);
console.timeEnd('subprocess');
// Target: <50ms on average system
```

**2. Process Pooling (if needed):**
```typescript
// Keep x402-dev processes warm
const processPool = new ProcessPool('x402-dev', { size: 3 });
const result = await processPool.exec(['mock', '--port', '3402']);
```

**3. Fallback to Library Mode:**
If subprocess overhead is unacceptable (>200ms p95), pivot to Rust library integration (already analyzed in TECHNICAL-ARCHITECTURE.md).

**4. Caching for Idempotent Operations:**
```typescript
// Cache tool results for 5 seconds
const cache = new TTLCache<string, ToolResponse>({ ttl: 5000 });
const cacheKey = `${toolName}:${JSON.stringify(params)}`;
if (cache.has(cacheKey)) return cache.get(cacheKey);
```

**Acceptance Criteria:**
- P95 latency <200ms (Phase 1 benchmark)
- If >200ms, implement mitigation by Phase 2

**Risk Owner:** MCP Server developer

---

#### Risk 3: MCP Protocol Changes
**ID:** R3
**Category:** Technical
**Likelihood:** Low (1)
**Impact:** High (3)
**Risk Score:** 3 (MEDIUM)

**Description:**
Model Context Protocol introduces breaking changes (v2025-12-01) that require significant MCP server refactoring.

**Scenarios:**
1. stdio transport deprecated
2. Tool schema format changes
3. New required capabilities

**Consequences:**
- MCP server stops working with Claude Code
- Emergency refactor required
- Users stuck on old protocol version

**Mitigation Strategies:**

**1. Protocol Version Pinning:**
```typescript
import { Server } from '@modelcontextprotocol/sdk/server/index.js';

const server = new Server({
  name: 'x402-dev-mcp',
  version: '0.1.0',
  protocolVersion: '2025-06-18'  // Pin to stable version
});
```

**2. Monitor MCP Releases:**
- Subscribe to MCP GitHub releases
- Join MCP community Discord
- Test with pre-release versions

**3. Compatibility Layer:**
If protocol changes, add compatibility layer:
```typescript
function adaptToNewProtocol(oldRequest: OldRequest): NewRequest {
  // Transform old format to new format
}
```

**Contingency Plan:**
- Protocol changes announced 3+ months in advance (MCP policy)
- Update within 1 month of new protocol release
- Maintain backwards compatibility for 1 version

**Risk Owner:** MCP Server maintainer

---

#### Risk 4: Low AI Agent Adoption
**ID:** R4
**Category:** Market
**Likelihood:** Medium (2)
**Impact:** Low (1)
**Risk Score:** 2 (LOW)

**Description:**
Developers don't adopt MCP server; prefer manual CLI usage. Result: Low NPM downloads, no community engagement.

**Scenarios:**
1. Documentation unclear (users don't understand value)
2. Installation too complex
3. AI coding tools not widely adopted yet

**Consequences:**
- Wasted development effort (3-4 weeks)
- No community contributions
- Project abandoned

**Mitigation Strategies:**

**1. Excellent Documentation:**
- Clear value proposition ("90 seconds to testing")
- 3-step installation guide
- Video demos (seeing is believing)
- Example workflows (copy-paste ready)

**2. Community Engagement:**
- Announce in Solana Hackathon channels
- Post to Reddit (r/rust, r/solana)
- Tweet with demo video
- Engage with early adopters

**3. Lower Barrier to Entry:**
- NPM package (no compilation)
- One-command installation
- Works out of box (no configuration)

**4. Gather Feedback Early:**
- Alpha release to 5-10 testers
- Iterate based on feedback
- Fix pain points before public release

**Acceptance Criteria:**
- 50+ NPM downloads in week 1
- 3+ GitHub stars
- At least 1 community contribution

**Risk Owner:** Project lead

---

#### Risk 5: Competition from Alternatives
**ID:** R5
**Category:** Market
**Likelihood:** Medium (2)
**Impact:** Medium (2)
**Risk Score:** 4 (MEDIUM)

**Description:**
Another team releases similar MCP server for x402 protocol, or x402-dev team builds official MCP server, making this project redundant.

**Scenarios:**
1. Official x402-dev MCP server announced
2. Better implementation by experienced team
3. x402-dev integrates MCP natively (no separate server needed)

**Consequences:**
- Project becomes redundant
- Users migrate to alternative
- No return on development investment

**Mitigation Strategies:**

**1. First-Mover Advantage:**
- Ship v1.0 before alternatives emerge
- Build community early
- Establish brand (known as "the x402 MCP server")

**2. Quality > Speed:**
- Excellent documentation (better than alternatives)
- Robust error handling (better UX)
- Comprehensive testing (more reliable)

**3. Community Ownership:**
- Open source (MIT license)
- Accept contributions
- Build ecosystem (plugins, extensions)

**4. Official Collaboration:**
If x402-dev team wants official MCP server:
- Offer to donate codebase
- Collaborate on integration
- Transition to official maintainer

**Contingency Plan:**
- If official MCP server announced, evaluate integration
- If redundant, archive project gracefully
- Document lessons learned for community

**Risk Owner:** Project lead

---

#### Risk 6: x402-dev Project Sunset
**ID:** R6
**Category:** Market
**Likelihood:** Low (1)
**Impact:** Medium (2)
**Risk Score:** 2 (LOW)

**Description:**
x402-dev project is abandoned or deprecated, making MCP server useless.

**Scenarios:**
1. x402-dev maintainer stops development
2. x402 protocol loses relevance
3. Solana ecosystem shifts to different standard

**Consequences:**
- MCP server has no users
- Wasted development effort

**Mitigation Strategies:**

**1. Assess x402-dev Health:**
- Check recent commits (active development?)
- Review issue response time
- Check community size

**2. Contingency:**
If x402-dev shows signs of abandonment:
- Fork x402-dev (maintain separately)
- Pivot to generic payment protocol MCP server
- Archive MCP server gracefully

**3. Diversification:**
Future: Support multiple payment protocols
- x402-dev (Solana)
- Payment Protocol X (Ethereum)
- Generic HTTP 402

**Risk Owner:** Project lead

---

### 6.3 Risk Monitoring Plan

**Weekly Risk Review:**
- Review risk likelihood/impact
- Check mitigation effectiveness
- Identify new risks

**Trigger Points:**
- R1: x402-dev releases new version → Test compatibility immediately
- R2: Benchmark shows >200ms latency → Implement process pooling
- R3: MCP announces protocol change → Plan migration within 1 week
- R4: <20 downloads in week 1 → Increase marketing effort
- R5: Alternative MCP server announced → Evaluate differentiation
- R6: x402-dev no commits for 3 months → Consider fork

**Risk Owner Responsibilities:**
- Monitor assigned risks weekly
- Report issues immediately
- Execute mitigation plans
- Escalate if needed

---

## 7. Success Metrics & KPIs

### 7.1 Technical Success Criteria

**Must-Have (Block Release):**

1. **All 7 Tools Functional**
   - ✅ Each tool executes successfully with valid inputs
   - ✅ Each tool handles all error scenarios
   - ✅ Each tool returns proper response format

2. **Performance: <200ms Latency (P95)**
   - ✅ Average tool execution: <100ms
   - ✅ P95 latency: <200ms
   - ✅ P99 latency: <500ms
   - ✅ Load test (10 concurrent): All pass <200ms

3. **Test Coverage: 80%+**
   - ✅ Unit test coverage: 80%+
   - ✅ Integration test coverage: 60%+
   - ✅ All error paths tested
   - ✅ Edge cases covered

4. **Claude Code Integration Working**
   - ✅ Installation via `claude mcp add` succeeds
   - ✅ Tools discoverable in Claude Code
   - ✅ Tool execution successful
   - ✅ Error messages display correctly

5. **Security: Zero Critical Vulnerabilities**
   - ✅ npm audit shows 0 critical/high issues
   - ✅ Injection tests pass
   - ✅ DoS tests pass (no crash under load)
   - ✅ Security review complete

**Should-Have (v1.0 Quality Bar):**

6. **Documentation Complete**
   - ✅ README with installation guide
   - ✅ API reference for all 7 tools
   - ✅ Integration guide (Claude Code)
   - ✅ Troubleshooting guide (10+ common issues)
   - ✅ Example workflows (3 scenarios)

7. **CI/CD Pipeline Working**
   - ✅ GitHub Actions tests on push
   - ✅ Tests run on macOS + Linux
   - ✅ Build succeeds on Node 18 + 20
   - ✅ Automated NPM publish

---

### 7.2 User Success Criteria

**Onboarding Metrics:**

1. **Installation Time: <5 minutes**
   - Target: 3 minutes (p50), 5 minutes (p95)
   - Measured: Time from `npm install` to first tool call
   - Blockers tracked: Installation failures, missing dependencies

2. **First Tool Call Success: <5 minutes**
   - Target: 3 minutes (p50), 5 minutes (p95)
   - Measured: Time from installation to first successful tool result
   - Includes: Claude Code setup, first invocation

3. **Documentation Clarity**
   - ✅ 90%+ of users complete setup without support
   - ✅ Troubleshooting guide covers 80%+ of issues
   - ✅ <5 "how do I...?" issues per week

**Engagement Metrics:**

4. **NPM Downloads**
   - Week 1: 50+ downloads (target: 100)
   - Month 1: 200+ downloads (target: 500)
   - Month 3: 500+ downloads (target: 1000)

5. **GitHub Engagement**
   - Week 1: 3+ stars (target: 10)
   - Month 1: 10+ stars (target: 25)
   - Month 3: 25+ stars (target: 50)
   - Community contributions: 5+ (issues, PRs, discussions)

6. **Tool Usage**
   - Most used tool: `x402__server_mock_start` (expected)
   - Tool invocations: 1000+ per week (by week 4)
   - Average tools per workflow: 3-5

**Quality Metrics:**

7. **Error Rate: <10%**
   - Failed tool calls / total calls < 10%
   - Measured via telemetry (if opt-in added)
   - Target: <5% by Month 2

8. **User Satisfaction**
   - Positive feedback: >80% (GitHub reactions, reviews)
   - Net Promoter Score: >30 (if survey sent)
   - Repeat usage: 60%+ users invoke tools 5+ times

---

### 7.3 Ecosystem Success Criteria

**Community Growth:**

1. **MCP Directory Listing**
   - ✅ Listed at https://mcpcat.io within Week 4
   - ✅ Complete profile (description, keywords, examples)
   - ✅ Screenshot/demo video

2. **Solana Hackathon Showcase**
   - ✅ Featured in hackathon materials
   - ✅ Demo video available (5 minutes)
   - ✅ Hackathon participants aware (50+ views)

3. **AI Tool Integrations**
   - ✅ Claude Code (primary, Week 4)
   - Target: Cline (Month 2, community-led)
   - Target: Continue.dev (Month 3, community-led)
   - Target: Cursor (Month 3, community-led)

**Thought Leadership:**

4. **Content Published**
   - ✅ Blog post announcement (Week 4)
   - ✅ Demo video (Week 4, 5 minutes)
   - Target: Tutorial post (Month 1)
   - Target: Technical deep dive (Month 2)

5. **Social Media Reach**
   - Twitter/X: 100+ impressions (Week 4)
   - Reddit posts: 50+ upvotes total
   - Solana Discord: Active discussion thread

**Contributions:**

6. **Community Involvement**
   - Week 4: 1+ external contributor
   - Month 1: 3+ external contributors
   - Month 3: 5+ external contributors
   - Types: Bug reports, PRs, documentation, examples

7. **Ecosystem Tools**
   - Month 3: 1+ tool built on top (plugin, extension)
   - Month 6: Integration into another project
   - Month 6: Featured in x402-dev documentation

---

### 7.4 Measurement Plan

**Technical Metrics (Automated):**
```typescript
// Prometheus metrics (if added)
const metrics = {
  tool_invocations_total: Counter,
  tool_execution_duration: Histogram,
  tool_errors_total: Counter,
  active_sessions: Gauge
};

// CI/CD metrics
const ciMetrics = {
  test_coverage: parseFloat(coverage),  // From Jest
  build_time: buildDuration,            // From GitHub Actions
  test_pass_rate: passedTests / totalTests
};
```

**User Metrics (Manual + Automated):**
```bash
# NPM downloads
npm info @x402-dev/mcp-server downloads

# GitHub stats
gh api repos/x402-dev/mcp-server --jq '.stargazers_count'

# Issues/PRs
gh issue list --state all | wc -l
```

**Qualitative Feedback:**
- GitHub Discussions (monitor weekly)
- Reddit/Twitter mentions (Google Alerts)
- Direct user emails (support@x402-dev.com)
- Community survey (Month 1, optional)

**Reporting Cadence:**
- **Weekly:** Technical metrics (coverage, latency, errors)
- **Bi-weekly:** User metrics (downloads, stars, issues)
- **Monthly:** Ecosystem metrics (integrations, content, contributions)

---

### 7.5 Success Thresholds

**MVP Success (Week 4):**
- ✅ All 7 tools functional and tested (P0)
- ✅ <200ms P95 latency (P0)
- ✅ 80%+ test coverage (P0)
- ✅ NPM published (P0)
- ✅ Claude Code working (P0)

**v1.0 Success (Month 1):**
- ✅ 50+ NPM downloads (P1)
- ✅ 5+ GitHub stars (P1)
- ✅ MCP directory listed (P1)
- ✅ Complete documentation (P1)
- ✅ Zero critical bugs (P0)

**Long-Term Success (Month 3):**
- ✅ 200+ NPM downloads
- ✅ 10+ GitHub stars
- ✅ 5+ community contributions
- ✅ Integration with 2+ AI tools
- ✅ Featured in x402-dev docs

**Failure Criteria (Abort Signal):**
- <20 NPM downloads in Week 4 → Investigate why, pivot if needed
- >50% tool error rate → Critical quality issue, halt release
- Zero community engagement (Month 1) → Reevaluate value proposition

---

## 8. Next Steps

### 8.1 Immediate Actions (This Week)

1. **Review & Approve Epic**
   - [ ] Stakeholder review (x402-dev team)
   - [ ] Budget approval (if needed)
   - [ ] Timeline confirmation (3-4 weeks)

2. **Team Assignment**
   - [ ] Assign lead developer
   - [ ] Identify SMEs (x402-dev experts)
   - [ ] Set up communication channels

3. **Environment Setup**
   - [ ] Create GitHub repo (x402-dev/mcp-server)
   - [ ] Set up project board (GitHub Projects)
   - [ ] Configure CI/CD (GitHub Actions)

### 8.2 Phase 1 Kickoff (Week 1)

**Day 1 Tasks:**
1. Initialize TypeScript project
2. Install MCP SDK dependencies
3. Create project structure
4. First commit to `main` branch

**Week 1 Goal:**
- Working MCP server with 1 tool
- Tests passing
- Verified with Claude Code

### 8.3 Communication Plan

**Internal:**
- Daily standups (async in Slack/Discord)
- Weekly progress report (Friday EOD)
- Blocker escalation (immediate)

**External:**
- Week 2: Preview blog post (draft)
- Week 3: Alpha testing (invite 5-10 users)
- Week 4: Public announcement (blog + social media)

### 8.4 Go/No-Go Decision Points

**Phase 1 (Week 1):**
- ✅ 1 tool working end-to-end
- ✅ stdio transport stable
- ✅ Tests passing
- **GO:** Continue to Phase 2
- **NO-GO:** If fundamental blocker (e.g., MCP SDK incompatibility), reevaluate approach

**Phase 2 (Week 2):**
- ✅ 5 tools functional
- ✅ 60%+ test coverage
- ✅ No critical bugs
- **GO:** Continue to Phase 3
- **NO-GO:** If quality below threshold, extend timeline or descope

**Phase 3 (Week 3):**
- ✅ All 7 tools complete
- ✅ 80%+ test coverage
- ✅ Documentation complete
- **GO:** Proceed to Phase 4 (production)
- **NO-GO:** If critical gap, delay release by 1 week

**Phase 4 (Week 4):**
- ✅ Security review passed
- ✅ Performance benchmarks met
- ✅ NPM package ready
- **GO:** Publish to NPM, announce publicly
- **NO-GO:** If critical issue found, delay until fixed

---

## Appendices

### A. Glossary

| Term | Definition |
|------|------------|
| **MCP** | Model Context Protocol - Open protocol for AI agent tool integration |
| **stdio** | Standard input/output - Transport mechanism using stdin/stdout |
| **Tool** | Executable function exposed via MCP for AI agents to invoke |
| **Workflow** | Multi-step process composed of multiple tool invocations |
| **x402 Protocol** | HTTP 402 Payment Required protocol for Solana payments |
| **AI Agent** | Software that can autonomously perform tasks (e.g., Claude Code) |
| **Subprocess** | Child process spawned to execute CLI commands |

### B. References

1. **MCP Specification:** https://modelcontextprotocol.io/specification/2025-06-18
2. **MCP SDK (TypeScript):** https://github.com/modelcontextprotocol/typescript-sdk
3. **x402-dev Documentation:** https://github.com/x402-dev/x402-dev
4. **Claude Code MCP Guide:** https://docs.anthropic.com/en/docs/claude-code/mcp
5. **Solana Hackathon 2025:** [TBD]

### C. Related Epics

- **Epic 1:** Foundation & CLI Infrastructure (Complete)
- **Epic 2:** Mock Facilitator Server (Complete)
- **Epic 3:** Automated Test Suite (Complete)
- **Epic 4:** Validation Tools (Complete)
- **Epic 5:** Policy Engine (Complete)
- **Epic 6:** Developer Experience (Complete)
- **Epic 7:** Documentation (Complete)
- **Epic 8:** MCP Server Integration **(THIS EPIC)**

### D. Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1 | 2025-11-12 | x402-dev Team | Initial draft |

---

**END OF EPIC SPECIFICATION**

**Total Word Count:** ~15,000 words
**Total Pages:** ~30 pages
**Review Status:** Draft
**Next Review:** [Date TBD]

For questions or feedback, please open a GitHub Discussion or contact the x402-dev team.
