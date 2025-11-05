# MCPay.tech Guide

**Quick Links:** [Website](https://mcpay.tech) | [GitHub](https://github.com/microchipgnu/MCPay) | [Docs](https://docs.mcpay.tech)
**License:** Open Source | **Integration Difficulty:** Easy

## Overview

MCPay is a **payments infrastructure platform for Model Context Protocol (MCP) servers**, enabling micropayments for AI agent tool access. Built on the x402 protocol, MCPay allows developers to monetize MCP resources through non-intrusive middleware without rewriting existing infrastructure.

## Key Features

- ✅ **Per-Tool Pricing:** Set individual prices for each tool call (minimum $0.001)
- ✅ **Non-Intrusive Integration:** Drop-in payments without infrastructure rewrite
- ✅ **Multi-Blockchain Support:** Both EVM and Solana networks
- ✅ **MCP Registry Integration:** Works with Smithery, KlavisAI, and Composio
- ✅ **Flexible Funding:** Credit card, Apple Pay, and cryptocurrency deposits
- ✅ **SDK with Extensibility:** Comprehensive SDK with plugin support

## Technical Specifications

### Technology Stack
- **Protocol:** x402 (HTTP 402 Payment Required)
- **MCP Integration:** Native Model Context Protocol support
- **Payment Rails:** USDC stablecoin on EVM and Solana
- **Settlement Time:** <2 seconds
- **Architecture:** Middleware-based payment layer

### Supported Networks

| Network | Support | Primary Token | Status |
|---------|---------|---------------|--------|
| **Solana** | Primary | USDC (SPL) | Production |
| **EVM Chains** | Primary | USDC (ERC-20) | Production |
| **Base** | Secondary | USDC | Production |
| **Polygon** | Secondary | USDC | Production |

### Payment Method
- USDC micropayments
- HTTP 402 Payment Required protocol
- Settlement required before tool execution
- On-chain payment verification

## Integration Methods

### MCP Server Integration

**Use Case:** Monetize existing MCP tools
**Effort:** Minimal - wrap existing tools with payment handler

```javascript
import { createMcpPaidHandler } from "mcpay/handler"

export const paidMcp = createMcpPaidHandler(
  async (server) => {
    server.paidTool(
      "hello",
      "pay for hello",
      "$0.001",
      {},
      async ({}) => ({
        content: [{
          type: 'text',
          text: `Hello, world!`
        }]
      })
    )
  },
  {
    recipient: {
      evm: {
        address: '0x036CbD53842c5426634e7929541eC2318f3dCF7e'
      }
    }
  }
)
```

**Installation:**
```bash
npm install mcpay
```

### Client Integration

**Use Case:** AI agents accessing paid MCP tools
**Effort:** Easy - configure wallet and call paid tools

```javascript
import { MCPayClient } from 'mcpay';

const client = new MCPayClient({
  wallet: myWallet,
  maxAmount: 0.01 // USDC spending limit
});

// Call paid tool - payment handled automatically
const result = await client.callTool('hello', {});
```

### Payment Flow

1. **Tool Registration:** Define tool with price and recipient address
2. **Client Request:** AI agent requests tool execution
3. **Payment Required:** Server responds with 402 + payment details
4. **Payment Settlement:** Client signs and submits USDC payment
5. **Verification:** MCPay verifies on-chain transaction
6. **Tool Execution:** Server executes tool and returns result

## Use Cases for Hackathon

### 1. Premium Data Access
**Scenario:** AI agent needs access to proprietary datasets
**Implementation:** Wrap data query tools with MCPay pricing
**Revenue Model:** $0.01 per query

### 2. Computational Resources
**Scenario:** AI agent requires GPU compute for inference
**Implementation:** Gate compute endpoints with x402 payments
**Revenue Model:** $0.10 per compute minute

### 3. Specialized AI Models
**Scenario:** Fine-tuned models for specific domains
**Implementation:** Charge per inference call
**Revenue Model:** $0.05 per model inference

### 4. Market Data Feeds
**Scenario:** Real-time crypto/stock market data
**Implementation:** Pay-per-request pricing for live quotes
**Revenue Model:** $0.001 per quote

## MCP Registry Support

MCPay works with major MCP registries:

| Registry | Integration | Tool Discovery | Status |
|----------|-------------|----------------|--------|
| **Smithery** | Native | Yes | Live |
| **KlavisAI** | Native | Yes | Live |
| **Composio** | Native | Yes | Live |

## Pricing & Business Model

### Framework Costs
- **Platform:** FREE for developers
- **Transaction Fees:** Network gas fees only
- **No Subscriptions:** Pure pay-per-use
- **Minimum Tool Price:** $0.001 USDC

### Revenue Model for Developers

| Pricing Tier | Tool Type | Suggested Price |
|--------------|-----------|-----------------|
| **Micro** | Simple queries | $0.001-$0.01 |
| **Standard** | Data retrieval | $0.01-$0.10 |
| **Premium** | Compute/AI inference | $0.10-$1.00 |
| **Enterprise** | Complex workflows | $1.00+ |

### Cost Comparison

| Model | MCPay | Traditional API Key |
|-------|-------|---------------------|
| Setup | $0 | $0-$100 |
| Monthly Fee | $0 | $10-$500 |
| Per Request | $0.001-$1.00 | Included or overage |
| Payment Rails | On-chain USDC | Credit card fees (2.9%) |

## Notable Achievements

### Awards & Recognition
- **1st Place:** Coinbase Agents in Action
- **Finalist:** ETHGlobal Prague
- **2nd Place:** ETHGlobal Trifecta

### Partnerships
- **Coinbase Developer Platform** - Technical collaboration
- **vLayer** - Infrastructure support

## Documentation Quality: MEDIUM-HIGH

**Available Resources:**
- **Getting Started Guide** - Initial setup
- **API Reference** - Handler and client docs
- **Payment Flow Docs** - x402 integration details
- **GitHub Examples** - Sample implementations

**GitHub:**
- URL: https://github.com/microchipgnu/MCPay
- Status: Active development
- Examples: Available in repository
- Issues: Open for community feedback

## Community & Support

**Channels:**
- **GitHub Issues** - Primary support channel
- **Documentation** - docs.mcpay.tech
- **Twitter/X** - @microchipgnu (creator)

**Response Time:** Community-driven, variable

## Integration Difficulty Breakdown

### Easy ✅
- Simple handler API
- Clear code examples
- Minimal dependencies
- Drop-in middleware pattern
- Well-documented payment flow

### Medium ⚠️
- MCP protocol understanding helpful
- Blockchain wallet setup required
- USDC funding needed for testing
- x402 protocol concepts

### Advanced 🔧
- Custom payment policies
- Multi-chain recipient configuration
- Advanced error handling
- Custom MCP tool development

## Unique Selling Points

1. **MCP-Native Design** - Built specifically for Model Context Protocol
2. **Non-Intrusive** - Add payments without rewriting tools
3. **Proven Track Record** - Multiple hackathon wins
4. **Multi-Chain Support** - EVM and Solana compatibility
5. **Low Minimum Price** - $0.001 enables true micropayments
6. **Flexible Funding** - Traditional and crypto payment methods

## When to Choose MCPay

**✅ Choose MCPay if you:**
- Are building with Model Context Protocol
- Want to monetize existing MCP tools
- Need micropayment support ($0.001+)
- Prefer simple middleware integration
- Want multi-chain payment support
- Are building AI agent marketplaces

**❌ Consider alternatives if you:**
- Not using MCP (use Corbits/Faremeter)
- Need pure HTTP API payments (use PayAI)
- Want enterprise support (use Crossmint)
- Need sub-cent pricing
- Require advanced facilitator features

## Quick Start Checklist

- [ ] Install `mcpay` package
- [ ] Set up USDC wallet (Solana or EVM)
- [ ] Define MCP tools with `paidTool()` handler
- [ ] Configure recipient addresses
- [ ] Set pricing per tool
- [ ] Test with small amounts ($0.001-$0.01)
- [ ] Fund wallet for testing
- [ ] Deploy to MCP registry (Smithery/KlavisAI)

## Code Example: Complete Server

```javascript
import { createMcpPaidHandler } from "mcpay/handler"

export const paidMcp = createMcpPaidHandler(
  async (server) => {
    // Premium data tool - $0.05 per query
    server.paidTool(
      "get_market_data",
      "Get real-time market data",
      "$0.05",
      {
        symbol: { type: "string", description: "Trading symbol" }
      },
      async ({ symbol }) => {
        const data = await fetchMarketData(symbol);
        return {
          content: [{
            type: 'text',
            text: JSON.stringify(data)
          }]
        };
      }
    );

    // Compute tool - $0.10 per execution
    server.paidTool(
      "run_inference",
      "Run AI model inference",
      "$0.10",
      {
        prompt: { type: "string", description: "Input prompt" }
      },
      async ({ prompt }) => {
        const result = await runModel(prompt);
        return {
          content: [{
            type: 'text',
            text: result
          }]
        };
      }
    );
  },
  {
    recipient: {
      solana: {
        address: 'YOUR_SOLANA_ADDRESS'
      },
      evm: {
        address: '0xYOUR_EVM_ADDRESS'
      }
    }
  }
)
```

## Hackathon Tips

### Prize Track Alignment
- **Best x402 Agent Application** ✅ (Primary target)
- **Best Corbits Project** ❌ (Not Corbits-based)
- **Best Agent Money Protocol** ✅ (MCP-focused)

### Competitive Advantages
1. **Unique MCP Angle:** Only tool focused on MCP monetization
2. **Multi-Chain:** Appeal to both EVM and Solana judges
3. **Proven Technology:** Multiple hackathon wins validate approach
4. **Clear Use Case:** AI agent tool marketplace is compelling

### Integration Time
- **Basic Setup:** 30-60 minutes
- **Production Ready:** 2-4 hours
- **Advanced Features:** 8+ hours

---

**Related Docs:**
- [SDK Comparison Reference](../reference/sdk-comparison.md)
- [MCP Integration Patterns](../guides/integration-patterns.md)
- [x402 Protocol Specification](../x402-protocol-specification.md)
- [Code Repositories](../reference/code-repositories.md)
