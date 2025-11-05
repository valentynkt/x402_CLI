# PayAI Network Guide

**Quick Links:** [Website](https://payai.network/) | [Docs](https://docs.payai.network/) | [GitHub](https://github.com/PayAINetwork)
**License:** Proprietary | **Integration Difficulty:** Easy

## Overview

PayAI is a **Solana-first, multi-chain** payment facilitator designed for the AI agent economy. Features sub-second settlement, **network fees covered** for both parties, and the broadest blockchain support (7 chains) in the x402 ecosystem.

**Market Position:** 2nd largest facilitator (14% market share), fastest growing

## Key Features

- ✅ **7 Blockchain Networks:** Solana, Base, Polygon, Avalanche, Sei, IoTeX, Peaq
- ✅ **Zero Network Fees:** Covered for both buyers and merchants (unique)
- ✅ **Sub-second Settlement:** <1s on Solana
- ✅ **Free Testing:** Echo Merchant with guaranteed refunds
- ✅ **No API Keys Required:** Simplest integration
- ✅ **ElizaOS Integration:** Built-in ai16z ecosystem support
- ✅ **Free Tier:** 100K settlements/month

## Technical Specifications

### Architecture

**Core Infrastructure:**
- Solana-primary with multi-chain support
- x402 protocol standard implementation
- Decentralized infrastructure (libp2p + IPFS)
- ElizaOS AI agent framework integration

**Technology Stack:**
- Hosted facilitator service
- REST API endpoints
- WebSocket support (real-time)
- libp2p peer-to-peer networking

### Supported Networks

| Network | Mainnet | Testnet | Primary Token |
|---------|---------|---------|---------------|
| **Solana** | ✓ | ✓ | SOL, USDC |
| **Base** | ✓ | ✓ | USDC |
| **Polygon** | ✓ | ✓ | MATIC, USDC |
| **Avalanche** | ✓ | ✓ | AVAX, USDC |
| **Sei** | ✓ | ✓ | SEI |
| **IoTeX** | ✓ | ✓ | IOTX |
| **Peaq** | ✓ | ✓ | PEAQ |

**Broadest network coverage in x402 ecosystem**

### Token Support

**Supported Standards:**
- ERC-20 tokens (Ethereum, Base, Polygon, Avalanche)
- SPL tokens (Solana classic)
- Token-2022 (Solana new standard)
- Custom tokens (any compatible token)

**Primary Tokens:**
- USDC (all chains)
- SOL (Solana)
- Native tokens (MATIC, AVAX, SEI, IOTX, PEAQ)

## Performance Metrics

| Metric | Value | Details |
|--------|-------|---------|
| **Minimum Transaction** | $0.01 | Economic floor |
| **Settlement Speed** | <1 second | Solana-optimized |
| **Market Share** | 14% | 2nd largest facilitator |
| **Network Fees** | $0 | Covered by PayAI (unique feature) |
| **Uptime** | 24/7 | Always-on infrastructure |
| **Free Tier** | 100K settlements/mo | No credit card required |

**Settlement Performance by Network:**
- Solana: <1 second
- Base: ~2 seconds
- Avalanche: ~3 seconds
- Polygon: ~3 seconds

## Pricing Tiers

| Tier | Settlements/Month | Rate Limit | Burst Limit | Cost |
|------|------------------|------------|-------------|------|
| **FREE** | 100,000 | 4 req/s | 480 req/min | $0 |
| **BASIC** | 500,000 | 10 req/s | 1,200 req/min | $1,500 PAYAI tokens |
| **PRO** | 1,000,000 | 25 req/s | 3,000 req/min | $2,800 PAYAI tokens |

**Status:**
- FREE tier: Available now ✅
- BASIC/PRO tiers: Coming soon 🔜

**PAYAI Token Benefits:**
- Fee reduction
- Governance capabilities
- Platform credit system

## API Endpoints

### REST API

**Base URL:** `https://payai.network`

```bash
# Verify payment before broadcasting
POST /verify

# Broadcast and settle transaction
POST /settle

# List available resources
GET /list
```

### Example API Call

```bash
curl -X POST https://payai.network/pay \
  -H "Content-Type: application/json" \
  -d '{
    "amount": "0.001",
    "token": "SOL",
    "recipient": "AgentWalletAddress..."
  }'
```

**Response:**
```json
{
  "tx_hash": "5k...xyz",
  "status": "confirmed",
  "settlement_time": "0.8s",
  "fee_covered": true
}
```

## Integration Guide

### Integration Difficulty: EASY ✅

**Why Easy:**
- No API keys required
- No authentication tokens
- No account creation
- No KYC process
- Simple environment variables

### Setup Steps (3 Steps)

**1. Set Environment Variables:**
```bash
# .env file
X402_FACILITATOR_URL=https://payai.network
X402_NETWORK=solana
X402_TOKEN=USDC
```

**2. Configure in Code:**
```javascript
const X402_CONFIG = {
  facilitator: 'https://payai.network',
  network: 'solana',  // or base, polygon, avalanche, sei, iotex, peaq
  token: 'USDC'       // or SOL, MATIC, AVAX, etc.
};
```

**3. Deploy:**
- Payments automatically route through PayAI
- No additional configuration needed

### Using with Faremeter

```typescript
import { fareMiddleware } from '@faremeter/middleware';

app.use(fareMiddleware({
  facilitator: 'https://payai.network',  // Use PayAI as facilitator
  price: 0.001,
  merchantWallet: process.env.MERCHANT_WALLET
}));
```

## Product Ecosystem

### Live Products

**1. x402 Facilitator (Core Service)**
- Multi-chain payment settlement
- Sub-second processing
- Fee coverage for both parties
- REST and WebSocket APIs

**2. x402 Echo Merchant** 🎯
- **Purpose:** Zero-cost testing environment
- **Feature:** Full refunds on all test transactions
- **Use Case:** Risk-free development and testing
- **Access:** Free for all developers
- **Benefit:** Test without spending real money

**3. Freelance AI Marketplace**
- Agent-to-agent hiring platform
- x402-powered payments
- 24/7 autonomous operation
- Reputation system for agents

### Coming Soon

**1. CT Agent Monetization**
- Twitter (X) AI agent revenue generation
- Social media payment integration
- Content monetization via x402

**2. Token Gateway**
- Token-gated access control
- Three tiers: Basic, Pro, Platinum
- PAYAI token utility integration

## ElizaOS Integration

**ElizaOS:** Open-source AI agent framework from ai16z ($2.6B project)

**PayAI + ElizaOS Integration:**
```javascript
import { PayAIPlugin } from '@payai/eliza-plugin';

const agent = new ElizaAgent({
  plugins: [
    new PayAIPlugin({
      facilitator: 'https://payai.network',
      wallet: myWallet,
      network: 'solana'
    })
  ]
});

// Agent can now make autonomous payments
await agent.purchaseData('https://api.example.com/data');
```

**Benefits:**
- Direct integration with ai16z ecosystem
- Autonomous agent payments
- Multi-chain support for agents
- Built-in payment intelligence

## Documentation Quality: MEDIUM

### Available ✅
- Introduction and overview
- API endpoint specifications
- Network configuration guides
- Basic integration examples
- Pricing tier details

### Limited ⚠️
- Advanced integration patterns
- Error handling guides
- Performance optimization tips
- Custom token integration
- WebSocket API details

**Documentation is functional but could be more comprehensive**

## Community & Support

**Primary Channels:**
- **Discord** - Most active, real-time support ⭐
- **Twitter** - Announcements and updates
- **LinkedIn** - Professional network
- **Telegram** - Community chat
- **GitHub** - Code repositories

**Response Time:** <24 hours on Discord

**Legal & Compliance:**
- Privacy Policy published ✅
- Terms of Service available ✅
- Company incorporated ✅

## Comparison: PayAI vs Alternatives

### PayAI vs Coinbase CDP

| Feature | PayAI | Coinbase CDP |
|---------|-------|--------------|
| **Market Share** | 14% | 77-80% |
| **Network Focus** | Solana-first | Multi-chain |
| **Networks** | 7 chains | 4 chains |
| **Network Fees** | Covered ✅ | User pays ❌ |
| **Pricing** | Free tier (100K) | Usage-based only |
| **Settlement** | <1 second | <2 seconds |
| **Testing** | Echo Merchant (free refunds) | Standard testnet |
| **Target Audience** | AI agents | General crypto |
| **API Keys** | Not required | Required |

### PayAI vs Corbits

| Feature | PayAI | Corbits |
|---------|-------|---------|
| **Hosting** | Managed (hosted) | Self-hosted option |
| **License** | Proprietary | Open-source (LGPL) |
| **Network Fees** | Covered | User pays |
| **Integration** | Easier | More flexible |
| **Customization** | Limited | Full control |
| **Maintenance** | Handled by PayAI | User responsibility |

## Unique Selling Points

1. **Only facilitator covering network fees** for both buyers and merchants
2. **Broadest network coverage** (7 blockchains vs 4 for competitors)
3. **Free testing environment** (Echo Merchant) with guaranteed refunds
4. **Solana-optimized** for lowest latency (<1s settlement)
5. **Built-in AI agent marketplace** (Freelance AI)
6. **PAYAI token utility** for discounts and governance
7. **No API keys required** - simplest possible integration
8. **ElizaOS integration** - direct ai16z ecosystem connection

## When to Choose PayAI

**✅ Choose PayAI if you:**
- Want fastest time-to-market (easiest integration)
- Need multi-chain support (7 networks)
- Want network fees covered
- Building AI agent applications
- Need free testing with Echo Merchant
- Prefer hosted/managed solution
- Want Solana-first optimization
- Building on ElizaOS framework

**❌ Consider alternatives if you:**
- Need full code control (use Corbits)
- Want self-hosting (use Corbits)
- Need enterprise SLA/support (use Crossmint)
- Require extensive customization (use Corbits)

## Quick Start Checklist

- [ ] Set `X402_FACILITATOR_URL=https://payai.network`
- [ ] Choose network (Solana recommended)
- [ ] Select token (USDC most common)
- [ ] Test with Echo Merchant (free refunds)
- [ ] Configure merchant wallet address
- [ ] Deploy application
- [ ] Monitor via Discord community

## Testing with Echo Merchant

**Echo Merchant Benefits:**
- ✅ All transactions fully refunded
- ✅ Risk-free development
- ✅ Real transaction flow testing
- ✅ No cost to test
- ✅ Same API as production

**How to Use:**
1. Point to PayAI facilitator
2. Make test transactions
3. Receive full refunds automatically
4. Validate integration works
5. Switch to production when ready

---

**Related Docs:**
- [Integration Patterns Guide](../guides/integration-patterns.md)
- [Facilitator Comparison Reference](../reference/facilitator-comparison.md)
- [Blockchain Networks Reference](../reference/blockchain-networks.md)
- [Solana Implementation Guide](../guides/solana-implementation.md)
