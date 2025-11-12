use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::collections::VecDeque;

/// Policy configuration loaded from policy.yaml
#[derive(Debug, Deserialize, Clone)]
struct PolicyConfig {
    version: String,
    spending_cap: SpendingCap,
    allowlist: Allowlist,
    rate_limit: RateLimit,
}

#[derive(Debug, Deserialize, Clone)]
struct SpendingCap {
    max_amount: f64,
    currency: String,
    period: String, // "hourly", "daily", "monthly"
}

#[derive(Debug, Deserialize, Clone)]
struct Allowlist {
    endpoints: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct RateLimit {
    requests_per_minute: u32,
}

/// AI Agent that enforces x402 payment policies
struct PolicyEnforcedAgent {
    client: Client,
    policy: PolicyConfig,
    current_spending: f64,
    spending_window_start: SystemTime,
    request_timestamps: VecDeque<SystemTime>,
}

impl PolicyEnforcedAgent {
    /// Create a new agent with policy enforcement
    fn new(policy: PolicyConfig) -> Self {
        Self {
            client: Client::new(),
            policy,
            current_spending: 0.0,
            spending_window_start: SystemTime::now(),
            request_timestamps: VecDeque::new(),
        }
    }

    /// Make an API call with policy enforcement
    ///
    /// This demonstrates all three policy types:
    /// 1. Spending cap - prevents budget overruns
    /// 2. Allowlist - restricts accessible endpoints
    /// 3. Rate limit - controls request frequency
    async fn call_api(&mut self, endpoint: &str, estimated_cost: f64) -> Result<String, String> {
        // POLICY 1: Check spending cap before making request
        // WHY: Prevents unexpected costs from runaway AI agents
        self.enforce_spending_cap(estimated_cost)?;

        // POLICY 2: Validate endpoint is in allowlist
        // WHY: Prevents access to unauthorized or expensive services
        self.enforce_allowlist(endpoint)?;

        // POLICY 3: Apply rate limiting
        // WHY: Prevents abuse and manages costs through request throttling
        self.enforce_rate_limit().await?;

        // Make the actual API call with x402 headers
        let response = self.client
            .get(endpoint)
            .header("X-402-Policy", "ai-agent-policy")
            .header("X-402-Budget-Remaining", self.remaining_budget().to_string())
            .header("X-402-Currency", &self.policy.spending_cap.currency)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        // Handle policy violation responses
        match response.status() {
            StatusCode::OK => {
                // Success - update spending tracker
                self.record_spending(estimated_cost);
                response.text().await.map_err(|e| format!("Failed to read response: {}", e))
            }
            StatusCode::PAYMENT_REQUIRED => {
                Err("Payment required - spending cap may be exceeded at server".to_string())
            }
            StatusCode::FORBIDDEN => {
                Err("Forbidden - endpoint not in server-side allowlist".to_string())
            }
            StatusCode::TOO_MANY_REQUESTS => {
                Err("Rate limit exceeded at server".to_string())
            }
            status => {
                Err(format!("Unexpected status: {}", status))
            }
        }
    }

    /// Enforce spending cap policy
    /// Prevents requests that would exceed the budget for current period
    fn enforce_spending_cap(&mut self, estimated_cost: f64) -> Result<(), String> {
        // Check if we need to reset the spending window
        let window_duration = match self.policy.spending_cap.period.as_str() {
            "hourly" => Duration::from_secs(3600),
            "daily" => Duration::from_secs(86400),
            "monthly" => Duration::from_secs(2592000), // 30 days
            _ => Duration::from_secs(86400), // default to daily
        };

        if self.spending_window_start.elapsed().unwrap() > window_duration {
            // Reset spending for new period
            self.current_spending = 0.0;
            self.spending_window_start = SystemTime::now();
        }

        // Check if this request would exceed the cap
        if self.current_spending + estimated_cost > self.policy.spending_cap.max_amount {
            return Err(format!(
                "Spending cap exceeded: {:.2} + {:.2} > {:.2} {}",
                self.current_spending,
                estimated_cost,
                self.policy.spending_cap.max_amount,
                self.policy.spending_cap.currency
            ));
        }

        Ok(())
    }

    /// Enforce allowlist policy
    /// Only permits calls to pre-approved endpoints
    fn enforce_allowlist(&self, endpoint: &str) -> Result<(), String> {
        // Check if endpoint matches any allowlist entry
        let is_allowed = self.policy.allowlist.endpoints.iter().any(|allowed| {
            // Support wildcard matching (e.g., /api/*)
            if allowed.ends_with("/*") {
                let prefix = &allowed[..allowed.len() - 2];
                endpoint.starts_with(prefix)
            } else {
                endpoint == allowed
            }
        });

        if !is_allowed {
            return Err(format!(
                "Endpoint '{}' not in allowlist. Allowed: {:?}",
                endpoint, self.policy.allowlist.endpoints
            ));
        }

        Ok(())
    }

    /// Enforce rate limit policy
    /// Throttles requests to prevent excessive API usage
    async fn enforce_rate_limit(&mut self) -> Result<(), String> {
        let now = SystemTime::now();
        let one_minute_ago = now - Duration::from_secs(60);

        // Remove timestamps older than 1 minute
        while let Some(&timestamp) = self.request_timestamps.front() {
            if timestamp < one_minute_ago {
                self.request_timestamps.pop_front();
            } else {
                break;
            }
        }

        // Check if we're at the rate limit
        if self.request_timestamps.len() >= self.policy.rate_limit.requests_per_minute as usize {
            // Calculate how long to wait
            if let Some(&oldest) = self.request_timestamps.front() {
                let wait_until = oldest + Duration::from_secs(60);
                if let Ok(wait_duration) = wait_until.duration_since(now) {
                    println!(
                        "Rate limit reached ({} req/min). Waiting {:.1}s...",
                        self.policy.rate_limit.requests_per_minute,
                        wait_duration.as_secs_f64()
                    );
                    tokio::time::sleep(wait_duration).await;
                }
            }
        }

        // Record this request timestamp
        self.request_timestamps.push_back(now);
        Ok(())
    }

    /// Record spending after successful request
    fn record_spending(&mut self, amount: f64) {
        self.current_spending += amount;
        println!(
            "Spent: {:.2} {} (Remaining: {:.2})",
            amount,
            self.policy.spending_cap.currency,
            self.remaining_budget()
        );
    }

    /// Calculate remaining budget for current period
    fn remaining_budget(&self) -> f64 {
        (self.policy.spending_cap.max_amount - self.current_spending).max(0.0)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load policy configuration
    let policy_yaml = include_str!("policy.yaml");
    let policy: PolicyConfig = serde_yaml::from_str(policy_yaml)?;

    println!("AI Agent starting with policies:");
    println!("  Spending Cap: {} {} per {}",
        policy.spending_cap.max_amount,
        policy.spending_cap.currency,
        policy.spending_cap.period
    );
    println!("  Rate Limit: {} requests/minute", policy.rate_limit.requests_per_minute);
    println!("  Allowlist: {} endpoints", policy.allowlist.endpoints.len());
    println!();

    // Create policy-enforced agent
    let mut agent = PolicyEnforcedAgent::new(policy);

    // Example 1: Successful API calls within policy
    println!("Example 1: Making allowed API calls");
    match agent.call_api("/api/data", 0.50).await {
        Ok(response) => println!("✓ Success: {}", response),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Example 2: Test rate limiting
    println!("\nExample 2: Testing rate limit (making rapid requests)");
    for i in 1..=5 {
        match agent.call_api("/api/ai-query", 0.25).await {
            Ok(_) => println!("✓ Request {} succeeded", i),
            Err(e) => println!("✗ Request {} failed: {}", i, e),
        }
    }

    // Example 3: Test allowlist violation
    println!("\nExample 3: Testing allowlist (accessing forbidden endpoint)");
    match agent.call_api("/api/forbidden", 1.0).await {
        Ok(_) => println!("✓ Unexpected success"),
        Err(e) => println!("✓ Correctly blocked: {}", e),
    }

    // Example 4: Test spending cap
    println!("\nExample 4: Testing spending cap");
    match agent.call_api("/api/data", 15.0).await {
        Ok(_) => println!("✗ Should have been blocked"),
        Err(e) => println!("✓ Correctly blocked: {}", e),
    }

    println!("\nPolicy enforcement demonstration complete!");
    Ok(())
}
