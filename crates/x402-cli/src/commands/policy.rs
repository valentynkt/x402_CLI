// Epic 5: Policy Management Command
// Implements FR-5.6 (validate), FR-6.1 (Express), FR-6.2 (Fastify)

use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use colored::Colorize;
use std::path::PathBuf;
use x402_core::policy::rules::PolicyFile;
use x402_core::policy::types::PolicyConfig;
use x402_core::policy::{
    codegen::{generate_express_middleware, generate_fastify_plugin},
    validate_policies, IssueType, ValidationReport,
};

#[derive(Args)]
#[command(after_help = "\
EXAMPLES:
  # Validate policy file
  x402-dev policy validate policy.yaml

  # Generate Express middleware
  x402-dev policy generate policy.yaml --framework express --output middleware.js

  # Generate Fastify plugin
  x402-dev policy generate policy.yaml --framework fastify --output plugin.js

  # Print to stdout (for piping)
  x402-dev policy generate policy.yaml --framework express

SEE ALSO:
  x402-dev monitor   Monitor policy enforcement
  x402-dev init      Initialize project with policies
")]
pub struct PolicyArgs {
    #[command(subcommand)]
    pub command: PolicyCommand,
}

#[derive(Subcommand)]
pub enum PolicyCommand {
    /// Validate policy file syntax and detect conflicts (FR-5.6)
    Validate {
        /// Path to policy YAML file
        file: PathBuf,
    },

    /// Generate middleware code from policy file (FR-6.1, FR-6.2)
    Generate {
        /// Path to policy YAML file
        file: PathBuf,

        /// Target framework (express or fastify)
        #[arg(long, short, value_name = "FRAMEWORK")]
        framework: Framework,

        /// Output file path (prints to stdout if omitted)
        #[arg(long, short, value_name = "FILE")]
        output: Option<PathBuf>,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Framework {
    Express,
    Fastify,
}

pub fn handle_policy_command(args: PolicyArgs) -> Result<()> {
    match args.command {
        PolicyCommand::Validate { file } => validate_command(file),
        PolicyCommand::Generate {
            file,
            framework,
            output,
        } => generate_command(file, framework, output),
    }
}

/// FR-5.6: Policy validation with conflict detection
fn validate_command(file: PathBuf) -> Result<()> {
    println!("{}", "Policy Validation".bold().cyan());
    println!("File: {}\n", file.display());

    // Load policy file
    let policy_content = std::fs::read_to_string(&file)
        .with_context(|| format!("Failed to read policy file: {}", file.display()))?;

    let policy_file: PolicyFile =
        serde_yaml::from_str(&policy_content).context("Failed to parse YAML policy file")?;

    // Convert PolicyFile to PolicyConfig for validation
    let policy_config = PolicyConfig {
        policies: policy_file.policies.clone(),
    };

    // Validate policies
    let report = validate_policies(&policy_config);

    // Display validation results
    display_validation_report(&report);

    if report.has_errors {
        anyhow::bail!("Policy validation failed with errors");
    }

    if !report.has_warnings {
        println!("\n{} Policy file is valid!", "".green().bold());
    } else {
        println!(
            "\n{} Policy file is valid (with warnings)",
            "�".yellow().bold()
        );
    }

    Ok(())
}

/// FR-6.1, FR-6.2: Generate middleware code
fn generate_command(file: PathBuf, framework: Framework, output: Option<PathBuf>) -> Result<()> {
    println!("{}", "Code Generation".bold().cyan());
    println!("Policy file: {}", file.display());
    println!("Framework: {:?}\n", framework);

    // Load policy file
    let policy_content = std::fs::read_to_string(&file)
        .with_context(|| format!("Failed to read policy file: {}", file.display()))?;

    let policy_file: PolicyFile =
        serde_yaml::from_str(&policy_content).context("Failed to parse YAML policy file")?;

    // Convert PolicyFile to PolicyConfig for validation
    let policy_config = PolicyConfig {
        policies: policy_file.policies.clone(),
    };

    // Validate before generation
    let report = validate_policies(&policy_config);
    if report.has_errors {
        eprintln!("{}", " Policy validation failed:".red().bold());
        display_validation_report(&report);
        anyhow::bail!("Cannot generate code from invalid policy file");
    }

    // Generate code with policy filename
    let policy_filename = file
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("policy.yaml");

    let generated_code = match framework {
        Framework::Express => generate_express_middleware(&policy_file, policy_filename),
        Framework::Fastify => generate_fastify_plugin(&policy_file.policies, Some(policy_filename)),
    };

    // Output code
    if let Some(output_path) = output {
        std::fs::write(&output_path, &generated_code)
            .with_context(|| format!("Failed to write output file: {}", output_path.display()))?;
        println!(
            "{} Generated middleware: {}",
            "".green().bold(),
            output_path.display()
        );
        println!("  Lines: {}", generated_code.lines().count());
        println!("  Size: {} bytes", generated_code.len());
    } else {
        // Print to stdout
        println!("{}", generated_code);
    }

    Ok(())
}

/// Display validation report with colored output
fn display_validation_report(report: &ValidationReport) {
    if report.issues.is_empty() {
        return;
    }

    println!("{}", "Validation Issues:".bold());
    println!();

    for (i, issue) in report.issues.iter().enumerate() {
        let prefix = match issue.issue_type {
            IssueType::Error => format!("{}  ERROR", "".red().bold()),
            IssueType::Warning => format!("{}  WARNING", "�".yellow().bold()),
            IssueType::Info => format!("{}  INFO", "9".cyan()),
        };

        println!("{} {}", prefix, issue.message);

        if let Some(details) = &issue.details {
            println!("   {}", details.dimmed());
        }

        if !issue.policy_indices.is_empty() {
            let indices: Vec<String> = issue
                .policy_indices
                .iter()
                .map(|i| format!("#{}", i))
                .collect();
            println!("   Policies: {}", indices.join(", ").dimmed());
        }

        if !issue.suggestions.is_empty() {
            for suggestion in &issue.suggestions {
                println!("   {} {}", "=�".cyan(), suggestion.description);
                println!("      � {}", suggestion.action.italic());
            }
        }

        if i < report.issues.len() - 1 {
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framework_enum() {
        // Just ensure Framework enum works with clap
        assert!(matches!(Framework::Express, Framework::Express));
        assert!(matches!(Framework::Fastify, Framework::Fastify));
    }
}
