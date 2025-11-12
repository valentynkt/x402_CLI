# x402-dev CLI Reference

**Quick reference for all x402-dev commands**

The x402-dev CLI is a comprehensive toolkit for developing, testing, and deploying applications using the x402 Protocol Standard. This reference provides detailed documentation for all available commands.

---

## Table of Contents

- [Quick Reference](#quick-reference)
- [Global Flags](#global-flags)
- [Commands](#commands)
  - [mock](#x402-dev-mock)
  - [test](#x402-dev-test)
  - [verify](#x402-dev-verify)
  - [check](#x402-dev-check)
  - [doctor](#x402-dev-doctor)
  - [policy](#x402-dev-policy)
  - [examples](#x402-dev-examples)
  - [init](#x402-dev-init)
  - [version](#x402-dev-version)
  - [config](#x402-dev-config)
  - [monitor](#x402-dev-monitor)
- [Configuration](#configuration)
- [Exit Codes](#exit-codes)

---

## Quick Reference

| Command | Description | Example |
|---------|-------------|---------|
| **mock** | Start mock facilitator server | `x402-dev mock --port 3402` |
| **test** | Run automated test suites | `x402-dev test tests/suite.yaml` |
| **verify** | Verify x402 protocol compliance | `x402-dev verify --strict` |
| **check** | Validate API endpoint compliance | `x402-dev check http://localhost:3402/api/data` |
| **doctor** | Diagnose setup issues | `x402-dev doctor --fix` |
| **policy** | Generate/validate payment policies | `x402-dev policy validate policy.yaml` |
| **examples** | Browse example implementations | `x402-dev examples list` |
| **init** | Initialize new x402 project | `x402-dev init --template minimal` |
| **version** | Show version and updates | `x402-dev version` |
| **config** | Manage configuration settings | `x402-dev config show` |
| **monitor** | Monitor transactions (planned) | `x402-dev monitor --interval 5` |

---

## Global Flags

These flags work with any command:

| Flag | Short | Description | Example |
|------|-------|-------------|---------|
| `--verbose` | `-v` | Enable verbose output | `x402-dev mock -v` |
| `--debug` | `-d` | Enable debug output with stack traces | `x402-dev test suite.yaml -d` |

**Examples:**

```bash
# Verbose output for detailed information
x402-dev doctor --verbose

# Debug mode for troubleshooting
x402-dev mock --port 3402 --debug
```

---

## Commands

### x402-dev mock

**Description:** Start a mock facilitator server for local development and testing.

**Usage:**
```bash
x402-dev mock [OPTIONS] [SUBCOMMAND]
```

**Options:**

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--port` | `-p` | u16 | 3402 | Port for the mock server |
| `--pricing` | | f64 | 0.01 | Override default pricing amount in SOL/USDC |

**Subcommands:**

| Subcommand | Description |
|------------|-------------|
| `stop` | Stop the running mock server |
| `status` | Check mock server status |
| `restart` | Restart the mock server |

**Examples:**

```bash
# Start mock server on default port (3402)
x402-dev mock

# Start on custom port with custom pricing
x402-dev mock --port 8888 --pricing 0.02

# Check server status
x402-dev mock status

# Stop the server
x402-dev mock stop

# Restart the server
x402-dev mock restart
```

**Expected Output:**
```
Starting mock facilitator server on port 3402...
Server running at http://localhost:3402
Default pricing: 0.01 SOL/USDC

Press Ctrl+C to stop
```

**Exit Codes:**
- `0`: Success
- `1`: General error
- `2`: Configuration error
- `3`: Network error (port already in use)

**See Also:**
- [`x402-dev test`](#x402-dev-test) - Run tests against mock server
- [`x402-dev verify`](#x402-dev-verify) - Verify protocol compliance
- [`x402-dev config`](#x402-dev-config) - Configure default settings

---

### x402-dev test

**Description:** Run automated test suites defined in YAML files.

**Usage:**
```bash
x402-dev test <SUITE> [OPTIONS]
```

**Arguments:**

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `suite` | path | ✅ | Path to YAML test suite file |

**Options:**

| Option | Short | Type | Description |
|--------|-------|------|-------------|
| `--json` | | flag | Output results in JSON format (CI/CD) |
| `--quiet` | `-q` | flag | Suppress verbose output, show summary only |
| `--junit` | | path | Generate JUnit XML report |
| `--html` | | path | Generate HTML report |

**Examples:**

```bash
# Run test suite with standard output
x402-dev test tests/suite.yaml

# Generate JSON output for CI/CD
x402-dev test tests/suite.yaml --json

# Quiet mode with JUnit report
x402-dev test tests/suite.yaml --quiet --junit report.xml

# Generate HTML report
x402-dev test tests/suite.yaml --html results.html

# Combine multiple output formats
x402-dev test tests/suite.yaml --json --junit report.xml --html results.html
```

**Test Suite Format:**
```yaml
# tests/suite.yaml
name: "x402 API Test Suite"
tests:
  - name: "Check protected endpoint"
    request:
      url: "http://localhost:3402/api/data"
      method: GET
    expect:
      status: 402
      headers:
        - name: "x402-price"
          value: "0.01"
```

**Expected Output:**
```
Running test suite: tests/suite.yaml

Test 1/5: Check protected endpoint ..................... PASS (124ms)
Test 2/5: Payment verification ......................... PASS (89ms)
Test 3/5: Invalid signature ............................ PASS (45ms)
Test 4/5: Expired payment .............................. PASS (67ms)
Test 5/5: Rate limiting ................................ PASS (103ms)

Summary: 5 passed, 0 failed, 0 skipped (428ms total)
```

**Exit Codes:**
- `0`: All tests passed
- `1`: One or more tests failed
- `2`: Configuration error (invalid suite file)

**See Also:**
- [`x402-dev mock`](#x402-dev-mock) - Start mock server for testing
- [`x402-dev verify`](#x402-dev-verify) - Verify compliance after tests

---

### x402-dev verify

**Description:** Verify x402 protocol compliance for your implementation.

**Usage:**
```bash
x402-dev verify [OPTIONS]
```

**Options:**

| Option | Type | Description |
|--------|------|-------------|
| `--strict` | flag | Enable strict compliance checking |
| `--output` | string | Output format: text, json (default: text) |

**Examples:**

```bash
# Standard verification
x402-dev verify

# Strict mode with all checks
x402-dev verify --strict

# JSON output for automation
x402-dev verify --output json
```

**Expected Output:**
```
x402 Protocol Compliance Verification

✅ Header format ............................ PASS
✅ Signature validation ..................... PASS
✅ Timestamp format ......................... PASS
✅ Payment verification ..................... PASS
⚠️  Rate limiting ........................... WARN (optional)

Status: COMPLIANT (4/4 required checks passed)
```

**Exit Codes:**
- `0`: Fully compliant
- `1`: Non-compliant (failed required checks)

**See Also:**
- [`x402-dev test`](#x402-dev-test) - Run automated test suites
- [`x402-dev check`](#x402-dev-check) - Check specific endpoint

**Note:** Currently under development (coming in Epic 3).

---

### x402-dev check

**Description:** Validate a specific API endpoint for x402 compliance.

**Usage:**
```bash
x402-dev check <URL> [OPTIONS]
```

**Arguments:**

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `url` | string | ✅ | URL to check for x402 compliance |

**Options:**

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--format` | string | text | Output format: text or json |

**Examples:**

```bash
# Check endpoint with text output
x402-dev check http://localhost:3402/api/data

# JSON format for scripting
x402-dev check http://localhost:3402/api/data --format json

# Check remote endpoint
x402-dev check https://api.example.com/protected --format text
```

**Expected Output:**
```
Checking: http://localhost:3402/api/data

Response Analysis:
  Status Code: 402 Payment Required ✅
  Headers:
    ✅ x402-price: 0.01
    ✅ x402-token-address: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
    ✅ x402-recipient-address: 9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin

Status: COMPLIANT
```

**JSON Output:**
```json
{
  "url": "http://localhost:3402/api/data",
  "compliant": true,
  "status_code": 402,
  "headers": {
    "x402-price": "0.01",
    "x402-token-address": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    "x402-recipient-address": "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin"
  },
  "checks": {
    "status_402": true,
    "required_headers": true,
    "valid_addresses": true
  }
}
```

**Exit Codes:**
- `0`: Endpoint is compliant
- `1`: Endpoint is not compliant
- `3`: Network error (cannot reach endpoint)

**See Also:**
- [`x402-dev doctor`](#x402-dev-doctor) - Diagnose and fix issues
- [`x402-dev verify`](#x402-dev-verify) - Verify full protocol compliance

---

### x402-dev doctor

**Description:** Diagnose issues and validate your x402 setup.

**Usage:**
```bash
x402-dev doctor [OPTIONS]
```

**Options:**

| Option | Type | Description |
|--------|------|-------------|
| `--fix` | flag | Automatically fix common issues |

**Examples:**

```bash
# Run diagnostics
x402-dev doctor

# Run diagnostics and auto-fix issues
x402-dev doctor --fix
```

**Expected Output:**
```
x402 Environment Diagnostics

Checking configuration...
  ✅ Config file exists (.x402dev.yaml)
  ✅ Port 3402 is available
  ✅ Solana RPC connection: https://api.devnet.solana.com
  ⚠️  Rust toolchain not found (optional)

Checking dependencies...
  ✅ Network connectivity
  ✅ File permissions
  ✅ Disk space (15.2 GB available)

Checking mock server...
  ❌ Server not running on port 3402
  💡 Run: x402-dev mock

Summary: 7 passed, 1 failed, 1 warning

Suggested actions:
  1. Start mock server: x402-dev mock
  2. Consider installing Rust for local development
```

**Exit Codes:**
- `0`: All checks passed
- `1`: One or more critical checks failed
- `2`: Configuration errors detected

**See Also:**
- [`x402-dev check`](#x402-dev-check) - Quick health check
- [`x402-dev config`](#x402-dev-config) - View configuration
- [`x402-dev version`](#x402-dev-version) - Check version info

---

### x402-dev policy

**Description:** Manage payment policies and rules. Validate policy files and generate framework-specific middleware.

**Usage:**
```bash
x402-dev policy <SUBCOMMAND>
```

**Subcommands:**

#### policy validate

Validate policy file syntax and detect conflicts.

**Usage:**
```bash
x402-dev policy validate <FILE>
```

**Arguments:**

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `file` | path | ✅ | Path to policy YAML file |

**Examples:**

```bash
# Validate policy file
x402-dev policy validate policy.yaml

# Validate with verbose output
x402-dev policy validate policy.yaml --verbose
```

**Expected Output:**
```
Policy Validation
File: policy.yaml

Validation Issues:

⚠️  WARNING Overlapping path patterns detected
   Policies #2 and #3 both match "/api/users/*"
   💡 Combine into single rule or use more specific paths
      » Consider: "/api/users/premium/*" vs "/api/users/*"

✅ Policy file is valid (with warnings)
```

#### policy generate

Generate middleware code from policy file.

**Usage:**
```bash
x402-dev policy generate <FILE> --framework <FRAMEWORK> [OPTIONS]
```

**Arguments:**

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `file` | path | ✅ | Path to policy YAML file |

**Options:**

| Option | Short | Type | Required | Description |
|--------|-------|------|----------|-------------|
| `--framework` | `-f` | string | ✅ | Target framework: express or fastify |
| `--output` | `-o` | path | | Output file path (prints to stdout if omitted) |

**Examples:**

```bash
# Generate Express middleware
x402-dev policy generate policy.yaml --framework express --output middleware.js

# Generate Fastify plugin
x402-dev policy generate policy.yaml --framework fastify --output plugin.js

# Print to stdout (for piping)
x402-dev policy generate policy.yaml --framework express

# Pipe to file
x402-dev policy generate policy.yaml --framework express > middleware.js
```

**Policy File Format:**
```yaml
# policy.yaml
policies:
  - path: "/api/data"
    price: 0.01
    rate_limit:
      requests: 100
      window: 60

  - path: "/api/premium/*"
    price: 0.05
    rate_limit:
      requests: 50
      window: 60
```

**Expected Output:**
```
Code Generation
Policy file: policy.yaml
Framework: Express

✅ Generated middleware: middleware.js
  Lines: 127
  Size: 4521 bytes
```

**Exit Codes:**
- `0`: Success
- `1`: Validation failed or generation error
- `2`: Invalid policy file format

**See Also:**
- [`x402-dev monitor`](#x402-dev-monitor) - Monitor policy enforcement
- [`x402-dev init`](#x402-dev-init) - Initialize project with policies

---

### x402-dev examples

**Description:** Browse and view example implementations and usage patterns.

**Usage:**
```bash
x402-dev examples [COMMAND] [NAME]
```

**Arguments:**

| Argument | Type | Description |
|----------|------|-------------|
| `command` | string | Subcommand: list, info, or init (optional) |
| `name` | string | Example name (for info and init commands) |

**Examples:**

```bash
# List all available examples
x402-dev examples
x402-dev examples list

# Show details for specific example
x402-dev examples info express-basic

# Initialize project with example
x402-dev examples init express-basic

# Filter by language
x402-dev examples --language typescript

# Filter by topic
x402-dev examples --topic payments
```

**Expected Output:**
```
x402 Examples

Available Examples:

📦 express-basic
   Language: JavaScript
   Description: Basic Express.js integration with x402
   Topics: express, middleware, payments

📦 fastify-advanced
   Language: TypeScript
   Description: Advanced Fastify plugin with rate limiting
   Topics: fastify, typescript, policies

📦 nextjs-integration
   Language: TypeScript
   Description: Next.js API routes with x402 protection
   Topics: nextjs, react, api-routes

Use: x402-dev examples info <name> for details
Use: x402-dev examples init <name> to create project
```

**Exit Codes:**
- `0`: Success
- `1`: Example not found

**See Also:**
- [`x402-dev init`](#x402-dev-init) - Initialize new project

---

### x402-dev init

**Description:** Initialize a new x402 project with configuration and templates.

**Usage:**
```bash
x402-dev init [OPTIONS]
```

**Options:**

| Option | Type | Description |
|--------|------|-------------|
| `--defaults` | flag | Skip interactive prompts, use defaults |
| `--template` | string | Project template: minimal, express, fastify |

**Examples:**

```bash
# Interactive initialization
x402-dev init

# Use defaults (non-interactive)
x402-dev init --defaults

# Initialize with template
x402-dev init --template minimal

# Express project
x402-dev init --template express

# Fastify project
x402-dev init --template fastify
```

**Interactive Mode:**
```
x402 Project Initialization

? Project name: my-x402-app
? Description: My x402-enabled API
? Template: express
? Port: 3402
? Pricing (SOL/USDC): 0.01

Creating project structure...
  ✅ .x402dev.yaml
  ✅ src/server.js
  ✅ src/middleware/x402.js
  ✅ package.json
  ✅ README.md

Next steps:
  1. cd my-x402-app
  2. npm install
  3. x402-dev mock         # Start mock server
  4. npm start             # Start your app
```

**Generated Project Structure:**
```
my-x402-app/
├── .x402dev.yaml          # x402 configuration
├── package.json           # Dependencies
├── README.md              # Getting started guide
└── src/
    ├── server.js          # Express/Fastify server
    └── middleware/
        └── x402.js        # x402 middleware
```

**Exit Codes:**
- `0`: Project initialized successfully
- `1`: Initialization failed
- `2`: Configuration error

**See Also:**
- [`x402-dev config`](#x402-dev-config) - Manage configuration
- [`x402-dev examples`](#x402-dev-examples) - View example code

---

### x402-dev version

**Description:** Display version information and check for updates.

**Usage:**
```bash
x402-dev version [OPTIONS]
```

**Options:**

| Option | Type | Description |
|--------|------|-------------|
| `--no-update-check` | flag | Skip checking for updates |

**Examples:**

```bash
# Show version with update check
x402-dev version

# Skip update check
x402-dev version --no-update-check
```

**Expected Output:**
```
x402-dev v0.2.0
Platform: darwin-aarch64

✨ Update available: 0.2.0 → 0.3.0
Run: cargo install x402-dev
```

**Update Check:**
- Checks crates.io for new versions (max once per 7 days)
- Cached in `~/.x402dev/update-check.json`
- Silent failures (never blocks version command)

**Exit Codes:**
- `0`: Always succeeds

**See Also:**
- [`x402-dev doctor`](#x402-dev-doctor) - Diagnose issues
- [`x402-dev config`](#x402-dev-config) - View configuration

---

### x402-dev config

**Description:** Manage configuration settings and view merged configuration.

**Usage:**
```bash
x402-dev config <SUBCOMMAND> [OPTIONS]
```

**Subcommands:**

#### config show

Display current configuration with sources.

**Usage:**
```bash
x402-dev config show [OPTIONS]
```

**Global Options (affect configuration):**

| Option | Type | Description |
|--------|------|-------------|
| `--port` | u16 | Override port setting |
| `--solana-rpc` | string | Override Solana RPC URL |
| `--log-level` | string | Override log level (error\|warn\|info\|debug\|trace) |

**Examples:**

```bash
# Show current configuration
x402-dev config show

# Override port via CLI flag
x402-dev config show --port 8888

# Test environment variable
X402_DEV_PORT=9999 x402-dev config show

# Multiple overrides
x402-dev config show --port 8888 --log-level debug
```

**Expected Output:**
```
x402-dev Configuration
=====================

Configuration Priority:
  CLI flags > Environment variables > Project config > Global config > Defaults

Current Configuration:
  port: 3402 (source: project (.x402dev.yaml))
  solana_rpc: https://api.devnet.solana.com (source: default)
  log_level: info (source: default)

Config File Locations:
  Global: ~/.x402dev/config.yaml
  Project: ./.x402dev.yaml

Environment Variables:
  X402_DEV_PORT
  X402_DEV_SOLANA_RPC
  X402_DEV_LOG_LEVEL
```

**Configuration Priority Order:**
1. **CLI flags** (highest) - `--port 8888`
2. **Environment variables** - `X402_DEV_PORT=9999`
3. **Project config** - `./.x402dev.yaml`
4. **Global config** - `~/.x402dev/config.yaml`
5. **Defaults** (lowest)

**Exit Codes:**
- `0`: Success
- `2`: Configuration validation error

**See Also:**
- [`x402-dev init`](#x402-dev-init) - Initialize project configuration
- [`x402-dev check`](#x402-dev-check) - Validate configuration

---

### x402-dev monitor

**Description:** Monitor x402 transactions and performance metrics in real-time.

**Usage:**
```bash
x402-dev monitor [OPTIONS]
```

**Options:**

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--interval` | u64 | 5 | Update interval in seconds |
| `--verbose` | flag | false | Show detailed transaction logs |

**Examples:**

```bash
# Start monitoring with default interval
x402-dev monitor

# Custom update interval
x402-dev monitor --interval 10

# Verbose mode
x402-dev monitor --verbose --interval 2
```

**Expected Output:**
```
x402 Transaction Monitor
Monitoring interval: 5s

[10:45:23] ✅ Payment verified: 0.01 SOL
           Transaction: 5KJp7...9mNq
           Endpoint: /api/data

[10:45:31] ✅ Payment verified: 0.05 SOL
           Transaction: 8xZr2...4pLk
           Endpoint: /api/premium/users

Metrics (last 5min):
  Requests: 142
  Payments: 138 (97.2%)
  Failed: 4 (2.8%)
  Revenue: 1.89 SOL
```

**Exit Codes:**
- `0`: Monitoring stopped gracefully
- `1`: Configuration or connection error

**See Also:**
- [`x402-dev policy`](#x402-dev-policy) - Manage payment policies

**Note:** Currently under development (coming in Epic 5).

---

## Configuration

### Configuration Files

x402-dev uses a hierarchical configuration system with multiple sources:

**Global Config:** `~/.x402dev/config.yaml`
```yaml
port: 3402
solana_rpc: "https://api.devnet.solana.com"
log_level: info
pricing:
  default: 0.01
  per_resource:
    "/api/premium/*": 0.05
simulation_mode: success
timeout_delay_ms: 5000
```

**Project Config:** `./.x402dev.yaml`
```yaml
port: 8402
pricing:
  default: 0.02
  per_resource:
    "/api/data": 0.01
    "/api/admin/*": 0.10
```

### Environment Variables

| Variable | Type | Description | Example |
|----------|------|-------------|---------|
| `X402_DEV_PORT` | u16 | Override port | `X402_DEV_PORT=8888` |
| `X402_DEV_SOLANA_RPC` | string | Override Solana RPC URL | `X402_DEV_SOLANA_RPC=https://api.mainnet-beta.solana.com` |
| `X402_DEV_LOG_LEVEL` | string | Override log level | `X402_DEV_LOG_LEVEL=debug` |

**Examples:**

```bash
# Set port via environment variable
export X402_DEV_PORT=9999
x402-dev mock

# One-time override
X402_DEV_LOG_LEVEL=debug x402-dev doctor

# Multiple environment variables
X402_DEV_PORT=8888 X402_DEV_LOG_LEVEL=trace x402-dev mock
```

### Priority Order

Configuration values are merged with this priority (highest to lowest):

1. **CLI Flags** - `x402-dev mock --port 8888`
2. **Environment Variables** - `X402_DEV_PORT=9999`
3. **Project Config** - `./.x402dev.yaml`
4. **Global Config** - `~/.x402dev/config.yaml`
5. **Defaults** - Built-in defaults

---

## Exit Codes

x402-dev follows POSIX exit code conventions:

| Code | Name | Description | Example Commands |
|------|------|-------------|------------------|
| `0` | Success | Command completed successfully | All commands on success |
| `1` | General Error | Generic failure or validation error | Failed tests, invalid arguments |
| `2` | Config Error | Configuration validation failed | Invalid config file, bad port range |
| `3` | Network Error | Network connectivity issues | Port in use, RPC unreachable |

**Usage in Scripts:**

```bash
# Check if command succeeded
if x402-dev test suite.yaml; then
  echo "Tests passed!"
else
  echo "Tests failed with exit code: $?"
fi

# Conditional execution based on exit code
x402-dev doctor || echo "Health check failed!"

# Capture exit code
x402-dev mock
EXIT_CODE=$?
if [ $EXIT_CODE -eq 3 ]; then
  echo "Port already in use"
fi
```

---

## Troubleshooting

### Common Issues

**1. Port Already in Use (Exit Code 3)**

```bash
❌ Network Error: Failed to bind to port 3402
💡 Port may already be in use. Try a different port with --port

# Solution:
x402-dev mock --port 8888
# or
x402-dev mock stop  # Stop existing server first
```

**2. Invalid Configuration (Exit Code 2)**

```bash
❌ Config Error: Invalid port: 99999. Port must be between 1024 and 65535.
💡 Set port to a value in the valid range, e.g., 8402

# Solution: Edit .x402dev.yaml
port: 8402
```

**3. Tests Failing (Exit Code 1)**

```bash
# Use verbose mode to see details
x402-dev test suite.yaml --verbose

# Or debug mode for stack traces
x402-dev test suite.yaml --debug
```

**4. Update Check Slow**

```bash
# Skip update check
x402-dev version --no-update-check

# Clear update cache
rm ~/.x402dev/update-check.json
```

### Debug Tips

**Enable verbose output:**
```bash
x402-dev mock --verbose
```

**Enable debug output with stack traces:**
```bash
x402-dev test suite.yaml --debug
```

**Check configuration sources:**
```bash
x402-dev config show
```

**Run health diagnostics:**
```bash
x402-dev doctor
```

---

## Additional Resources

- **Documentation:** https://docs.x402.dev
- **GitHub:** https://github.com/x402-protocol/x402-dev
- **Examples:** https://github.com/x402-protocol/examples
- **Protocol Spec:** https://x402.dev/spec

---

**Version:** x402-dev v0.2.0
**Last Updated:** 2024-11-12
