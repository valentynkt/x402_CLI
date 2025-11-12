use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;

use crate::cli::DoctorArgs;
use crate::config::load_merged_config;

/// Status indicator for checks
enum CheckStatus {
    Pass,
    Warning,
    Fail,
}

impl CheckStatus {
    fn symbol(&self) -> String {
        match self {
            CheckStatus::Pass => "✅".to_string(),
            CheckStatus::Warning => "⚠️".to_string(),
            CheckStatus::Fail => "❌".to_string(),
        }
    }

    fn color_text(&self, text: &str) -> colored::ColoredString {
        match self {
            CheckStatus::Pass => text.green(),
            CheckStatus::Warning => text.yellow(),
            CheckStatus::Fail => text.red(),
        }
    }
}

/// Track diagnostic results
struct DiagnosticResults {
    warnings: Vec<String>,
    failures: Vec<String>,
    suggestions: Vec<String>,
}

impl DiagnosticResults {
    fn new() -> Self {
        DiagnosticResults {
            warnings: Vec::new(),
            failures: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    fn add_warning(&mut self, message: String) {
        self.warnings.push(message);
    }

    fn add_failure(&mut self, message: String) {
        self.failures.push(message);
    }

    fn add_suggestion(&mut self, suggestion: String) {
        self.suggestions.push(suggestion);
    }

    fn has_issues(&self) -> bool {
        !self.warnings.is_empty() || !self.failures.is_empty()
    }
}

/// Run the doctor command for system diagnostics
pub async fn run(_args: &DoctorArgs) -> Result<()> {
    println!("{}", "x402-dev System Diagnostics".bold());
    println!("{}", "===========================".bold());
    println!();

    let mut results = DiagnosticResults::new();

    // Check environment
    check_environment(&mut results).await?;
    println!();

    // Check configuration
    check_configuration(&mut results).await?;
    println!();

    // Check x402 ecosystem packages
    check_ecosystem(&mut results).await?;
    println!();

    // Print summary
    print_summary(&results);

    // Always exit with success (diagnostics don't fail)
    Ok(())
}

/// Check environment (Rust toolchain optional, npm availability)
async fn check_environment(results: &mut DiagnosticResults) -> Result<()> {
    println!("{}", "Environment:".bold());

    // Check x402-dev version
    let version = env!("CARGO_PKG_VERSION");
    println!(
        "  {} x402-dev binary: {}",
        CheckStatus::Pass.symbol(),
        format!("v{}", version).cyan()
    );

    // Check Rust toolchain (optional)
    match check_rust_version() {
        Some(version) => {
            println!(
                "  {} Rust toolchain: {}",
                CheckStatus::Pass.symbol(),
                version.cyan()
            );
        }
        None => {
            println!(
                "  {} Rust toolchain: {}",
                CheckStatus::Warning.symbol(),
                CheckStatus::Warning.color_text("Not detected (optional for binary users)")
            );
            results.add_warning("Rust toolchain not detected".to_string());
        }
    }

    // Check npm availability
    match check_npm_version() {
        Some(version) => {
            println!(
                "  {} npm: {}",
                CheckStatus::Pass.symbol(),
                version.cyan()
            );
        }
        None => {
            println!(
                "  {} npm: {}",
                CheckStatus::Warning.symbol(),
                CheckStatus::Warning.color_text("Not detected (optional)")
            );
            results.add_warning("npm not detected".to_string());
            results.add_suggestion("Install Node.js/npm for x402 ecosystem packages: https://nodejs.org/".to_string());
        }
    }

    Ok(())
}

/// Check configuration files and port availability
async fn check_configuration(results: &mut DiagnosticResults) -> Result<()> {
    println!("{}", "Configuration:".bold());

    // Check for .x402dev.yaml
    let config_path = PathBuf::from(".x402dev.yaml");
    let config_exists = config_path.exists();

    if config_exists {
        println!(
            "  {} Config file: {}",
            CheckStatus::Pass.symbol(),
            ".x402dev.yaml".cyan()
        );

        // Try to load and validate config
        match load_merged_config(None) {
            Ok(config) => {
                println!(
                    "  {} Config syntax: {}",
                    CheckStatus::Pass.symbol(),
                    "Valid".green()
                );

                // Check port availability
                check_port_availability(config.port, results);
            }
            Err(e) => {
                println!(
                    "  {} Config syntax: {}",
                    CheckStatus::Fail.symbol(),
                    CheckStatus::Fail.color_text(&format!("Invalid - {}", e))
                );
                results.add_failure(format!("Configuration validation failed: {}", e));
                results.add_suggestion("Fix configuration errors in .x402dev.yaml".to_string());
            }
        }
    } else {
        println!(
            "  {} Config file: {}",
            CheckStatus::Warning.symbol(),
            CheckStatus::Warning.color_text("Not found (.x402dev.yaml)")
        );
        results.add_warning("No project configuration file".to_string());
        results.add_suggestion(
            "Create .x402dev.yaml configuration file with: x402-dev init".to_string(),
        );

        // Check default port (3402 is the x402-dev mock server default)
        check_port_availability(3402, results);
    }

    Ok(())
}

/// Check port availability
fn check_port_availability(port: u16, results: &mut DiagnosticResults) {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => {
            println!(
                "  {} Port {}: {}",
                CheckStatus::Pass.symbol(),
                port,
                "Available".green()
            );
        }
        Err(_) => {
            println!(
                "  {} Port {}: {}",
                CheckStatus::Warning.symbol(),
                port,
                CheckStatus::Warning.color_text("In use")
            );
            results.add_warning(format!("Port {} is already in use", port));
            results.add_suggestion(format!(
                "Stop the process using port {} or use a different port: x402-dev mock --port <PORT>",
                port
            ));
        }
    }
}

/// Check for x402 ecosystem packages
async fn check_ecosystem(results: &mut DiagnosticResults) -> Result<()> {
    println!("{}", "x402 Ecosystem:".bold());

    let package_json_path = PathBuf::from("package.json");

    if !package_json_path.exists() {
        println!(
            "  {} Corbits SDK: {}",
            CheckStatus::Fail.symbol(),
            CheckStatus::Fail.color_text("Not detected (package.json not found)")
        );
        println!(
            "  {} PayAI packages: {}",
            CheckStatus::Fail.symbol(),
            CheckStatus::Fail.color_text("Not detected (package.json not found)")
        );
        println!(
            "  {} CDP SDK: {}",
            CheckStatus::Fail.symbol(),
            CheckStatus::Fail.color_text("Not detected (package.json not found)")
        );

        results.add_failure("No package.json found".to_string());
        results.add_suggestion(
            "Initialize Node.js project: npm init -y (if needed)".to_string(),
        );
        results.add_suggestion(
            "Install Corbits SDK: npm install @corbits/sdk".to_string(),
        );
        results.add_suggestion(
            "Install PayAI packages: npm install @payai/core @payai/solana".to_string(),
        );
        results.add_suggestion("Install CDP SDK: npm install @cdp/sdk".to_string());
        return Ok(());
    }

    // Read and parse package.json
    let content = fs::read_to_string(&package_json_path)?;
    let package_json: serde_json::Value = serde_json::from_str(&content)?;

    // Check for dependencies
    let dependencies = package_json.get("dependencies").and_then(|v| v.as_object());
    let dev_dependencies = package_json
        .get("devDependencies")
        .and_then(|v| v.as_object());

    // Check Corbits SDK
    check_package(
        "Corbits SDK",
        &["@corbits/sdk", "corbits"],
        dependencies,
        dev_dependencies,
        results,
    );

    // Check PayAI packages
    check_package(
        "PayAI packages",
        &["@payai/core", "@payai/solana", "payai"],
        dependencies,
        dev_dependencies,
        results,
    );

    // Check CDP SDK
    check_package(
        "CDP SDK",
        &["@cdp/sdk", "cdp"],
        dependencies,
        dev_dependencies,
        results,
    );

    Ok(())
}

/// Check if a package is installed
fn check_package(
    name: &str,
    package_names: &[&str],
    dependencies: Option<&serde_json::Map<String, serde_json::Value>>,
    dev_dependencies: Option<&serde_json::Map<String, serde_json::Value>>,
    results: &mut DiagnosticResults,
) {
    let found = package_names.iter().any(|pkg| {
        dependencies.map_or(false, |deps| deps.contains_key(*pkg))
            || dev_dependencies.map_or(false, |deps| deps.contains_key(*pkg))
    });

    if found {
        println!(
            "  {} {}: {}",
            CheckStatus::Pass.symbol(),
            name,
            "Detected".green()
        );
    } else {
        println!(
            "  {} {}: {}",
            CheckStatus::Fail.symbol(),
            name,
            CheckStatus::Fail.color_text("Not detected")
        );
        results.add_failure(format!("{} not installed", name));

        // Add specific installation suggestion
        match name {
            "Corbits SDK" => {
                results.add_suggestion("Install Corbits SDK: npm install @corbits/sdk".to_string())
            }
            "PayAI packages" => results.add_suggestion(
                "Install PayAI packages: npm install @payai/core @payai/solana".to_string(),
            ),
            "CDP SDK" => {
                results.add_suggestion("Install CDP SDK: npm install @cdp/sdk".to_string())
            }
            _ => {}
        }
    }
}

/// Print summary and suggestions
fn print_summary(results: &DiagnosticResults) {
    if !results.suggestions.is_empty() {
        println!("{}", "💡 Suggestions:".bold().cyan());
        for suggestion in &results.suggestions {
            println!("  - {}", suggestion);
        }
        println!();
    }

    if !results.has_issues() {
        println!(
            "{} {}",
            "Overall:".bold(),
            "✅ ALL CHECKS PASSED".green().bold()
        );
    } else if results.failures.is_empty() {
        println!(
            "{} {}",
            "Overall:".bold(),
            "⚠️  WARNINGS DETECTED (not blocking)".yellow().bold()
        );
    } else {
        println!(
            "{} {}",
            "Overall:".bold(),
            "❌ ISSUES DETECTED".red().bold()
        );
        println!();
        println!("{}", "For more help:".bold());
        println!("  - Documentation: https://docs.x402-dev.com/setup");
        println!("  - Run: x402-dev config show");
        println!("  - Run: x402-dev version");
    }
}

/// Check Rust version (optional)
fn check_rust_version() -> Option<String> {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()?;

    if output.status.success() {
        let version = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();
        Some(version)
    } else {
        None
    }
}

/// Check npm version
fn check_npm_version() -> Option<String> {
    let output = Command::new("npm")
        .arg("--version")
        .output()
        .ok()?;

    if output.status.success() {
        let version = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();
        Some(format!("v{}", version))
    } else {
        None
    }
}
