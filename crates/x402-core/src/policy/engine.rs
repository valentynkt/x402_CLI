// Policy evaluation engine

use super::state::PolicyState as RuntimePolicyState;
use super::runtime_types::{Policy, PolicyDecision, Request, RateLimitConfig, SpendingCapConfig};
use super::types::PolicyAction;
use anyhow::Result;
use std::time::SystemTime;

/// Policy evaluation engine with runtime state tracking
pub struct PolicyEngine {
    /// Ordered list of policies (sorted by priority)
    policies: Vec<Policy>,
    /// Runtime state for rate limiting and spending tracking
    state: RuntimePolicyState,
}

impl PolicyEngine {
    /// Create a new policy engine with the given policies
    ///
    /// Policies are automatically sorted by priority (highest first)
    pub fn new(mut policies: Vec<Policy>) -> Self {
        // Sort by priority (higher priority evaluated first)
        policies.sort_by(|a, b| b.priority.cmp(&a.priority));

        Self {
            policies,
            state: RuntimePolicyState::new(),
        }
    }

    /// Evaluate a request against all policies
    ///
    /// Policies are evaluated in priority order. First deny policy that matches
    /// causes immediate rejection (fail-fast). If no deny policies match and at
    /// least one allow policy matches, request is allowed.
    ///
    /// # FR-5.2: Policy Evaluation Order
    /// Policies are evaluated in the order defined by their priority field.
    ///
    /// # FR-5.2: Fail-fast on Deny
    /// Returns immediately on first deny policy match.
    pub fn evaluate(&self, request: &Request) -> Result<PolicyDecision> {
        let now = request.timestamp;

        // Cleanup expired state before evaluation
        self.state.cleanup_expired(now);

        // Evaluate policies in order (highest priority first)
        for policy in &self.policies {
            // Check if policy applies to this request
            if !self.matches_patterns(&policy.agent_patterns, &request.agent_id) {
                continue;
            }

            if !self.matches_patterns(&policy.endpoint_patterns, &request.endpoint) {
                continue;
            }

            // Check rate limit if configured (FR-5.4)
            if let Some(rate_config) = &policy.rate_limit {
                if !self.check_rate_limit(policy, request, rate_config, now)? {
                    return Ok(PolicyDecision::Deny {
                        reason: format!(
                            "Rate limit exceeded: {} requests per {}s",
                            rate_config.max_requests,
                            rate_config.window.as_secs()
                        ),
                        policy_id: policy.id.clone(),
                    });
                }
            }

            // Check spending cap if configured (FR-5.5)
            if let Some(spending_config) = &policy.spending_cap {
                if !self.check_spending_cap(policy, request, spending_config, now)? {
                    return Ok(PolicyDecision::Deny {
                        reason: format!(
                            "Spending cap exceeded: {} limit per {}s",
                            spending_config.max_amount,
                            spending_config.window.as_secs()
                        ),
                        policy_id: policy.id.clone(),
                    });
                }
            }

            // Policy matched - return immediately (highest priority wins)
            match &policy.action {
                PolicyAction::Deny(reason) => {
                    return Ok(PolicyDecision::Deny {
                        reason: reason.clone(),
                        policy_id: policy.id.clone(),
                    });
                }
                PolicyAction::Allow => {
                    // Update state for rate limiting and spending tracking
                    self.update_state(policy, request, now)?;
                    // Return immediately on allow (priority wins)
                    return Ok(PolicyDecision::Allow {
                        policy_id: policy.id.clone(),
                    });
                }
            }
        }

        // No policies matched - default deny
        Ok(PolicyDecision::Deny {
            reason: "No matching allow policy".to_string(),
            policy_id: "default".to_string(),
        })
    }

    /// Check if a value matches any pattern in the list
    ///
    /// # FR-5.3: Wildcard Pattern Matching
    /// Supports * wildcard matching (e.g., "agent-*" matches "agent-abc-123")
    fn matches_patterns(&self, patterns: &[String], value: &str) -> bool {
        if patterns.is_empty() {
            return true; // Empty patterns match everything
        }

        patterns.iter().any(|pattern| self.matches_pattern(pattern, value))
    }

    /// Check if a value matches a single pattern with wildcard support
    ///
    /// Supports:
    /// - Exact match: "agent-123" matches "agent-123"
    /// - Prefix wildcard: "agent-*" matches "agent-abc-123"
    /// - Suffix wildcard: "*-agent" matches "test-agent"
    /// - Contains wildcard: "*agent*" matches "test-agent-123"
    fn matches_pattern(&self, pattern: &str, value: &str) -> bool {
        if pattern == "*" {
            return true;
        }

        if !pattern.contains('*') {
            return pattern == value;
        }

        // Split pattern by wildcards
        let parts: Vec<&str> = pattern.split('*').collect();

        if parts.len() == 2 {
            // Single wildcard
            let (prefix, suffix) = (parts[0], parts[1]);

            if prefix.is_empty() {
                // Suffix match: "*suffix"
                return value.ends_with(suffix);
            } else if suffix.is_empty() {
                // Prefix match: "prefix*"
                return value.starts_with(prefix);
            } else {
                // Contains match: "prefix*suffix"
                return value.starts_with(prefix) && value.ends_with(suffix) && value.len() >= prefix.len() + suffix.len();
            }
        }

        // Multiple wildcards - check all parts appear in order
        let mut pos = 0;
        for (i, part) in parts.iter().enumerate() {
            if part.is_empty() {
                continue;
            }

            if i == 0 {
                // First part must be at start
                if !value[pos..].starts_with(part) {
                    return false;
                }
                pos += part.len();
            } else if i == parts.len() - 1 {
                // Last part must be at end
                return value[pos..].ends_with(part);
            } else {
                // Middle parts must appear in order
                if let Some(idx) = value[pos..].find(part) {
                    pos += idx + part.len();
                } else {
                    return false;
                }
            }
        }

        true
    }

    /// Check rate limit for a policy
    ///
    /// # FR-5.4: Sliding Window Rate Limiting
    /// Implements sliding window algorithm, not fixed window
    fn check_rate_limit(
        &self,
        policy: &Policy,
        request: &Request,
        config: &RateLimitConfig,
        now: SystemTime,
    ) -> Result<bool> {
        let key = format!("rate:{}:{}", policy.id, request.agent_id);
        let state = self.state.get_rate_limit_state(&key);

        Ok(state.check_limit(config.window, config.max_requests, now))
    }

    /// Check spending cap for a policy
    ///
    /// # FR-5.5: Spending Cap Tracking
    /// Tracks spending within configurable time windows
    fn check_spending_cap(
        &self,
        policy: &Policy,
        request: &Request,
        config: &SpendingCapConfig,
        now: SystemTime,
    ) -> Result<bool> {
        let key = format!("spend:{}:{}", policy.id, request.agent_id);
        let state = self.state.get_spending_state(&key);

        // Convert f64 amounts to u64 cents (multiply by 100)
        let max_amount_cents = (config.max_amount * 100.0) as u64;
        let request_amount_cents = (request.amount * 100.0) as u64;

        Ok(state.check_cap(config.window, max_amount_cents, request_amount_cents, now))
    }

    /// Update state after allowing a request
    fn update_state(&self, policy: &Policy, request: &Request, now: SystemTime) -> Result<()> {
        // Update rate limit state
        if policy.rate_limit.is_some() {
            let key = format!("rate:{}:{}", policy.id, request.agent_id);
            let mut state = self.state.get_rate_limit_state(&key);
            state.add_request(now);
            self.state.update_rate_limit_state(key, state);
        }

        // Update spending state
        if policy.spending_cap.is_some() {
            let key = format!("spend:{}:{}", policy.id, request.agent_id);
            let mut state = self.state.get_spending_state(&key);
            // Convert f64 to u64 cents
            let amount_cents = (request.amount * 100.0) as u64;
            state.add_spending(now, amount_cents);
            self.state.update_spending_state(key, state);
        }

        Ok(())
    }

    /// Get the current policies
    pub fn policies(&self) -> &[Policy] {
        &self.policies
    }

    /// Get a reference to the policy state
    pub fn state(&self) -> &RuntimePolicyState {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn create_test_request(agent_id: &str, amount: u64, endpoint: &str) -> Request {
        Request {
            agent_id: agent_id.to_string(),
            wallet_address: None,
            ip_address: None,
            amount: amount as f64,
            endpoint: endpoint.to_string(),
            timestamp: SystemTime::now(),
        }
    }

    fn create_allow_policy(id: &str, agent_patterns: Vec<String>) -> Policy {
        Policy {
            id: id.to_string(),
            description: "Test allow policy".to_string(),
            action: PolicyAction::Allow,
            priority: 0,
            agent_patterns,
            endpoint_patterns: vec![],
            rate_limit: None,
            spending_cap: None,
        }
    }

    fn create_deny_policy(id: &str, agent_patterns: Vec<String>) -> Policy {
        Policy {
            id: id.to_string(),
            description: "Test deny policy".to_string(),
            action: PolicyAction::Deny("Policy denied".to_string()),
            priority: 0,
            agent_patterns,
            endpoint_patterns: vec![],
            rate_limit: None,
            spending_cap: None,
        }
    }

    #[test]
    fn test_pattern_matching_exact() {
        let engine = PolicyEngine::new(vec![]);

        assert!(engine.matches_pattern("agent-123", "agent-123"));
        assert!(!engine.matches_pattern("agent-123", "agent-456"));
    }

    #[test]
    fn test_pattern_matching_prefix_wildcard() {
        let engine = PolicyEngine::new(vec![]);

        assert!(engine.matches_pattern("agent-*", "agent-123"));
        assert!(engine.matches_pattern("agent-*", "agent-abc-def"));
        assert!(!engine.matches_pattern("agent-*", "other-123"));
    }

    #[test]
    fn test_pattern_matching_suffix_wildcard() {
        let engine = PolicyEngine::new(vec![]);

        assert!(engine.matches_pattern("*-agent", "test-agent"));
        assert!(engine.matches_pattern("*-agent", "my-test-agent"));
        assert!(!engine.matches_pattern("*-agent", "agent-test"));
    }

    #[test]
    fn test_pattern_matching_contains_wildcard() {
        let engine = PolicyEngine::new(vec![]);

        assert!(engine.matches_pattern("*agent*", "test-agent-123"));
        assert!(engine.matches_pattern("*agent*", "agent"));
        assert!(engine.matches_pattern("*agent*", "my-agent-test"));
        assert!(!engine.matches_pattern("*agent*", "test-123"));
    }

    #[test]
    fn test_pattern_matching_match_all() {
        let engine = PolicyEngine::new(vec![]);

        assert!(engine.matches_pattern("*", "anything"));
        assert!(engine.matches_pattern("*", ""));
    }

    #[test]
    fn test_pattern_matching_multiple_wildcards() {
        let engine = PolicyEngine::new(vec![]);

        assert!(engine.matches_pattern("agent-*-test-*", "agent-123-test-456"));
        assert!(engine.matches_pattern("*agent*test*", "my-agent-is-test-ok"));
        assert!(!engine.matches_pattern("agent-*-test", "agent-123-other"));
    }

    #[test]
    fn test_simple_allow_policy() {
        let policies = vec![create_allow_policy("allow-all", vec!["*".to_string()])];
        let engine = PolicyEngine::new(policies);

        let request = create_test_request("agent-123", 100, "/api/test");
        let decision = engine.evaluate(&request).unwrap();

        assert!(decision.is_allowed());
    }

    #[test]
    fn test_simple_deny_policy() {
        let policies = vec![create_deny_policy("deny-all", vec!["*".to_string()])];
        let engine = PolicyEngine::new(policies);

        let request = create_test_request("agent-123", 100, "/api/test");
        let decision = engine.evaluate(&request).unwrap();

        assert!(decision.is_denied());
    }

    #[test]
    fn test_policy_priority_order() {
        let policies = vec![
            Policy {
                id: "low-priority".to_string(),
                description: "Low priority deny".to_string(),
                action: PolicyAction::Deny("Low priority deny".to_string()),
                priority: 1,
                agent_patterns: vec!["agent-*".to_string()],
                endpoint_patterns: vec![],
                rate_limit: None,
                spending_cap: None,
            },
            Policy {
                id: "high-priority".to_string(),
                description: "High priority allow".to_string(),
                action: PolicyAction::Allow,
                priority: 10,
                agent_patterns: vec!["agent-*".to_string()],
                endpoint_patterns: vec![],
                rate_limit: None,
                spending_cap: None,
            },
        ];

        let engine = PolicyEngine::new(policies);

        // High priority allow should be evaluated first
        let request = create_test_request("agent-123", 100, "/api/test");
        let decision = engine.evaluate(&request).unwrap();

        assert!(decision.is_allowed());
    }

    #[test]
    fn test_deny_policy_fail_fast() {
        let policies = vec![
            create_deny_policy("deny-specific", vec!["agent-bad".to_string()]),
            create_allow_policy("allow-all", vec!["*".to_string()]),
        ];

        let engine = PolicyEngine::new(policies);

        // Should deny immediately for agent-bad
        let request = create_test_request("agent-bad", 100, "/api/test");
        let decision = engine.evaluate(&request).unwrap();
        assert!(decision.is_denied());

        // Should allow for other agents
        let request = create_test_request("agent-good", 100, "/api/test");
        let decision = engine.evaluate(&request).unwrap();
        assert!(decision.is_allowed());
    }

    #[test]
    fn test_rate_limiting() {
        let policies = vec![Policy {
            id: "rate-limited".to_string(),
            description: "Rate limited policy".to_string(),
            action: PolicyAction::Allow,
            priority: 0,
            agent_patterns: vec!["*".to_string()],
            endpoint_patterns: vec![],
            rate_limit: Some(RateLimitConfig {
                max_requests: 3,
                window: Duration::from_secs(60),
            }),
            spending_cap: None,
        }];

        let engine = PolicyEngine::new(policies);

        // First 3 requests should succeed
        for _ in 0..3 {
            let request = create_test_request("agent-123", 100, "/api/test");
            let decision = engine.evaluate(&request).unwrap();
            assert!(decision.is_allowed());
        }

        // Fourth request should be denied
        let request = create_test_request("agent-123", 100, "/api/test");
        let decision = engine.evaluate(&request).unwrap();
        assert!(decision.is_denied());
    }

    #[test]
    fn test_spending_cap() {
        let policies = vec![Policy {
            id: "spending-capped".to_string(),
            description: "Spending capped policy".to_string(),
            action: PolicyAction::Allow,
            priority: 0,
            agent_patterns: vec!["*".to_string()],
            endpoint_patterns: vec![],
            rate_limit: None,
            spending_cap: Some(SpendingCapConfig {
                max_amount: 500.0,
                currency: "USD".to_string(),
                window: Duration::from_secs(3600),
            }),
        }];

        let engine = PolicyEngine::new(policies);

        // Spend 200
        let request = create_test_request("agent-123", 200, "/api/test");
        let decision = engine.evaluate(&request).unwrap();
        assert!(decision.is_allowed());

        // Spend another 200 (total 400)
        let request = create_test_request("agent-123", 200, "/api/test");
        let decision = engine.evaluate(&request).unwrap();
        assert!(decision.is_allowed());

        // Try to spend 200 more (would be 600, over limit)
        let request = create_test_request("agent-123", 200, "/api/test");
        let decision = engine.evaluate(&request).unwrap();
        assert!(decision.is_denied());

        // Smaller amount within limit should work
        let request = create_test_request("agent-123", 50, "/api/test");
        let decision = engine.evaluate(&request).unwrap();
        assert!(decision.is_allowed());
    }

    #[test]
    fn test_endpoint_pattern_matching() {
        let policies = vec![Policy {
            id: "endpoint-specific".to_string(),
            description: "Endpoint specific policy".to_string(),
            action: PolicyAction::Allow,
            priority: 0,
            agent_patterns: vec!["*".to_string()],
            endpoint_patterns: vec!["/api/allowed/*".to_string()],
            rate_limit: None,
            spending_cap: None,
        }];

        let engine = PolicyEngine::new(policies);

        // Should allow matching endpoint
        let request = create_test_request("agent-123", 100, "/api/allowed/test");
        let decision = engine.evaluate(&request).unwrap();
        assert!(decision.is_allowed());

        // Should deny non-matching endpoint
        let request = create_test_request("agent-123", 100, "/api/denied/test");
        let decision = engine.evaluate(&request).unwrap();
        assert!(decision.is_denied());
    }

    #[test]
    fn test_no_matching_policy_default_deny() {
        let policies = vec![create_allow_policy("specific", vec!["agent-allowed".to_string()])];
        let engine = PolicyEngine::new(policies);

        // Non-matching agent should be denied
        let request = create_test_request("agent-other", 100, "/api/test");
        let decision = engine.evaluate(&request).unwrap();
        assert!(decision.is_denied());
    }

    #[test]
    fn test_sliding_window_expiration() {
        let policies = vec![Policy {
            id: "rate-limited".to_string(),
            description: "Rate limited with short window".to_string(),
            action: PolicyAction::Allow,
            priority: 0,
            agent_patterns: vec!["*".to_string()],
            endpoint_patterns: vec![],
            rate_limit: Some(RateLimitConfig {
                max_requests: 2,
                window: Duration::from_secs(1),
            }),
            spending_cap: None,
        }];

        let engine = PolicyEngine::new(policies);
        let base_time = SystemTime::now();

        // First request
        let mut request = create_test_request("agent-123", 100, "/api/test");
        request.timestamp = base_time;
        assert!(engine.evaluate(&request).unwrap().is_allowed());

        // Second request (still within limit)
        request.timestamp = base_time + Duration::from_millis(500);
        assert!(engine.evaluate(&request).unwrap().is_allowed());

        // Third request within window (should deny)
        request.timestamp = base_time + Duration::from_millis(800);
        assert!(engine.evaluate(&request).unwrap().is_denied());

        // Fourth request after window expired (should allow)
        request.timestamp = base_time + Duration::from_secs(2);
        assert!(engine.evaluate(&request).unwrap().is_allowed());
    }
}
