# Google A2A x402 Extension Guide

**Quick Links:** [GitHub](https://github.com/google-agentic-commerce/a2a-x402) | [AP2 Protocol](https://developers.google.com/agent-payments) | [Releases](https://github.com/google-agentic-commerce/a2a-x402/releases)
**License:** Open Source | **Integration Difficulty:** Medium-Advanced

## Overview

The **A2A x402 Extension** brings cryptocurrency payments to Google's Agent-to-Agent (A2A) protocol, enabling AI agents to monetize their services through on-chain payments. Developed in collaboration with Coinbase, Ethereum Foundation, and MetaMask, this extension revives HTTP 402 "Payment Required" for the decentralized agent ecosystem.

## Key Features

- ✅ **Agent-to-Agent Payments:** Purpose-built for AI agent commerce
- ✅ **Google-Backed:** Part of Google's Agent Payments Protocol (AP2)
- ✅ **Industry Collaboration:** Coinbase, Ethereum Foundation, MetaMask
- ✅ **Multi-Language Support:** TypeScript, Python, Go libraries
- ✅ **Production-Ready:** Specification v0.1 released
- ✅ **Cryptocurrency Native:** On-chain settlement via Solana/EVM

## Technical Specifications

### Technology Stack
- **Base Protocol:** A2A (Agent-to-Agent)
- **Payment Layer:** x402 extension
- **Core Protocol:** Google AP2 (Agent Payments Protocol)
- **Blockchain Support:** Solana (in progress), EVM chains
- **Languages:** TypeScript, Python, Go

### Repository Structure
```
a2a-x402/
├── v0.1/
│   ├── schemes/          # Payment schemes specification
│   ├── typescript/       # TS implementation
│   ├── python/           # Python implementation
│   └── go/               # Go implementation
├── spec/                 # Protocol specification
└── examples/             # Reference implementations
```

### Supported Networks

| Network | Support | Primary Token | Status |
|---------|---------|---------------|--------|
| **EVM Chains** | Primary | USDC (ERC-20) | Production |
| **Solana** | In Progress | USDC (SPL) | Development |
| **Base** | Primary | USDC | Production |
| **Polygon** | Secondary | USDC | Production |

### A2A x402 Architecture

**Three-Message Flow:**

1. **Payment-Required Message**
   - Merchant agent responds with payment terms
   - Includes amount, recipient, blockchain details

2. **Payment-Submitted Message**
   - Client agent signs and submits payment
   - Includes transaction details, proof

3. **Payment-Completed Message**
   - Merchant verifies on-chain
   - Delivers requested service

## A2A Protocol Context

### What is A2A?

Google's Agent-to-Agent (A2A) protocol enables AI agents to discover and interact with each other's services. The x402 extension adds the critical **payment layer** to enable commercial agent interactions.

### AP2 (Agent Payments Protocol)

Google's broader vision for agent commerce:
- **Discovery:** Find agent services
- **Communication:** A2A messaging protocol
- **Payments:** x402 extension (this project)
- **Trust:** Reputation and verification

## Integration Methods

### TypeScript Integration

**Use Case:** Node.js-based AI agents
**Effort:** Medium - requires A2A protocol understanding

```typescript
import { A2AX402Client } from '@google-agentic-commerce/a2a-x402';
import { Connection, Keypair } from '@solana/web3.js';

const client = new A2AX402Client({
  agentId: 'my-trading-agent',
  wallet: solanaKeypair,
  maxAmount: 1.0,
  chains: ['solana', 'base']
});

// Discover and call paid agent service
const response = await client.callAgent({
  agentId: 'market-data-agent',
  method: 'getPriceData',
  params: { symbol: 'SOL' }
});

// Payment handled automatically via x402 flow
console.log('Price data:', response.data);
console.log('Payment tx:', response.paymentTx);
```

**Installation:**
```bash
npm install @google-agentic-commerce/a2a-x402
```

### Python Integration

**Use Case:** AI/ML agents, data science workflows
**Effort:** Medium - Python async patterns

```python
from google_agentic_commerce.a2a_x402 import A2AClient
import asyncio

async def call_paid_agent():
    client = A2AClient(
        agent_id="my-data-agent",
        wallet=my_wallet,
        max_amount=0.5
    )

    # Call agent service with automatic payment
    response = await client.call_agent(
        agent_id="ml-inference-agent",
        method="run_inference",
        params={"model": "gpt-4", "prompt": "Hello"}
    )

    print(f"Result: {response.data}")
    print(f"Payment: {response.payment_tx}")

asyncio.run(call_paid_agent())
```

**Installation:**
```bash
pip install google-agentic-commerce-a2a-x402
```

### Go Integration

**Use Case:** High-performance backend agents
**Effort:** Medium-Advanced - Go concurrency patterns

```go
package main

import (
    "github.com/google-agentic-commerce/a2a-x402/go"
)

func main() {
    client := a2ax402.NewClient(a2ax402.Config{
        AgentID: "my-go-agent",
        Wallet:  solanaWallet,
        MaxAmount: 1.0,
    })

    // Call paid agent service
    response, err := client.CallAgent(context.Background(), a2ax402.Request{
        AgentID: "data-provider",
        Method:  "GetMarketData",
        Params:  map[string]interface{}{"symbol": "SOL"},
    })

    if err != nil {
        log.Fatal(err)
    }

    fmt.Printf("Data: %v\n", response.Data)
    fmt.Printf("Payment: %s\n", response.PaymentTx)
}
```

## Payment Flow Details

### Step-by-Step Example

```typescript
// Merchant Agent (selling service)
import { A2AMerchant } from '@google-agentic-commerce/a2a-x402';

const merchant = new A2AMerchant({
  services: {
    'getPriceData': {
      price: 0.05, // USDC
      description: 'Real-time price data',
      handler: async (params) => {
        return await fetchPriceData(params.symbol);
      }
    }
  },
  wallet: merchantWallet
});

// Automatically handles:
// 1. Receiving service requests
// 2. Responding with payment-required
// 3. Verifying on-chain payment
// 4. Executing service
// 5. Returning payment-completed + data
```

```typescript
// Client Agent (buying service)
import { A2AClient } from '@google-agentic-commerce/a2a-x402';

const client = new A2AClient({
  wallet: clientWallet,
  maxAmount: 0.1
});

// Automatically handles:
// 1. Sending service request
// 2. Receiving payment-required
// 3. Signing payment transaction
// 4. Submitting payment-submitted message
// 5. Receiving payment-completed + service result
const result = await client.call('merchant-agent-id', 'getPriceData', {
  symbol: 'SOL'
});
```

## Use Cases for Hackathon

### 1. AI Agent Marketplace
**Scenario:** Decentralized marketplace for AI services
**Implementation:** A2A for discovery + x402 for payments
**Revenue Model:** Commission on agent-to-agent transactions

### 2. Data Broker Network
**Scenario:** Agents buying/selling specialized datasets
**Implementation:** A2A discovery + per-query pricing
**Revenue Model:** $0.01-$1.00 per dataset query

### 3. Compute Brokerage
**Scenario:** GPU/compute time marketplace
**Implementation:** Agent-to-agent compute allocation
**Revenue Model:** $0.50+ per compute hour

### 4. Multi-Agent Workflows
**Scenario:** Complex tasks requiring multiple specialist agents
**Implementation:** Orchestrator agent pays specialist agents
**Revenue Model:** Workflow-based pricing

## Collaboration Partners

### Coinbase
- **Contribution:** x402 protocol expertise
- **Integration:** CDP (Coinbase Developer Platform) compatibility
- **Benefit:** Enterprise-grade payment infrastructure

### Ethereum Foundation
- **Contribution:** Blockchain standards alignment
- **Integration:** EVM compatibility
- **Benefit:** Multi-chain agent commerce

### MetaMask
- **Contribution:** Wallet integration patterns
- **Integration:** Browser-based agent wallets
- **Benefit:** User-friendly agent payment UX

## Integration Difficulty Breakdown

### Easy ✅
- Multi-language SDKs available
- Google-quality documentation
- Clear protocol specification
- Standard HTTP/JSON patterns

### Medium ⚠️
- A2A protocol learning curve
- Async/concurrent programming required
- Blockchain wallet management
- Multi-chain complexity

### Advanced 🔧
- Custom payment schemes
- Agent discovery implementation
- Cross-chain atomic swaps
- Advanced error handling

## Unique Selling Points

1. **Google-Backed:** Enterprise credibility and support
2. **Industry Collaboration:** Best practices from multiple leaders
3. **Multi-Language:** TS, Python, Go - broadest language support
4. **Agent-Native:** Built specifically for agent-to-agent commerce
5. **AP2 Ecosystem:** Part of broader agent payments vision
6. **Production Specification:** v0.1 spec released, not experimental

## When to Choose A2A x402

**✅ Choose A2A x402 if you:**
- Building agent-to-agent marketplace
- Need multi-language support (TS/Python/Go)
- Want Google ecosystem integration
- Require enterprise-grade solution
- Building complex agent workflows
- Need agent discovery + payments together

**❌ Consider alternatives if you:**
- Building simple HTTP API (use Corbits/Faremeter)
- Solana-first project (Solana support still in progress)
- Need fastest time-to-market (use PayAI)
- Want MCP integration (use MCPay)
- Prefer simpler integration (use native x402)

## Documentation Quality: HIGH

**Available Resources:**
- **GitHub Repository** - Complete source code
- **Protocol Specification** - v0.1 spec document
- **Multi-Language Docs** - TS, Python, Go
- **Example Implementations** - Reference code
- **Payment Schemes** - Detailed flow diagrams

**GitHub:**
- URL: https://github.com/google-agentic-commerce/a2a-x402
- Organization: google-agentic-commerce
- Stars: Growing community
- Status: Active development
- Releases: Regular updates

## Community & Support

**Channels:**
- **GitHub Issues** - Primary support
- **Google Developers** - Official documentation
- **Discord** - Community discussions (check repo for link)

**Response Time:** Google-backed, professional support

## Current Status & Roadmap

### Current Status (November 2025)
- ✅ v0.1 specification released
- ✅ TypeScript implementation production-ready
- ✅ Python implementation available
- ✅ Go implementation available
- ⏳ Solana support in progress
- ✅ EVM chains fully supported

### Roadmap
- Solana integration completion
- Additional blockchain support
- Agent discovery enhancements
- Reputation systems
- Advanced payment schemes

## Pricing & Business Model

### Framework Costs
- **A2A x402 Extension:** FREE (open source)
- **Transaction Fees:** Blockchain gas only
- **No Platform Fees:** Peer-to-peer payments
- **Google Services:** Standard GCP pricing (if used)

### Cost Breakdown

| Component | Cost | Notes |
|-----------|------|-------|
| SDK/Library | FREE | Open source |
| Transaction Gas | $0.0005-$0.01 | Chain-dependent |
| Agent Discovery | FREE | A2A protocol |
| Google Cloud (optional) | Pay-as-you-go | If hosting agents on GCP |

## Quick Start Checklist

- [ ] Review A2A protocol documentation
- [ ] Choose language (TypeScript, Python, or Go)
- [ ] Install SDK for chosen language
- [ ] Set up wallet (Solana or EVM)
- [ ] Create agent identity/profile
- [ ] Define agent services and pricing
- [ ] Implement payment handlers
- [ ] Test on devnet/testnet
- [ ] Deploy to production

## Code Example: Complete Agent

```typescript
import { A2AMerchant, A2AClient } from '@google-agentic-commerce/a2a-x402';

// === MERCHANT AGENT ===
const merchant = new A2AMerchant({
  agentId: 'market-data-pro',
  displayName: 'Market Data Pro Agent',
  description: 'Real-time crypto market data',
  wallet: merchantWallet,
  services: {
    getPrice: {
      price: 0.01,
      currency: 'USDC',
      description: 'Get current price for symbol',
      parameters: {
        symbol: { type: 'string', required: true }
      },
      handler: async (params) => {
        const price = await fetchPrice(params.symbol);
        return { symbol: params.symbol, price, timestamp: Date.now() };
      }
    },
    getHistorical: {
      price: 0.05,
      currency: 'USDC',
      description: 'Get historical price data',
      parameters: {
        symbol: { type: 'string', required: true },
        days: { type: 'number', default: 7 }
      },
      handler: async (params) => {
        const data = await fetchHistorical(params.symbol, params.days);
        return { symbol: params.symbol, data, days: params.days };
      }
    }
  }
});

await merchant.start();

// === CLIENT AGENT ===
const client = new A2AClient({
  agentId: 'trading-bot',
  wallet: clientWallet,
  maxAmount: 0.10 // Max spend per transaction
});

// Call merchant services with automatic payment
const priceData = await client.callAgent({
  agentId: 'market-data-pro',
  method: 'getPrice',
  params: { symbol: 'SOL' }
});

console.log('Price:', priceData.price);
console.log('Payment tx:', priceData._meta.paymentTx);
```

## Hackathon Tips

### Prize Track Alignment
- **Best x402 Agent Application** ✅✅ (Google-backed credibility)
- **Best Agent Money Protocol** ✅✅ (Perfect fit - agent-native)
- **CDP Integration** ✅ (Coinbase collaboration)

### Competitive Advantages
1. **Google Credibility:** Enterprise judges appreciate Google backing
2. **Multi-Language:** Broader developer appeal
3. **Agent-Native Design:** Built for agent commerce from ground up
4. **Industry Collaboration:** Coinbase + Ethereum + MetaMask logos
5. **Production Spec:** Not experimental, v0.1 released

### Integration Time
- **Basic Setup:** 2-4 hours
- **Production Agent:** 8-16 hours
- **Complex Marketplace:** 24+ hours

### Demo Strategy
1. Emphasize Google/Coinbase/Ethereum collaboration
2. Show multi-agent workflow (multiple specialists)
3. Demonstrate automatic payment handling
4. Highlight cross-language compatibility
5. Show agent discovery + payment integration

---

**Related Docs:**
- [x402 Protocol Specification](../x402-protocol-specification.md)
- [Agent Architecture Patterns](../guides/agent-patterns.md)
- [SDK Comparison Reference](../reference/sdk-comparison.md)
- [Code Repositories](../reference/code-repositories.md)
