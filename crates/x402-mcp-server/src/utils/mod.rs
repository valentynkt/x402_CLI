// Utility modules
//
// Epic 8: Error translation, validation, etc.

/// Error translation layer
///
/// Converts x402-core errors to MCP error format with:
/// - Structured error codes
/// - Actionable suggestions
/// - Documentation links
pub mod errors {
    use crate::types::McpError;
    use anyhow::Error as CoreError;

    /// Translate anyhow error to MCP error
    ///
    /// Library API for Phase 2 error handling
    #[allow(dead_code)]
    pub fn translate_core_error(err: &CoreError) -> McpError {
        // TODO: Implement error translation (Phase 2, Day 3-4)
        // Map error types to MCP error codes
        McpError::new("E9999", format!("Core error: {}", err))
            .with_suggestion("Check x402-dev logs for details")
    }
}

/// Parameter validation helpers
///
/// Utilities for validating MCP tool parameters
pub mod validation {
    use anyhow::{bail, Result};

    /// Validate port number (1024-65535)
    ///
    /// Library API for Phase 2 parameter validation
    #[allow(dead_code)]
    pub fn validate_port(port: u16) -> Result<()> {
        if port < 1024 {
            bail!("Port must be >= 1024, got {}", port);
        }
        // u16 max is 65535, so no need to check upper bound
        Ok(())
    }

    /// Validate positive amount
    ///
    /// Library API for Phase 2 parameter validation
    #[allow(dead_code)]
    pub fn validate_positive_amount(amount: f64) -> Result<()> {
        if amount <= 0.0 {
            bail!("Amount must be positive, got {}", amount);
        }
        Ok(())
    }
}
