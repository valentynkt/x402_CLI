// Assertion framework (FR-2.2)

use super::parser::Expectations;
use anyhow::Result;
use regex::Regex;
use reqwest::Response;
use std::time::Duration;

/// Result of an assertion check
#[derive(Debug, Clone)]
pub struct AssertionResult {
    pub passed: bool,
    pub description: String,
    pub expected: String,
    pub actual: String,
}

/// Assertion types
#[derive(Debug)]
pub enum Assertion {
    StatusCode(u16),
    HeaderExists(String),
    HeaderValue { name: String, value: String },
    HeaderContains { name: String, substring: String },
    HeaderRegex { name: String, pattern: String },
    InvoiceAmount(f64),
    ResponseTime(Duration),
}

impl Assertion {
    /// Check assertion against HTTP response
    pub async fn check(
        &self,
        response: &Response,
        response_time: Duration,
    ) -> Result<AssertionResult> {
        match self {
            Assertion::StatusCode(expected) => {
                let actual = response.status().as_u16();
                Ok(AssertionResult {
                    passed: actual == *expected,
                    description: format!("Status code is {}", expected),
                    expected: expected.to_string(),
                    actual: actual.to_string(),
                })
            }

            Assertion::HeaderExists(name) => {
                let exists = response.headers().contains_key(name);
                Ok(AssertionResult {
                    passed: exists,
                    description: format!("Header '{}' exists", name),
                    expected: "header present".to_string(),
                    actual: if exists {
                        "header present".to_string()
                    } else {
                        "header missing".to_string()
                    },
                })
            }

            Assertion::HeaderValue { name, value } => {
                let actual_value = response
                    .headers()
                    .get(name)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("");

                Ok(AssertionResult {
                    passed: actual_value == value,
                    description: format!("Header '{}' equals '{}'", name, value),
                    expected: value.clone(),
                    actual: actual_value.to_string(),
                })
            }

            Assertion::HeaderContains { name, substring } => {
                let actual_value = response
                    .headers()
                    .get(name)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("");

                let contains = actual_value.contains(substring);

                Ok(AssertionResult {
                    passed: contains,
                    description: format!("Header '{}' contains '{}'", name, substring),
                    expected: format!("contains '{}'", substring),
                    actual: actual_value.to_string(),
                })
            }

            Assertion::HeaderRegex { name, pattern } => {
                let actual_value = response
                    .headers()
                    .get(name)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("");

                let re = Regex::new(pattern)?;
                let matches = re.is_match(actual_value);

                Ok(AssertionResult {
                    passed: matches,
                    description: format!("Header '{}' matches regex '{}'", name, pattern),
                    expected: format!("matches /{}/", pattern),
                    actual: actual_value.to_string(),
                })
            }

            Assertion::InvoiceAmount(expected_amount) => {
                // Parse invoice amount from WWW-Authenticate header
                let header_value = response
                    .headers()
                    .get("WWW-Authenticate")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("");

                let actual_amount = parse_invoice_amount(header_value);

                let passed = if let Some(actual) = actual_amount {
                    (actual - expected_amount).abs() < 0.000001 // Float comparison
                } else {
                    false
                };

                Ok(AssertionResult {
                    passed,
                    description: format!("Invoice amount is {}", expected_amount),
                    expected: expected_amount.to_string(),
                    actual: actual_amount
                        .map(|a| a.to_string())
                        .unwrap_or_else(|| "not found".to_string()),
                })
            }

            Assertion::ResponseTime(max_duration) => {
                let passed = response_time <= *max_duration;
                Ok(AssertionResult {
                    passed,
                    description: format!("Response time <= {}ms", max_duration.as_millis()),
                    expected: format!("<={}ms", max_duration.as_millis()),
                    actual: format!("{}ms", response_time.as_millis()),
                })
            }
        }
    }
}

/// Parse amount from WWW-Authenticate header (x402-solana format)
fn parse_invoice_amount(header: &str) -> Option<f64> {
    // Example: "x402-solana recipient=... amount=0.01 currency=USDC ..."
    for part in header.split_whitespace() {
        if let Some(amount_str) = part.strip_prefix("amount=") {
            return amount_str.parse::<f64>().ok();
        }
    }
    None
}

/// Build assertions from expectations
pub fn build_assertions(expect: &Expectations) -> Vec<Assertion> {
    let mut assertions = Vec::new();

    // Status code assertion
    if let Some(status) = expect.status {
        assertions.push(Assertion::StatusCode(status));
    }

    // Header assertions
    if let Some(headers) = &expect.headers {
        for header in headers {
            if let Some(true) = header.exists {
                assertions.push(Assertion::HeaderExists(header.name.clone()));
            }

            if let Some(value) = &header.value {
                assertions.push(Assertion::HeaderValue {
                    name: header.name.clone(),
                    value: value.clone(),
                });
            }

            if let Some(substring) = &header.contains {
                assertions.push(Assertion::HeaderContains {
                    name: header.name.clone(),
                    substring: substring.clone(),
                });
            }

            if let Some(pattern) = &header.regex {
                assertions.push(Assertion::HeaderRegex {
                    name: header.name.clone(),
                    pattern: pattern.clone(),
                });
            }
        }
    }

    // Invoice amount assertion
    if let Some(amount) = expect.invoice_amount {
        assertions.push(Assertion::InvoiceAmount(amount));
    }

    // Response time assertion
    if let Some(ms) = expect.response_time_ms {
        assertions.push(Assertion::ResponseTime(Duration::from_millis(ms)));
    }

    assertions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_invoice_amount() {
        let header = "x402-solana recipient=abc amount=0.05 currency=USDC memo=test";
        assert_eq!(parse_invoice_amount(header), Some(0.05));
    }
}
