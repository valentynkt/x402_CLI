// Test execution engine (FR-2.3)

use super::assertions::{build_assertions, AssertionResult};
use super::parser::{Test, TestSuite};
use anyhow::Result;
use reqwest::Client;
use std::time::{Duration, Instant};

/// Result of a single test execution
#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub url: String,
    pub method: String,
    pub passed: bool,
    pub duration: Duration,
    pub assertions: Vec<AssertionResult>,
    pub error: Option<String>,
}

/// Result of entire test suite execution
#[derive(Debug)]
pub struct SuiteResult {
    pub tests: Vec<TestResult>,
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub duration: Duration,
}

impl SuiteResult {
    pub fn exit_code(&self) -> i32 {
        if self.failed > 0 {
            1 // FR-2.4: Exit code 1 if any test fails
        } else {
            0 // FR-2.4: Exit code 0 if all pass
        }
    }
}

/// Execute a complete test suite
pub async fn execute_test_suite(suite: &TestSuite) -> Result<SuiteResult> {
    let start = Instant::now();
    let client = Client::builder().timeout(Duration::from_secs(30)).build()?;

    let mut test_results = Vec::new();
    let mut passed_count = 0;
    let mut failed_count = 0;

    // FR-2.3: Execute tests sequentially
    for test in &suite.tests {
        let result = execute_single_test(&client, test).await;

        if result.passed {
            passed_count += 1;
        } else {
            failed_count += 1;
        }

        test_results.push(result);
        // FR-2.3: Continue execution on test failure (fail-soft)
    }

    let total_duration = start.elapsed();

    Ok(SuiteResult {
        tests: test_results,
        total: suite.tests.len(),
        passed: passed_count,
        failed: failed_count,
        duration: total_duration,
    })
}

/// Execute a single test
async fn execute_single_test(client: &Client, test: &Test) -> TestResult {
    let start = Instant::now();

    // Build HTTP request
    let request = match test.method.to_uppercase().as_str() {
        "GET" => client.get(&test.url),
        "POST" => client.post(&test.url),
        "PUT" => client.put(&test.url),
        "DELETE" => client.delete(&test.url),
        "PATCH" => client.patch(&test.url),
        "HEAD" => client.head(&test.url),
        _ => client.get(&test.url), // Default to GET
    };

    // Execute request
    let response_result = request.send().await;
    let request_duration = start.elapsed();

    match response_result {
        Ok(response) => {
            // Build assertions from expectations
            let assertions_to_check = build_assertions(&test.expect);
            let mut assertion_results = Vec::new();
            let mut all_passed = true;

            // Check each assertion
            for assertion in assertions_to_check {
                match assertion.check(&response, request_duration).await {
                    Ok(result) => {
                        if !result.passed {
                            all_passed = false;
                        }
                        assertion_results.push(result);
                    }
                    Err(e) => {
                        // Assertion error (e.g., regex compilation failed)
                        all_passed = false;
                        assertion_results.push(AssertionResult {
                            passed: false,
                            description: format!("Assertion error: {}", e),
                            expected: "valid assertion".to_string(),
                            actual: format!("error: {}", e),
                        });
                    }
                }
            }

            TestResult {
                name: test.name.clone(),
                url: test.url.clone(),
                method: test.method.clone(),
                passed: all_passed,
                duration: request_duration,
                assertions: assertion_results,
                error: None,
            }
        }
        Err(e) => {
            // HTTP request failed
            TestResult {
                name: test.name.clone(),
                url: test.url.clone(),
                method: test.method.clone(),
                passed: false,
                duration: request_duration,
                assertions: vec![],
                error: Some(format!("HTTP request failed: {}", e)),
            }
        }
    }
}
