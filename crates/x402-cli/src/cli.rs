use clap::{Args, Parser, Subcommand};

/// Parse log level from string for CLI argument
fn parse_log_level(s: &str) -> Result<crate::config::LogLevel, String> {
    s.parse()
}

#[derive(Parser)]
#[command(name = "x402-dev", about = "x402 Protocol Standard Toolkit", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output
    #[arg(global = true, short, long)]
    pub verbose: bool,

    /// Enable debug output with stack traces
    #[arg(global = true, short, long)]
    pub debug: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start mock facilitator server (Epic 2)
    Mock(MockArgs),

    /// Run automated test suites (Epic 3)
    Test(TestArgs),

    /// Verify x402 protocol compliance (Epic 3)
    Verify(VerifyArgs),

    /// Check configuration and system health (Epic 4)
    Check(CheckArgs),

    /// Monitor x402 transactions and performance (Epic 5)
    Monitor(MonitorArgs),

    /// Manage payment policies and rules (Epic 5)
    Policy(PolicyArgs),

    /// Show example implementations and usage (Epic 6)
    Examples(ExamplesArgs),

    /// Diagnose issues and validate setup (Epic 4)
    Doctor(DoctorArgs),

    /// Initialize a new x402 project (Epic 6)
    Init(InitArgs),

    /// Display version and update information (Story 1.3)
    Version(VersionArgs),

    /// Manage configuration settings (Story 1.4)
    Config(ConfigArgs),
}

// Placeholder argument structs for each command
// Arguments will be populated in their respective epics

#[derive(Args)]
#[command(after_help = "\
EXAMPLES:
  x402-dev mock --port 3402          Start server
  x402-dev mock --pricing 0.02       Start with custom default pricing
  x402-dev mock stop                 Stop server
  x402-dev mock status               Check status
  x402-dev mock restart              Restart server

SEE ALSO:
  x402-dev test      Run test suites against mock server
  x402-dev verify    Verify protocol compliance
  x402-dev doctor    Diagnose setup issues
")]
pub struct MockArgs {
    /// Port for the mock server (default: 3402)
    #[arg(long, short, default_value = "3402")]
    pub port: u16,

    /// Override default pricing amount in SOL/USDC (overrides config file)
    #[arg(long, value_name = "AMOUNT")]
    pub pricing: Option<f64>,

    #[command(subcommand)]
    pub command: Option<MockSubcommand>,
}

#[derive(Subcommand)]
pub enum MockSubcommand {
    /// Stop the running mock server
    Stop,
    /// Check mock server status
    Status,
    /// Restart the mock server
    Restart,
}

#[derive(Args)]
#[command(after_help = "\
EXAMPLES:
  x402-dev test tests/suite.yaml
  x402-dev test tests/suite.yaml --json
  x402-dev test tests/suite.yaml --quiet
  x402-dev test tests/suite.yaml --junit report.xml

SEE ALSO:
  x402-dev mock      Start mock server for testing
  x402-dev verify    Verify compliance after tests
")]
pub struct TestArgs {
    /// Path to YAML test suite file
    pub suite: std::path::PathBuf,

    /// Output results in JSON format (for CI/CD integration)
    #[arg(long)]
    pub json: bool,

    /// Suppress verbose output, only show summary
    #[arg(long, short)]
    pub quiet: bool,

    /// Generate JUnit XML report (for CI/CD integration)
    #[arg(long, value_name = "FILE")]
    pub junit: Option<std::path::PathBuf>,

    /// Generate HTML report (optional)
    #[arg(long, value_name = "FILE")]
    pub html: Option<std::path::PathBuf>,
}

#[derive(Args)]
#[command(after_help = "\
EXAMPLES:
  x402-dev verify
  x402-dev verify --strict
  x402-dev verify --output json

SEE ALSO:
  x402-dev test      Run automated test suites
  x402-dev check     Check system health
")]
pub struct VerifyArgs {
    // Epic 3: Protocol verification arguments
}

#[derive(Args)]
#[command(after_help = "\
EXAMPLES:
  x402-dev check
  x402-dev check --verbose

SEE ALSO:
  x402-dev doctor    Diagnose and fix issues
  x402-dev config    View configuration
  x402-dev verify    Verify protocol compliance
")]
pub struct CheckArgs {
    // Epic 4: Health check arguments
}

#[derive(Args)]
#[command(after_help = "\
EXAMPLES:
  x402-dev monitor
  x402-dev monitor --interval 5
  x402-dev monitor --verbose

SEE ALSO:
  x402-dev policy    Manage payment policies
")]
pub struct MonitorArgs {
    // Epic 5: Transaction monitoring arguments
}

// PolicyArgs is now defined in commands/policy.rs
pub use crate::commands::policy::PolicyArgs;

#[derive(Args)]
#[command(after_help = "\
EXAMPLES:
  x402-dev examples
  x402-dev examples --language typescript
  x402-dev examples --topic payments

SEE ALSO:
  x402-dev init      Initialize new project
")]
pub struct ExamplesArgs {
    // Epic 6: Example display arguments
}

#[derive(Args)]
#[command(after_help = "\
EXAMPLES:
  x402-dev doctor
  x402-dev doctor --fix

SEE ALSO:
  x402-dev check     Quick health check
  x402-dev config    View configuration
  x402-dev version   Check version info
")]
pub struct DoctorArgs {
    // Epic 4: Diagnostic arguments
}

#[derive(Args)]
#[command(after_help = "\
EXAMPLES:
  x402-dev init
  x402-dev init --defaults
  x402-dev init --template minimal

SEE ALSO:
  x402-dev config    Manage configuration
  x402-dev examples  View example code
")]
pub struct InitArgs {
    // Epic 6: Project initialization arguments
}

#[derive(Args)]
#[command(after_help = "\
EXAMPLES:
  x402-dev version
  x402-dev version --no-update-check

SEE ALSO:
  x402-dev doctor    Diagnose issues
  x402-dev config    View configuration
")]
pub struct VersionArgs {
    /// Skip checking for updates
    #[arg(long)]
    pub no_update_check: bool,
}

#[derive(Args)]
#[command(after_help = "\
EXAMPLES:
  x402-dev config show
  x402-dev config show --port 8888
  X402_DEV_PORT=9999 x402-dev config show

PRIORITY ORDER:
  CLI flags > Environment variables > Project config > Global config > Defaults

CONFIG FILES:
  Global: ~/.x402dev/config.yaml
  Project: ./.x402dev.yaml

ENVIRONMENT VARIABLES:
  X402_DEV_PORT         Override port (e.g., 8402)
  X402_DEV_SOLANA_RPC   Override Solana RPC URL
  X402_DEV_LOG_LEVEL    Override log level (error|warn|info|debug|trace)

SEE ALSO:
  x402-dev init      Initialize project configuration
  x402-dev check     Validate configuration
")]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,

    /// Override port setting
    #[arg(long, global = true)]
    pub port: Option<u16>,

    /// Override Solana RPC URL
    #[arg(long, global = true)]
    pub solana_rpc: Option<String>,

    /// Override log level (error, warn, info, debug, trace)
    #[arg(long, global = true, value_parser = parse_log_level)]
    pub log_level: Option<crate::config::LogLevel>,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Display current configuration with sources
    Show,
}
