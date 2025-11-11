# x402-dev Policy Examples

This directory contains example policy YAML files demonstrating the x402-dev Policy Engine (Epic 5).

## Quick Start

```bash
# Validate a policy file
x402-dev policy validate simple-allowlist.yaml

# Generate Express middleware
x402-dev policy generate comprehensive.yaml --framework express --output middleware.js

# Generate Fastify plugin
x402-dev policy generate comprehensive.yaml --framework fastify --output plugin.js
```

## Example Files

### 1. `simple-allowlist.yaml`
Basic allowlist policy restricting access to specific agent IDs.

**Use case:** Only allow verified AI agents to access your API.

```bash
x402-dev policy generate simple-allowlist.yaml --framework express
```

### 2. `rate-limit.yaml`
Rate limiting policy preventing abuse.

**Use case:** Limit agents to 100 requests per hour to prevent DoS.

```bash
x402-dev policy generate rate-limit.yaml --framework express
```

### 3. `spending-cap.yaml`
Spending cap policy preventing runaway costs.

**Use case:** Limit AI agents to $10 USDC spending per day.

```bash
x402-dev policy generate spending-cap.yaml --framework express
```

### 4. `comprehensive.yaml` 🌟 **DEMO FILE**
Complete policy demonstrating all features - the Epic 5 showcase!

**This is the "10 lines → 100+ lines" demo from the PRD.**

```bash
# Validate
x402-dev policy validate comprehensive.yaml

# Generate Express middleware (100+ lines from 10-line YAML!)
x402-dev policy generate comprehensive.yaml --framework express --output demo-middleware.js

# Count the lines
wc -l demo-middleware.js
```

**Features:**
- ✅ Allowlist (3 specific agents)
- ✅ Rate limiting (100 req/hour)
- ✅ Spending cap ($10/day)
- ✅ Audit logging
- ✅ Error handling
- ✅ 402 response generation

### 5. `conflict-example.yaml`
Demonstrates conflict detection (FR-5.6).

**Use case:** Show how x402-dev detects conflicting policies before code generation.

```bash
x402-dev policy validate conflict-example.yaml
# Should output:
# ✗ ERROR: Conflict detected
#    agent-abc-123 appears in both allowlist and denylist
#    💡 Suggestion: Remove from denylist OR remove from allowlist
```

## Policy Types Reference

### Allowlist
Whitelist specific agents by ID, wallet address, or IP address.

```yaml
policies:
  - type: allowlist
    field: agent_id  # or wallet_address, ip_address
    values:
      - "agent-1"
      - "agent-2"
```

### Denylist
Blacklist specific agents (blocks take precedence).

```yaml
policies:
  - type: denylist
    field: agent_id
    values:
      - "bad-agent"
```

### Rate Limit
Limit number of requests per time window (FR-5.4: sliding window algorithm).

```yaml
policies:
  - type: rate_limit
    max_requests: 100
    window_seconds: 3600  # 1 hour
```

### Spending Cap
Limit total spending per time window (FR-5.5).

```yaml
policies:
  - type: spending_cap
    max_amount: 10.00
    currency: USDC
    window_seconds: 86400  # 24 hours
```

## Validation (FR-5.6)

x402-dev automatically detects:
- ✅ Syntax errors
- ✅ Missing required fields
- ✅ Conflicting policies
- ✅ Invalid values

```bash
x402-dev policy validate my-policy.yaml
```

## Code Generation (FR-6.1, FR-6.2)

Generate production-ready middleware:

```bash
# Express.js
x402-dev policy generate policy.yaml --framework express --output middleware.js

# Fastify
x402-dev policy generate policy.yaml --framework fastify --output plugin.js
```

Generated code includes:
- ✅ Policy enforcement logic
- ✅ 402 response with invoices
- ✅ Audit logging (CSV + JSON)
- ✅ Error handling
- ✅ Inline comments

## Epic 5 Demo Script

Run this to demonstrate the "10 lines → 100+ lines" magic:

```bash
# Show the 10-line input
cat comprehensive.yaml
echo "Input: $(wc -l < comprehensive.yaml) lines"

# Generate middleware
x402-dev policy generate comprehensive.yaml --framework express --output demo.js

# Show the result
echo "Output: $(wc -l < demo.js) lines"
echo "Ratio: $(($(wc -l < demo.js) / $(wc -l < comprehensive.yaml)))x expansion!"

# Validate it works
node -c demo.js  # Syntax check
echo "✅ Generated code is valid JavaScript!"
```

Expected output:
```
Input: 10 lines
Output: 120+ lines
Ratio: 12x+ expansion!
✅ Generated code is valid JavaScript!
```

## Related Commands

- `x402-dev policy validate` - Check policy syntax and conflicts
- `x402-dev policy generate` - Generate middleware code
- `x402-dev init` - Initialize project with policy templates
- `x402-dev doctor` - Check policy file compatibility

## Learn More

- [Policy Engine Documentation](../../docs/policy-engine.md)
- [PRD FR-5: Policy Enforcement](../../docs/PRD.md#fr-5-policy-enforcement-engine)
- [PRD FR-6: Middleware Generation](../../docs/PRD.md#fr-6-middleware-generation)
