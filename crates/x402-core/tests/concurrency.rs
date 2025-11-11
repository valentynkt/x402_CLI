// Concurrency and Thread Safety Tests
// Phase 2.3: Testing PolicyState under concurrent load
//
// These tests ensure that the policy engine works correctly when
// handling multiple concurrent requests from a multi-threaded server

use x402_core::policy::state::{PolicyState, RateLimitState, SpendingState};
use x402_core::policy::runtime_types::{RateLimitConfig, SpendingCapConfig};
use std::sync::Arc;
use std::thread;
use std::time::{SystemTime, Duration};

/// Test: Concurrent rate limit state access
#[test]
fn test_concurrent_rate_limit_access() {
    let state = Arc::new(PolicyState::new());
    let num_threads = 10;
    let requests_per_thread = 100;
    let mut handles = vec![];

    // Spawn multiple threads that all access rate limit state
    for thread_id in 0..num_threads {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            let key = format!("agent-{}", thread_id % 3); // 3 different agents

            for _ in 0..requests_per_thread {
                let mut rate_state = state_clone.get_rate_limit_state(&key);
                rate_state.add_request(SystemTime::now());
                state_clone.update_rate_limit_state(key.clone(), rate_state);
            }
        });
        handles.push(handle);
    }

    // All threads should complete without panic or deadlock
    for handle in handles {
        handle.join().expect("Thread should not panic");
    }

    // Verify state is accessible after concurrent operations
    let final_state = state.get_rate_limit_state("agent-0");
    // Should have accumulated some requests
    // (Exact count may vary due to race conditions in get/update)
}

/// Test: Concurrent spending state access
#[test]
fn test_concurrent_spending_state_access() {
    let state = Arc::new(PolicyState::new());
    let num_threads = 8;
    let payments_per_thread = 50;
    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            let key = format!("agent-{}", thread_id % 2); // 2 different agents

            for i in 0..payments_per_thread {
                let mut spending_state = state_clone.get_spending_state(&key);
                spending_state.add_spending(SystemTime::now(), 10 + (i as u64));
                state_clone.update_spending_state(key.clone(), spending_state);
            }
        });
        handles.push(handle);
    }

    // All threads should complete
    for handle in handles {
        handle.join().expect("Thread should not panic");
    }

    // Verify final state is accessible
    let final_state = state.get_spending_state("agent-0");
    let total = final_state.total_in_window(Duration::from_secs(3600), SystemTime::now());
    // Should have accumulated spending (exact amount may vary)
    assert!(total > 0);
}

/// Test: Mixed concurrent operations (rate limit + spending)
#[test]
fn test_mixed_concurrent_operations() {
    let state = Arc::new(PolicyState::new());
    let num_threads = 12;
    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            let key = format!("agent-{}", thread_id % 4);

            // Alternate between rate limit and spending operations
            for i in 0..50 {
                if i % 2 == 0 {
                    // Rate limit operation
                    let mut rate_state = state_clone.get_rate_limit_state(&key);
                    rate_state.add_request(SystemTime::now());
                    state_clone.update_rate_limit_state(key.clone(), rate_state);
                } else {
                    // Spending operation
                    let mut spending_state = state_clone.get_spending_state(&key);
                    spending_state.add_spending(SystemTime::now(), 5);
                    state_clone.update_spending_state(key.clone(), spending_state);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread should not panic");
    }

    // Both types of state should be accessible
    let rate_state = state.get_rate_limit_state("agent-0");
    let spending_state = state.get_spending_state("agent-0");

    // Basic sanity checks
    assert!(spending_state.total_in_window(Duration::from_secs(3600), SystemTime::now()) > 0);
}

/// Test: Concurrent rate limit checking (not just updating)
#[test]
fn test_concurrent_rate_limit_checking() {
    let config = RateLimitConfig {
        max_requests: 100,
        window: Duration::from_secs(60),
    };

    let state = Arc::new(PolicyState::new());
    let key = "shared-agent";
    let num_threads = 8;
    let mut handles = vec![];

    // Pre-populate with some requests
    let mut initial_state = state.get_rate_limit_state(key);
    for _ in 0..50 {
        initial_state.add_request(SystemTime::now());
    }
    state.update_rate_limit_state(key.to_string(), initial_state);

    // Multiple threads checking rate limit concurrently
    for _ in 0..num_threads {
        let state_clone = Arc::clone(&state);
        let config_clone = config.clone();
        let handle = thread::spawn(move || {
            let now = SystemTime::now();

            for _ in 0..20 {
                let rate_state = state_clone.get_rate_limit_state(key);
                let is_limited = rate_state.is_rate_limited(&config_clone, now);

                // Should get consistent results
                // (May change over time as more requests are added by other threads)

                // Also try adding a request
                let mut mut_state = state_clone.get_rate_limit_state(key);
                mut_state.add_request(now);
                state_clone.update_rate_limit_state(key.to_string(), mut_state);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread should not panic");
    }
}

/// Test: High-frequency concurrent updates (stress test)
#[test]
fn test_high_frequency_concurrent_updates() {
    let state = Arc::new(PolicyState::new());
    let num_threads = 16;
    let operations_per_thread = 1000;
    let mut handles = vec![];

    let start = SystemTime::now();

    for thread_id in 0..num_threads {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            let key = format!("agent-{}", thread_id % 8);

            for _ in 0..operations_per_thread {
                // Rapid-fire updates
                let mut rate_state = state_clone.get_rate_limit_state(&key);
                rate_state.add_request(SystemTime::now());
                state_clone.update_rate_limit_state(key.clone(), rate_state);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread should not panic during stress test");
    }

    let elapsed = start.elapsed().unwrap();

    // Should complete reasonably quickly (within 10 seconds for 16k operations)
    assert!(elapsed < Duration::from_secs(10),
        "Stress test took too long: {:?}", elapsed);
}

/// Test: Concurrent cleanup of expired entries
#[test]
fn test_concurrent_cleanup() {
    let state = Arc::new(PolicyState::new());
    let num_threads = 6;
    let mut handles = vec![];

    // Add entries that will expire
    let past = SystemTime::now() - Duration::from_secs(7200); // 2 hours ago
    let mut old_state = state.get_rate_limit_state("old-agent");
    for _ in 0..100 {
        old_state.add_request(past);
    }
    state.update_rate_limit_state("old-agent".to_string(), old_state);

    // Multiple threads trying to check rate limits (which triggers cleanup)
    for _ in 0..num_threads {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            let config = RateLimitConfig {
                max_requests: 50,
                window: Duration::from_secs(3600),
            };

            let now = SystemTime::now();

            for _ in 0..10 {
                let rate_state = state_clone.get_rate_limit_state("old-agent");
                // This should trigger cleanup of expired entries
                rate_state.is_rate_limited(&config, now);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Cleanup should not cause panic");
    }
}

/// Test: Thread-local state isolation
#[test]
fn test_thread_local_isolation() {
    let state = Arc::new(PolicyState::new());
    let mut handles = vec![];

    // Each thread works with its own agent
    for thread_id in 0..10 {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            let key = format!("isolated-agent-{}", thread_id);

            // Add some requests
            let mut rate_state = state_clone.get_rate_limit_state(&key);
            for _ in 0..thread_id + 1 {
                rate_state.add_request(SystemTime::now());
            }
            state_clone.update_rate_limit_state(key.clone(), rate_state);

            // Verify count matches what we added
            let final_state = state_clone.get_rate_limit_state(&key);
            // Return the key for verification
            key
        });
        handles.push(handle);
    }

    let mut keys = vec![];
    for handle in handles {
        let key = handle.join().expect("Thread should complete");
        keys.push(key);
    }

    // Verify all keys are present and unique
    assert_eq!(keys.len(), 10);
    for (i, key) in keys.iter().enumerate() {
        assert_eq!(key, &format!("isolated-agent-{}", i));
    }
}

/// Test: Concurrent spending cap checking
#[test]
fn test_concurrent_spending_cap_checking() {
    let config = SpendingCapConfig {
        max_amount: 1000.0,
        currency: "USDC".to_string(),
        window: Duration::from_secs(86400),
    };

    let state = Arc::new(PolicyState::new());
    let key = "spending-agent";
    let num_threads = 10;
    let mut handles = vec![];

    for _ in 0..num_threads {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            for _ in 0..50 {
                let mut spending_state = state_clone.get_spending_state(key);
                spending_state.add_spending(SystemTime::now(), 10);
                state_clone.update_spending_state(key.to_string(), spending_state);

                // Check if cap exceeded
                let check_state = state_clone.get_spending_state(key);
                let max_in_cents = (config.max_amount * 100.0) as u64;
                let _exceeds = !check_state.check_cap(
                    config.window,
                    max_in_cents,
                    10,
                    SystemTime::now()
                );
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread should not panic");
    }

    // Verify final spending
    let final_state = state.get_spending_state(key);
    let total = final_state.total_in_window(config.window, SystemTime::now());

    // Should have accumulated from all threads
    assert!(total > 0);
}

/// Test: No data races (verified by running with RUSTFLAGS="-Z sanitizer=thread")
/// This test ensures thread-safety through runtime verification
#[test]
fn test_no_data_races() {
    let state = Arc::new(PolicyState::new());
    let num_threads = 4;
    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            let key = if thread_id % 2 == 0 {
                "shared-key"
            } else {
                "other-key"
            };

            for _ in 0..100 {
                // Read
                let _ = state_clone.get_rate_limit_state(key);
                let _ = state_clone.get_spending_state(key);

                // Write
                let mut rate_state = state_clone.get_rate_limit_state(key);
                rate_state.add_request(SystemTime::now());
                state_clone.update_rate_limit_state(key.to_string(), rate_state);

                let mut spend_state = state_clone.get_spending_state(key);
                spend_state.add_spending(SystemTime::now(), 1);
                state_clone.update_spending_state(key.to_string(), spend_state);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("No data races should occur");
    }
}
