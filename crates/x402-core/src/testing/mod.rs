// Testing module for x402-dev (Epic 3, FR-2)
//
// Implements YAML-based test suites for x402 payment flows

mod assertions;
mod executor;
mod parser;
mod reporter;

pub use assertions::{Assertion, AssertionResult};
pub use executor::{execute_test_suite, SuiteResult, TestResult};
pub use parser::{Expectations, HeaderAssertion, Test, TestSuite};
pub use reporter::{format_json, format_summary, generate_junit_xml};
