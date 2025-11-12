// Test reporting (FR-2.5)

use super::executor::{SuiteResult, TestResult};
use colored::Colorize;
use serde_json::json;

/// Format results as JSON (FR-2.4: --json flag)
pub fn format_json(result: &SuiteResult) -> String {
    let tests_json: Vec<_> = result
        .tests
        .iter()
        .map(|test| {
            json!({
                "name": test.name,
                "url": test.url,
                "method": test.method,
                "passed": test.passed,
                "duration_ms": test.duration.as_millis(),
                "assertions": test.assertions.iter().map(|a| {
                    json!({
                        "description": a.description,
                        "passed": a.passed,
                        "expected": a.expected,
                        "actual": a.actual,
                    })
                }).collect::<Vec<_>>(),
                "error": test.error,
            })
        })
        .collect();

    let summary = json!({
        "total": result.total,
        "passed": result.passed,
        "failed": result.failed,
        "duration_ms": result.duration.as_millis(),
        "exit_code": result.exit_code(),
        "tests": tests_json,
    });

    serde_json::to_string_pretty(&summary).unwrap_or_else(|_| "{}".to_string())
}

/// Format results as human-readable summary (FR-2.5)
pub fn format_summary(result: &SuiteResult, quiet: bool) -> String {
    let mut output = String::new();

    // Test results (unless quiet mode)
    if !quiet {
        for test in &result.tests {
            format_test_result(test, &mut output);
        }

        output.push('\n');
    }

    // Summary (FR-2.5: total, passed, failed, duration)
    output.push_str(&format!(
        "\n{}\n",
        "Test Suite Summary".bold().cyan()
    ));
    output.push_str(&format!(
        "  Total:    {}\n",
        result.total.to_string().bold()
    ));
    output.push_str(&format!(
        "  Passed:   {}\n",
        result.passed.to_string().green().bold()
    ));
    output.push_str(&format!(
        "  Failed:   {}\n",
        result.failed.to_string().red().bold()
    ));
    output.push_str(&format!(
        "  Duration: {}ms\n",
        result.duration.as_millis().to_string().bold()
    ));

    // Overall status
    output.push('\n');
    if result.failed == 0 {
        output.push_str(&format!("{}\n", "✓ All tests passed!".green().bold()));
    } else {
        output.push_str(&format!(
            "{}\n",
            format!("✗ {} test(s) failed", result.failed).red().bold()
        ));
    }

    output
}

/// Format individual test result
fn format_test_result(test: &TestResult, output: &mut String) {
    let status_icon = if test.passed { "✓".green() } else { "✗".red() };
    let status_text = if test.passed {
        "PASS".green().bold()
    } else {
        "FAIL".red().bold()
    };

    output.push_str(&format!(
        "{} {} {} ({} {}ms)\n",
        status_icon,
        status_text,
        test.name.bold(),
        test.method,
        test.duration.as_millis()
    ));

    // Show error if present
    if let Some(error) = &test.error {
        output.push_str(&format!("  Error: {}\n", error.red()));
    }

    // Show failed assertions
    for assertion in &test.assertions {
        if !assertion.passed {
            output.push_str(&format!(
                "  ✗ {}\n    Expected: {}\n    Actual:   {}\n",
                assertion.description,
                assertion.expected,
                assertion.actual.yellow()
            ));
        }
    }
}

/// Generate JUnit XML report (FR-2.5: SHOULD support)
pub fn generate_junit_xml(result: &SuiteResult) -> String {
    let mut xml = String::new();

    xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push('\n');
    xml.push_str(&format!(
        r#"<testsuite name="x402-dev Test Suite" tests="{}" failures="{}" time="{:.3}">"#,
        result.total,
        result.failed,
        result.duration.as_secs_f64()
    ));
    xml.push('\n');

    for test in &result.tests {
        xml.push_str(&format!(
            r#"  <testcase name="{}" classname="{}" time="{:.3}">"#,
            escape_xml(&test.name),
            escape_xml(&test.url),
            test.duration.as_secs_f64()
        ));
        xml.push('\n');

        if !test.passed {
            if let Some(error) = &test.error {
                xml.push_str(&format!(
                    r#"    <failure message="{}">{}</failure>"#,
                    escape_xml("Request failed"),
                    escape_xml(error)
                ));
                xml.push('\n');
            } else {
                // Collect failed assertions
                let failed_assertions: Vec<_> = test
                    .assertions
                    .iter()
                    .filter(|a| !a.passed)
                    .collect();

                for assertion in failed_assertions {
                    xml.push_str(&format!(
                        r#"    <failure message="{}">Expected: {} / Actual: {}</failure>"#,
                        escape_xml(&assertion.description),
                        escape_xml(&assertion.expected),
                        escape_xml(&assertion.actual)
                    ));
                    xml.push('\n');
                }
            }
        }

        xml.push_str("  </testcase>\n");
    }

    xml.push_str("</testsuite>\n");
    xml
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("foo & bar"), "foo &amp; bar");
        assert_eq!(escape_xml("<test>"), "&lt;test&gt;");
    }
}
