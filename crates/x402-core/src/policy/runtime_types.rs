// Runtime types for policy engine evaluation
// These types are used during policy enforcement, separate from YAML parsing types

use std::time::SystemTime;

/// Runtime request being evaluated
#[derive(Debug, Clone)]
pub struct Request {
    pub agent_id: String,
    pub wallet_address: Option<String>,
    pub ip_address: Option<String>,
    pub endpoint: String,
    pub amount: f64,
    pub timestamp: SystemTime,
}

/// Policy evaluation decision
#[derive(Debug, Clone, PartialEq)]
pub enum PolicyDecision {
    Allow { policy_id: String },
    Deny { reason: String, policy_id: String },
}

impl PolicyDecision {
    /// Check if the decision is allow
    pub fn is_allowed(&self) -> bool {
        matches!(self, PolicyDecision::Allow { .. })
    }

    /// Check if the decision is deny
    pub fn is_denied(&self) -> bool {
        matches!(self, PolicyDecision::Deny { .. })
    }
}

/// Rate limit configuration for runtime
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub max_requests: u32,
    pub window: std::time::Duration,
}

/// Spending cap configuration for runtime
#[derive(Debug, Clone)]
pub struct SpendingCapConfig {
    pub max_amount: f64,
    pub currency: String,
    pub window: std::time::Duration,
}

/// Runtime policy representation (converted from YAML PolicyRule)
#[derive(Debug, Clone)]
pub struct Policy {
    pub id: String,
    pub description: String,
    pub priority: u32,
    pub agent_patterns: Vec<String>,
    pub endpoint_patterns: Vec<String>,
    pub action: crate::policy::types::PolicyAction,
    pub rate_limit: Option<RateLimitConfig>,
    pub spending_cap: Option<SpendingCapConfig>,
}
