// x402-dev test command - Automated test runner (Epic 3, FR-2)
//
// Implements YAML-based test suites for x402 payment flows

use crate::cli::TestArgs;
use anyhow::Result;
use colored::Colorize;
use x402_core::testing::{format_json, format_summary, generate_junit_xml, SuiteResult, TestSuite};

/// Execute test command and return result (library-friendly, no process::exit)
///
/// This function is designed for library integration (e.g., MCP server).
/// It executes the test suite and returns the SuiteResult without exiting the process.
///
/// For CLI usage, see `execute()` which wraps this and handles process exit.
pub async fn execute_with_result(args: &TestArgs) -> Result<SuiteResult> {
    // Load test suite from YAML file
    let suite_path = &args.suite;

    if !suite_path.exists() {
        anyhow::bail!("Test suite file not found: {}", suite_path.display());
    }

    if !args.quiet {
        println!("{} {}", "Loading test suite:".cyan(), suite_path.display());
    }

    // Parse test suite (FR-2.1)
    let suite = TestSuite::from_file(suite_path)?;

    if !args.quiet {
        println!(
            "{} {} tests\n",
            "Found".cyan(),
            suite.tests.len().to_string().bold()
        );
    }

    // Execute test suite (FR-2.3)
    let result = x402_core::testing::execute_test_suite(&suite).await?;

    // Output results based on flags
    if args.json {
        // FR-2.4: JSON output for CI/CD
        println!("{}", format_json(&result));
    } else {
        // FR-2.5: Human-readable summary
        println!("{}", format_summary(&result, args.quiet));
    }

    // Generate JUnit XML if requested (FR-2.5)
    if let Some(junit_path) = &args.junit {
        let xml = generate_junit_xml(&result);
        std::fs::write(junit_path, xml)?;
        if !args.quiet {
            println!(
                "\n{} {}",
                "JUnit XML report written to:".cyan(),
                junit_path.display()
            );
        }
    }

    // Return result for library integration
    Ok(result)
}

/// Execute test command (CLI entry point)
///
/// This function is the CLI entry point. It calls `execute_with_result()`
/// and exits the process with the appropriate exit code.
///
/// Exit codes (FR-2.4):
/// - 0: All tests passed
/// - 1: One or more tests failed
pub async fn execute(args: &TestArgs) -> Result<()> {
    let result = execute_with_result(args).await?;

    // FR-2.4: Exit with appropriate code
    std::process::exit(result.exit_code());
}
