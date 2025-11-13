use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::ExamplesArgs;

/// Example metadata structure
#[derive(Debug)]
struct ExampleInfo {
    name: &'static str,
    description: &'static str,
    complexity: &'static str,
    files: &'static [&'static str],
    prerequisites: &'static [&'static str],
}

/// Available examples catalog
const EXAMPLES: &[ExampleInfo] = &[
    ExampleInfo {
        name: "mcp-server-starter",
        description: "Basic MCP server with x402 payments",
        complexity: "~50 lines",
        files: &["server.rs", "README.md", ".x402dev.yaml", "Cargo.toml"],
        prerequisites: &["Rust 1.75+", "x402-dev mock server"],
    },
    ExampleInfo {
        name: "ai-agent-policies",
        description: "AI agent with spending limits and allowlists",
        complexity: "~100 lines",
        files: &[
            "agent.rs",
            "policy.yaml",
            "README.md",
            "Cargo.toml",
            "middleware.js",
        ],
        prerequisites: &[
            "Rust 1.75+",
            "x402-dev policy engine",
            "Node.js (for middleware)",
        ],
    },
    ExampleInfo {
        name: "cicd-testing",
        description: "GitHub Actions workflow for automated testing",
        complexity: "YAML config",
        files: &[
            ".github/workflows/x402-test.yaml",
            "tests/suite.yaml",
            "README.md",
            ".x402dev.yaml",
        ],
        prerequisites: &["GitHub repository", "x402-dev installed"],
    },
];

/// Run the examples command
pub async fn run(args: &ExamplesArgs) -> Result<()> {
    match args.command.as_deref() {
        Some("list") | None => list_examples(),
        Some("info") => {
            let name = args
                .name
                .as_deref()
                .context("Example name required. Usage: x402-dev examples info <name>")?;
            show_info(name)
        }
        Some("init") => {
            let name = args
                .name
                .as_deref()
                .context("Example name required. Usage: x402-dev examples init <name>")?;
            init_example(name).await
        }
        Some(cmd) => {
            anyhow::bail!(
                "Unknown examples subcommand: '{}'\nUse: list, info, or init",
                cmd
            )
        }
    }
}

/// List all available examples
fn list_examples() -> Result<()> {
    println!("{}", "Available Examples:".bold().cyan());
    println!("{}", "===================".cyan());
    println!();

    for (idx, example) in EXAMPLES.iter().enumerate() {
        println!(
            "{}. {} {} - {}",
            (idx + 1).to_string().bold(),
            example.name.green().bold(),
            format!("({})", example.complexity).dimmed(),
            example.description
        );
    }

    println!();
    println!("{}", "Usage:".bold());
    println!(
        "  {} {}",
        "x402-dev examples info".dimmed(),
        "<name>".yellow()
    );
    println!(
        "  {} {}",
        "x402-dev examples init".dimmed(),
        "<name>".yellow()
    );
    println!();
    println!("{}", "Examples:".dimmed());
    println!("  x402-dev examples info mcp-server-starter");
    println!("  x402-dev examples init mcp-server-starter");

    Ok(())
}

/// Show detailed information about an example
fn show_info(name: &str) -> Result<()> {
    let example = EXAMPLES.iter().find(|e| e.name == name).with_context(|| {
        format!(
            "Example '{}' not found. Run 'x402-dev examples list' to see available examples.",
            name
        )
    })?;

    println!();
    println!(
        "{} {}",
        "Example:".bold().cyan(),
        example.name.green().bold()
    );
    println!("{}", "=".repeat(40).cyan());
    println!();

    println!("{} {}", "Description:".bold(), example.description);
    println!("{} {}", "Complexity:".bold(), example.complexity);
    println!();

    println!("{}", "Prerequisites:".bold());
    for prereq in example.prerequisites {
        println!("  {} {}", "•".cyan(), prereq);
    }
    println!();

    println!("{}", "Files included:".bold());
    for file in example.files {
        println!("  {} {}", "📄".dimmed(), file.yellow());
    }
    println!();

    println!("{}", "Next steps:".bold().green());
    println!("  1. Initialize example:");
    println!(
        "     {} {}",
        "x402-dev examples init".dimmed(),
        example.name.yellow()
    );
    println!();
    println!("  2. Follow README.md in the created directory");
    println!();
    println!(
        "  3. Estimated setup time: {} ⏱️",
        "<2 minutes".green().bold()
    );
    println!();

    Ok(())
}

/// Initialize an example project in the current directory
async fn init_example(name: &str) -> Result<()> {
    // Validate example exists
    let example = EXAMPLES.iter().find(|e| e.name == name).with_context(|| {
        format!(
            "Example '{}' not found. Run 'x402-dev examples list' to see available examples.",
            name
        )
    })?;

    println!();
    println!(
        "{} {} {}",
        "Initializing".cyan(),
        example.name.green().bold(),
        "example...".cyan()
    );
    println!();

    // Get the examples source directory (relative to binary location)
    let examples_dir = get_examples_dir()?;
    let source_path = examples_dir.join(name);

    // Verify source directory exists
    if !source_path.exists() {
        anyhow::bail!(
            "Example template not found at: {}\n\n\
            {} This example is documented but the template files are not yet created.\n\
            {} Expected location: examples/{}/\n\n\
            {} Run 'x402-dev examples list' to see available examples.",
            source_path.display(),
            "⚠️".yellow(),
            "📁".dimmed(),
            name,
            "💡".cyan()
        );
    }

    // Get current directory as destination
    let dest_path = PathBuf::from(".");

    // Check for existing files and warn user
    let conflicts = check_conflicts(&source_path, &dest_path)?;
    if !conflicts.is_empty() {
        println!(
            "{} {} existing files will be overwritten:",
            "⚠️".yellow(),
            "Warning:".bold().yellow()
        );
        for conflict in &conflicts {
            println!("  {} {}", "•".yellow(), conflict.dimmed());
        }
        println!();

        // In non-interactive mode, we'll skip the confirmation for now
        // TODO: Add dialoguer confirmation prompt
        println!("{} Proceeding with initialization...", "→".cyan());
        println!();
    }

    // Copy files
    let copied_files = copy_example_files(&source_path, &dest_path)?;

    // Success message
    println!(
        "{} {} example initialized successfully!",
        "✅".green(),
        example.name.green().bold()
    );
    println!();
    println!("{} {} files created:", "📝".cyan(), "Files:".bold());
    for file in &copied_files {
        println!("  {} {}", "✓".green(), file.yellow());
    }
    println!();

    // Next steps
    println!("{}", "💡 Next steps:".bold().cyan());
    println!();
    println!("  1. Read the README:");
    println!("     {} {}", "cat".dimmed(), "README.md".yellow());
    println!();
    println!("  2. Review the code and configuration");
    println!();
    println!("  3. Follow the quickstart guide in README.md");
    println!();
    println!(
        "⏱️ Estimated setup time: {} (from init to running)",
        "<2 minutes".green().bold()
    );
    println!();

    Ok(())
}

/// Get the examples directory path
fn get_examples_dir() -> Result<PathBuf> {
    // Try multiple locations:
    // 1. ./examples (when running from project root)
    // 2. ../examples (when running from crates/x402-cli)
    // 3. Relative to the binary location

    let candidates = vec![
        PathBuf::from("./examples"),
        PathBuf::from("../examples"),
        PathBuf::from("../../examples"),
    ];

    for candidate in candidates {
        if candidate.exists() && candidate.is_dir() {
            return Ok(candidate);
        }
    }

    anyhow::bail!(
        "Examples directory not found. Searched locations:\n\
        - ./examples\n\
        - ../examples\n\
        - ../../examples\n\n\
        {} Run this command from the project root directory.",
        "💡".cyan()
    )
}

/// Check for file conflicts between source and destination
fn check_conflicts(source: &Path, dest: &Path) -> Result<Vec<String>> {
    let mut conflicts = Vec::new();

    fn visit_dir(source: &Path, dest: &Path, conflicts: &mut Vec<String>) -> Result<()> {
        if !source.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name();

            let dest_file = dest.join(&file_name);

            if path.is_dir() {
                visit_dir(&path, &dest_file, conflicts)?;
            } else if dest_file.exists() {
                conflicts.push(file_name.to_string_lossy().to_string());
            }
        }

        Ok(())
    }

    visit_dir(source, dest, &mut conflicts)?;
    Ok(conflicts)
}

/// Copy example files from source to destination
fn copy_example_files(source: &Path, dest: &Path) -> Result<Vec<String>> {
    let mut copied_files = Vec::new();

    fn copy_dir(source: &Path, dest: &Path, copied: &mut Vec<String>, base: &Path) -> Result<()> {
        if !source.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name();
            let dest_path = dest.join(&file_name);

            if path.is_dir() {
                // Create destination directory
                fs::create_dir_all(&dest_path)?;
                copy_dir(&path, &dest_path, copied, base)?;
            } else {
                // Copy file
                fs::copy(&path, &dest_path)?;

                // Track relative path for reporting
                let relative = dest_path
                    .strip_prefix(base)
                    .unwrap_or(&dest_path)
                    .to_string_lossy()
                    .to_string();
                copied.push(relative);
            }
        }

        Ok(())
    }

    copy_dir(source, dest, &mut copied_files, dest)?;
    Ok(copied_files)
}
