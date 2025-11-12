// State tracking for policy evaluation

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

/// Thread-safe policy state management
#[derive(Debug, Clone)]
pub struct PolicyState {
    rate_limits: Arc<RwLock<HashMap<String, RateLimitState>>>,
    spending: Arc<RwLock<HashMap<String, SpendingState>>>,
}

impl PolicyState {
    /// Create a new empty policy state
    pub fn new() -> Self {
        Self {
            rate_limits: Arc::new(RwLock::new(HashMap::new())),
            spending: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get or create rate limit state for a key
    pub fn get_rate_limit_state(&self, key: &str) -> RateLimitState {
        let state = self.rate_limits.read()
            .expect("CRITICAL: Rate limit state lock poisoned - thread panic detected");
        state.get(key).cloned().unwrap_or_default()
    }

    /// Update rate limit state for a key
    pub fn update_rate_limit_state(&self, key: String, state: RateLimitState) {
        let mut states = self.rate_limits.write()
            .expect("CRITICAL: Rate limit state lock poisoned - thread panic detected");
        states.insert(key, state);
    }

    /// Get or create spending state for a key
    pub fn get_spending_state(&self, key: &str) -> SpendingState {
        let state = self.spending.read()
            .expect("CRITICAL: Spending state lock poisoned - thread panic detected");
        state.get(key).cloned().unwrap_or_default()
    }

    /// Update spending state for a key
    pub fn update_spending_state(&self, key: String, state: SpendingState) {
        let mut states = self.spending.write()
            .expect("CRITICAL: Spending state lock poisoned - thread panic detected");
        states.insert(key, state);
    }

    /// Clear expired entries from all states
    pub fn cleanup_expired(&self, now: SystemTime) {
        // Cleanup rate limits
        {
            let mut states = self.rate_limits.write()
                .expect("CRITICAL: Rate limit state lock poisoned - thread panic detected");
            for state in states.values_mut() {
                state.cleanup_expired(now);
            }
        }

        // Cleanup spending
        {
            let mut states = self.spending.write()
                .expect("CRITICAL: Spending state lock poisoned - thread panic detected");
            for state in states.values_mut() {
                state.cleanup_expired(now);
            }
        }
    }
}

impl Default for PolicyState {
    fn default() -> Self {
        Self::new()
    }
}

/// Rate limiting state using sliding window algorithm
#[derive(Debug, Clone, Default)]
pub struct RateLimitState {
    /// Timestamps of requests within the current window
    request_times: Vec<SystemTime>,
}

impl RateLimitState {
    /// Create a new empty rate limit state
    pub fn new() -> Self {
        Self {
            request_times: Vec::new(),
        }
    }

    /// Check if a request would exceed the rate limit
    ///
    /// Uses sliding window algorithm: counts requests within `window` duration
    /// from the current time.
    ///
    /// # Security: Future Timestamp Protection
    /// Also enforces upper bound (time <= now) to prevent attackers from bypassing
    /// rate limits by submitting requests with future timestamps.
    pub fn check_limit(&self, window: Duration, max_requests: u32, now: SystemTime) -> bool {
        let window_start = now.checked_sub(window).unwrap_or(now);

        // Count requests within the sliding window
        // SECURITY: Must check both lower AND upper bounds to prevent future timestamp attacks
        let count = self
            .request_times
            .iter()
            .filter(|&&time| time >= window_start && time <= now)
            .count();

        count < max_requests as usize
    }

    /// Record a new request
    pub fn add_request(&mut self, timestamp: SystemTime) {
        self.request_times.push(timestamp);
    }

    /// Remove expired request timestamps outside the window
    pub fn cleanup_expired(&mut self, now: SystemTime) {
        // Keep only recent requests (last hour for safety margin)
        // SECURITY: Also reject future timestamps to prevent time manipulation attacks
        let cutoff = now.checked_sub(Duration::from_secs(3600)).unwrap_or(now);
        self.request_times.retain(|&time| time >= cutoff && time <= now);
    }

    /// Get current request count within window
    pub fn count_in_window(&self, window: Duration, now: SystemTime) -> usize {
        let window_start = now.checked_sub(window).unwrap_or(now);
        self.request_times
            .iter()
            .filter(|&&time| time >= window_start && time <= now)
            .count()
    }
}

/// Spending tracking state with time window
#[derive(Debug, Clone, Default)]
pub struct SpendingState {
    /// Amounts spent with timestamps
    spending_records: Vec<(SystemTime, u64)>,
}

impl SpendingState {
    /// Create a new empty spending state
    pub fn new() -> Self {
        Self {
            spending_records: Vec::new(),
        }
    }

    /// Check if adding an amount would exceed the spending cap
    pub fn check_cap(&self, window: Duration, max_amount: u64, amount: u64, now: SystemTime) -> bool {
        let current_total = self.total_in_window(window, now);
        current_total + amount <= max_amount
    }

    /// Record a new spending transaction
    pub fn add_spending(&mut self, timestamp: SystemTime, amount: u64) {
        self.spending_records.push((timestamp, amount));
    }

    /// Calculate total spending within the time window
    pub fn total_in_window(&self, window: Duration, now: SystemTime) -> u64 {
        let window_start = now.checked_sub(window).unwrap_or(now);

        self.spending_records
            .iter()
            .filter(|(time, _)| *time >= window_start && *time <= now)
            .map(|(_, amount)| amount)
            .sum()
    }

    /// Remove expired spending records outside the window
    pub fn cleanup_expired(&mut self, now: SystemTime) {
        // Keep only recent records (last hour for safety margin)
        // SECURITY: Also reject future timestamps to prevent time manipulation attacks
        let cutoff = now.checked_sub(Duration::from_secs(3600)).unwrap_or(now);
        self.spending_records.retain(|(time, _)| *time >= cutoff && *time <= now);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::UNIX_EPOCH;

    #[test]
    fn test_rate_limit_sliding_window() {
        let mut state = RateLimitState::new();
        let now = SystemTime::now();
        let window = Duration::from_secs(60);

        // Add 3 requests
        state.add_request(now);
        state.add_request(now);
        state.add_request(now);

        // Should allow with limit of 5
        assert!(state.check_limit(window, 5, now));

        // Should deny with limit of 3
        assert!(!state.check_limit(window, 3, now));

        // Should allow with limit of 3 (we have exactly 3)
        assert!(state.check_limit(window, 4, now));
    }

    #[test]
    fn test_rate_limit_expiration() {
        let mut state = RateLimitState::new();
        let base_time = UNIX_EPOCH + Duration::from_secs(1000);
        let window = Duration::from_secs(60);

        // Add requests at different times
        state.add_request(base_time);
        state.add_request(base_time + Duration::from_secs(30));
        state.add_request(base_time + Duration::from_secs(70)); // Outside window from base_time

        // Check from base_time perspective (only first request in window)
        let count = state.count_in_window(window, base_time);
        assert_eq!(count, 1);

        // Check from later time (second and third in window)
        let later = base_time + Duration::from_secs(70);
        let count = state.count_in_window(window, later);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_spending_cap_tracking() {
        let mut state = SpendingState::new();
        let now = SystemTime::now();
        let window = Duration::from_secs(3600); // 1 hour

        // Add spending
        state.add_spending(now, 100);
        state.add_spending(now, 200);

        // Total should be 300
        assert_eq!(state.total_in_window(window, now), 300);

        // Should allow 200 more with cap of 500
        assert!(state.check_cap(window, 500, 200, now));

        // Should deny 300 more with cap of 500
        assert!(!state.check_cap(window, 500, 300, now));
    }

    #[test]
    fn test_spending_window_expiration() {
        let mut state = SpendingState::new();
        let base_time = UNIX_EPOCH + Duration::from_secs(1000);
        let window = Duration::from_secs(60);

        // Add spending at different times
        state.add_spending(base_time, 100);
        state.add_spending(base_time + Duration::from_secs(30), 200);
        state.add_spending(base_time + Duration::from_secs(70), 300); // Outside window

        // From base_time perspective
        assert_eq!(state.total_in_window(window, base_time), 100);

        // From later time
        let later = base_time + Duration::from_secs(70);
        assert_eq!(state.total_in_window(window, later), 500); // 200 + 300
    }

    #[test]
    fn test_policy_state_thread_safety() {
        let state = PolicyState::new();

        // Test concurrent access
        let state_clone = state.clone();
        let handle = std::thread::spawn(move || {
            let mut rl_state = state_clone.get_rate_limit_state("test");
            rl_state.add_request(SystemTime::now());
            state_clone.update_rate_limit_state("test".to_string(), rl_state);
        });

        let mut rl_state = state.get_rate_limit_state("test2");
        rl_state.add_request(SystemTime::now());
        state.update_rate_limit_state("test2".to_string(), rl_state);

        handle.join().expect("Thread should not panic during test");

        // Both states should exist
        assert_eq!(state.get_rate_limit_state("test").request_times.len(), 1);
        assert_eq!(state.get_rate_limit_state("test2").request_times.len(), 1);
    }

    #[test]
    fn test_cleanup_expired() {
        let state = PolicyState::new();
        let old_time = UNIX_EPOCH + Duration::from_secs(1000);
        let now = SystemTime::now();

        // Add old rate limit entries
        let mut rl_state = RateLimitState::new();
        rl_state.add_request(old_time);
        rl_state.add_request(now);
        state.update_rate_limit_state("test".to_string(), rl_state);

        // Add old spending entries
        let mut sp_state = SpendingState::new();
        sp_state.add_spending(old_time, 100);
        sp_state.add_spending(now, 200);
        state.update_spending_state("test".to_string(), sp_state);

        // Cleanup
        state.cleanup_expired(now);

        // Check that recent entries remain
        let rl_state = state.get_rate_limit_state("test");
        assert!(rl_state.request_times.len() > 0);

        let sp_state = state.get_spending_state("test");
        assert!(sp_state.spending_records.len() > 0);
    }
}
