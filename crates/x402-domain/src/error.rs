use thiserror::Error;

/// Domain-specific errors for type validation
#[derive(Error, Debug, Clone, PartialEq)]
pub enum DomainError {
    #[error("Invalid agent ID: {0}")]
    InvalidAgentId(String),

    #[error("Invalid policy ID: {0}")]
    InvalidPolicyId(String),

    #[error("Invalid invoice memo: {0}")]
    InvalidInvoiceMemo(String),

    #[error("Invalid Solana address: {0}")]
    InvalidSolanaAddress(String),

    #[error("Invalid amount: {0}")]
    InvalidAmount(String),

    #[error("Invalid port: {0}")]
    InvalidPort(String),

    #[error("Invalid resource path: {0}")]
    InvalidResourcePath(String),

    #[error("Arithmetic overflow: {0}")]
    ArithmeticOverflow(String),
}

pub type DomainResult<T> = Result<T, DomainError>;
