// Security Tests - Testing protection against malicious inputs
// These tests specifically target security vulnerabilities that could be exploited

use x402_core::policy::state::{RateLimitState, SpendingState};
use std::time::{SystemTime, Duration};

/// Test: Future timestamp attack on rate limiting
///
/// VULNERABILITY: Attacker could bypass rate limits by submitting requests
/// with future timestamps. Without upper bound check (time <= now), these
/// would be counted in the sliding window calculation.
///
/// SECURITY FIX: Added upper bound check in check_limit() to reject future timestamps
#[test]
fn test_future_timestamp_attack_rate_limit() {
    let mut state = RateLimitState::new();
    let now = SystemTime::now();
    let window = Duration::from_secs(60);
    let max_requests = 2;

    // Attacker makes 1 legitimate request
    state.add_request(now);

    // Should still allow 1 more (1 < 2)
    assert!(state.check_limit(window, max_requests, now));

    // Attacker tries to bypass limit by adding future timestamps
    state.add_request(now + Duration::from_secs(100));
    state.add_request(now + Duration::from_secs(200));

    // SECURITY CHECK: Future timestamps should NOT count toward limit
    // Should still allow 1 more since only 1 legitimate request is within [window_start, now]
    assert!(state.check_limit(window, max_requests, now),
            "Future timestamp attack successful - security vulnerability!");

    // Add second legitimate request
    state.add_request(now);

    // Now should hit limit (2 requests within valid window)
    assert!(!state.check_limit(window, max_requests, now),
            "Rate limit should be enforced after 2 valid requests");
}

/// Test: Future timestamp attack on spending cap
///
/// VULNERABILITY: Similar to rate limiting, attacker could manipulate spending
/// tracking by using future timestamps.
///
/// SECURITY FIX: total_in_window() already has upper bound check
#[test]
fn test_future_timestamp_attack_spending() {
    let mut state = SpendingState::new();
    let now = SystemTime::now();
    let window = Duration::from_secs(86400); // 24 hours
    let max_amount = 1000u64;

    // Legitimate spending
    state.add_spending(now, 500);

    // Should allow another 500
    assert!(state.check_cap(window, max_amount, 500, now));

    // Attacker tries to bypass cap by adding future spending
    state.add_spending(now + Duration::from_secs(100000), 300);
    state.add_spending(now + Duration::from_secs(200000), 300);

    // SECURITY CHECK: Future spending should NOT count toward cap
    // Should still allow another 500 since only 500 is within valid window
    assert!(state.check_cap(window, max_amount, 500, now),
            "Future timestamp attack bypassed spending cap!");

    // Total should only include legitimate spending
    assert_eq!(state.total_in_window(window, now), 500,
               "Future spending should not be counted");
}

/// Test: Massive future timestamp attack (stress test)
#[test]
fn test_massive_future_timestamp_flood() {
    let mut state = RateLimitState::new();
    let now = SystemTime::now();
    let window = Duration::from_secs(60);
    let max_requests = 10;

    // Make 5 legitimate requests
    for _ in 0..5 {
        state.add_request(now);
    }

    // Attacker floods with 1000 future requests
    for i in 0..1000 {
        state.add_request(now + Duration::from_secs(60 + i));
    }

    // SECURITY CHECK: Should still have room for 5 more requests
    // Only the 5 legitimate requests should count
    assert!(state.check_limit(window, max_requests, now),
            "Massive future timestamp flood bypassed rate limit!");

    // Count should be exactly 5
    assert_eq!(state.count_in_window(window, now), 5,
               "Future timestamps should not be counted");
}

/// Test: Past timestamp attack (expired requests)
#[test]
fn test_expired_timestamp_attack() {
    let mut state = RateLimitState::new();
    let now = SystemTime::now();
    let window = Duration::from_secs(60);
    let max_requests = 3;

    // Add requests from 2 hours ago (outside window)
    let old_time = now.checked_sub(Duration::from_secs(7200)).unwrap();
    for _ in 0..5 {
        state.add_request(old_time);
    }

    // Should not count toward current limit
    assert!(state.check_limit(window, max_requests, now),
            "Expired requests should not count");

    // Add 3 recent requests
    for _ in 0..3 {
        state.add_request(now);
    }

    // Should now be at limit
    assert!(!state.check_limit(window, max_requests, now),
            "Should be rate limited after 3 current requests");
}

/// Test: Cleanup properly removes both expired and future timestamps
#[test]
fn test_cleanup_removes_invalid_timestamps() {
    let mut state = RateLimitState::new();
    let now = SystemTime::now();

    // Add various timestamps
    let old_time = now.checked_sub(Duration::from_secs(7200)).unwrap(); // 2 hours ago
    state.add_request(old_time); // Should be removed
    state.add_request(now); // Should stay
    state.add_request(now + Duration::from_secs(100)); // Future - should be removed

    // Before cleanup
    assert_eq!(state.count_in_window(Duration::from_secs(3600), now), 1);

    // Run cleanup
    state.cleanup_expired(now);

    // After cleanup - only the valid timestamp should remain
    // Note: cleanup uses a 1-hour safety margin, so recent requests stay
    let valid_count = state.count_in_window(Duration::from_secs(3600), now);
    assert_eq!(valid_count, 1, "Only valid timestamp should remain after cleanup");
}

/// Test: Spending cap with mixed valid and invalid timestamps
#[test]
fn test_spending_mixed_timestamps() {
    let mut state = SpendingState::new();
    let now = SystemTime::now();
    let window = Duration::from_secs(3600);

    // Old spending (outside window)
    let old = now.checked_sub(Duration::from_secs(7200)).unwrap();
    state.add_spending(old, 500);

    // Current spending
    state.add_spending(now, 300);

    // Future spending (invalid)
    state.add_spending(now + Duration::from_secs(1000), 400);

    // Only current spending should count
    assert_eq!(state.total_in_window(window, now), 300,
               "Only current spending within valid window should be counted");
}

/// Test: Consistency between check_limit and count_in_window
///
/// CRITICAL: These two methods must give consistent results. Previously,
/// count_in_window had upper bound check but check_limit didn't, creating
/// an inconsistency that could be exploited.
#[test]
fn test_check_limit_count_in_window_consistency() {
    let mut state = RateLimitState::new();
    let now = SystemTime::now();
    let window = Duration::from_secs(60);
    let max_requests = 5;

    // Add mix of timestamps
    state.add_request(now);
    state.add_request(now + Duration::from_secs(100)); // Future
    state.add_request(now);
    state.add_request(now + Duration::from_secs(200)); // Future
    state.add_request(now);

    // Count valid requests
    let count = state.count_in_window(window, now);

    // check_limit should be consistent with count
    let has_capacity = state.check_limit(window, max_requests, now);
    let expected_has_capacity = count < max_requests as usize;

    assert_eq!(has_capacity, expected_has_capacity,
               "check_limit and count_in_window must be consistent! \
                count={}, max={}, has_capacity={}, expected={}",
                count, max_requests, has_capacity, expected_has_capacity);
}

/// Test: Edge case - request timestamp exactly at window boundary
#[test]
fn test_window_boundary_timestamps() {
    let mut state = RateLimitState::new();
    let now = SystemTime::now();
    let window = Duration::from_secs(60);

    // Request exactly at window start
    let window_start = now.checked_sub(window).unwrap();
    state.add_request(window_start);

    // Request exactly at now
    state.add_request(now);

    // Both should be counted (inclusive boundaries)
    assert_eq!(state.count_in_window(window, now), 2,
               "Boundary timestamps should be included");
}

/// Test: Zero-width window edge case
#[test]
fn test_zero_window() {
    let mut state = RateLimitState::new();
    let now = SystemTime::now();
    let zero_window = Duration::from_secs(0);

    // Add requests slightly in the past
    state.add_request(now.checked_sub(Duration::from_millis(1)).unwrap());
    state.add_request(now);

    // With zero window, only exact timestamp match should count
    let count = state.count_in_window(zero_window, now);
    assert_eq!(count, 1, "Zero window should only count exact timestamp matches");
}
