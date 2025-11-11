//! Error types for policy parsing and validation
//!
//! This module defines comprehensive error types for policy operations,
//! providing clear error messages for debugging and user feedback.

use thiserror::Error;

/// Errors that can occur during policy parsing and validation
#[derive(Debug, Error, PartialEq)]
pub enum PolicyError {
    /// YAML parsing failed
    #[error("Failed to parse YAML policy file: {0}")]
    YamlParseError(String),

    /// Missing required field in policy definition
    #[error("Missing required field '{field}' in {policy_type} policy")]
    MissingField {
        field: String,
        policy_type: String,
    },

    /// Invalid policy type
    #[error("Unknown policy type: {0}. Valid types are: allowlist, denylist, rate_limit, spending_cap")]
    InvalidPolicyType(String),

    /// Invalid field name for allowlist/denylist
    #[error("Invalid field '{0}' for allowlist/denylist. Valid fields are: agent_id, wallet_address, ip_address")]
    InvalidField(String),

    /// Empty values array
    #[error("Values array cannot be empty for {0} policy")]
    EmptyValues(String),

    /// Invalid rate limit configuration
    #[error("Invalid rate limit: max_requests must be positive and window_seconds must be greater than 0")]
    InvalidRateLimit,

    /// Invalid spending cap configuration
    #[error("Invalid spending cap: max_amount must be positive")]
    InvalidSpendingCap,

    /// Missing currency for spending cap
    #[error("Currency must be specified for spending_cap policy")]
    MissingCurrency,

    /// Invalid currency code
    #[error("Invalid currency code: {0}. Must be a valid ISO 4217 currency code (e.g., USD, EUR)")]
    InvalidCurrency(String),

    /// Policy validation failed
    #[error("Policy validation failed: {0}")]
    ValidationError(String),

    /// Empty policy list
    #[error("Policy file must contain at least one policy")]
    EmptyPolicyList,

    /// IO error when reading policy file
    #[error("Failed to read policy file: {0}")]
    IoError(String),
}

impl From<serde_yaml::Error> for PolicyError {
    fn from(err: serde_yaml::Error) -> Self {
        PolicyError::YamlParseError(err.to_string())
    }
}

impl From<std::io::Error> for PolicyError {
    fn from(err: std::io::Error) -> Self {
        PolicyError::IoError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = PolicyError::MissingField {
            field: "agent_id".to_string(),
            policy_type: "allowlist".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Missing required field 'agent_id' in allowlist policy"
        );

        let err = PolicyError::InvalidPolicyType("unknown".to_string());
        assert!(err.to_string().contains("allowlist"));
        assert!(err.to_string().contains("denylist"));

        let err = PolicyError::InvalidField("unknown_field".to_string());
        assert!(err.to_string().contains("agent_id"));
        assert!(err.to_string().contains("wallet_address"));
    }
}
