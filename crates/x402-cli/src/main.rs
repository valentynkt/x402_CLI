mod cli;
mod commands;
mod config;
mod errors;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{config as config_cmd, init, mock, policy, test, version};
use errors::{convert_anyhow_to_cli_error, print_error};

// ADR-002: Use multi-thread runtime (no V8 constraints in pure Rust)
#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Mock(args) => mock::run(&args).await,
        Commands::Test(args) => test::execute(&args).await,
        Commands::Verify(_) => {
            println!("Command 'verify' not yet implemented - coming in Epic 3");
            Ok(())
        }
        Commands::Check(_) => {
            println!("Command 'check' not yet implemented - coming in Epic 4");
            Ok(())
        }
        Commands::Monitor(_) => {
            println!("Command 'monitor' not yet implemented - coming in Epic 5");
            Ok(())
        }
        Commands::Policy(args) => policy::handle_policy_command(args),
        Commands::Examples(_) => {
            println!("Command 'examples' not yet implemented - coming in Epic 6");
            Ok(())
        }
        Commands::Doctor(_) => {
            println!("Command 'doctor' not yet implemented - coming in Epic 4");
            Ok(())
        }
        Commands::Init(args) => init::run(&args).await,
        Commands::Version(args) => version::run(&args).await,
        Commands::Config(args) => config_cmd::run(&args).await,
    };

    // Handle errors with proper formatting and exit codes
    if let Err(e) = result {
        let cli_error = convert_anyhow_to_cli_error(e);
        print_error(&cli_error, cli.verbose, cli.debug);
        std::process::exit(cli_error.exit_code());
    }
}
