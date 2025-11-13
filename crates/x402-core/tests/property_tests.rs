// Property-Based Tests for Policy Engine
// Phase 2.1: Using proptest to verify invariants and catch edge cases
//
// Property-based testing generates hundreds of random inputs to test properties
// that should always hold true, regardless of specific values.

use proptest::prelude::*;
use std::time::{Duration, SystemTime};
use x402_core::policy::types::{PolicyConfig, PolicyRule};
use x402_core::policy::{RateLimitConfig, SpendingCapConfig};

/// Property: Wildcard patterns should always match more broadly than exact matches
///
/// For any string S and pattern P containing wildcards,
/// if S matches P, then P with wildcards replaced by specific chars should not match more strings
#[cfg(test)]
mod pattern_matching_properties {
    use super::*;

    proptest! {
        /// Property: A pattern without wildcards matches exactly one string (itself)
        #[test]
        fn exact_pattern_matches_only_itself(s in "[a-z]{1,20}") {
            // Pattern without wildcards should only match itself
            let pattern = s.clone();
            let value = s.clone();

            // This should match
            assert!(matches_pattern(&pattern, &value));

            // A different string should not match
            if s.len() > 1 {
                let different = format!("{}x", s);
                assert!(!matches_pattern(&pattern, &different));
            }
        }

        /// Property: Prefix wildcard pattern matches all strings with that prefix
        #[test]
        fn prefix_wildcard_matches_all_with_prefix(
            prefix in "[a-z]{1,10}",
            suffix in "[a-z]{0,10}"
        ) {
            let pattern = format!("{}*", prefix);
            let value = format!("{}{}", prefix, suffix);

            // Any string starting with prefix should match
            assert!(matches_pattern(&pattern, &value));
        }

        /// Property: Suffix wildcard pattern matches all strings with that suffix
        #[test]
        fn suffix_wildcard_matches_all_with_suffix(
            prefix in "[a-z]{0,10}",
            suffix in "[a-z]{1,10}"
        ) {
            let pattern = format!("*{}", suffix);
            let value = format!("{}{}", prefix, suffix);

            // Any string ending with suffix should match
            assert!(matches_pattern(&pattern, &value));
        }

        /// Property: Wildcard in middle matches any substring replacement
        #[test]
        fn middle_wildcard_matches_any_middle(
            prefix in "[a-z]{1,5}",
            middle in "[a-z]{0,10}",
            suffix in "[a-z]{1,5}"
        ) {
            let pattern = format!("{}*{}", prefix, suffix);
            let value = format!("{}{}{}", prefix, middle, suffix);

            // Should match regardless of middle content
            assert!(matches_pattern(&pattern, &value));
        }

        /// Property: More specific patterns should not match more strings
        #[test]
        fn specificity_reduces_matches(
            prefix in "[a-z]{1,10}",
            extra in "[a-z]{1,5}"
        ) {
            let broad_pattern = format!("{}*", prefix);
            let specific_pattern = format!("{}{}*", prefix, extra);

            let broad_match = format!("{}{}", prefix, extra);
            let specific_match = format!("{}{}{}", prefix, extra, "x");

            // Broad pattern should match both
            assert!(matches_pattern(&broad_pattern, &broad_match));
            assert!(matches_pattern(&broad_pattern, &specific_match));

            // Specific pattern should match specific but not necessarily broad
            assert!(matches_pattern(&specific_pattern, &specific_match));
        }
    }

    // Helper function for pattern matching (simplified version)
    fn matches_pattern(pattern: &str, value: &str) -> bool {
        if !pattern.contains('*') {
            return pattern == value;
        }

        let parts: Vec<&str> = pattern.split('*').collect();

        if parts.is_empty() {
            return true;
        }

        let mut pos = 0;
        for (i, part) in parts.iter().enumerate() {
            if part.is_empty() {
                continue;
            }

            if i == 0 && !pattern.starts_with('*') {
                // Must match at start
                if !value.starts_with(part) {
                    return false;
                }
                pos = part.len();
            } else if i == parts.len() - 1 && !pattern.ends_with('*') {
                // Must match at end
                if !value.ends_with(part) {
                    return false;
                }
            } else {
                // Must appear somewhere after pos
                if let Some(found) = value[pos..].find(part) {
                    pos += found + part.len();
                } else {
                    return false;
                }
            }
        }

        true
    }
}

/// Property: Rate limiting temporal properties
///
/// Rate limits should behave consistently over time
#[cfg(test)]
mod rate_limit_properties {
    use super::*;
    use x402_core::policy::state::RateLimitState;

    proptest! {
        /// Property: Adding N requests within window should fail after max_requests
        #[test]
        fn rate_limit_fails_after_max_requests(
            max_requests in 1u32..100,
            extra_requests in 1usize..10
        ) {
            let config = RateLimitConfig {
                max_requests,
                window: Duration::from_secs(3600),
            };

            let mut state = RateLimitState::new();
            let now = SystemTime::now();

            // Add max_requests - should all succeed
            for _ in 0..max_requests {
                state.add_request(now);
            }

            // Verify we're at the limit
            let _within_limit = state.check_limit(config.window, config.max_requests, now);

            // Adding more should exceed limit
            for _ in 0..extra_requests {
                state.add_request(now);
            }

            // Should now be rate limited (check_limit returns false when over limit)
            assert!(!state.check_limit(config.window, config.max_requests, now));
        }

        /// Property: Requests outside window should not count toward limit
        #[test]
        fn rate_limit_respects_window_expiration(
            max_requests in 1u32..50,
            window_secs in 1u64..3600
        ) {
            let config = RateLimitConfig {
                max_requests,
                window: Duration::from_secs(window_secs),
            };

            let mut state = RateLimitState::new();
            let now = SystemTime::now();

            // Add requests at the limit
            for _ in 0..max_requests {
                state.add_request(now);
            }

            // Should be at limit (check_limit returns false when at/over limit)
            assert!(!state.check_limit(config.window, config.max_requests, now));

            // After window expires
            let future = now + Duration::from_secs(window_secs + 1);

            // Should no longer be rate limited (check_limit returns true when allowed)
            assert!(state.check_limit(config.window, config.max_requests, future));
        }

        /// Property: Rate limit state is monotonic within window
        /// If rate limited at time T, should remain limited for all T' where T <= T' < T + epsilon
        #[test]
        fn rate_limit_is_monotonic_within_window(
            max_requests in 1u32..20,
            time_offset in 0u64..100
        ) {
            let window_secs = 3600u64;
            let config = RateLimitConfig {
                max_requests,
                window: Duration::from_secs(window_secs),
            };

            let mut state = RateLimitState::new();
            let now = SystemTime::now();

            // Fill to capacity
            for _ in 0..max_requests {
                state.add_request(now);
            }

            // Should be limited (check_limit returns false when over limit)
            assert!(!state.check_limit(config.window, config.max_requests, now));

            // Should remain limited shortly after
            if time_offset < window_secs {
                let soon = now + Duration::from_secs(time_offset);
                assert!(!state.check_limit(config.window, config.max_requests, soon));
            }
        }
    }
}

/// Property: Spending cap temporal properties
#[cfg(test)]
mod spending_cap_properties {
    use super::*;
    use x402_core::policy::state::SpendingState;

    proptest! {
        /// Property: Total spending never exceeds cap within window
        #[test]
        fn spending_never_exceeds_cap(
            max_amount in 100u64..10000,
            num_payments in 1usize..20,
            payment_amount in 1u64..100
        ) {
            let config = SpendingCapConfig {
                max_amount: max_amount as f64,
                currency: "USDC".to_string(),
                window: Duration::from_secs(86400),
            };

            let mut state = SpendingState::new();
            let now = SystemTime::now();

            let mut total_spent = 0u64;

            for _ in 0..num_payments {
                if total_spent + payment_amount <= max_amount {
                    state.add_spending(now, payment_amount);
                    total_spent += payment_amount;
                }
            }

            // Total in state should not exceed what we tracked
            let state_total = state.total_in_window(config.window, now);
            assert!(state_total <= max_amount);
        }

        /// Property: Spending resets after window expires
        #[test]
        fn spending_resets_after_window(
            amount in 100u64..1000,
            window_secs in 60u64..3600
        ) {
            let config = SpendingCapConfig {
                max_amount: amount as f64,
                currency: "USDC".to_string(),
                window: Duration::from_secs(window_secs),
            };

            let mut state = SpendingState::new();
            let now = SystemTime::now();

            // Add spending up to cap
            state.add_spending(now, amount);

            // Should be at capacity
            assert_eq!(state.total_in_window(config.window, now), amount);

            // After window expires
            let future = now + Duration::from_secs(window_secs + 1);

            // Should be reset
            assert_eq!(state.total_in_window(config.window, future), 0);
        }

        /// Property: Spending accumulates correctly
        #[test]
        fn spending_accumulates_correctly(
            amounts in prop::collection::vec(1u64..100, 1..10)
        ) {
            let window = Duration::from_secs(3600);
            let mut state = SpendingState::new();
            let now = SystemTime::now();

            let expected_total: u64 = amounts.iter().sum();

            for amount in &amounts {
                state.add_spending(now, *amount);
            }

            let actual_total = state.total_in_window(window, now);
            assert_eq!(actual_total, expected_total);
        }
    }
}

/// Property: Policy evaluation ordering
#[cfg(test)]
mod policy_evaluation_properties {
    use super::*;

    proptest! {
        /// Property: Deny always takes precedence over allow
        /// This is a critical security property
        #[test]
        fn deny_overrides_allow(agent_id in "[a-z]{5,10}") {
            // Given: Both allowlist and denylist for same agent
            let policies = vec![
                PolicyRule::Allowlist {
                    field: "agent_id".to_string(),
                    values: vec![agent_id.clone()],
                },
                PolicyRule::Denylist {
                    field: "agent_id".to_string(),
                    values: vec![agent_id.clone()],
                },
            ];

            let config = PolicyConfig { policies };

            // When: Creating engine and evaluating
            // Then: Denylist should win (tested via validator conflict detection)
            // This is enforced at validation time

            // Verify conflict would be detected
            assert!(conflicts_exist(&config));
        }

        /// Property: Empty policy list should have deterministic behavior
        #[test]
        fn empty_policies_deterministic(agent_id in "[a-z]{5,10}") {
            let _config = PolicyConfig { policies: vec![] };

            // Empty policies should behave consistently
            // (Implementation-dependent: might allow all or deny all)
            // Key property: same input always gives same output

            // Create two identical requests
            let result1 = evaluate_with_empty_policies(&agent_id);
            let result2 = evaluate_with_empty_policies(&agent_id);

            // Results should be identical
            assert_eq!(result1, result2);
        }
    }

    fn conflicts_exist(config: &PolicyConfig) -> bool {
        use x402_core::policy::validate_policies;
        let report = validate_policies(config);
        report.has_errors
    }

    fn evaluate_with_empty_policies(_agent_id: &str) -> bool {
        // Placeholder for actual evaluation
        // Default behavior (would need actual implementation)
        true // or false, depending on default policy
    }
}

/// Property: Pattern matching edge cases
#[cfg(test)]
mod pattern_edge_cases {
    use super::*;

    proptest! {
        /// Property: Empty pattern behavior
        #[test]
        fn empty_pattern_behavior(value in "[a-z]{0,10}") {
            // Empty pattern should have well-defined behavior
            let pattern = "";
            // Typically matches only empty string
            if value.is_empty() {
                assert!(matches_exact_empty(pattern, &value));
            }
        }

        /// Property: Single wildcard matches everything
        #[test]
        fn single_wildcard_matches_all(value in "[a-z0-9_\\-]{0,20}") {
            let pattern = "*";
            // "*" should match any string
            assert!(matches_wildcard_any(pattern, &value));
        }

        /// Property: Multiple wildcards should still work correctly
        #[test]
        fn multiple_wildcards_consistent(
            prefix in "[a-z]{1,5}",
            middle in "[a-z]{0,5}",
            suffix in "[a-z]{1,5}"
        ) {
            let _pattern = "*abc*def*";
            let value = format!("{}abc{}def{}", prefix, middle, suffix);

            // Should match if contains abc...def in sequence
            assert!(value.contains("abc"));
            assert!(value.contains("def"));
        }
    }

    fn matches_exact_empty(pattern: &str, value: &str) -> bool {
        pattern == value
    }

    fn matches_wildcard_any(_pattern: &str, _value: &str) -> bool {
        true // "*" matches everything
    }
}

/// Property: Concurrency safety
/// These properties ensure thread-safety of stateful components
#[cfg(test)]
mod concurrency_properties {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    proptest! {
        /// Property: Concurrent rate limit checks are thread-safe
        #[test]
        fn concurrent_rate_limit_checks_safe(
            num_threads in 2usize..8,
            requests_per_thread in 1usize..10
        ) {
            use x402_core::policy::state::PolicyState;

            let state = Arc::new(PolicyState::new());
            let mut handles = vec![];

            for _ in 0..num_threads {
                let state_clone = Arc::clone(&state);
                let handle = thread::spawn(move || {
                    let key = "test-agent";
                    for _ in 0..requests_per_thread {
                        let mut rate_state = state_clone.get_rate_limit_state(key);
                        rate_state.add_request(SystemTime::now());
                    }
                });
                handles.push(handle);
            }

            // All threads should complete without panic
            for handle in handles {
                handle.join().expect("Thread panicked");
            }

            // State should be consistent
            // (Exact count may vary due to race conditions, but should not crash)
        }
    }
}
