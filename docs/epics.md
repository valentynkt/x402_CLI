# Hackaton - Epic Breakdown

**Author:** Valik
**Date:** 2025-11-05
**Project Level:** Level 2 - Medium complexity
**Target Scale:** 6 epics, 37-49 stories, 6-day hackathon timeline

---

## Overview

This document provides the complete epic and story breakdown for x402-dev, decomposing the requirements from the [PRD](./PRD.md) into implementable stories.

### Epic Structure (Enhanced via 5 Elicitation Methods)

x402-dev delivers the complete x402 Protocol Standard Toolkit through 7 value-focused epics:

1. **Foundation & CLI Infrastructure** - Install and run first command in <5 minutes
2. **Mock Server (Core Demo)** - Test payment flows locally in 30 seconds without blockchain ⭐ DEMO CHECKPOINT
3. **Automated Test Runner** - CI/CD-ready test automation for payment flows
4. **Validation Tools (Simplified)** - Quick compliance checks and debugging
5. **Policy Engine & Security** - Enforce security with 10-line YAML instead of 100+ lines code ⭐ DEMO CHECKPOINT
6. **Developer Experience & Distribution** - Discover via examples, onboard in <2 minutes ⭐ DEMO CHECKPOINT
7. **Launch Preparation** - Demo video, beta testimonials, and release polish

**Key Changes from Initial Structure:**
- ✅ Split Epic 2 into separate Mock Server (2) and Test Runner (3) for clearer priorities
- ✅ Simplified Epic 4 (now 4) - reduced scope, focus on essentials
- ✅ Renamed Epic 6 to "Launch Preparation" for clarity
- ✅ Added demo checkpoints after Epics 2, 5, 6 to ensure demo-ability
- ✅ Adjusted numbering: 6 epics → 7 epics (but simpler individual scopes)

### Priority Ranking (If Timeline Slips)

**MUST HAVE (Core Demo):**
1. Epic 1: Foundation & CLI Infrastructure
2. Epic 2: Mock Server (Core Demo)

**CRITICAL (Differentiation):**
3. Epic 5: Policy Engine & Security
4. Epic 6: Developer Experience & Distribution (Examples)

**IMPORTANT (Complete Toolkit):**
5. Epic 3: Automated Test Runner

**NICE TO HAVE (Can Cut):**
6. Epic 4: Validation Tools

**COMPRESS (4 hours minimum):**
7. Epic 7: Launch Preparation

### Demo Strategy

**Epic 2 Demo Checkpoint:** "30 seconds vs 30 minutes"
- Visual comparison: Manual testing (stopwatch) vs x402-dev command
- Show: `x402-dev mock` running + `curl` request → 402 response in <30 sec

**Epic 5 Demo Checkpoint:** "100 lines → 10 lines"
- Split screen: Custom security code (100+ lines) vs YAML policy (10 lines)
- Show: `x402-dev policy generate` creating middleware from YAML

**Epic 6 Demo Checkpoint:** "Working in 2 minutes"
- Screencast: `x402-dev examples init mcp-server-starter` → working MCP server
- Include: Corbits-specific example for partnership ($5k bonus)

### Sequencing Strategy

**Critical Path (Dependency Mapping):**
```
Epic 1 (Foundation) [BLOCKS ALL]
├─→ Epic 2 (Mock Server) [BLOCKS 3,4,6]
│   ├─→ Epic 3 (Test Runner) [SOFT DEP on Mock]
│   ├─→ Epic 4 (Validation) [SOFT DEP on Mock]
│   └─→ Epic 6 (Examples) [BLOCKS: needs mock]
├─→ Epic 5 (Policy Engine) [BLOCKS 6,7]
│   └─→ Epic 6 (Examples) [BLOCKS: AI agent uses policies]
├─→ Epic 6 (Examples) [BLOCKS 7]
└─→ Epic 7 (Launch Prep) [REQUIRES ALL]
```

**Parallelization Opportunities:**
- ✅ After Epic 1 complete: Start Epic 2
- ✅ After Epic 2 complete: Start Epic 3 AND Epic 5 in parallel (no dependencies!)
- ✅ While Epic 3/5 running: Start Epic 4 (uses Epic 2 only)
- ✅ After Epic 3,4,5 complete: Start Epic 6
- ❌ Epic 7 must be last (requires all epics)

**Value Delivery:**
- After Epic 1: CLI works
- After Epic 2: **DEMO READY** - Core value prop (30 sec testing) ⭐
- After Epic 3: Test automation complete
- After Epic 4: Validation tools available
- After Epic 5: **DEMO READY** - Security story (10-line YAML) ⭐
- After Epic 6: **DEMO READY** - Distribution strategy (examples) ⭐
- After Epic 7: Launch-ready with demo video

**Timeline:** 6 days (Nov 5-11, 2024)
- Day 1: Epic 1 (Foundation)
- Day 2-3: Epic 2 (Mock Server) - MUST complete by end Day 2
- Day 3: Epic 3 (Test Runner) + Epic 5 (Policy) - START IN PARALLEL
- Day 4: Epic 4 (Validation) + Continue Epic 5
- Day 4-5: Epic 6 (Examples) - Requires Epic 2,5 complete
- Day 6: Epic 7 (Launch Prep) - Can compress to 4 hours if needed

### Risk Mitigations (Pre-mortem Analysis)

**HIGH-RISK DEPENDENCIES:**
- ❌ **RISK:** Epic 1 delay cascades entire timeline
  - **Mitigation:** Hardcode defaults, "Hello World" CLI in 4 hours max

- ❌ **RISK:** Corbits SDK integration fails (6 hours wasted)
  - **Mitigation:** 2-hour time box, fallback to manual HTTP/invoice generation

- ❌ **RISK:** Policy engine too complex (Days 4-5 slip)
  - **Mitigation:** Start with allowlist only, add rate limiting if time permits

- ❌ **RISK:** No beta users respond (Demo lacks social proof)
  - **Mitigation:** START Discord/x402 community outreach TODAY (Day 1), not Day 5

- ❌ **RISK:** Mock server underestimated (Day 2-3 slip)
  - **Mitigation:** Use simplest Express.js implementation, defer advanced features

**HARD DEADLINES:**
- End of Day 1: "Hello World" CLI command working
- End of Day 2: Mock server responding with 402 + invoices
- End of Day 3: Epic 2 MUST be demo-ready (first checkpoint)
- End of Day 5: Epics 2,5,6 MUST be demo-ready (all checkpoints)
- Day 6: Demo video recording (cannot slip)

**CUT PRIORITY (if timeline slips):**
1. First to cut: Epic 4 (Validation Tools) - not essential for demo
2. Second to cut: Epic 3 advanced features (parallel execution, JUnit XML)
3. Third to cut: Epic 5 advanced features (rate limiting, spending caps)
4. Last resort: Simplify Epic 6 to 1 example only (MCP server)

---

## Epic 1: Foundation & CLI Infrastructure

**Epic Goal:** Establish the technical foundation that enables all subsequent development. Developers can install x402-dev via npm and run their first command (`x402-dev version`) within 5 minutes, with clear error messages and helpful documentation.

**Value Statement:** "Install and run first command in <5 minutes"

**Timeline:** Day 1 (Nov 5-6)
**Hard Deadline:** End of Day 1 - "Hello World" CLI command working

---

### Story 1.1: Project Scaffolding & Build System

As a developer,
I want a TypeScript project with proper build tooling,
So that I can develop and package x402-dev efficiently.

**Acceptance Criteria:**

**Given** a new project directory
**When** I run the build command
**Then** TypeScript compiles to JavaScript without errors

**And** the output includes both ESM and CJS formats

**And** the package.json includes correct bin entry for CLI executable

**And** npm pack produces a bundle <10MB

**Prerequisites:** None (first story in project)

**Technical Notes:**
- Use tsup for zero-config bundling
- TypeScript 5.x with strict mode enabled
- Target ES2022 for modern features
- Include .gitignore, .npmignore for clean package
- Set up package.json with: name, version, bin, engines (Node 18+)
- Dependencies: typescript, tsup, @types/node

---

### Story 1.2: CLI Framework Integration

As a developer,
I want Clap integrated as the CLI framework,
So that I can define commands with options and help text.

**Acceptance Criteria:**

**Given** the CLI is invoked with `x402-dev --help`
**When** the command runs
**Then** it displays a list of available commands

**And** the help text is formatted with colors (via Clap's built-in styling)

**And** invalid commands show "did you mean?" suggestions

**Prerequisites:** Story 1.1 (project scaffolding must exist)

**Technical Notes:**
- Add clap = { version = "4.5", features = ["derive", "color", "suggestions"] } to Cargo.toml
- Add colored or anstyle for terminal colors (Clap includes anstyle by default)
- Create crates/x402-cli/src/cli.rs as main CLI module
- Use Clap derive macros: `#[derive(Parser)]` for main CLI struct
- Define subcommands using `#[command(subcommand)]` attribute
- Implement command structure with derive API (not builder API for cleaner code)
- Clap automatically provides help formatting, "did you mean?" suggestions, and color output
- Example structure:
  ```rust
  #[derive(Parser)]
  #[command(name = "x402-dev", about = "x402 Protocol Standard Toolkit")]
  struct Cli {
      #[command(subcommand)]
      command: Commands,
  }

  #[derive(Subcommand)]
  enum Commands {
      Mock(MockArgs),
      Test(TestArgs),
      // ... other commands
  }
  ```

---

### Story 1.3: Version Command & Update Notifications

As a developer,
I want to check x402-dev version information,
So that I know which version I'm running and if updates are available.

**Acceptance Criteria:**

**Given** the CLI is installed
**When** I run `x402-dev version`
**Then** it displays x402-dev version, Rust version, and platform

**And** it checks npm registry for newer versions (weekly)

**And** it displays "Update available" message if newer version exists

**And** it supports `--no-update-check` flag to disable check

**Prerequisites:** Story 1.2 (CLI framework must exist)

**Technical Notes:**
- Read version from Cargo.toml using env!("CARGO_PKG_VERSION") macro
- Use std::env::consts::ARCH and std::env::consts::OS for platform detection
- Display Rust version using rustc_version crate or compile-time CARGO_PKG_RUST_VERSION
- Implement weekly update check (cache last check timestamp)
- npm registry API: `https://registry.npmjs.org/x402-dev/latest`
- Use reqwest crate for HTTP requests to npm registry
- Store timestamp in ~/.x402dev/update-check.json using serde_json
- Use directories crate to get platform-specific config directory
- Graceful fallback if network unavailable (use Result<> pattern)
- Example implementation:
  ```rust
  const VERSION: &str = env!("CARGO_PKG_VERSION");
  println!("x402-dev v{}", VERSION);
  println!("Platform: {}-{}", std::env::consts::OS, std::env::consts::ARCH);
  ```

---

### Story 1.4: Configuration Management System

As a developer,
I want multi-tier configuration support,
So that I can customize x402-dev behavior via CLI flags, env vars, or config files.

**Acceptance Criteria:**

**Given** configuration options exist at multiple levels
**When** I run a command
**Then** CLI flags override environment variables

**And** environment variables override project config (.x402dev.yaml)

**And** project config overrides global config (~/.x402dev/config.yaml)

**And** global config overrides built-in defaults

**And** invalid config shows clear error with fix suggestion

**Prerequisites:** Story 1.2 (CLI framework must exist)

**Technical Notes:**
- Install cosmiconfig 8.x for config file discovery
- Install js-yaml 4.x for YAML parsing
- Install dotenv 16.x for .env file support
- Config priority: CLI > ENV > .x402dev.yaml > ~/.x402dev/config.yaml > defaults
- Environment variables: X402_DEV_PORT, X402_DEV_SOLANA_RPC, etc.
- Validate config schema with clear error messages
- Support: `x402-dev config show` to display merged configuration

---

### Story 1.5: Error Handling Infrastructure

As a developer,
I want clear, actionable error messages,
So that I can quickly resolve issues without frustration.

**Acceptance Criteria:**

**Given** an error occurs during command execution
**When** the error is displayed
**Then** it shows error message in red color

**And** it suggests next steps or fixes in yellow

**And** it includes documentation link if available

**And** it exits with appropriate exit code (1=failure, 2=config error, 3=network error)

**And** `--verbose` flag shows detailed logs

**And** `--debug` flag shows stack traces

**Prerequisites:** Story 1.2 (CLI framework must exist)

**Technical Notes:**
- Create error classes: ConfigError, NetworkError, ValidationError
- Implement error formatter with Chalk colors
- Add context to errors (what was being attempted)
- Link to docs: `https://docs.x402-dev.com/errors/<error-code>`
- Implement verbose/debug flag handling
- Log errors to ~/.x402dev/logs/error.log (optional)
- Never show raw stack traces in normal mode (user-hostile)

---

### Story 1.6: Help System & Documentation

As a developer,
I want built-in help documentation,
So that I can discover and learn commands without leaving terminal.

**Acceptance Criteria:**

**Given** I need help with a command
**When** I run `x402-dev help` or `x402-dev <command> --help`
**Then** it displays command usage with examples

**And** it shows available options and flags

**And** it includes description of what the command does

**And** it suggests related commands

**And** help text is formatted with colors and structure

**Prerequisites:** Story 1.2 (CLI framework must exist)

**Technical Notes:**
- Use Clap's built-in help generation (automatically enabled with derive macros)
- Customize help with `#[command(about = "...", long_about = "...")]` attributes
- Include usage examples using `#[command(after_help = "EXAMPLES:\n  x402-dev mock --port 3402\n  ...")]`
- Add "See also" section in after_help for related commands
- Clap automatically formats: command name, description, usage, options
- Clap automatically supports both `x402-dev help <command>` and `<command> --help`
- Customize help template with `#[command(help_template = "...")]` if needed (optional)
- Use `#[arg(help = "...", long_help = "...")]` for detailed option descriptions
- Example:
  ```rust
  #[derive(Parser)]
  #[command(
      about = "Start mock facilitator server",
      long_about = "Starts a local mock x402 facilitator server for testing payment flows without blockchain dependency",
      after_help = "EXAMPLES:\n  x402-dev mock --port 3402\n  x402-dev mock --pricing 0.01\n\nSEE ALSO:\n  x402-dev test    Run automated test suites"
  )]
  struct MockArgs { ... }
  ```

---

### Story 1.7: Init Command for Project Setup

As a developer,
I want interactive project initialization,
So that I can quickly set up x402-dev configuration.

**Acceptance Criteria:**

**Given** I'm in a new project directory
**When** I run `x402-dev init`
**Then** it prompts for: port, pricing, Solana network

**And** it generates .x402dev.yaml with my choices

**And** it detects existing config and offers to update

**And** it validates all inputs before writing config

**And** it creates config directory if missing

**And** it completes in <2 minutes

**Prerequisites:** Story 1.4 (configuration system must exist)

**Technical Notes:**
- Add dialoguer = "0.11" to Cargo.toml for interactive prompts
- Default values: port=3402, pricing=0.01 USDC, network=devnet
- Use dialoguer::Input for text inputs with validation
- Use dialoguer::Select for network selection (devnet/testnet/mainnet)
- Validate port (1024-65535) using .validate() method
- Validate pricing (positive number) using .validate() method
- Create .x402dev.yaml using serde_yaml crate
- Check if .x402dev.yaml exists → use dialoguer::Confirm for overwrite prompt
- Use std::fs::create_dir_all for creating config directory
- Example implementation:
  ```rust
  use dialoguer::{Input, Select, Confirm};

  let port: u16 = Input::new()
      .with_prompt("Mock server port")
      .default(3402)
      .validate_with(|input: &u16| -> Result<(), &str> {
          if *input >= 1024 && *input <= 65535 { Ok(()) }
          else { Err("Port must be between 1024-65535") }
      })
      .interact()?;

  let network_options = vec!["devnet", "testnet", "mainnet"];
  let network = Select::new()
      .with_prompt("Solana network")
      .items(&network_options)
      .default(0)
      .interact()?;
  ```
- Generate config template:
  ```yaml
  # x402-dev configuration (generated by: x402-dev init)
  mock_server:
    port: 3402
    pricing:
      default: 0.01
  solana:
    rpc_url: https://api.devnet.solana.com
    network: devnet
  ```

---

### Story 1.8: TypeScript Runtime Integration

As a developer,
I want deno_core V8 runtime integrated with TypeScript bundling,
So that Epic 2 (Mock Server) and Epic 5 (Policy Engine) can use TypeScript/Corbits SDK.

**Acceptance Criteria:**

**Given** the Rust workspace is scaffolded
**When** I build the project
**Then** TypeScript code is automatically compiled and bundled

**And** the bundled JavaScript is embedded into the Rust binary

**And** deno_core JsRuntime initializes successfully

**And** Rust can call TypeScript functions via deno_core ops

**And** the build process completes in <30 seconds

**Prerequisites:** Story 1.1 (project scaffolding must exist)

**Technical Notes:**
- Create TypeScript project structure in `ts/` directory:
  ```
  ts/
  ├── package.json
  ├── tsconfig.json
  ├── src/
  │   ├── runtime.ts (entry point)
  │   ├── corbits/ (Corbits SDK wrapper - placeholder for Epic 2)
  │   ├── server/ (Express server - placeholder for Epic 2)
  │   └── utils/
  └── dist/ (build output)
  ```
- Set up TypeScript dependencies in `ts/package.json`:
  ```json
  {
    "name": "x402-dev-runtime",
    "scripts": {
      "build": "tsup src/runtime.ts --format esm --minify"
    },
    "devDependencies": {
      "typescript": "^5.3",
      "tsup": "^8.0",
      "@types/node": "^20.0"
    }
  }
  ```
- Configure `ts/tsconfig.json`:
  ```json
  {
    "compilerOptions": {
      "target": "ES2022",
      "module": "ESNext",
      "moduleResolution": "bundler",
      "strict": true
    }
  }
  ```
- Add deno_core to `crates/x402-core/Cargo.toml`:
  ```toml
  [dependencies]
  deno_core = "0.311"
  ```
- Create `crates/x402-core/build.rs` to bundle TypeScript at build time:
  ```rust
  use std::process::Command;

  fn main() {
      println!("cargo:rerun-if-changed=../../ts/src");

      // Run npm build
      let status = Command::new("npm")
          .args(&["run", "build"])
          .current_dir("../../ts")
          .status()
          .expect("Failed to build TypeScript");

      assert!(status.success(), "TypeScript build failed");
  }
  ```
- Create `crates/x402-core/src/runtime/js_runtime.rs` to initialize deno_core:
  ```rust
  use deno_core::{JsRuntime, RuntimeOptions};

  pub struct JavaScriptRuntime {
      runtime: JsRuntime,
  }

  impl JavaScriptRuntime {
      pub fn new() -> Result<Self, anyhow::Error> {
          // Embed bundled JavaScript
          let js_code = include_str!("../../../ts/dist/runtime.js");

          let mut runtime = JsRuntime::new(RuntimeOptions {
              extensions: vec![],
              ..Default::default()
          });

          // Execute bundled code
          runtime.execute_script("<runtime>", js_code)?;

          Ok(Self { runtime })
      }
  }
  ```
- Add `mod runtime;` to `crates/x402-core/src/lib.rs`
- Test integration with simple TypeScript function call
- Estimated effort: 4-6 hours (includes TypeScript setup + deno_core integration + testing)

---

