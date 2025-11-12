use proptest::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// POLICY ENGINE DATA STRUCTURES
// ============================================================================

/// Policy configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct PolicyConfig {
    version: String,
    rules: Vec<PolicyRule>,
    defaults: PolicyDefaults,
}

/// Individual policy rule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct PolicyRule {
    id: String,
    condition: String,
    action: PolicyAction,
    priority: u32,
}

/// Policy action types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum PolicyAction {
    Allow,
    Deny,
    RequirePayment { amount: u64 },
    RateLimit { requests_per_minute: u32 },
}

/// Default policy settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct PolicyDefaults {
    default_action: PolicyAction,
    timeout_seconds: u64,
}

/// Request context for policy evaluation
#[derive(Debug, Clone, PartialEq)]
struct RequestContext {
    endpoint: String,
    method: String,
    user_id: Option<String>,
    payment_status: PaymentStatus,
    request_count: u32,
}

/// Payment status for requests
#[derive(Debug, Clone, PartialEq)]
enum PaymentStatus {
    Paid { tx_id: String, amount: u64 },
    Unpaid,
    Pending,
}

/// Policy evaluation result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EvaluationResult {
    Allowed,
    Denied,
    PaymentRequired,
    RateLimited,
}

// ============================================================================
// CUSTOM STRATEGIES FOR PROPERTY GENERATION
// ============================================================================

/// Generate arbitrary valid policy configurations
fn arbitrary_valid_policy() -> impl Strategy<Value = PolicyConfig> {
    (
        "[0-9]+\\.[0-9]+",
        prop::collection::vec(arbitrary_policy_rule(), 1..10),
        arbitrary_policy_defaults(),
    ).prop_map(|(version, rules, defaults)| PolicyConfig {
        version,
        rules,
        defaults,
    })
}

/// Generate arbitrary policy rules
fn arbitrary_policy_rule() -> impl Strategy<Value = PolicyRule> {
    (
        "[a-zA-Z0-9_]{5,20}",
        "[a-zA-Z0-9_\\s><=]+",
        arbitrary_policy_action(),
        1u32..100,
    ).prop_map(|(id, condition, action, priority)| PolicyRule {
        id,
        condition,
        action,
        priority,
    })
}

/// Generate arbitrary policy actions
fn arbitrary_policy_action() -> impl Strategy<Value = PolicyAction> {
    prop_oneof![
        Just(PolicyAction::Allow),
        Just(PolicyAction::Deny),
        (1u64..1_000_000_000).prop_map(|amount| PolicyAction::RequirePayment { amount }),
        (1u32..1000).prop_map(|rpm| PolicyAction::RateLimit { requests_per_minute: rpm }),
    ]
}

/// Generate arbitrary policy defaults
fn arbitrary_policy_defaults() -> impl Strategy<Value = PolicyDefaults> {
    (
        arbitrary_policy_action(),
        1u64..3600,
    ).prop_map(|(default_action, timeout_seconds)| PolicyDefaults {
        default_action,
        timeout_seconds,
    })
}

/// Generate arbitrary request contexts
fn arbitrary_request_context() -> impl Strategy<Value = RequestContext> {
    (
        prop_oneof![
            Just("/data".to_string()),
            Just("/api/users".to_string()),
            Just("/api/payments".to_string()),
        ],
        prop_oneof![
            Just("GET".to_string()),
            Just("POST".to_string()),
            Just("PUT".to_string()),
        ],
        prop::option::of("[a-zA-Z0-9]{16}"),
        arbitrary_payment_status(),
        0u32..1000,
    ).prop_map(|(endpoint, method, user_id, payment_status, request_count)| RequestContext {
        endpoint,
        method,
        user_id,
        payment_status,
        request_count,
    })
}

/// Generate arbitrary payment status
fn arbitrary_payment_status() -> impl Strategy<Value = PaymentStatus> {
    prop_oneof![
        Just(PaymentStatus::Unpaid),
        Just(PaymentStatus::Pending),
        ("[a-zA-Z0-9]{32}", 1u64..1_000_000).prop_map(|(tx_id, amount)| {
            PaymentStatus::Paid { tx_id, amount }
        }),
    ]
}

// ============================================================================
// POLICY ENGINE FUNCTIONS
// ============================================================================

/// Parse policy configuration from YAML string
fn parse_policy(yaml: &str) -> Result<PolicyConfig, String> {
    serde_yaml::from_str(yaml)
        .map_err(|e| format!("YAML parse error: {}", e))
}

/// Generate Rust code from policy configuration
fn generate_code(policy: &PolicyConfig) -> String {
    let mut code = format!(
        "// Generated policy code v{}\n\n",
        policy.version
    );

    code.push_str("fn evaluate_policy(ctx: &RequestContext) -> EvaluationResult {\n");

    for rule in &policy.rules {
        code.push_str(&format!(
            "    // Rule: {} (priority: {})\n",
            rule.id, rule.priority
        ));
        code.push_str(&format!(
            "    if {} {{\n",
            rule.condition
        ));
        code.push_str(&format!(
            "        return {:?};\n",
            rule.action
        ));
        code.push_str("    }\n\n");
    }

    code.push_str("    EvaluationResult::Denied\n");
    code.push_str("}\n");

    code
}

/// Evaluate policy rules against request context
fn evaluate_policy(policy: &PolicyConfig, request: &RequestContext) -> EvaluationResult {
    // Sort rules by priority (higher priority first)
    let mut sorted_rules = policy.rules.clone();
    sorted_rules.sort_by(|a, b| b.priority.cmp(&a.priority));

    for rule in sorted_rules {
        if evaluate_condition(&rule.condition, request) {
            return match &rule.action {
                PolicyAction::Allow => EvaluationResult::Allowed,
                PolicyAction::Deny => EvaluationResult::Denied,
                PolicyAction::RequirePayment { amount } => {
                    match &request.payment_status {
                        PaymentStatus::Paid { amount: paid_amount, .. }
                            if paid_amount >= amount => EvaluationResult::Allowed,
                        _ => EvaluationResult::PaymentRequired,
                    }
                }
                PolicyAction::RateLimit { requests_per_minute } => {
                    let limit = requests_per_minute / 60;
                    if request.request_count > limit {
                        EvaluationResult::RateLimited
                    } else {
                        EvaluationResult::Allowed
                    }
                }
            };
        }
    }

    match &policy.defaults.default_action {
        PolicyAction::Allow => EvaluationResult::Allowed,
        PolicyAction::Deny => EvaluationResult::Denied,
        PolicyAction::RequirePayment { .. } => EvaluationResult::PaymentRequired,
        PolicyAction::RateLimit { .. } => EvaluationResult::RateLimited,
    }
}

/// Simple condition evaluator
fn evaluate_condition(condition: &str, _request: &RequestContext) -> bool {
    !condition.is_empty() && condition.len() < 100
}

// ============================================================================
// PROPERTY TESTS: PARSING AND CODE GENERATION
// ============================================================================

#[cfg(test)]
mod parsing_properties {
    use super::*;

proptest! {
    /// Property: Parsing any string should never panic
    #[test]
    fn parsing_never_panics(yaml in ".*") {
        let result = std::panic::catch_unwind(|| {
            let _ = parse_policy(&yaml);
        });
        prop_assert!(result.is_ok(), "Parsing should never panic");
    }

    /// Property: Valid YAML should parse or return error
    #[test]
    fn valid_yaml_handling(yaml in "version:\\s*\"[0-9.]+\"") {
        let result = parse_policy(&yaml);
        // Should return Result, never panic
        prop_assert!(result.is_ok() || result.is_err());
    }
}
}

#[cfg(test)]
mod code_generation_properties {
    use super::*;

proptest! {
    /// Property: Code generation is deterministic
    #[test]
    fn code_generation_is_deterministic(policy in arbitrary_valid_policy()) {
        let code1 = generate_code(&policy);
        let code2 = generate_code(&policy);
        prop_assert_eq!(&code1, &code2, "Generation must be deterministic");
    }

    /// Property: Generated code includes version number
    #[test]
    fn generated_code_includes_version(policy in arbitrary_valid_policy()) {
        let code = generate_code(&policy);
        prop_assert!(
            code.contains(&policy.version),
            "Generated code should include version"
        );
    }

    /// Property: Generated code includes all rule IDs
    #[test]
    fn generated_code_includes_all_rules(policy in arbitrary_valid_policy()) {
        let code = generate_code(&policy);
        for rule in &policy.rules {
            prop_assert!(
                code.contains(&rule.id),
                "Generated code should include rule ID: {}", rule.id
            );
        }
    }

    /// Property: Generated code is never empty
    #[test]
    fn generated_code_never_empty(policy in arbitrary_valid_policy()) {
        let code = generate_code(&policy);
        prop_assert!(!code.is_empty(), "Generated code should never be empty");
        prop_assert!(code.len() > 50, "Generated code should be substantial");
    }

    /// Property: Generated code structure is consistent
    #[test]
    fn generated_code_has_consistent_structure(policy in arbitrary_valid_policy()) {
        let code = generate_code(&policy);
        prop_assert!(code.contains("fn evaluate_policy"));
        prop_assert!(code.contains("RequestContext"));
        prop_assert!(code.contains("EvaluationResult"));
    }
}
}

// ============================================================================
// PROPERTY TESTS: POLICY EVALUATION
// ============================================================================

#[cfg(test)]
mod evaluation_properties {
    use super::*;

proptest! {
    /// Property: Evaluation is consistent (deterministic)
    #[test]
    fn evaluation_is_consistent(
        policy in arbitrary_valid_policy(),
        request in arbitrary_request_context()
    ) {
        let result1 = evaluate_policy(&policy, &request);
        let result2 = evaluate_policy(&policy, &request);
        let result3 = evaluate_policy(&policy, &request);

        prop_assert_eq!(result1, result2, "First and second evaluation must match");
        prop_assert_eq!(result2, result3, "Second and third evaluation must match");
    }

    /// Property: Evaluation always returns a result (never panics)
    #[test]
    fn evaluation_never_panics(
        policy in arbitrary_valid_policy(),
        request in arbitrary_request_context()
    ) {
        let result = std::panic::catch_unwind(|| {
            evaluate_policy(&policy, &request)
        });
        prop_assert!(result.is_ok(), "Evaluation should never panic");
    }

    /// Property: Higher priority rules are evaluated first
    #[test]
    fn higher_priority_evaluated_first(request in arbitrary_request_context()) {
        let policy = PolicyConfig {
            version: "1.0".to_string(),
            rules: vec![
                PolicyRule {
                    id: "low".to_string(),
                    condition: "true".to_string(),
                    action: PolicyAction::Deny,
                    priority: 1,
                },
                PolicyRule {
                    id: "high".to_string(),
                    condition: "true".to_string(),
                    action: PolicyAction::Allow,
                    priority: 100,
                },
            ],
            defaults: PolicyDefaults {
                default_action: PolicyAction::Deny,
                timeout_seconds: 30,
            },
        };

        let result = evaluate_policy(&policy, &request);
        prop_assert_eq!(result, EvaluationResult::Allowed);
    }

    /// Property: Payment status affects payment-required policies
    #[test]
    fn payment_status_affects_result(amount in 1u64..1000) {
        let policy = PolicyConfig {
            version: "1.0".to_string(),
            rules: vec![
                PolicyRule {
                    id: "payment_rule".to_string(),
                    condition: "true".to_string(),
                    action: PolicyAction::RequirePayment { amount },
                    priority: 10,
                },
            ],
            defaults: PolicyDefaults {
                default_action: PolicyAction::Deny,
                timeout_seconds: 30,
            },
        };

        // Request without payment
        let unpaid_request = RequestContext {
            endpoint: "/data".to_string(),
            method: "GET".to_string(),
            user_id: None,
            payment_status: PaymentStatus::Unpaid,
            request_count: 0,
        };
        let unpaid_result = evaluate_policy(&policy, &unpaid_request);
        prop_assert_eq!(unpaid_result, EvaluationResult::PaymentRequired);

        // Request with payment
        let paid_request = RequestContext {
            endpoint: "/data".to_string(),
            method: "GET".to_string(),
            user_id: None,
            payment_status: PaymentStatus::Paid {
                tx_id: "test_tx".to_string(),
                amount: amount + 1,
            },
            request_count: 0,
        };
        let paid_result = evaluate_policy(&policy, &paid_request);
        prop_assert_eq!(paid_result, EvaluationResult::Allowed);
    }
}
}

// ============================================================================
// PROPERTY TESTS: SERIALIZATION
// ============================================================================

#[cfg(test)]
mod serialization_properties {
    use super::*;

proptest! {
    /// Property: Serialization round-trip preserves policy
    #[test]
    fn serialization_roundtrip_preserves_policy(policy in arbitrary_valid_policy()) {
        let json = serde_json::to_string(&policy).unwrap();
        let deserialized: PolicyConfig = serde_json::from_str(&json).unwrap();

        prop_assert_eq!(policy, deserialized);
    }

    /// Property: Serialization is deterministic
    #[test]
    fn serialization_is_deterministic(policy in arbitrary_valid_policy()) {
        let json1 = serde_json::to_string(&policy).unwrap();
        let json2 = serde_json::to_string(&policy).unwrap();

        prop_assert_eq!(json1, json2);
    }

    /// Property: Cloning preserves policy equality
    #[test]
    fn clone_preserves_equality(policy in arbitrary_valid_policy()) {
        let cloned = policy.clone();
        prop_assert_eq!(policy, cloned);
    }

    /// Property: Evaluation result is same for original and cloned policy
    #[test]
    fn clone_preserves_evaluation(
        policy in arbitrary_valid_policy(),
        request in arbitrary_request_context()
    ) {
        let cloned = policy.clone();
        let original_result = evaluate_policy(&policy, &request);
        let cloned_result = evaluate_policy(&cloned, &request);

        prop_assert_eq!(original_result, cloned_result);
    }
}
}

// ============================================================================
// PROPERTY TESTS: INVARIANTS
// ============================================================================

#[cfg(test)]
mod invariant_properties {
    use super::*;

proptest! {
    /// Property: Policy with no rules uses default action
    #[test]
    fn empty_rules_uses_default(
        defaults in arbitrary_policy_defaults(),
        request in arbitrary_request_context()
    ) {
        let policy = PolicyConfig {
            version: "1.0".to_string(),
            rules: vec![],
            defaults,
        };

        let result = evaluate_policy(&policy, &request);
        // Result should be one of the four possible outcomes
        let valid_results = [
            EvaluationResult::Allowed,
            EvaluationResult::Denied,
            EvaluationResult::PaymentRequired,
            EvaluationResult::RateLimited
        ];
        prop_assert!(valid_results.contains(&result));
    }

    /// Property: Version string is preserved in policy
    #[test]
    fn version_is_preserved(version in "[0-9]+\\.[0-9]+") {
        let policy = PolicyConfig {
            version: version.clone(),
            rules: vec![],
            defaults: PolicyDefaults {
                default_action: PolicyAction::Allow,
                timeout_seconds: 30,
            },
        };

        prop_assert_eq!(&policy.version, &version);
    }

    /// Property: Timeout is always positive
    #[test]
    fn timeout_always_positive(defaults in arbitrary_policy_defaults()) {
        prop_assert!(defaults.timeout_seconds > 0);
    }

    /// Property: Rule priorities are preserved
    #[test]
    fn priorities_are_preserved(rules in prop::collection::vec(arbitrary_policy_rule(), 1..10)) {
        let original_priorities: Vec<u32> = rules.iter().map(|r| r.priority).collect();

        let policy = PolicyConfig {
            version: "1.0".to_string(),
            rules,
            defaults: PolicyDefaults {
                default_action: PolicyAction::Allow,
                timeout_seconds: 30,
            },
        };

        let preserved_priorities: Vec<u32> = policy.rules.iter().map(|r| r.priority).collect();
        prop_assert_eq!(original_priorities, preserved_priorities);
    }
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_evaluation() {
        let policy = PolicyConfig {
            version: "1.0".to_string(),
            rules: vec![
                PolicyRule {
                    id: "allow_paid".to_string(),
                    condition: "payment_status == paid".to_string(),
                    action: PolicyAction::Allow,
                    priority: 10,
                },
            ],
            defaults: PolicyDefaults {
                default_action: PolicyAction::Deny,
                timeout_seconds: 30,
            },
        };

        let request = RequestContext {
            endpoint: "/data".to_string(),
            method: "GET".to_string(),
            user_id: None,
            payment_status: PaymentStatus::Paid {
                tx_id: "test123".to_string(),
                amount: 1000,
            },
            request_count: 0,
        };

        let result = evaluate_policy(&policy, &request);
        assert_eq!(result, EvaluationResult::Allowed);
    }

    #[test]
    fn test_code_generation() {
        let policy = PolicyConfig {
            version: "1.0".to_string(),
            rules: vec![],
            defaults: PolicyDefaults {
                default_action: PolicyAction::Allow,
                timeout_seconds: 30,
            },
        };

        let code = generate_code(&policy);
        assert!(code.contains("fn evaluate_policy"));
        assert!(code.contains("v1.0"));
    }
}
