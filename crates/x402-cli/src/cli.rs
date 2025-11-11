use clap::{Args, Parser, Subcommand};

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
pub struct MockArgs {
    // Epic 2: Mock facilitator server arguments
}

#[derive(Args)]
pub struct TestArgs {
    // Epic 3: Testing framework arguments
}

#[derive(Args)]
pub struct VerifyArgs {
    // Epic 3: Protocol verification arguments
}

#[derive(Args)]
pub struct CheckArgs {
    // Epic 4: Health check arguments
}

#[derive(Args)]
pub struct MonitorArgs {
    // Epic 5: Transaction monitoring arguments
}

#[derive(Args)]
pub struct PolicyArgs {
    // Epic 5: Policy management arguments
}

#[derive(Args)]
pub struct ExamplesArgs {
    // Epic 6: Example display arguments
}

#[derive(Args)]
pub struct DoctorArgs {
    // Epic 4: Diagnostic arguments
}

#[derive(Args)]
pub struct InitArgs {
    // Epic 6: Project initialization arguments
}

#[derive(Args)]
pub struct VersionArgs {
    /// Skip checking for updates
    #[arg(long)]
    pub no_update_check: bool,
}

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,

    /// Override port setting
    #[arg(long, global = true)]
    pub port: Option<u16>,

    /// Override Solana RPC URL
    #[arg(long, global = true)]
    pub solana_rpc: Option<String>,

    /// Override log level
    #[arg(long, global = true)]
    pub log_level: Option<String>,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Display current configuration with sources
    Show,
}
