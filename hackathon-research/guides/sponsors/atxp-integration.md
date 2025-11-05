# ATXP (Agentic Transaction Protocol) Integration Guide

**Prize:** $10,000 in ATXP credits
**Sponsor:** Circuit & Chisel
**Funding:** $19.2M seed (September 2025)
**Difficulty:** High
**Est. Integration Time:** 8-16 hours

---

## Overview

ATXP is a web-wide protocol enabling AI agents to handle complete commerce lifecycle—from service discovery to payment settlement—without human oversight. Founded by Stripe's former Head of Crypto & AI Partnerships (Louis Amira, CEO) and Head of Crypto Engineering (David Noel-Romas, CTO).

### Key Benefits

- **Multi-Protocol Support:** Works with x402, Visa TAP, and custom payment protocols
- **Multi-Chain:** Supports Base, Solana, Polygon (and expanding)
- **Nested Transactions:** Compose complex multi-step workflows atomically
- **Autonomous Discovery:** Agents find and use services independently
- **Strong Backing:** $19.2M from Primary Venture Partners, ParaFi, Stripe, Coinbase Ventures, Solana Ventures

### Why Use ATXP?

$10,000 prize for "Best Multi-Protocol Agent." Newest protocol (Sept 2025) with less competition. Demonstrates advanced technical capability. Future-proof multi-blockchain strategy backed by Web2 (Stripe, Samsung) and Web3 (Coinbase, Solana, Polygon) leaders.

---

## What is ATXP?

Current agent payment solutions are fragmented: x402 handles HTTP-native payments on specific chains, Visa TAP handles agent verification, each protocol requires separate integration, cross-chain operations are complex, multi-step transactions lack atomicity.

### The ATXP Solution

ATXP provides a **universal protocol layer** that:

1. **Abstracts Payment Complexity:** Works across x402, TAP, and future protocols
2. **Enables Tool Discovery:** Agents autonomously find services
3. **Supports Nested Transactions:** Complex multi-step workflows as single atomic operation
4. **Delegates Authority:** Users grant spending permissions with policy controls
5. **Optimizes Routing:** Selects optimal payment path based on cost/speed/availability

### ATXP vs Other Protocols

See [visa-tap-integration.md](./visa-tap-integration.md) for protocol comparison.

**Relationship:** ATXP orchestrates, x402/TAP execute

---

## Technical Architecture

### Core Components

#### 1. Tool Discovery Service

ATXP provides decentralized registry where services advertise capabilities and pricing:

```javascript
const tools = await atxp.discover({
  category: 'data',
  maxPrice: 0.05,
  requiredProtocols: ['x402'],
  minRating: 4.0,
  chains: ['solana']
});
```

#### 2. Nested Transaction Composition

ATXP enables atomic multi-step workflows:

```javascript
const nestedTx = {
  type: 'atomic_workflow',
  steps: [
    {
      service: 'data-fetcher',
      action: 'fetch_market_data',
      input: { symbols: ['SOL', 'BTC'] },
      price: 0.02,
      protocol: 'x402',
      chain: 'solana'
    },
    {
      service: 'ml-analyzer',
      action: 'predict_trend',
      input: { data: '@step[0].output' },  // Reference previous step
      price: 0.10,
      protocol: 'x402',
      chain: 'base'
    }
  ],
  totalPrice: 0.12,
  atomic: true  // All steps succeed or all fail
};
```

#### 3. Policy-Based Spending

Users control agent spending through programmable policies:

```javascript
const spendingPolicy = {
  maxSingleTransaction: 1.00,
  maxHourlySpend: 5.00,
  maxDailySpend: 20.00,
  allowedCategories: ['data', 'analytics', 'trading'],
  preferredChains: ['solana', 'base'],
  requiresApproval: (tx) => tx.amount > 0.50
};
```

---

## Developer Resources

### Official Resources (2025)

**Documentation:**
- Official Docs: https://docs.atxp.ai
- API Reference: https://docs.atxp.ai/api
- Protocol Specification: https://docs.atxp.ai/spec

**Company:**
- Website: https://circuitandchisel.com
- Support: developers@circuitandchisel.com

### SDK Installation

```bash
# Core ATXP SDK
npm install @atxp/sdk

# Protocol adapters
npm install @atxp/x402-adapter
npm install @atxp/tap-adapter

# Blockchain connectors
npm install @atxp/solana-connector
npm install @atxp/base-connector
```

---

## Step-by-Step Integration

### Prerequisites

Contact Circuit & Chisel for developer access (protocol is new):

```bash
ATXP_API_KEY=atxp_sk_live_abc123def456
ATXP_AGENT_ID=agent_xyz789
ATXP_ENVIRONMENT=sandbox
```

### Initialize ATXP Client

```javascript
const { ATXPClient } = require('@atxp/sdk');
const { SolanaConnector } = require('@atxp/solana-connector');

const solanaWallet = new SolanaConnector({
  privateKey: process.env.SOLANA_PRIVATE_KEY,
  network: 'devnet'
});

const atxp = new ATXPClient({
  apiKey: process.env.ATXP_API_KEY,
  agentId: process.env.ATXP_AGENT_ID,
  environment: 'sandbox',
  wallets: { solana: solanaWallet },
  policy: {
    maxSingleTransaction: 1.00,
    maxDailySpend: 10.00,
    allowedCategories: ['data', 'analytics'],
    preferredChains: ['solana']
  }
});
```

### Discover Services

```javascript
async function discoverAnalyticsTools() {
  const tools = await atxp.discover({
    category: 'analytics',
    maxPrice: 0.10,
    minRating: 4.0,
    protocols: ['x402'],
    chains: ['solana'],
    capabilities: ['real_time_analysis']
  });

  return tools;
}
```

### Create Nested Transaction

```javascript
async function executeWorkflow() {
  const workflow = await atxp.createWorkflow({
    name: 'market-analysis-and-trade',
    atomic: true,
    steps: [
      {
        service: 'market-data-api',
        action: 'fetch_prices',
        input: { symbols: ['SOL', 'BTC', 'ETH'] },
        maxPrice: 0.02,
        chain: 'solana'
      },
      {
        service: 'ml-analyzer',
        action: 'predict_movement',
        input: { data: '$step[0].output' },
        maxPrice: 0.15,
        chain: 'base'
      },
      {
        service: 'dex-executor',
        action: 'swap',
        input: { prediction: '$step[1].output' },
        condition: '$step[1].output.confidence > 0.8',
        maxPrice: 0.05,
        chain: 'solana'
      }
    ]
  });

  const result = await atxp.executeWorkflow(workflow);
  return result;
}
```

### Policy Enforcement

```javascript
const advancedPolicy = {
  maxSingleTransaction: 2.00,
  maxDailySpend: 50.00,

  validate: async (transaction) => {
    if (transaction.service.rating < 4.5) {
      return { allowed: false, reason: 'Service rating too low' };
    }

    const hour = new Date().getHours();
    if (hour < 9 || hour > 17) {
      return { allowed: false, reason: 'Trading only during market hours' };
    }

    return { allowed: true };
  }
};

atxp.setPolicy(advancedPolicy);
```

---

## Multi-Protocol Compatibility

### ATXP + x402

```javascript
const x402Services = await atxp.discover({
  protocols: ['x402'],
  chains: ['solana']
});

const result = await atxp.execute({
  service: x402Services[0].service_id,
  action: 'fetch_data'
});
```

### ATXP + Visa TAP

```javascript
const tapServices = await atxp.discover({
  protocols: ['tap'],
  verified: true
});

const result = await atxp.execute({
  service: tapServices[0].service_id,
  action: 'purchase',
  tapVerification: true
});
```

---

## Use Cases for Hackathon

### 1. Cross-Chain Arbitrage Agent

**Concept:** Agent that finds price differences across chains and executes atomic arbitrage

**ATXP Usage:**
- Discover price feeds on multiple chains
- Compose atomic buy+sell transaction
- Policy enforcement prevents losses

**Tech Stack:** ATXP + Solana + Base + Switchboard

**Prize Potential:** ATXP ($10k) + Best x402 Application ($10k) = $20k

---

### 2. Multi-Service AI Orchestrator

**Concept:** Agent that discovers and chains multiple AI services for complex tasks

**ATXP Usage:**
- Autonomous service discovery
- Nested transaction for multi-step AI pipeline
- Cost optimization via service comparison

**Tech Stack:** ATXP + x402 + CDP Wallets + Multiple AI APIs

**Prize Potential:** ATXP ($10k) + CDP Wallets ($5k) = $15k

---

### 3. Decentralized Tool Marketplace

**Concept:** Platform where AI services advertise via ATXP discovery

**ATXP Usage:**
- Service registration and discovery
- Automatic payment routing
- Multi-chain settlement

**Tech Stack:** ATXP + Solana + React + PostgreSQL

**Prize Potential:** ATXP ($10k) + Best x402 Agent ($10k) = $20k

---

### 4. Policy-Enforced Research Agent

**Concept:** Academic research agent with strict budget controls

**ATXP Usage:**
- Budget enforcement via policies
- Multi-protocol data access
- Nested research workflows

**Tech Stack:** ATXP + Multiple data APIs + x402

**Prize Potential:** ATXP ($10k) + Best x402 API Integration ($5k) = $15k

---

## Troubleshooting

See [common-troubleshooting.md](./common-troubleshooting.md) for generic issues.

### ATXP-Specific Issues

#### Service discovery returns empty

**Solutions:**
```javascript
// Try broader search first
const tools = await atxp.discover({
  category: 'data',
  chains: ['solana', 'base', 'polygon']  // Search all chains
});

const filtered = tools.filter(t => t.pricing.per_request < 0.10);
```

#### Nested transaction fails

**Solutions:**
```javascript
// Increase timeout
const workflow = await atxp.createWorkflow({
  steps: [...],
  maxDuration: 60000,  // 60 seconds
  retryFailed: true,
  retryAttempts: 3
});

// Add logging
workflow.on('stepComplete', (step, result) => {
  console.log(`Step ${step} completed in ${result.duration}ms`);
});
```

#### Policy rejection

**Solutions:**
```javascript
// Check current spend
const spending = await atxp.getSpendingStats();
console.log('Today:', spending.today);
console.log('Policy limits:', atxp.policy);

// Request policy override (if supported)
await atxp.requestPolicyOverride({
  reason: 'Urgent market opportunity',
  amount: 15.00
});
```

---

## Additional Resources

### Official Links
- Docs: https://docs.atxp.ai
- Company: https://circuitandchisel.com
- Support: developers@circuitandchisel.com

### Related Guides
- [Multi-Sponsor Strategies](./multi-sponsor-strategies.md)
- [x402 Protocol](../../x402-protocol-specification.md)
- [Visa TAP Integration](./visa-tap-integration.md)

---

**Last Updated:** November 4, 2025
**Hackathon Deadline:** November 11, 2025
**Integration Difficulty:** High (8-16 hours)
