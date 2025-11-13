//! Integration tests for x402-dev CLI
//!
//! This module contains integration tests that test the complete x402-dev binary
//! by invoking CLI commands and verifying their behavior end-to-end.
//!
//! ## Test Organization
//!
//! - `cli_integration_test` - General CLI command integration tests
//! - `check_workflow_test` - Check command workflow scenarios
//! - `doctor_workflow_test` - Doctor command workflow scenarios

mod check_workflow_test;
mod cli_integration_test;
mod doctor_workflow_test;
