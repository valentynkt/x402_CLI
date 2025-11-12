# x402-dev CLI Integration Analysis for MCP Server

## Executive Summary

This document provides a **comprehensive deep-dive analysis** of the x402-dev CLI implementation to understand exactly how to integrate MCP server functionality cleanly. The CLI is extremely well-structured for MCP integration with **high reusability** (85%+ of command logic can be directly wrapped).

### Key Findings

- **Architecture**: Clean separation between CLI layer and business logic
- **Async Model**: tokio multi-threaded runtime throughout
- **Reusability Score**: 85% - Most commands can be wrapped with minimal changes
- **Integration Complexity**: LOW - Commands are already library functions
- **Recommended Approach**: **Direct library wrapping** (no refactoring needed)

---

## 1. Command Execution Flow Analysis

### 1.1 Entry Point: main.rs

**File**: `/crates/x402-cli/src/main.rs`

```rust
#[tokio::main]
async fn main() {
    let cli = Cli::parse();  // Parse CLI args with clap

    let result = match cli.command {
        Commands::Mock(args) => mock::run(&args).await,
        Commands::Test(args) => test::execute(&args).await,
        Commands::Check(args) => check::run(&args).await,
        Commands::Policy(args) => policy::handle_policy_command(args),
        Commands::Doctor(args) => doctor::run(&args).await,
        Commands::Init(args) => init::run(&args).await,
        Commands::Version(args) => version::run(&args).await,
        Commands::Config(args) => config_cmd::run(&args).await,
        Commands::Examples(args) => examples::run(&args).await,
        // ... more commands
    };

    // Handle errors and exit codes
    if let Err(e) = result {
        let cli_error = convert_anyhow_to_cli_error(e);
        print_error(&cli_error, cli.verbose, cli.debug);
        std::process::exit(cli_error.exit_code());
    }
}
```

**Flow Summary**:
1. **Parse**: clap parses command-line arguments into structured types
2. **Dispatch**: Pattern match routes to appropriate command module
3. **Execute**: Command runs asynchronously with tokio
4. **Error Handling**: Unified error conversion and formatting
5. **Exit**: Process exits with appropriate code (0, 1, 2, 3)

**Key Observations**:
- ✅ **No CLI-specific logic in commands** - All functions take structured args
- ✅ **Already async** - Easy to call from MCP server handlers
- ✅ **Return Result<()>** - Standard error handling pattern
- ✅ **No stdin/stdout coupling** - Commands can be called programmatically

---

## 2. Command-by-Command Analysis

### 2.1 Mock Command (Server Lifecycle)

**File**: `/crates/x402-cli/src/commands/mock.rs`

#### Execution Path

```
CLI: x402-dev mock --port 3402
  ↓
main.rs: Commands::Mock(args)
  ↓
mock::run(&args)
  ↓
match args.command:
  - None → build_server_config() → server_start()
  - Stop → handle_stop() → server_stop()
  - Status → handle_status() → server_status_check()
  - Restart → handle_restart() → server_restart()
```

#### Dependencies

```rust
use x402_server::{
    start_server, stop_server, server_status, restart_server,
    Config, MockServerConfig, PricingConfig, PricingMatcher,
    InvoiceGenerator
};
use crate::config::{load_merged_config, CliOverrides};
```

#### Core Logic Location

**Server implementation**: `/crates/x402-server/src/`
- `lifecycle.rs`: start_server(), stop_server(), restart_server()
- `process.rs`: PID management, process checking
- `server.rs`: HTTP server setup, actix-web configuration

#### Parameters

```rust
pub struct MockArgs {
    pub port: u16,                    // Server port (default: 3402)
    pub pricing: Option<f64>,         // Override default pricing
    pub command: Option<MockSubcommand>,  // Stop/Status/Restart
}
```

#### Configuration Handling

```rust
fn build_server_config(args: &MockArgs) -> Result<MockServerConfig> {
    // 1. Load merged config (CLI > Env > Project > Global > Defaults)
    let cli_overrides = CliOverrides {
        pricing: args.pricing,
        // ... other overrides
    };
    let config = load_merged_config(Some(&cli_overrides))?;

    // 2. Convert to server config
    let server_config = Config {
        port: args.port,
        solana_rpc: config.solana_rpc,
        log_level: config.log_level.to_string(),
        pricing: PricingConfig { /* ... */ },
        simulation_mode: convert_simulation_mode(config.simulation_mode),
        timeout_delay_ms: config.timeout_delay_ms,
    };

    // 3. Create server config with dependencies
    Ok(MockServerConfig {
        port: args.port,
        pricing_matcher: PricingMatcher::new(server_config.pricing.clone()),
        invoice_generator: InvoiceGenerator::new(),
        config: server_config,
    })
}
```

#### Side Effects

- **File I/O**: Writes PID file to `~/.x402dev/mock-server.pid`
- **Network**: Binds to TCP port, starts HTTP server
- **Process**: Creates background process (for CLI), runs in-process (for library)

#### Error Handling

```rust
// Port already in use
Err(e) if e.kind() == AddrInUse => exit code 2

// Already running
if is_server_running(pid) => exit code 3

// General errors
_ => exit code 1
```

#### Return Types

```rust
pub async fn run(args: &MockArgs) -> Result<()>
pub async fn handle_stop() -> Result<()>
pub async fn handle_status() -> Result<()>
pub async fn handle_restart(args: &MockArgs) -> Result<()>
```

#### **MCP Integration Assessment**

| Aspect | Status | Notes |
|--------|--------|-------|
| **Reusability** | ✅ HIGH | Core logic in x402-server crate |
| **Wrappability** | ✅ EXCELLENT | Functions already take structured args |
| **CLI Coupling** | ✅ MINIMAL | Only stdout printing and exit codes |
| **Refactoring Needed** | ❌ NONE | Can wrap directly |

**MCP Tool Wrapper Pattern**:

```rust
// In MCP server handler
async fn mcp_mock_start(port: u16, pricing: Option<f64>) -> Result<String> {
    let args = MockArgs {
        port,
        pricing,
        command: None,
    };

    // Capture stdout instead of printing
    let result = mock::run(&args).await?;
    Ok(format!("Server started on port {}", port))
}

async fn mcp_mock_stop() -> Result<String> {
    handle_stop().await?;
    Ok("Server stopped successfully".to_string())
}
```

---

### 2.2 Test Command (Automated Testing)

**File**: `/crates/x402-cli/src/commands/test.rs`

#### Execution Path

```
CLI: x402-dev test tests/suite.yaml --json
  ↓
test::execute(&args)
  ↓
1. TestSuite::from_file(suite_path)      # Parse YAML
2. execute_test_suite(&suite)            # Run tests
3. format_json(&result) / format_summary # Format output
4. generate_junit_xml (optional)         # CI/CD integration
5. std::process::exit(result.exit_code())
```

#### Dependencies

```rust
use x402_core::testing::{
    TestSuite,           // YAML test suite parser
    execute_test_suite,  // Test runner
    format_json,         // JSON formatter
    format_summary,      // Human-readable formatter
    generate_junit_xml,  // JUnit XML generator
};
```

#### Core Logic Location

**Testing framework**: `/crates/x402-core/src/testing/`
- `parser.rs`: YAML parsing, TestSuite struct
- `executor.rs`: Test execution engine
- `reporter.rs`: Result formatting (JSON, summary, JUnit)
- `assertions.rs`: Test assertions and validations

#### Parameters

```rust
pub struct TestArgs {
    pub suite: PathBuf,              // Path to YAML test suite
    pub json: bool,                  // JSON output for CI/CD
    pub quiet: bool,                 // Suppress verbose output
    pub junit: Option<PathBuf>,      // JUnit XML report path
    pub html: Option<PathBuf>,       // HTML report path (optional)
}
```

#### Side Effects

- **File I/O**: Reads test suite YAML, writes JUnit/HTML reports
- **Network**: Makes HTTP requests to test endpoints
- **Process**: Exits with test result code (0 = pass, 1 = fail)

#### Return Types

```rust
pub async fn execute(args: &TestArgs) -> Result<()>
```

**NOTE**: Function calls `std::process::exit()` directly - needs refactoring for MCP

#### **MCP Integration Assessment**

| Aspect | Status | Notes |
|--------|--------|-------|
| **Reusability** | ⚠️ MEDIUM | Core logic reusable, exit() problematic |
| **Wrappability** | ⚠️ NEEDS CHANGE | Remove direct exit() call |
| **CLI Coupling** | ⚠️ MODERATE | stdout printing and exit codes |
| **Refactoring Needed** | ✅ MINOR | Extract result handling |

**Recommended Refactoring**:

```rust
// NEW: Return result instead of exit
pub async fn execute_with_result(args: &TestArgs) -> Result<TestResult> {
    let suite = TestSuite::from_file(&args.suite)?;
    let result = x402_core::testing::execute_test_suite(&suite).await?;

    // Generate reports if requested
    if let Some(junit_path) = &args.junit {
        let xml = generate_junit_xml(&result);
        std::fs::write(junit_path, xml)?;
    }

    Ok(result)
}

// EXISTING: Keep for CLI compatibility
pub async fn execute(args: &TestArgs) -> Result<()> {
    let result = execute_with_result(args).await?;

    if args.json {
        println!("{}", format_json(&result));
    } else {
        println!("{}", format_summary(&result, args.quiet));
    }

    std::process::exit(result.exit_code());
}
```

**MCP Tool Wrapper**:

```rust
async fn mcp_test_run(suite_path: String) -> Result<serde_json::Value> {
    let args = TestArgs {
        suite: PathBuf::from(suite_path),
        json: true,
        quiet: false,
        junit: None,
        html: None,
    };

    let result = test::execute_with_result(&args).await?;

    // Return structured result (not string)
    Ok(serde_json::json!({
        "passed": result.passed,
        "failed": result.failed,
        "total": result.total,
        "exit_code": result.exit_code(),
    }))
}
```

---

### 2.3 Check Command (Protocol Compliance)

**File**: `/crates/x402-cli/src/commands/check.rs`

#### Execution Path

```
CLI: x402-dev check http://localhost:3402/api/data
  ↓
check::run(&args)
  ↓
1. HTTP GET request (with timeout)
2. Check HTTP 402 status code
3. Check WWW-Authenticate header presence
4. parse_www_authenticate() → HashMap<String, String>
5. validate_invoice() → Vec<(String, bool, String)>
6. Print results or JSON output
7. exit(0) or exit(1)
```

#### Dependencies

```rust
use reqwest;  // HTTP client
use colored;  // Terminal colors
use serde_json;  // JSON output
```

**No x402-core dependencies** - entirely self-contained!

#### Core Logic

All validation logic is **inline in check.rs**:

```rust
fn parse_www_authenticate(header: &str) -> Result<HashMap<String, String>> {
    // Parse "x402-solana recipient=<addr> amount=<val> ..." format
    let parts: Vec<&str> = header.split_whitespace().collect();
    // ... parsing logic
}

fn validate_invoice(fields: &HashMap<String, String>) -> Vec<(String, bool, String)> {
    // Validate recipient (Base58, 32-44 chars)
    // Validate amount (parseable, positive)
    // Validate currency (USDC)
    // Validate memo (starts with "req-")
    // Validate network (devnet/testnet/mainnet-beta)
}
```

#### Parameters

```rust
pub struct CheckArgs {
    pub url: String,         // URL to check for x402 compliance
    pub format: String,      // "text" or "json"
}
```

#### Side Effects

- **Network**: HTTP GET request to target URL (10 second timeout)
- **Process**: Exits with code 0 (pass) or 1 (fail)

#### Return Types

```rust
pub async fn run(args: &CheckArgs) -> Result<()>
```

**NOTE**: Also calls `std::process::exit()` directly

#### **MCP Integration Assessment**

| Aspect | Status | Notes |
|--------|--------|-------|
| **Reusability** | ✅ HIGH | All logic self-contained |
| **Wrappability** | ⚠️ NEEDS CHANGE | Remove direct exit() call |
| **CLI Coupling** | ⚠️ MODERATE | stdout printing and exit codes |
| **Refactoring Needed** | ✅ MINOR | Extract result handling |

**Recommended Refactoring**:

```rust
// NEW: Structured result type
pub struct CheckResult {
    pub status_ok: bool,
    pub header_ok: bool,
    pub validation_results: Vec<(String, bool, String)>,
    pub checks_passed: usize,
    pub checks_total: usize,
}

impl CheckResult {
    pub fn is_compliant(&self) -> bool {
        self.checks_passed == self.checks_total
    }
}

// NEW: Return result instead of exit
pub async fn check_with_result(args: &CheckArgs) -> Result<CheckResult> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let response = client.get(&args.url).send().await?;

    let mut checks_passed = 0;
    let mut checks_total = 0;

    // Check status code
    checks_total += 1;
    let status_ok = response.status().as_u16() == 402;
    if status_ok { checks_passed += 1; }

    // Check header
    checks_total += 1;
    let www_auth = response.headers().get("www-authenticate");
    let header_ok = www_auth.is_some();
    if header_ok { checks_passed += 1; }

    // Validate invoice
    let validation_results = if header_ok {
        let header_value = www_auth.unwrap().to_str()?;
        let fields = parse_www_authenticate(header_value)?;
        let results = validate_invoice(&fields);
        checks_total += results.len();
        checks_passed += results.iter().filter(|(_, passed, _)| *passed).count();
        results
    } else {
        vec![]
    };

    Ok(CheckResult {
        status_ok,
        header_ok,
        validation_results,
        checks_passed,
        checks_total,
    })
}
```

**MCP Tool Wrapper**:

```rust
async fn mcp_check_compliance(url: String) -> Result<serde_json::Value> {
    let args = CheckArgs {
        url: url.clone(),
        format: "json".to_string(),
    };

    let result = check::check_with_result(&args).await?;

    Ok(serde_json::json!({
        "url": url,
        "compliant": result.is_compliant(),
        "checks_passed": result.checks_passed,
        "checks_total": result.checks_total,
        "details": {
            "status_code_402": result.status_ok,
            "www_authenticate_header": result.header_ok,
            "invoice_validation": result.validation_results,
        }
    }))
}
```

---

### 2.4 Policy Command (Code Generation & Validation)

**File**: `/crates/x402-cli/src/commands/policy.rs`

#### Execution Path

```
CLI: x402-dev policy validate policy.yaml
  ↓
policy::handle_policy_command(args)
  ↓
match args.command:
  - Validate → validate_command(file)
  - Generate → generate_command(file, framework, output)

Validate flow:
1. Read policy YAML file
2. Parse into PolicyFile struct
3. Convert to PolicyConfig
4. validate_policies(&config) → ValidationReport
5. Display results with colors
6. Return Ok() or Err()

Generate flow:
1. Read and validate policy file
2. Generate code:
   - Express → generate_express_middleware()
   - Fastify → generate_fastify_plugin()
3. Write to file or stdout
```

#### Dependencies

```rust
use x402_core::policy::{
    codegen::{generate_express_middleware, generate_fastify_plugin},
    validate_policies,
    ValidationReport, IssueType,
};
use x402_core::policy::rules::PolicyFile;
use x402_core::policy::types::PolicyConfig;
```

#### Core Logic Location

**Policy engine**: `/crates/x402-core/src/policy/`
- `validator.rs`: validate_policies() function
- `codegen/express.rs`: Express middleware generation
- `codegen/fastify.rs`: Fastify plugin generation
- `rules.rs`: PolicyFile struct
- `types.rs`: PolicyConfig struct

#### Parameters

```rust
pub enum PolicyCommand {
    Validate { file: PathBuf },
    Generate {
        file: PathBuf,
        framework: Framework,  // Express or Fastify
        output: Option<PathBuf>,
    },
}

pub enum Framework {
    Express,
    Fastify,
}
```

#### Side Effects

- **File I/O**: Reads policy YAML, writes generated code
- **Process**: Returns Ok(()) or anyhow::bail!()

#### Return Types

```rust
pub fn handle_policy_command(args: PolicyArgs) -> Result<()>
fn validate_command(file: PathBuf) -> Result<()>
fn generate_command(file: PathBuf, framework: Framework, output: Option<PathBuf>) -> Result<()>
```

#### **MCP Integration Assessment**

| Aspect | Status | Notes |
|--------|--------|-------|
| **Reusability** | ✅ EXCELLENT | Core logic in x402-core |
| **Wrappability** | ✅ EXCELLENT | No refactoring needed |
| **CLI Coupling** | ✅ MINIMAL | Only stdout printing |
| **Refactoring Needed** | ❌ NONE | Perfect as-is |

**MCP Tool Wrapper** (No Changes Needed!):

```rust
async fn mcp_policy_validate(file_path: String) -> Result<serde_json::Value> {
    let policy_content = std::fs::read_to_string(&file_path)?;
    let policy_file: PolicyFile = serde_yaml::from_str(&policy_content)?;
    let policy_config = PolicyConfig {
        policies: policy_file.policies.clone(),
    };

    let report = validate_policies(&policy_config);

    Ok(serde_json::json!({
        "valid": !report.has_errors,
        "has_warnings": report.has_warnings,
        "issues": report.issues.iter().map(|issue| {
            serde_json::json!({
                "type": format!("{:?}", issue.issue_type),
                "message": issue.message,
                "details": issue.details,
                "suggestions": issue.suggestions,
            })
        }).collect::<Vec<_>>(),
    }))
}

async fn mcp_policy_generate(
    file_path: String,
    framework: String,  // "express" or "fastify"
) -> Result<String> {
    let policy_content = std::fs::read_to_string(&file_path)?;
    let policy_file: PolicyFile = serde_yaml::from_str(&policy_content)?;

    let code = match framework.as_str() {
        "express" => generate_express_middleware(&policy_file, &file_path),
        "fastify" => generate_fastify_plugin(&policy_file.policies, Some(&file_path)),
        _ => anyhow::bail!("Invalid framework: {}", framework),
    };

    Ok(code)
}
```

---

### 2.5 Doctor Command (System Diagnostics)

**File**: `/crates/x402-cli/src/commands/doctor.rs`

#### Execution Path

```
CLI: x402-dev doctor
  ↓
doctor::run(&args)
  ↓
1. check_environment() → Rust version, npm, x402-dev version
2. check_configuration() → Config file, port availability
3. check_ecosystem() → package.json, dependencies
4. print_summary() → Warnings, failures, suggestions
5. Always returns Ok() (diagnostics never fail)
```

#### Dependencies

```rust
use std::process::Command;  // Run external commands (rustc, npm)
use std::net::TcpListener;  // Check port availability
use std::fs;                // Read package.json
use colored;                // Terminal colors
use crate::config::load_merged_config;
```

**No external crate dependencies** - uses stdlib!

#### Core Logic

All diagnostic logic is **inline in doctor.rs**:

```rust
fn check_rust_version() -> Option<String> {
    Command::new("rustc").arg("--version").output().ok()?
    // Parse version from output
}

fn check_npm_version() -> Option<String> {
    Command::new("npm").arg("--version").output().ok()?
}

fn check_port_availability(port: u16, results: &mut DiagnosticResults) {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => println!("Port {} available"),
        Err(_) => results.add_warning("Port in use"),
    }
}

fn check_package(name: &str, package_names: &[&str], ...) {
    // Check if package exists in dependencies or devDependencies
}
```

#### Parameters

```rust
pub struct DoctorArgs {
    // No arguments yet (reserved for --fix flag)
}
```

#### Side Effects

- **File I/O**: Reads `.x402dev.yaml`, `package.json`
- **Process**: Spawns `rustc`, `npm` subprocesses
- **Network**: Binds to port temporarily to check availability
- **Always returns Ok()** - Never fails!

#### Return Types

```rust
pub async fn run(_args: &DoctorArgs) -> Result<()>
```

#### **MCP Integration Assessment**

| Aspect | Status | Notes |
|--------|--------|-------|
| **Reusability** | ✅ EXCELLENT | Self-contained, no dependencies |
| **Wrappability** | ✅ EXCELLENT | Returns structured results |
| **CLI Coupling** | ⚠️ MODERATE | stdout printing for diagnostics |
| **Refactoring Needed** | ✅ MINOR | Extract DiagnosticResults return |

**Recommended Enhancement**:

```rust
// NEW: Make DiagnosticResults public
pub struct DiagnosticResults {
    pub warnings: Vec<String>,
    pub failures: Vec<String>,
    pub suggestions: Vec<String>,
}

impl DiagnosticResults {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "warnings": self.warnings,
            "failures": self.failures,
            "suggestions": self.suggestions,
            "has_issues": self.has_issues(),
        })
    }
}

// NEW: Return results instead of printing
pub async fn run_with_result(_args: &DoctorArgs) -> Result<DiagnosticResults> {
    let mut results = DiagnosticResults::new();

    check_environment(&mut results).await?;
    check_configuration(&mut results).await?;
    check_ecosystem(&mut results).await?;

    Ok(results)
}
```

**MCP Tool Wrapper**:

```rust
async fn mcp_doctor_diagnose() -> Result<serde_json::Value> {
    let results = doctor::run_with_result(&DoctorArgs {}).await?;
    Ok(results.to_json())
}
```

---

### 2.6 Init Command (Project Initialization)

**File**: `/crates/x402-cli/src/commands/init.rs`

#### Execution Path

```
CLI: x402-dev init
  ↓
init::run(&args)
  ↓
1. Check if .x402dev.yaml exists → Confirm overwrite
2. Prompt for configuration:
   - Port (default 8402)
   - Solana network (devnet/testnet/mainnet)
   - Log level (error/warn/info/debug/trace)
3. Create Config struct
4. Validate configuration
5. Serialize to YAML
6. Write to .x402dev.yaml
7. Print success message with next steps
```

#### Dependencies

```rust
use dialoguer::{Confirm, Input, Select};  // Interactive prompts
use serde;  // Serialization
use crate::config::{Config, LogLevel, PricingConfig, SimulationMode};
```

#### Core Logic

All initialization logic is **inline in init.rs** - creates config interactively

#### Parameters

```rust
pub struct InitArgs {
    // No arguments yet (reserved for --defaults flag)
}
```

#### Side Effects

- **File I/O**: Writes `.x402dev.yaml` to current directory
- **Interactive**: Uses stdin for prompts (dialoguer)
- **Process**: Returns Ok() or Err()

#### Return Types

```rust
pub async fn run(_args: &InitArgs) -> Result<()>
```

#### **MCP Integration Assessment**

| Aspect | Status | Notes |
|--------|--------|-------|
| **Reusability** | ⚠️ LOW | Interactive prompts require user input |
| **Wrappability** | ⚠️ DIFFICULT | stdin coupling |
| **CLI Coupling** | ❌ HIGH | Requires terminal interaction |
| **Refactoring Needed** | ✅ MAJOR | Create non-interactive variant |

**Recommended Refactoring**:

```rust
// NEW: Non-interactive variant for MCP
pub async fn create_config_file(
    port: Option<u16>,
    solana_network: Option<&str>,
    log_level: Option<LogLevel>,
    overwrite: bool,
) -> Result<PathBuf> {
    let config_path = PathBuf::from(".x402dev.yaml");

    // Check if exists and overwrite flag
    if config_path.exists() && !overwrite {
        anyhow::bail!("Configuration file already exists");
    }

    // Use provided values or defaults
    let config = Config {
        port: port.unwrap_or(8402),
        solana_rpc: match solana_network.unwrap_or("devnet") {
            "devnet" => "https://api.devnet.solana.com",
            "testnet" => "https://api.testnet.solana.com",
            "mainnet-beta" => "https://api.mainnet-beta.solana.com",
            _ => anyhow::bail!("Invalid network"),
        }.to_string(),
        log_level: log_level.unwrap_or(LogLevel::Info),
        pricing: PricingConfig::default(),
        simulation_mode: SimulationMode::default(),
        timeout_delay_ms: 5000,
    };

    config.validate()?;

    let yaml = serde_yaml::to_string(&config)?;
    std::fs::write(&config_path, yaml)?;

    Ok(config_path)
}

// EXISTING: Keep for CLI interactive mode
pub async fn run(_args: &InitArgs) -> Result<()> {
    // ... interactive prompts ...
    // Call create_config_file() at the end
}
```

**MCP Tool Wrapper**:

```rust
async fn mcp_init_project(
    port: Option<u16>,
    network: Option<String>,
    overwrite: bool,
) -> Result<String> {
    let config_path = init::create_config_file(
        port,
        network.as_deref(),
        None,
        overwrite,
    ).await?;

    Ok(format!("Configuration created: {}", config_path.display()))
}
```

---

### 2.7 Config Command (Configuration Management)

**File**: `/crates/x402-cli/src/commands/config.rs`

#### Execution Path

```
CLI: x402-dev config show --port 8888
  ↓
config::run(&args)
  ↓
match args.command:
  - Show → show_config(args)
    ↓
    1. Build CliOverrides from global flags
    2. load_merged_config_with_sources()
    3. Print configuration with sources
       - "CLI flags > Env > Project > Global > Defaults"
```

#### Dependencies

```rust
use crate::config::{load_merged_config_with_sources, CliOverrides};
```

#### Core Logic Location

**Configuration system**: `/crates/x402-cli/src/config.rs`
- Multi-tier config loading (CLI > Env > Project > Global > Defaults)
- Source tracking for display
- Validation

#### Parameters

```rust
pub struct ConfigArgs {
    pub command: ConfigCommands,  // Currently only "Show"
    pub port: Option<u16>,        // Override for show
    pub solana_rpc: Option<String>,
    pub log_level: Option<LogLevel>,
}

pub enum ConfigCommands {
    Show,
}
```

#### Side Effects

- **File I/O**: Reads `~/.x402dev/config.yaml`, `./.x402dev.yaml`
- **Environment**: Reads env vars (X402_DEV_PORT, etc.)
- **Process**: Returns Ok()

#### Return Types

```rust
pub async fn run(args: &ConfigArgs) -> Result<()>
async fn show_config(args: &ConfigArgs) -> Result<()>
```

#### **MCP Integration Assessment**

| Aspect | Status | Notes |
|--------|--------|-------|
| **Reusability** | ✅ EXCELLENT | Core config system reusable |
| **Wrappability** | ✅ EXCELLENT | Returns structured data |
| **CLI Coupling** | ✅ MINIMAL | Only stdout printing |
| **Refactoring Needed** | ❌ NONE | Perfect as-is |

**MCP Tool Wrapper** (No Changes Needed!):

```rust
async fn mcp_config_show() -> Result<serde_json::Value> {
    let config_with_sources = load_merged_config_with_sources(None)?;

    Ok(serde_json::json!({
        "config": {
            "port": config_with_sources.config.port,
            "solana_rpc": config_with_sources.config.solana_rpc,
            "log_level": config_with_sources.config.log_level.to_string(),
        },
        "sources": {
            "port": config_with_sources.port_source,
            "solana_rpc": config_with_sources.solana_rpc_source,
            "log_level": config_with_sources.log_level_source,
        },
        "priority": "CLI flags > Env vars > Project config > Global config > Defaults",
    }))
}
```

---

### 2.8 Examples Command (Example Project Templates)

**File**: `/crates/x402-cli/src/commands/examples.rs`

#### Execution Path

```
CLI: x402-dev examples init mcp-server-starter
  ↓
examples::run(&args)
  ↓
match args.command:
  - "list" → list_examples()
  - "info" → show_info(name)
  - "init" → init_example(name)
    ↓
    1. Find example in EXAMPLES catalog
    2. get_examples_dir() → Find examples/ directory
    3. check_conflicts() → Warn about overwrites
    4. copy_example_files() → Copy template files
    5. Print next steps
```

#### Dependencies

```rust
use std::fs;   // File operations
use std::path::{Path, PathBuf};
use colored;   // Terminal colors
```

**No external dependencies** - pure stdlib!

#### Core Logic

All logic is **inline in examples.rs**:

```rust
const EXAMPLES: &[ExampleInfo] = &[
    ExampleInfo {
        name: "mcp-server-starter",
        description: "Basic MCP server with x402 payments",
        complexity: "~50 lines",
        files: &["server.rs", "README.md", ".x402dev.yaml", "Cargo.toml"],
        prerequisites: &["Rust 1.75+", "x402-dev mock server"],
    },
    // ... more examples
];

fn copy_example_files(source: &Path, dest: &Path) -> Result<Vec<String>> {
    // Recursive directory copy
}
```

#### Parameters

```rust
pub struct ExamplesArgs {
    pub command: Option<String>,  // "list", "info", "init"
    pub name: Option<String>,     // Example name
}
```

#### Side Effects

- **File I/O**: Copies example files to current directory
- **Process**: Returns Ok() or Err()

#### Return Types

```rust
pub async fn run(args: &ExamplesArgs) -> Result<()>
fn list_examples() -> Result<()>
fn show_info(name: &str) -> Result<()>
async fn init_example(name: &str) -> Result<()>
```

#### **MCP Integration Assessment**

| Aspect | Status | Notes |
|--------|--------|-------|
| **Reusability** | ✅ HIGH | File operations easily reusable |
| **Wrappability** | ✅ GOOD | Minor refactoring for structure |
| **CLI Coupling** | ⚠️ MODERATE | stdout printing |
| **Refactoring Needed** | ✅ MINOR | Extract ExampleInfo access |

**MCP Tool Wrapper**:

```rust
async fn mcp_examples_list() -> Result<Vec<serde_json::Value>> {
    Ok(EXAMPLES.iter().map(|ex| {
        serde_json::json!({
            "name": ex.name,
            "description": ex.description,
            "complexity": ex.complexity,
            "files": ex.files,
            "prerequisites": ex.prerequisites,
        })
    }).collect())
}

async fn mcp_examples_init(name: String) -> Result<Vec<String>> {
    let example = EXAMPLES.iter()
        .find(|e| e.name == name)
        .ok_or_else(|| anyhow::anyhow!("Example '{}' not found", name))?;

    let examples_dir = get_examples_dir()?;
    let source_path = examples_dir.join(name);
    let dest_path = PathBuf::from(".");

    let copied_files = copy_example_files(&source_path, &dest_path)?;
    Ok(copied_files)
}
```

---

### 2.9 Version Command (Version Info & Updates)

**File**: `/crates/x402-cli/src/commands/version.rs`

#### Execution Path

```
CLI: x402-dev version
  ↓
version::run(&args)
  ↓
1. Print version: env!("CARGO_PKG_VERSION")
2. Print platform: OS, ARCH
3. If !args.no_update_check:
   - check_for_updates() (async)
     ↓
     a. Check cache (~/.x402dev/update-check.json)
     b. If stale, fetch from crates.io API
     c. Compare versions (semver)
     d. Update cache
     e. Print update notification if available
```

#### Dependencies

```rust
use reqwest;  // HTTP client for crates.io API
use semver;   // Version comparison
use serde;    // Cache serialization
use directories;  // Home directory
```

#### Core Logic

All logic is **inline in version.rs**:

```rust
async fn fetch_latest_version() -> Result<String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let response = client
        .get("https://crates.io/api/v1/crates/x402-dev")
        .send()
        .await?;

    let crate_info: CratesIoResponse = response.json().await?;
    Ok(crate_info.crate_info.max_version)
}

fn is_newer_version(latest: &str, current: &str) -> bool {
    match (Version::parse(latest), Version::parse(current)) {
        (Ok(latest_ver), Ok(current_ver)) => latest_ver > current_ver,
        _ => false,
    }
}
```

#### Parameters

```rust
pub struct VersionArgs {
    pub no_update_check: bool,  // Skip update check
}
```

#### Side Effects

- **File I/O**: Writes cache to `~/.x402dev/update-check.json`
- **Network**: HTTP GET to crates.io API (optional)
- **Process**: Returns Ok()

#### Return Types

```rust
pub async fn run(args: &VersionArgs) -> Result<()>
async fn check_for_updates() -> Result<()>
```

#### **MCP Integration Assessment**

| Aspect | Status | Notes |
|--------|--------|-------|
| **Reusability** | ✅ HIGH | Version logic easily extracted |
| **Wrappability** | ✅ EXCELLENT | Simple data structure |
| **CLI Coupling** | ✅ MINIMAL | Only stdout printing |
| **Refactoring Needed** | ❌ NONE | Works as-is |

**MCP Tool Wrapper**:

```rust
async fn mcp_version_info() -> Result<serde_json::Value> {
    let current_version = env!("CARGO_PKG_VERSION");

    // Try to get latest version (don't fail if network error)
    let latest_version = match fetch_latest_version().await {
        Ok(v) => Some(v),
        Err(_) => None,
    };

    let update_available = if let Some(ref latest) = latest_version {
        is_newer_version(latest, current_version)
    } else {
        false
    };

    Ok(serde_json::json!({
        "current": current_version,
        "latest": latest_version,
        "update_available": update_available,
        "platform": {
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
        }
    }))
}
```

---

## 3. Configuration Management Deep Dive

### 3.1 Multi-Tier Configuration System

**File**: `/crates/x402-cli/src/config.rs` (847 lines!)

#### Configuration Priority

```
CLI flags (highest)
  ↓
Environment variables
  ↓
Project config (.x402dev.yaml)
  ↓
Global config (~/.x402dev/config.yaml)
  ↓
Defaults (lowest)
```

#### Config Structure

```rust
pub struct Config {
    pub port: u16,                      // Server port
    pub solana_rpc: String,             // Solana RPC URL
    pub log_level: LogLevel,            // Logging verbosity
    pub pricing: PricingConfig,         // Pricing rules
    pub simulation_mode: SimulationMode, // Success/Failure/Timeout
    pub timeout_delay_ms: u64,          // Timeout simulation delay
}

pub struct PricingConfig {
    pub default: f64,                   // Default pricing (SOL/USDC)
    pub per_resource: HashMap<String, f64>, // Path-specific pricing
}

pub enum LogLevel {
    Error, Warn, Info, Debug, Trace
}

pub enum SimulationMode {
    Success, Failure, Timeout
}
```

#### Loading Flow

```rust
pub fn load_merged_config(cli_overrides: Option<&CliOverrides>) -> Result<Config> {
    let mut config = Config::default();

    // Step 1: Apply global config
    if let Some(global) = load_global_config()? {
        config.merge(global);
    }

    // Step 2: Apply project config
    if let Some(project) = load_project_config()? {
        config.merge(project);
    }

    // Step 3: Apply environment variables
    config.merge_env()?;

    // Step 4: Apply CLI flags
    if let Some(cli) = cli_overrides {
        config.merge_cli(cli);
    }

    // Step 5: Validate
    config.validate()?;

    Ok(config)
}
```

#### Validation

```rust
impl Config {
    pub fn validate(&self) -> Result<()> {
        // Port range check
        if !(1024..=65535).contains(&self.port) {
            anyhow::bail!("Port must be between 1024 and 65535");
        }

        // Solana RPC URL format
        if !self.solana_rpc.starts_with("http://")
            && !self.solana_rpc.starts_with("https://") {
            anyhow::bail!("Invalid Solana RPC URL");
        }

        // Pricing validation
        self.pricing.validate()?;

        // Timeout delay range
        if self.timeout_delay_ms < 100 || self.timeout_delay_ms > 60000 {
            anyhow::bail!("Timeout delay must be 100-60000ms");
        }

        Ok(())
    }
}
```

#### Pricing Matcher

```rust
pub struct PricingMatcher {
    config: PricingConfig,
}

impl PricingMatcher {
    pub fn get_price_for_path(&self, path: &str) -> f64 {
        // Priority 1: Exact match
        if let Some(&amount) = self.config.per_resource.get(path) {
            return amount;
        }

        // Priority 2: Wildcard prefix match (longest wins)
        let mut matches: Vec<(&str, f64)> = Vec::new();
        for (pattern, &amount) in &self.config.per_resource {
            if pattern.ends_with("/*") {
                let prefix = &pattern[..pattern.len() - 2];
                if path.starts_with(prefix) {
                    matches.push((prefix, amount));
                }
            }
        }

        if !matches.is_empty() {
            matches.sort_by_key(|(prefix, _)| std::cmp::Reverse(prefix.len()));
            return matches[0].1;
        }

        // Priority 3: Default
        self.config.default
    }
}
```

### 3.2 MCP Integration for Config

**Config management is ALREADY library-friendly!**

No refactoring needed - just use the existing functions:

```rust
// In MCP server
use x402_cli::config::{load_merged_config, CliOverrides, Config};

async fn mcp_config_load(
    port: Option<u16>,
    solana_rpc: Option<String>,
) -> Result<Config> {
    let cli_overrides = CliOverrides {
        port,
        solana_rpc,
        log_level: None,
        pricing: None,
    };

    let config = load_merged_config(Some(&cli_overrides))?;
    Ok(config)
}
```

---

## 4. Error Handling Patterns

### 4.1 Error Types

**File**: `/crates/x402-cli/src/errors.rs`

```rust
pub enum CliError {
    Config {
        message: String,
        suggestion: Option<String>,
        code: &'static str,
    },
    Network {
        message: String,
        suggestion: Option<String>,
        code: &'static str,
    },
    Validation {
        message: String,
        suggestion: Option<String>,
        code: &'static str,
    },
    Io {
        message: String,
        source: std::io::Error,
    },
    Other {
        message: String,
    },
}
```

### 4.2 Exit Codes

```rust
pub const EXIT_SUCCESS: i32 = 0;  // All checks passed
pub const EXIT_GENERAL: i32 = 1;  // General errors
pub const EXIT_CONFIG: i32 = 2;   // Configuration errors
pub const EXIT_NETWORK: i32 = 3;  // Network errors
```

### 4.3 Error Conversion

```rust
pub fn convert_anyhow_to_cli_error(error: anyhow::Error) -> CliError {
    // Try to downcast to known error types
    if let Some(io_err) = error.downcast_ref::<std::io::Error>() {
        return CliError::io("I/O operation failed", io_err.kind().into());
    }

    // Default to Other variant
    CliError::other(error.to_string())
}
```

### 4.4 MCP Error Handling

**For MCP integration, convert CliError to JSON**:

```rust
impl CliError {
    pub fn to_json(&self) -> serde_json::Value {
        let (error_type, message, suggestion) = match self {
            CliError::Config { message, suggestion, .. } => {
                ("config", message, suggestion)
            }
            CliError::Network { message, suggestion, .. } => {
                ("network", message, suggestion)
            }
            CliError::Validation { message, suggestion, .. } => {
                ("validation", message, suggestion)
            }
            CliError::Io { message, source } => {
                ("io", message, &Some(source.to_string()))
            }
            CliError::Other { message } => {
                ("other", message, &None)
            }
        };

        serde_json::json!({
            "error_type": error_type,
            "message": message,
            "suggestion": suggestion,
            "exit_code": self.exit_code(),
        })
    }
}
```

---

## 5. Type System Analysis

### 5.1 Domain Types

**File**: `/crates/x402-domain/src/lib.rs`

```rust
// Validated newtypes (NEW in codebase)
pub use types::{
    AgentId,       // Validated agent identifier
    PolicyId,      // Validated policy identifier
    InvoiceMemo,   // Validated invoice memo (req-UUID)
    SolanaAddress, // Base58-validated Solana address
    ResourcePath,  // Validated resource path
    Port,          // Validated port (1024-65535)
};

// Financial types (uses Decimal, NOT f64!)
pub use amount::{Amount, Currency};

// Re-export rust_decimal::Decimal
pub use rust_decimal::Decimal;
```

**Purpose**: Type-safe alternatives to primitive obsession (String, f64, u16)

### 5.2 Usage in Commands

**Current state**: Commands still use primitives (String, f64)
**Future state**: Should migrate to domain types

**Example Migration**:

```rust
// BEFORE (current)
pub struct MockArgs {
    pub port: u16,          // Primitive
    pub pricing: Option<f64>,  // Floating-point!
}

// AFTER (recommended)
use x402_domain::{Port, Amount};

pub struct MockArgs {
    pub port: Port,            // Validated type
    pub pricing: Option<Amount>,  // Decimal precision
}
```

### 5.3 MCP Integration with Domain Types

```rust
// MCP server should use domain types internally
use x402_domain::{Port, Amount, SolanaAddress};

async fn mcp_mock_start(
    port_num: u16,
    pricing_str: Option<String>,
) -> Result<String> {
    // Validate at MCP boundary
    let port = Port::new(port_num)?;
    let pricing = if let Some(p) = pricing_str {
        Some(Amount::from_decimal_str(&p)?)
    } else {
        None
    };

    // Use validated types internally
    // ...
}
```

---

## 6. Reusability Assessment

### 6.1 Reusability Matrix

| Command | Core Logic Location | Reusable? | Wrapping Complexity | Notes |
|---------|---------------------|-----------|---------------------|-------|
| **mock** | x402-server crate | ✅ YES | ⭐ TRIVIAL | Functions already take args |
| **test** | x402-core/testing | ✅ YES | ⭐ EASY | Remove exit(), return result |
| **check** | commands/check.rs | ✅ YES | ⭐ EASY | Extract validation logic |
| **policy** | x402-core/policy | ✅ YES | ⭐ TRIVIAL | Already library functions |
| **doctor** | commands/doctor.rs | ✅ YES | ⭐ EASY | Return DiagnosticResults |
| **init** | commands/init.rs | ⚠️ PARTIAL | ⭐⭐ MODERATE | Need non-interactive variant |
| **config** | config.rs | ✅ YES | ⭐ TRIVIAL | Already library functions |
| **examples** | commands/examples.rs | ✅ YES | ⭐ EASY | Extract file operations |
| **version** | commands/version.rs | ✅ YES | ⭐ EASY | Return version struct |

**Legend**:
- ⭐ TRIVIAL: No changes needed
- ⭐ EASY: Minor refactoring (<20 lines)
- ⭐⭐ MODERATE: Significant refactoring (50-100 lines)

### 6.2 Shared Code Opportunities

**Code that SHOULD be extracted to library**:

1. **Configuration System** (config.rs) - ✅ Already reusable
2. **Error Handling** (errors.rs) - ⚠️ Add JSON serialization
3. **Invoice Generation** (commands/invoice.rs) - ⚠️ Move to x402-core
4. **Protocol Validation** (check.rs parsing) - ⚠️ Move to x402-core
5. **Diagnostic Checks** (doctor.rs) - ⚠️ Extract DiagnosticResults API

**Code that MUST be duplicated for MCP**:

1. **Interactive prompts** (init.rs dialoguer usage) - Cannot be shared
2. **Terminal colors** (colored crate usage) - Not needed in MCP
3. **Process management** (std::process::exit) - Different for MCP

---

## 7. Integration Points for MCP Server

### 7.1 Architecture Options

**Option A: Direct Library Calls** (RECOMMENDED)

```
MCP Server
  ↓
Import x402-cli functions
  ↓
Call commands directly
  ↓
Convert results to JSON
```

**Pros**:
- ✅ No code duplication
- ✅ Automatic updates when CLI changes
- ✅ Shared configuration system
- ✅ Consistent behavior

**Cons**:
- ⚠️ Some functions need minor refactoring (remove exit())
- ⚠️ Dependency on x402-cli crate

---

**Option B: Shared Core Library** (FUTURE)

```
x402-core (expanded)
    ↓                ↓
x402-cli         MCP Server
```

**Pros**:
- ✅ Clean separation
- ✅ No CLI coupling

**Cons**:
- ❌ Major refactoring required
- ❌ Code duplication during migration

**Recommendation**: Start with Option A, migrate to Option B later

---

### 7.2 MCP Server Entry Points

**Proposed MCP tool structure**:

```rust
// crates/x402-mcp/src/tools/
mod mock;      // Server lifecycle
mod test;      // Test execution
mod check;     // Compliance checking
mod policy;    // Policy management
mod doctor;    // Diagnostics
mod config;    // Configuration
mod version;   // Version info
mod examples;  // Template management
```

**Each tool module wraps CLI command**:

```rust
// crates/x402-mcp/src/tools/mock.rs
use x402_cli::commands::mock;
use x402_cli::cli::MockArgs;

pub async fn start_server(
    port: u16,
    pricing: Option<f64>,
) -> Result<serde_json::Value> {
    let args = MockArgs {
        port,
        pricing,
        command: None,
    };

    mock::run(&args).await?;

    Ok(serde_json::json!({
        "status": "started",
        "port": port,
    }))
}
```

### 7.3 Dependency Graph

```
x402-mcp (NEW MCP SERVER)
  ↓
x402-cli (Existing CLI)
  ↓             ↓
x402-server   x402-core
  ↓             ↓
x402-domain   x402-domain
```

**MCP Cargo.toml**:

```toml
[dependencies]
x402-cli = { path = "../x402-cli" }
x402-server = { path = "../x402-server" }
x402-core = { path = "../x402-core" }
x402-domain = { path = "../x402-domain" }

# MCP protocol
mcp-server = "0.1"  # Or equivalent MCP SDK
serde_json = { workspace = true }
tokio = { workspace = true }
```

---

## 8. Integration Complexity Assessment

### 8.1 Per-Command Complexity

| Command | Changes Needed | Estimated Effort | Risk Level |
|---------|----------------|------------------|------------|
| **mock** | None | 1 hour | 🟢 LOW |
| **test** | Remove exit() | 2 hours | 🟢 LOW |
| **check** | Extract result type | 3 hours | 🟢 LOW |
| **policy** | None | 1 hour | 🟢 LOW |
| **doctor** | Return struct | 2 hours | 🟢 LOW |
| **init** | Non-interactive variant | 4 hours | 🟡 MEDIUM |
| **config** | None | 1 hour | 🟢 LOW |
| **examples** | Extract logic | 2 hours | 🟢 LOW |
| **version** | Return struct | 1 hour | 🟢 LOW |

**Total Estimated Effort**: 17 hours (~2-3 days)

### 8.2 Overall Assessment

**Integration Complexity**: 🟢 **LOW to MEDIUM**

**Reasons**:
1. ✅ Commands already use structured arguments
2. ✅ Most logic is in library crates (x402-server, x402-core)
3. ✅ Clean async/await throughout
4. ✅ Minimal CLI coupling
5. ⚠️ Minor refactoring needed for some commands

**Blockers**: NONE - All issues have clear solutions

---

## 9. Recommended Refactoring

### 9.1 High Priority Changes (Do First)

#### A. Remove Direct exit() Calls

**Files to change**:
- `commands/test.rs` - Line 60
- `commands/check.rs` - Lines 181, 199, 263

**Pattern**:

```rust
// BEFORE
pub async fn run(args: &TestArgs) -> Result<()> {
    let result = execute_test_suite(&suite).await?;
    println!("{}", format_summary(&result, args.quiet));
    std::process::exit(result.exit_code());  // ❌ Blocks reuse
}

// AFTER
pub async fn run_with_result(args: &TestArgs) -> Result<TestResult> {
    let result = execute_test_suite(&suite).await?;
    Ok(result)  // ✅ Returns result
}

pub async fn run(args: &TestArgs) -> Result<()> {
    let result = run_with_result(args).await?;
    println!("{}", format_summary(&result, args.quiet));
    std::process::exit(result.exit_code());
}
```

---

#### B. Extract Result Types

**Files to create**:
- `commands/test_result.rs`
- `commands/check_result.rs`
- `commands/doctor_result.rs`

**Pattern**:

```rust
// commands/check_result.rs
#[derive(Debug, Serialize)]
pub struct CheckResult {
    pub url: String,
    pub compliant: bool,
    pub checks_passed: usize,
    pub checks_total: usize,
    pub details: CheckDetails,
}

#[derive(Debug, Serialize)]
pub struct CheckDetails {
    pub status_code_402: bool,
    pub www_authenticate_header: bool,
    pub invoice_validation: Vec<ValidationResult>,
}
```

---

#### C. Add Non-Interactive Init

**File**: `commands/init.rs`

```rust
// NEW: Non-interactive API
pub async fn create_config_file(
    config: Config,
    overwrite: bool,
) -> Result<PathBuf> {
    // ... implementation
}

// EXISTING: Keep interactive mode
pub async fn run(args: &InitArgs) -> Result<()> {
    // Interactive prompts
    let config = Config { /* ... */ };
    create_config_file(config, false).await?;
    Ok(())
}
```

---

### 9.2 Low Priority Enhancements (Do Later)

#### D. Migrate to Domain Types

**Current**: Commands use primitives (String, u16, f64)
**Future**: Use validated types (Port, Amount, SolanaAddress)

**Benefits**:
- ✅ Compile-time validation
- ✅ No floating-point errors (Amount uses Decimal)
- ✅ Better type safety

**Effort**: 8-10 hours (affects all commands)

---

#### E. Move Invoice Logic to x402-core

**Current**: Invoice code is duplicated in:
- `commands/invoice.rs`
- `x402-server/src/server.rs`

**Future**: Single source in `x402-core/src/invoice.rs`

**Benefits**:
- ✅ No duplication
- ✅ Shared across CLI, server, MCP

**Effort**: 2-3 hours

---

## 10. MCP Tool Implementation Examples

### 10.1 Complete Mock Server Tool

```rust
// crates/x402-mcp/src/tools/mock.rs

use x402_cli::commands::mock;
use x402_cli::cli::MockArgs;
use x402_server::{Config, MockServerConfig, PricingMatcher, InvoiceGenerator};
use x402_cli::config::{load_merged_config, CliOverrides};
use serde_json::Value;
use anyhow::Result;

/// Start the mock facilitator server
pub async fn start(port: u16, pricing: Option<f64>) -> Result<Value> {
    let args = MockArgs {
        port,
        pricing,
        command: None,
    };

    // Build server config (reuses CLI logic)
    let cli_overrides = CliOverrides {
        port: None,
        solana_rpc: None,
        log_level: None,
        pricing,
    };

    let config = load_merged_config(Some(&cli_overrides))?;

    // Start server (spawns background task in MCP)
    let server_config = MockServerConfig {
        port,
        pricing_matcher: PricingMatcher::new(config.pricing.clone()),
        invoice_generator: InvoiceGenerator::new(),
        config: Config {
            port,
            solana_rpc: config.solana_rpc.clone(),
            log_level: config.log_level.to_string(),
            pricing: config.pricing,
            simulation_mode: config.simulation_mode,
            timeout_delay_ms: config.timeout_delay_ms,
        },
    };

    // Spawn server in background
    tokio::spawn(async move {
        let _ = x402_server::start_server(server_config).await;
    });

    Ok(serde_json::json!({
        "status": "started",
        "port": port,
        "pricing": pricing.unwrap_or(0.01),
    }))
}

/// Stop the mock server
pub async fn stop() -> Result<Value> {
    x402_server::stop_server().await?;

    Ok(serde_json::json!({
        "status": "stopped",
    }))
}

/// Get server status
pub async fn status() -> Result<Value> {
    match x402_server::server_status().await {
        Ok(_) => Ok(serde_json::json!({
            "running": true,
        })),
        Err(_) => Ok(serde_json::json!({
            "running": false,
        })),
    }
}
```

---

### 10.2 Complete Test Runner Tool

```rust
// crates/x402-mcp/src/tools/test.rs

use x402_cli::commands::test;
use x402_cli::cli::TestArgs;
use x402_core::testing::{TestSuite, execute_test_suite};
use serde_json::Value;
use anyhow::Result;
use std::path::PathBuf;

/// Execute a test suite
pub async fn run_suite(suite_path: String) -> Result<Value> {
    // Load test suite
    let path = PathBuf::from(suite_path);
    let suite = TestSuite::from_file(&path)?;

    // Execute tests
    let result = execute_test_suite(&suite).await?;

    // Return structured result (not stdout string!)
    Ok(serde_json::json!({
        "suite": path.display().to_string(),
        "passed": result.passed_tests,
        "failed": result.failed_tests,
        "total": result.total_tests,
        "duration_ms": result.duration_ms,
        "success": result.exit_code() == 0,
        "tests": result.test_results.iter().map(|t| {
            serde_json::json!({
                "name": t.name,
                "passed": t.passed,
                "duration_ms": t.duration_ms,
                "error": t.error,
            })
        }).collect::<Vec<_>>(),
    }))
}
```

---

### 10.3 Complete Policy Tool

```rust
// crates/x402-mcp/src/tools/policy.rs

use x402_core::policy::{
    validate_policies,
    codegen::{generate_express_middleware, generate_fastify_plugin},
    types::PolicyConfig,
    rules::PolicyFile,
};
use serde_json::Value;
use anyhow::Result;

/// Validate a policy file
pub async fn validate(file_path: String) -> Result<Value> {
    let content = std::fs::read_to_string(&file_path)?;
    let policy_file: PolicyFile = serde_yaml::from_str(&content)?;

    let policy_config = PolicyConfig {
        policies: policy_file.policies.clone(),
    };

    let report = validate_policies(&policy_config);

    Ok(serde_json::json!({
        "valid": !report.has_errors,
        "has_warnings": report.has_warnings,
        "issue_count": report.issues.len(),
        "issues": report.issues.iter().map(|issue| {
            serde_json::json!({
                "type": format!("{:?}", issue.issue_type),
                "message": issue.message,
                "details": issue.details,
                "policy_indices": issue.policy_indices,
                "suggestions": issue.suggestions.iter().map(|s| {
                    serde_json::json!({
                        "description": s.description,
                        "action": s.action,
                    })
                }).collect::<Vec<_>>(),
            })
        }).collect::<Vec<_>>(),
    }))
}

/// Generate middleware code
pub async fn generate(
    file_path: String,
    framework: String,
) -> Result<Value> {
    let content = std::fs::read_to_string(&file_path)?;
    let policy_file: PolicyFile = serde_yaml::from_str(&content)?;

    // Validate first
    let policy_config = PolicyConfig {
        policies: policy_file.policies.clone(),
    };
    let report = validate_policies(&policy_config);

    if report.has_errors {
        anyhow::bail!("Policy validation failed");
    }

    // Generate code
    let code = match framework.as_str() {
        "express" => generate_express_middleware(&policy_file, &file_path),
        "fastify" => generate_fastify_plugin(&policy_file.policies, Some(&file_path)),
        _ => anyhow::bail!("Invalid framework: {}", framework),
    };

    Ok(serde_json::json!({
        "framework": framework,
        "code": code,
        "lines": code.lines().count(),
        "size_bytes": code.len(),
    }))
}
```

---

## 11. Testing Considerations

### 11.1 Unit Testing MCP Tools

**Strategy**: Test MCP tools independently

```rust
// tests/mcp_mock_tests.rs
use x402_mcp::tools::mock;

#[tokio::test]
async fn test_mock_start() {
    let result = mock::start(3402, None).await.unwrap();
    assert_eq!(result["status"], "started");
    assert_eq!(result["port"], 3402);

    // Cleanup
    mock::stop().await.unwrap();
}

#[tokio::test]
async fn test_mock_status() {
    // Start server
    mock::start(3403, None).await.unwrap();

    // Check status
    let status = mock::status().await.unwrap();
    assert_eq!(status["running"], true);

    // Cleanup
    mock::stop().await.unwrap();
}
```

### 11.2 Integration Testing

**Strategy**: Test full MCP protocol flow

```rust
// tests/mcp_integration_tests.rs
use x402_mcp::server::McpServer;

#[tokio::test]
async fn test_full_workflow() {
    let mcp = McpServer::new();

    // 1. Start mock server
    let start_result = mcp.call_tool("mock_start", json!({
        "port": 3404,
    })).await.unwrap();
    assert_eq!(start_result["status"], "started");

    // 2. Check compliance
    let check_result = mcp.call_tool("check_compliance", json!({
        "url": "http://localhost:3404/api/test",
    })).await.unwrap();
    assert_eq!(check_result["compliant"], true);

    // 3. Stop server
    mcp.call_tool("mock_stop", json!({})).await.unwrap();
}
```

### 11.3 CLI Compatibility Tests

**Strategy**: Ensure MCP tools produce same results as CLI

```rust
// tests/cli_compatibility_tests.rs

#[tokio::test]
async fn test_policy_validate_compatibility() {
    // Run via CLI
    let cli_result = std::process::Command::new("x402-dev")
        .arg("policy")
        .arg("validate")
        .arg("tests/fixtures/policy.yaml")
        .arg("--json")
        .output()
        .unwrap();

    let cli_json: Value = serde_json::from_slice(&cli_result.stdout).unwrap();

    // Run via MCP
    let mcp_json = x402_mcp::tools::policy::validate(
        "tests/fixtures/policy.yaml".to_string()
    ).await.unwrap();

    // Compare results
    assert_eq!(cli_json["valid"], mcp_json["valid"]);
    assert_eq!(cli_json["has_warnings"], mcp_json["has_warnings"]);
}
```

---

## 12. Execution Flow Diagrams

### 12.1 Mock Server Lifecycle

```
User Input (MCP client)
  ↓
MCP Server receives "mock_start" tool call
  ↓
Parse JSON parameters (port, pricing)
  ↓
Call x402_mcp::tools::mock::start()
  ↓
Create MockArgs struct
  ↓
Call load_merged_config() [from x402-cli]
  ↓
Build MockServerConfig
  ↓
Call x402_server::start_server() [async]
  ↓
Spawn HTTP server in background
  ↓
Write PID file
  ↓
Return success JSON to MCP client
```

### 12.2 Test Execution Flow

```
MCP client → "test_run_suite" tool
  ↓
Parse suite_path parameter
  ↓
TestSuite::from_file() [x402-core]
  ↓
Parse YAML test definitions
  ↓
execute_test_suite() [x402-core]
  ↓
For each test:
  - Make HTTP request
  - Check assertions
  - Record result
  ↓
Aggregate results
  ↓
Convert to JSON (NOT stdout!)
  ↓
Return structured result to MCP client
```

### 12.3 Policy Validation Flow

```
MCP client → "policy_validate" tool
  ↓
Read policy YAML file
  ↓
Parse into PolicyFile [x402-core]
  ↓
Convert to PolicyConfig
  ↓
validate_policies() [x402-core]
  ↓
Check for conflicts, validate rules
  ↓
Generate ValidationReport
  ↓
Convert issues to JSON
  ↓
Return structured result to MCP client
```

---

## 13. Code Examples for MCP Integration

### 13.1 MCP Server Main Entry Point

```rust
// crates/x402-mcp/src/main.rs

use mcp_server::{McpServer, Tool, ToolHandler};
use serde_json::Value;
use anyhow::Result;

mod tools;

#[tokio::main]
async fn main() -> Result<()> {
    let server = McpServer::new("x402-mcp-server", "1.0.0");

    // Register tools
    server.register_tool(Tool {
        name: "mock_start",
        description: "Start x402 mock facilitator server",
        parameters: /* ... */,
        handler: tools::mock::start,
    });

    server.register_tool(Tool {
        name: "test_run_suite",
        description: "Execute x402 test suite",
        parameters: /* ... */,
        handler: tools::test::run_suite,
    });

    // More tools...

    server.listen().await
}
```

### 13.2 Tool Registration Pattern

```rust
// crates/x402-mcp/src/tools/mod.rs

pub mod mock;
pub mod test;
pub mod check;
pub mod policy;
pub mod doctor;
pub mod config;
pub mod version;
pub mod examples;

use mcp_server::{Tool, ParameterSchema};

/// Get all x402 MCP tools
pub fn get_all_tools() -> Vec<Tool> {
    vec![
        // Mock server tools
        Tool::new(
            "mock_start",
            "Start x402 mock facilitator server",
            ParameterSchema::object()
                .property("port", ParameterSchema::number().default(3402))
                .property("pricing", ParameterSchema::number().optional()),
            mock::start,
        ),
        Tool::new(
            "mock_stop",
            "Stop x402 mock facilitator server",
            ParameterSchema::object(),
            mock::stop,
        ),
        Tool::new(
            "mock_status",
            "Get mock server status",
            ParameterSchema::object(),
            mock::status,
        ),

        // Test tools
        Tool::new(
            "test_run_suite",
            "Execute x402 test suite from YAML file",
            ParameterSchema::object()
                .property("suite_path", ParameterSchema::string().required()),
            test::run_suite,
        ),

        // Check tools
        Tool::new(
            "check_compliance",
            "Check x402 protocol compliance of an endpoint",
            ParameterSchema::object()
                .property("url", ParameterSchema::string().required())
                .property("format", ParameterSchema::string().default("json")),
            check::compliance,
        ),

        // Policy tools
        Tool::new(
            "policy_validate",
            "Validate x402 policy file",
            ParameterSchema::object()
                .property("file_path", ParameterSchema::string().required()),
            policy::validate,
        ),
        Tool::new(
            "policy_generate",
            "Generate middleware code from policy file",
            ParameterSchema::object()
                .property("file_path", ParameterSchema::string().required())
                .property("framework", ParameterSchema::string()
                    .enum_values(vec!["express", "fastify"])
                    .required()),
            policy::generate,
        ),

        // Doctor tools
        Tool::new(
            "doctor_diagnose",
            "Run system diagnostics",
            ParameterSchema::object(),
            doctor::diagnose,
        ),

        // Config tools
        Tool::new(
            "config_show",
            "Show current configuration with sources",
            ParameterSchema::object(),
            config::show,
        ),

        // Version tools
        Tool::new(
            "version_info",
            "Get version information and check for updates",
            ParameterSchema::object(),
            version::info,
        ),

        // Example tools
        Tool::new(
            "examples_list",
            "List available example projects",
            ParameterSchema::object(),
            examples::list,
        ),
        Tool::new(
            "examples_init",
            "Initialize example project",
            ParameterSchema::object()
                .property("name", ParameterSchema::string().required()),
            examples::init,
        ),
    ]
}
```

---

## 14. Summary and Recommendations

### 14.1 Executive Summary

The x402-dev CLI is **exceptionally well-structured** for MCP integration:

✅ **High Reusability** (85%+):
- Most commands already use library functions
- Clean separation between CLI and business logic
- Structured arguments (not string parsing)

✅ **Minimal Refactoring Needed**:
- Only 3 commands need changes (test, check, doctor)
- Changes are minor (extract result types, remove exit())
- Estimated effort: 2-3 days

✅ **Clean Architecture**:
- Async/await throughout (tokio)
- anyhow::Result error handling
- Multi-tier configuration system
- Type-safe domain types (x402-domain)

### 14.2 Recommended Integration Approach

**Phase 1: Direct Library Wrapping** (Week 1)
1. Create `x402-mcp` crate
2. Wrap existing CLI commands
3. Minor refactoring for test/check/doctor
4. Implement MCP tool handlers
5. Add unit tests

**Phase 2: Polish & Testing** (Week 2)
1. Integration testing
2. CLI compatibility tests
3. Documentation
4. Error handling improvements
5. Performance optimization

**Phase 3: Future Enhancements** (Later)
1. Migrate to domain types
2. Move invoice logic to x402-core
3. Extract shared library (x402-lib)
4. Add advanced MCP features

### 14.3 Critical Success Factors

✅ **Do This**:
1. Extract result types from test/check/doctor commands
2. Remove direct exit() calls
3. Use existing config system (don't reinvent)
4. Return JSON from MCP tools (not strings)
5. Maintain CLI compatibility

❌ **Don't Do This**:
1. Don't refactor CLI architecture unnecessarily
2. Don't duplicate configuration logic
3. Don't bypass validation (use domain types)
4. Don't break existing CLI behavior
5. Don't skip testing

### 14.4 Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| CLI coupling | LOW | Medium | Refactor exit() calls |
| Breaking changes | LOW | High | Compatibility tests |
| Configuration conflicts | LOW | Medium | Use existing system |
| Performance issues | LOW | Low | Background execution |
| Testing gaps | MEDIUM | Medium | Comprehensive test suite |

**Overall Risk Level**: 🟢 **LOW**

---

## 15. Conclusion

The x402-dev CLI implementation is a **model of clean architecture** for MCP integration. With minimal refactoring (estimated 2-3 days), the entire CLI can be exposed as MCP tools while maintaining full CLI compatibility.

**Key Takeaways**:

1. ✅ **85%+ of code is already reusable** - Most commands are library functions
2. ✅ **Clean async/await throughout** - Easy to wrap in MCP handlers
3. ✅ **Structured arguments** - No string parsing needed
4. ✅ **Comprehensive config system** - Just use it!
5. ✅ **Type-safe domain types** - Future-proof architecture

**Next Steps**:

1. Implement Phase 1 refactoring (3 commands)
2. Create x402-mcp crate skeleton
3. Implement first 3 MCP tools (mock, policy, config)
4. Add unit tests
5. Iterate through remaining commands

**Estimated Timeline**:
- Phase 1 (Core Integration): 5-7 days
- Phase 2 (Polish & Testing): 3-5 days
- **Total**: 8-12 days for production-ready MCP server

---

## Appendix A: File Locations Reference

```
crates/x402-cli/
├── src/
│   ├── main.rs                  # Entry point, command routing
│   ├── cli.rs                   # clap argument definitions
│   ├── config.rs                # Configuration system (847 lines!)
│   ├── errors.rs                # Error types and handling
│   └── commands/
│       ├── mod.rs               # Module exports
│       ├── mock.rs              # Server lifecycle (86 lines)
│       ├── test.rs              # Test runner (62 lines)
│       ├── check.rs             # Compliance checker (266 lines)
│       ├── policy.rs            # Policy management (228 lines)
│       ├── doctor.rs            # Diagnostics (423 lines)
│       ├── init.rs              # Project init (134 lines)
│       ├── config.rs            # Config display (56 lines)
│       ├── examples.rs          # Template management (305 lines)
│       ├── version.rs           # Version info (158 lines)
│       └── invoice.rs           # Invoice generation (394 lines)

crates/x402-server/
├── src/
│   ├── lib.rs                   # Public API exports
│   ├── server.rs                # HTTP server setup (248 lines)
│   ├── lifecycle.rs             # Start/stop/restart (119 lines)
│   ├── process.rs               # PID management
│   └── handlers.rs              # Request handlers

crates/x402-core/
├── src/
│   ├── lib.rs                   # Core library exports
│   ├── policy/                  # Policy engine
│   │   ├── mod.rs
│   │   ├── validator.rs
│   │   ├── rules.rs
│   │   ├── types.rs
│   │   └── codegen/
│   │       ├── express.rs
│   │       └── fastify.rs
│   └── testing/                 # Test framework
│       ├── mod.rs
│       ├── parser.rs
│       ├── executor.rs
│       ├── reporter.rs
│       └── assertions.rs

crates/x402-domain/
├── src/
│   ├── lib.rs                   # Domain types exports
│   ├── types.rs                 # Validated newtypes
│   ├── amount.rs                # Decimal-based Amount
│   ├── pricing.rs               # Pricing types
│   ├── error.rs                 # Domain errors
│   ├── validation.rs            # Validation rules
│   └── conversions.rs           # Type conversions
```

---

## Appendix B: Dependency Tree

```
x402-cli
├── x402-core (policy, testing)
├── x402-server (lifecycle, server)
├── clap (CLI parsing)
├── tokio (async runtime)
├── anyhow (error handling)
├── reqwest (HTTP client)
├── serde + serde_yaml + serde_json (serialization)
├── colored (terminal colors)
├── dialoguer (interactive prompts)
├── directories (config paths)
├── semver (version comparison)
├── chrono (timestamps)
└── uuid (IDs)

x402-server
├── x402-core (policy engine)
├── actix-web + actix-cors (HTTP server)
├── tokio (async runtime)
├── anyhow (errors)
├── serde (serialization)
├── chrono (timestamps)
├── uuid (IDs)
├── sysinfo (process info)
├── nix (Unix signals)
└── directories (config paths)

x402-core
├── anyhow (errors)
├── chrono (timestamps)
├── serde + serde_yaml + serde_json (serialization)
├── reqwest (HTTP client for tests)
├── regex (pattern matching)
└── colored (terminal colors)

x402-domain
├── rust_decimal (precise financial math)
├── serde (serialization)
├── thiserror (error definitions)
└── anyhow (error handling)
```

---

**End of Analysis Document**
