# Epic 8: MCP Server Implementation Guide

**Target:** Developers building the x402-mcp-server
**Duration:** 3-4 weeks
**Prerequisites:** TypeScript, Node.js 18+, x402-dev CLI installed

---

## Quick Start

### Prerequisites Checklist

- [ ] Node.js 18+ installed (`node --version`)
- [ ] x402-dev CLI installed (`x402-dev --version`)
- [ ] Git configured
- [ ] TypeScript knowledge
- [ ] MCP concepts understood

### Initial Setup (15 minutes)

```bash
# 1. Create project
mkdir x402-mcp-server && cd x402-mcp-server
npm init -y

# 2. Install dependencies
npm install @modelcontextprotocol/sdk zod typescript
npm install -D jest @types/jest @types/node ts-jest

# 3. Initialize TypeScript
npx tsc --init --strict --target ES2022 --module ESNext --moduleResolution bundler

# 4. Create project structure
mkdir -p src/{tools,utils} tests/{unit,integration}
```

---

## Phase 1: Foundation (Week 1)

### Day 1-2: Project Setup & MCP SDK

**Goal:** Working TypeScript project with MCP SDK

#### Step 1.1: Configure TypeScript

`tsconfig.json`:
```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist", "tests"]
}
```

#### Step 1.2: Create Project Structure

```
x402-mcp-server/
├── src/
│   ├── index.ts           # Entry point
│   ├── server.ts          # MCP protocol handler
│   ├── tools/
│   │   ├── index.ts       # Tool registry
│   │   ├── mock.ts        # Mock server tools
│   │   ├── testing.ts     # Testing tools
│   │   └── policy.ts      # Policy tools
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

#### Step 1.3: Configure Jest

`jest.config.js`:
```javascript
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  testMatch: ['**/tests/**/*.test.ts'],
  collectCoverageFrom: ['src/**/*.ts'],
  coverageThreshold: {
    global: {
      branches: 80,
      functions: 80,
      lines: 80,
      statements: 80
    }
  }
};
```

**Validation:**
- ✅ `npm run build` succeeds
- ✅ `npm test` runs (no tests yet)
- ✅ TypeScript strict mode enabled

---

### Day 3-4: stdio Transport & Subprocess Executor

**Goal:** MCP server accepts connections, can execute CLI commands

#### Step 2.1: Implement stdio Transport

`src/index.ts`:
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

console.error('x402-mcp-server started');  // Use stderr for logging!
```

**Important:** All logging MUST go to stderr, not stdout (stdout is for JSON-RPC messages).

#### Step 2.2: Create Subprocess Executor

`src/utils/subprocess.ts`:
```typescript
import { spawn } from 'child_process';

export interface ExecResult {
  stdout: string;
  stderr: string;
  exitCode: number;
}

export interface ExecOptions {
  timeout?: number;        // Default: 120000ms (2 minutes)
  background?: boolean;    // Run as background process
  cwd?: string;
}

export async function execX402Dev(
  command: string,
  args: string[],
  options: ExecOptions = {}
): Promise<ExecResult> {
  const { timeout = 120000, background = false, cwd } = options;

  return new Promise((resolve, reject) => {
    const proc = spawn('x402-dev', [command, ...args], {
      stdio: background ? 'ignore' : ['ignore', 'pipe', 'pipe'],
      cwd,
    });

    let stdout = '';
    let stderr = '';

    if (!background) {
      proc.stdout?.on('data', (data) => { stdout += data.toString(); });
      proc.stderr?.on('data', (data) => { stderr += data.toString(); });
    }

    const timeoutId = setTimeout(() => {
      proc.kill('SIGTERM');
      reject(new Error(`Command timeout after ${timeout}ms`));
    }, timeout);

    proc.on('close', (exitCode) => {
      clearTimeout(timeoutId);
      resolve({ stdout, stderr, exitCode: exitCode ?? 1 });
    });

    proc.on('error', (err) => {
      clearTimeout(timeoutId);
      reject(new Error(`Failed to execute x402-dev: ${err.message}`));
    });
  });
}
```

#### Step 2.3: Create Error Translator

`src/utils/errors.ts`:
```typescript
import { z } from 'zod';

export interface McpError {
  code: string;
  message: string;
  suggestion?: string;
  docs_link?: string;
  context?: Record<string, any>;
}

export function translateCliError(
  stderr: string,
  exitCode: number,
  command: string,
  args: string[]
): McpError {
  // Parse CLI errors and map to MCP error codes
  if (stderr.includes('port already in use') || exitCode === 2) {
    return {
      code: 'E3001',
      message: 'Port already in use',
      suggestion: 'Stop existing server or use different port',
      context: { command, args, exit_code: exitCode }
    };
  }

  // Add more error mappings...

  return {
    code: 'E9003',
    message: `Command failed: ${stderr || 'Unknown error'}`,
    suggestion: 'Check x402-dev CLI logs for details',
    context: { command, args, exit_code: exitCode, stderr }
  };
}
```

**Validation:**
- ✅ stdio transport responds to MCP handshake
- ✅ Subprocess executor can call `x402-dev --version`
- ✅ Error translator handles common errors

---

### Day 5: First Tool (Proof of Concept)

**Goal:** One working tool end-to-end

#### Step 3.1: Implement `x402__server_mock_start`

`src/tools/mock.ts`:
```typescript
import { z } from 'zod';
import { execX402Dev, ExecResult } from '../utils/subprocess.js';
import { translateCliError, McpError } from '../utils/errors.js';

// Parameter schema
export const mockStartSchema = z.object({
  port: z.number().int().min(1024).max(65535).default(3402),
  pricing: z.number().min(0).default(0.01),
  simulation_mode: z.enum(['success', 'failure', 'timeout']).default('success')
});

export type MockStartParams = z.infer<typeof mockStartSchema>;

// Tool implementation
export async function serverMockStart(params: MockStartParams): Promise<ToolResponse> {
  // 1. Validate parameters
  const validated = mockStartSchema.parse(params);

  // 2. Check if already running (read PID file)
  const existingPid = await readPidFile();
  if (existingPid && isProcessRunning(existingPid)) {
    return errorResponse({
      code: 'E3001',
      message: 'Mock server already running',
      suggestion: 'Stop existing server with x402__server_mock_stop',
      context: { pid: existingPid }
    });
  }

  // 3. Build CLI command
  const args = [
    '--port', validated.port.toString(),
    '--pricing', validated.pricing.toString()
  ];

  // 4. Execute subprocess (background mode)
  try {
    const result = await execX402Dev('mock', args, { background: true });

    // 5. Return success response
    return successResponse({
      status: 'started',
      pid: result.pid,
      port: validated.port,
      server_url: `http://localhost:${validated.port}`,
      config: {
        pricing: validated.pricing,
        simulation_mode: validated.simulation_mode
      },
      started_at: new Date().toISOString()
    });
  } catch (err) {
    return errorResponse(translateCliError(err.stderr, err.exitCode, 'mock', args));
  }
}

// Helper functions
async function readPidFile(): Promise<number | null> {
  // Implementation...
}

function isProcessRunning(pid: number): boolean {
  try {
    process.kill(pid, 0);  // Signal 0 checks if process exists
    return true;
  } catch {
    return false;
  }
}
```

#### Step 3.2: Register Tool in Server

`src/server.ts`:
```typescript
import { CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js';
import { serverMockStart, mockStartSchema } from './tools/mock.js';

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;

  switch (name) {
    case 'x402__server_mock_start':
      return await serverMockStart(args);

    // More tools...

    default:
      throw new Error(`Unknown tool: ${name}`);
  }
});
```

#### Step 3.3: Test with Claude Code

```bash
# 1. Build project
npm run build

# 2. Add to Claude Code
claude mcp add x402-dev-mcp node /path/to/x402-mcp-server/dist/index.js

# 3. Test in Claude Code
# Ask Claude: "Start an x402 mock server on port 3402"
```

**Validation:**
- ✅ Tool callable from Claude Code
- ✅ Server starts successfully
- ✅ PID file created
- ✅ Error handling works (port in use)

---

## Phase 2: Core Tools (Week 2)

### Day 6-7: Complete Mock Server Tools

#### Implement `x402__server_mock_stop`

```typescript
export async function serverMockStop(): Promise<ToolResponse> {
  const pid = await readPidFile();

  if (!pid || !isProcessRunning(pid)) {
    return errorResponse({
      code: 'E3004',
      message: 'No mock server is currently running',
      suggestion: 'Start server first with x402__server_mock_start'
    });
  }

  try {
    process.kill(pid, 'SIGTERM');
    await waitForProcessExit(pid, 5000);  // Wait up to 5s

    return successResponse({
      status: 'stopped',
      pid,
      stopped_at: new Date().toISOString()
    });
  } catch (err) {
    return errorResponse({
      code: 'E3005',
      message: 'Failed to stop server',
      suggestion: 'May need manual kill: kill -9 ' + pid
    });
  }
}
```

#### Implement `x402__server_mock_status`

```typescript
export async function serverMockStatus(): Promise<ToolResponse> {
  const pid = await readPidFile();

  if (!pid || !isProcessRunning(pid)) {
    return successResponse({
      status: 'not_running',
      message: 'No mock server is currently running'
    });
  }

  const config = await readServerConfig();

  return successResponse({
    status: 'running',
    pid,
    port: config.port,
    server_url: `http://localhost:${config.port}`,
    uptime_seconds: calculateUptime(config.started_at),
    started_at: config.started_at,
    config: {
      pricing: config.pricing,
      simulation_mode: config.simulation_mode
    }
  });
}
```

**Validation:**
- ✅ Can start, stop, check status
- ✅ Integration test (start → status → stop)
- ✅ Error cases handled

---

### Day 8-9: Testing Tools

#### Implement `x402__testing_run_suite`

```typescript
export const runSuiteSchema = z.object({
  suite_yaml: z.string().min(1),
  output_format: z.enum(['json', 'junit', 'human']).default('json'),
  quiet: z.boolean().default(false)
});

export async function testingRunSuite(params: z.infer<typeof runSuiteSchema>): Promise<ToolResponse> {
  const validated = runSuiteSchema.parse(params);

  // Write YAML to temp file
  const tempFile = await writeTempFile(validated.suite_yaml, '.yaml');

  try {
    const result = await execX402Dev('test', [
      tempFile,
      '--format', validated.output_format
    ]);

    if (result.exitCode !== 0) {
      // Parse test failures (not an error, just failed tests)
      const testResult = JSON.parse(result.stdout);
      return successResponse(testResult);
    }

    const testResult = JSON.parse(result.stdout);
    return successResponse(testResult);
  } catch (err) {
    if (err.message.includes('Invalid YAML')) {
      return errorResponse({
        code: 'E4001',
        message: 'Invalid test suite YAML',
        suggestion: 'Check YAML syntax and structure',
        context: { error: err.message }
      });
    }
    throw err;
  } finally {
    await fs.unlink(tempFile);  // Cleanup
  }
}
```

#### Implement `x402__testing_check_compliance`

```typescript
export const checkComplianceSchema = z.object({
  url: z.string().url(),
  expected_recipient: z.string().optional(),
  expected_amount: z.number().positive().optional(),
  timeout_seconds: z.number().int().positive().default(10)
});

export async function testingCheckCompliance(params: z.infer<typeof checkComplianceSchema>): Promise<ToolResponse> {
  const validated = checkComplianceSchema.parse(params);

  const args = [validated.url, '--format', 'json'];
  const result = await execX402Dev('check', args, {
    timeout: validated.timeout_seconds * 1000
  });

  const complianceResult = JSON.parse(result.stdout);
  return successResponse(complianceResult);
}
```

**Validation:**
- ✅ Test suite execution works
- ✅ Compliance checking works
- ✅ YAML parsing robust
- ✅ Temp file cleanup

---

### Day 10: Error Handling & Integration Tests

#### Enhance Error Translation

```typescript
const ERROR_MAPPINGS = {
  'port already in use': { code: 'E3001', suggestion: 'Stop existing server or use different port' },
  'invalid port': { code: 'E3002', suggestion: 'Use port between 1024-65535' },
  'command not found': { code: 'E3003', suggestion: 'Install: cargo install x402-dev' },
  'invalid yaml': { code: 'E4001', suggestion: 'Check YAML syntax' },
  'endpoint unreachable': { code: 'E4003', suggestion: 'Verify URL and network' },
  // ...more mappings
};
```

#### Create Integration Tests

`tests/integration/workflow.test.ts`:
```typescript
describe('Complete Workflow', () => {
  it('should start server, run tests, stop server', async () => {
    // 1. Start server
    const startResult = await mcp.useTool('x402__server_mock_start', { port: 3402 });
    expect(startResult.status).toBe('started');

    // 2. Check status
    const statusResult = await mcp.useTool('x402__server_mock_status', {});
    expect(statusResult.status).toBe('running');

    // 3. Check compliance
    const checkResult = await mcp.useTool('x402__testing_check_compliance', {
      url: 'http://localhost:3402/api'
    });
    expect(checkResult.compliant).toBe(true);

    // 4. Stop server
    const stopResult = await mcp.useTool('x402__server_mock_stop', {});
    expect(stopResult.status).toBe('stopped');
  });
});
```

**Validation:**
- ✅ 60%+ test coverage
- ✅ All error paths tested
- ✅ Integration tests pass

---

## Phase 3: Polish (Week 3)

### Day 11-12: Policy Tools

(Similar implementation pattern as above for policy validation and code generation)

### Day 13: Error Enhancement

**Add Structured Error Codes:**
```typescript
export const ERROR_CODES = {
  E3001: { message: 'Port already in use', suggestion: 'Stop existing server or use different port' },
  E3002: { message: 'Invalid port number', suggestion: 'Use port between 1024-65535' },
  // ...all error codes
};
```

### Day 14: Documentation

**Create README.md:**
- Installation instructions (3 steps)
- Quick start guide
- All 7 tools documented
- Troubleshooting section

### Day 15: Example Workflows

**Workflow 1: Build Payment-Protected API**
```typescript
// 1. Start mock server
await mcp.useTool('x402__server_mock_start', { port: 3402 });

// 2. Validate policy
await mcp.useTool('x402__policy_validate', { policy_yaml: "..." });

// 3. Generate middleware
await mcp.useTool('x402__policy_generate_express', { policy_yaml: "..." });

// 4. Test endpoint
await mcp.useTool('x402__testing_check_compliance', {
  url: 'http://localhost:8080/api'
});
```

---

## Phase 4: Production (Week 4)

### Day 16-17: Testing & Optimization

**Load Testing:**
```typescript
const promises = Array(10).fill(null).map((_, i) =>
  mcp.useTool('x402__server_mock_start', { port: 3402 + i })
);
await Promise.all(promises);
// Verify: All succeed, <200ms latency
```

**Security Testing:**
- Injection tests (malicious parameters)
- DoS tests (1000 concurrent requests)
- Path traversal tests

### Day 18: NPM Packaging

`package.json`:
```json
{
  "name": "@x402-dev/mcp-server",
  "version": "0.1.0",
  "description": "MCP server for x402-dev payment protocol testing",
  "main": "dist/index.js",
  "bin": {
    "x402-mcp-server": "dist/index.js"
  },
  "scripts": {
    "build": "tsc",
    "test": "jest",
    "prepublishOnly": "npm run build && npm test"
  },
  "keywords": ["mcp", "x402", "payment", "testing", "solana"]
}
```

### Day 19-20: Release

```bash
# Publish to NPM
npm publish --access public

# Submit to MCP directory
# Create demo video
# Write announcement blog post
```

---

## Common Patterns

### Pattern 1: Tool Implementation Template

```typescript
// 1. Define schema
export const myToolSchema = z.object({
  param1: z.string(),
  param2: z.number().optional()
});

// 2. Implement tool
export async function myTool(params: z.infer<typeof myToolSchema>): Promise<ToolResponse> {
  // Validate
  const validated = myToolSchema.parse(params);

  // Execute
  const result = await execX402Dev('command', [validated.param1]);

  // Return
  return successResponse(result);
}

// 3. Register in server
server.setRequestHandler(CallToolRequestSchema, async (request) => {
  if (request.params.name === 'x402__my_tool') {
    return await myTool(request.params.arguments);
  }
});
```

### Pattern 2: Error Handling

```typescript
try {
  const result = await execX402Dev('command', args);
  return successResponse(result);
} catch (err) {
  return errorResponse({
    code: 'E3XXX',
    message: 'Error description',
    suggestion: 'How to fix',
    context: { command: 'command', args, error: err.message }
  });
}
```

### Pattern 3: Subprocess with Timeout

```typescript
const result = await execX402Dev('command', args, {
  timeout: 30000,  // 30 seconds
  background: false,
  cwd: '/some/path'
});
```

---

## Testing Strategy

### Unit Tests (80%+ coverage)

```typescript
describe('serverMockStart', () => {
  it('validates parameters', () => {
    expect(() => mockStartSchema.parse({ port: 99 })).toThrow();
    expect(() => mockStartSchema.parse({ port: 70000 })).toThrow();
  });

  it('detects already running server', async () => {
    // Mock PID file
    await writePidFile(12345);
    const result = await serverMockStart({ port: 3402 });
    expect(result.error.code).toBe('E3001');
  });
});
```

### Integration Tests (60%+ coverage)

```typescript
describe('End-to-end workflow', () => {
  beforeAll(async () => {
    // Ensure clean state
    await stopAllServers();
  });

  it('completes full workflow', async () => {
    // Test actual tool execution
  });
});
```

---

**Total:** ~3,500 words | Step-by-step implementation guide

For API specifications, see `API-REFERENCE.md`.
For technical details, see `TECHNICAL-APPENDIX.md`.
