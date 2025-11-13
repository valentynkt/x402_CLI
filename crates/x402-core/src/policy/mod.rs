// Policy engine module for x402-dev
//
// This module provides:
// - Policy rule definitions (YAML parsing)
// - Code generation for Express/Fastify middleware
// - Policy validation and conflict detection (FR-5.6)
// - Runtime policy evaluation with state tracking (Epic 5 Task 2)

pub mod codegen;
pub mod engine;
pub mod rules;
pub mod runtime_types;
pub mod state;
pub mod types;
pub mod validator;

pub use codegen::{generate_express_middleware, generate_fastify_plugin};
pub use rules::{PolicyFile, PolicyRule as RulesPolicyRule, PolicyType as RulesPolicyType};
pub use types::{PolicyAction, PolicyConfig, PolicyRule, PolicyType};
pub use validator::{
    validate_policies, IssueType, ResolutionSuggestion, ValidationIssue, ValidationReport,
};

// Re-export runtime evaluation types (Epic 5 Task 2)
pub use engine::PolicyEngine;
pub use runtime_types::{
    Policy as RuntimePolicy, PolicyDecision, RateLimitConfig, Request, SpendingCapConfig,
};
pub use state::{RateLimitState, SpendingState};
