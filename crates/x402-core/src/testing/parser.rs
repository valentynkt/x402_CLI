// YAML test suite parser (FR-2.1)

use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

/// A complete test suite from YAML file
#[derive(Debug, Deserialize)]
pub struct TestSuite {
    pub tests: Vec<Test>,
}

/// Individual test case
#[derive(Debug, Deserialize, Clone)]
pub struct Test {
    pub name: String,
    pub url: String,
    #[serde(default = "default_method")]
    pub method: String,
    pub expect: Expectations,
}

fn default_method() -> String {
    "GET".to_string()
}

/// Expected outcomes for a test
#[derive(Debug, Deserialize, Clone)]
pub struct Expectations {
    pub status: Option<u16>,
    pub headers: Option<Vec<HeaderAssertion>>,
    pub invoice_amount: Option<f64>,
    pub response_time_ms: Option<u64>,
}

/// Header assertion types
#[derive(Debug, Deserialize, Clone)]
pub struct HeaderAssertion {
    pub name: String,
    #[serde(default)]
    pub exists: Option<bool>,
    pub value: Option<String>,
    pub contains: Option<String>,
    pub regex: Option<String>,
}

impl TestSuite {
    /// Parse YAML test suite from file
    pub fn from_file(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let suite: TestSuite = serde_yaml::from_str(&contents)?;

        // Validate suite has at least one test
        if suite.tests.is_empty() {
            anyhow::bail!("Test suite must contain at least one test");
        }

        Ok(suite)
    }

    /// Parse YAML test suite from string
    pub fn from_str(yaml: &str) -> Result<Self> {
        let suite: TestSuite = serde_yaml::from_str(yaml)?;

        if suite.tests.is_empty() {
            anyhow::bail!("Test suite must contain at least one test");
        }

        Ok(suite)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_suite() {
        let yaml = r#"
tests:
  - name: "Test 402 response"
    url: "http://localhost:3402/api/data"
    method: GET
    expect:
      status: 402
      headers:
        - name: WWW-Authenticate
          exists: true
"#;

        let suite = TestSuite::from_str(yaml).unwrap();
        assert_eq!(suite.tests.len(), 1);
        assert_eq!(suite.tests[0].name, "Test 402 response");
        assert_eq!(suite.tests[0].method, "GET");
    }

    #[test]
    fn test_default_method() {
        let yaml = r#"
tests:
  - name: "Test"
    url: "http://localhost:3402/"
    expect:
      status: 200
"#;

        let suite = TestSuite::from_str(yaml).unwrap();
        assert_eq!(suite.tests[0].method, "GET");
    }
}
