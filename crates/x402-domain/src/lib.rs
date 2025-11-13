//! # x402-domain
//!
//! Type-safe domain types for the X402 system to eliminate primitive obsession.
//!
//! ## Overview
//!
//! This crate provides validated newtypes for:
//! - **Identifiers**: `AgentId`, `PolicyId`, `InvoiceMemo`
//! - **Addresses**: `SolanaAddress` (Base58 validated)
//! - **Resources**: `ResourcePath`, `Port`
//! - **Financial**: `Amount` (uses Decimal, NOT f64!), `Currency`
//!
//! ## Why This Crate?
//!
//! **Problem**: Primitive obsession with 200+ String/f64 usages:
//! ```rust,ignore
//! // ❌ OLD: No type safety, runtime errors, floating-point bugs
//! fn process_payment(agent_id: String, amount: f64) {
//!     // Can accidentally pass policy_id as agent_id!
//!     // f64 has rounding errors: 0.1 + 0.2 != 0.3
//! }
//! ```
//!
//! **Solution**: Type-safe newtypes with validation:
//! ```rust
//! use x402_domain::{AgentId, Amount, Currency};
//!
//! fn process_payment(agent_id: AgentId, amount: Amount) -> Result<(), Box<dyn std::error::Error>> {
//!     // ✅ Compile-time type safety
//!     // ✅ No floating-point errors (uses Decimal)
//!     // ✅ Validated at construction
//!     Ok(())
//! }
//! ```
//!
//! ## Examples
//!
//! ### Creating Validated Types
//! ```rust
//! use x402_domain::{AgentId, SolanaAddress, Port, Amount};
//!
//! // Valid construction
//! let agent = AgentId::new("agent-123").unwrap();
//! let port = Port::new(8080).unwrap();
//! let addr = SolanaAddress::new("7EqQdEULxWcraVx3mXKFjc84LhCkMGZCkRuDpvcMwJeK").unwrap();
//!
//! // Invalid construction fails at creation
//! assert!(AgentId::new("").is_err()); // Empty string
//! assert!(Port::new(80).is_err()); // Privileged port
//! ```
//!
//! ### Financial Calculations Without Floating-Point Errors
//! ```rust
//! use x402_domain::Amount;
//! use rust_decimal::Decimal;
//!
//! // ✅ No floating-point errors!
//! let a = Amount::from_decimal_str("0.1").unwrap();
//! let b = Amount::from_decimal_str("0.2").unwrap();
//! let sum = a.checked_add(&b).unwrap();
//!
//! let expected = Amount::from_decimal_str("0.3").unwrap();
//! assert_eq!(sum, expected); // This actually works with Decimal!
//! ```
//!
//! ### USDC Lamports Conversion
//! ```rust
//! use x402_domain::Amount;
//!
//! // 1 USDC = 1,000,000 lamports (6 decimals)
//! let amount = Amount::from_usdc_lamports(1_000_000).unwrap();
//! assert_eq!(amount.to_usdc_lamports(), 1_000_000);
//! ```

pub mod amount;
pub mod conversions;
pub mod error;
pub mod pricing;
pub mod types;
pub mod validation;

// Re-export main types for convenience
pub use amount::{Amount, Currency};
pub use error::{DomainError, DomainResult};
pub use pricing::PricingConfig;
pub use types::{AgentId, InvoiceMemo, PolicyId, Port, ResourcePath, SolanaAddress};

// Re-export rust_decimal for users
pub use rust_decimal::Decimal;
