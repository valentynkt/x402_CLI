mod cli;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

// ADR-002: Use multi-thread runtime (no V8 constraints in pure Rust)
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Mock(_) => {
            println!("Command 'mock' not yet implemented - coming in Epic 2");
        }
        Commands::Test(_) => {
            println!("Command 'test' not yet implemented - coming in Epic 3");
        }
        Commands::Verify(_) => {
            println!("Command 'verify' not yet implemented - coming in Epic 3");
        }
        Commands::Check(_) => {
            println!("Command 'check' not yet implemented - coming in Epic 4");
        }
        Commands::Monitor(_) => {
            println!("Command 'monitor' not yet implemented - coming in Epic 5");
        }
        Commands::Policy(_) => {
            println!("Command 'policy' not yet implemented - coming in Epic 5");
        }
        Commands::Examples(_) => {
            println!("Command 'examples' not yet implemented - coming in Epic 6");
        }
        Commands::Doctor(_) => {
            println!("Command 'doctor' not yet implemented - coming in Epic 4");
        }
        Commands::Init(_) => {
            println!("Command 'init' not yet implemented - coming in Epic 6");
        }
        Commands::Version(_) => {
            println!("Command 'version' not yet implemented - coming in Story 1.3");
        }
    }

    Ok(())
}
