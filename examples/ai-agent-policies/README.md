# AI Agent with x402 Payment Policies

A complete example demonstrating how AI agents can enforce payment policies using x402: spending caps, allowlists, and rate limiting.

## What are x402 Policies?

Policies provide automatic enforcement of payment constraints without manual tracking:

- **Spending Caps**: Limit total spending per time period (hourly/daily/monthly)
- **Allowlists**: Restrict which endpoints/services can be accessed
- **Rate Limits**: Control request frequency to prevent abuse and manage costs

## Why Use Policies?

Without policies, AI agents can:
- Exceed budgets through runaway API calls
- Access unauthorized services
- Generate unexpected costs through rapid-fire requests

With x402 policies, these constraints are **automatically enforced** at the payment layer.

## Prerequisites

- Rust 1.75 or higher
- x402-dev CLI installed (`cargo install x402-dev`)
- Node.js 18+ (for middleware generation)
- Basic understanding of async Rust

## Quick Start

**Estimated time: 3 minutes**

### 1. Initialize the Example

```bash
x402-dev examples init ai-agent-policies
cd examples/ai-agent-policies
```

### 2. Review the Policy Configuration

Open `policy.yaml` to see the configured policies:

```yaml
version: "1.0"
spending_cap:
  max_amount: 10.0
  currency: USDC
  period: daily

allowlist:
  endpoints:
    - /api/data
    - /api/ai-query

rate_limit:
  requests_per_minute: 10
```

### 3. Generate Middleware (Optional)

Generate Express.js middleware from the policy:

```bash
x402-dev policy generate policy.yaml --framework express --output middleware.js
```

This creates enforcement middleware you can use in your API server.

### 4. Build and Run the Agent

```bash
cargo build --release
cargo run
```

The agent will:
- Make API calls with x402 payment headers
- Enforce spending caps automatically
- Validate endpoints against the allowlist
- Apply rate limiting

## How It Works

### Spending Cap Enforcement

The agent tracks spending in real-time:

```rust
// Before each request, check if within budget
if current_spending + request_cost > spending_cap {
    return Err("Spending cap exceeded");
}
```

### Allowlist Validation

Only approved endpoints are accessible:

```rust
// Verify endpoint is in allowlist
if !policy.allowlist.contains(&endpoint) {
    return Err("Endpoint not in allowlist");
}
```

### Rate Limiting

Requests are throttled automatically:

```rust
// Track requests per time window
if requests_in_window >= rate_limit {
    sleep_until(next_window);
}
```

## Policy Examples

### Conservative Policy (Low Budget)

```yaml
spending_cap:
  max_amount: 1.0
  currency: USDC
  period: hourly
rate_limit:
  requests_per_minute: 5
```

### Production Policy (Balanced)

```yaml
spending_cap:
  max_amount: 100.0
  currency: USDC
  period: daily
rate_limit:
  requests_per_minute: 60
allowlist:
  endpoints:
    - /api/*
  providers:
    - trusted-ai-service.com
```

### Development Policy (Relaxed)

```yaml
spending_cap:
  max_amount: 1000.0
  currency: USDC
  period: monthly
rate_limit:
  requests_per_minute: 120
```

## Integration with AI Agents

### Adding x402 Headers

```rust
let response = client
    .get(&endpoint)
    .header("X-402-Policy", policy_id)
    .header("X-402-Budget", remaining_budget)
    .send()
    .await?;
```

### Handling Policy Violations

```rust
match response.status() {
    StatusCode::PAYMENT_REQUIRED => {
        // Spending cap exceeded
        log::warn!("Budget exhausted, pausing agent");
    }
    StatusCode::FORBIDDEN => {
        // Allowlist violation
        log::error!("Attempted access to forbidden endpoint");
    }
    StatusCode::TOO_MANY_REQUESTS => {
        // Rate limit hit
        sleep(Duration::from_secs(60));
    }
    _ => { /* process response */ }
}
```

## Testing Policies

### Test Spending Cap

```bash
# Set low cap to trigger quickly
cargo run -- --spending-cap 0.10
```

### Test Allowlist

```bash
# Try accessing non-allowlisted endpoint
cargo run -- --endpoint /api/forbidden
```

### Test Rate Limit

```bash
# Make rapid requests
cargo run -- --burst-mode
```

## Real-World Use Cases

1. **Customer Support Bots**: Limit per-user spending to prevent abuse
2. **Research Agents**: Cap monthly API costs while allowing flexibility
3. **Internal Tools**: Allowlist approved services only
4. **Multi-Tenant Systems**: Per-tenant spending limits

## Next Steps

- Explore `agent.rs` to see policy implementation
- Modify `policy.yaml` with your own constraints
- Generate middleware for your API framework
- Review `middleware.js` to understand server-side enforcement

## Learn More

- [x402 Policy Documentation](../docs/policies.md)
- [Middleware Generation Guide](../docs/middleware.md)
- [Best Practices](../docs/best-practices.md)

## License

MIT License - See LICENSE file for details
