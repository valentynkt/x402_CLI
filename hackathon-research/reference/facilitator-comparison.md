# Facilitator Comparison Reference

Quick reference for choosing the right payment facilitator for your x402 implementation.

## Comparison Matrix

| Facilitator | Market Share | Networks | Fees | Best For |
|-------------|--------------|----------|------|----------|
| **Coinbase CDP** | 77-80% | 4 chains | User pays gas | Enterprise, Base chain |
| **PayAI** | 14% | 7 chains | Fees covered | Solana-first, AI agents |
| **Self-Hosted** | <10% | Custom | DIY | Full control, custom needs |

---

## 1. Coinbase CDP (Market Leader)

**URL:** Part of Coinbase CDP platform
**Market Share:** 77-80% (dominant)
**Status:** Production, enterprise-grade

### Specifications

| Metric | Value |
|--------|-------|
| **Networks** | Base, Ethereum, Polygon, Arbitrum |
| **Settlement** | ~2 seconds (Base) |
| **Fees** | User pays network gas |
| **Uptime** | 99.9%+ SLA |
| **Free Tier** | No (usage-based) |

### Strengths
- ✅ Market leader (most liquidity)
- ✅ Official Coinbase backing
- ✅ Enterprise support
- ✅ Base chain optimized
- ✅ Comprehensive documentation
- ✅ Regulatory compliant

### Weaknesses
- ❌ User pays gas fees
- ❌ Fewer networks (4 vs 7 for PayAI)
- ❌ Base-focused (not Solana-first)
- ❌ No free tier

### When to Choose
- ✅ Building on Base primarily
- ✅ Need enterprise SLA
- ✅ Want market leader stability
- ✅ Require regulatory compliance
- ❌ Not ideal for Solana-first apps

---

## 2. PayAI Network (Solana Leader)

**URL:** https://payai.network
**Market Share:** 14% (2nd largest)
**Status:** Production

### Specifications

| Metric | Value |
|--------|-------|
| **Networks** | Solana, Base, Polygon, Avalanche, Sei, IoTeX, Peaq (7 total) |
| **Settlement** | <1 second (Solana) |
| **Fees** | $0 (covered by PayAI) |
| **Uptime** | 24/7 (no SLA published) |
| **Free Tier** | 100K settlements/month |

### Strengths
- ✅ **Most networks** (7 blockchains)
- ✅ **Fees covered** for both parties (unique)
- ✅ **Free tier** (100K/month)
- ✅ **Fastest** on Solana (<1s)
- ✅ **Echo Merchant** (free testing with refunds)
- ✅ **ElizaOS integration** (ai16z ecosystem)
- ✅ **No API keys** required

### Weaknesses
- ❌ Smaller market share (14% vs 77%)
- ❌ No published enterprise SLA
- ❌ Newer/less established

### When to Choose
- ✅ Building on Solana primarily
- ✅ Need multi-chain (7 networks)
- ✅ Want fees covered
- ✅ Need free tier for testing/scaling
- ✅ Building AI agent applications
- ✅ Want simplest integration (no API keys)

---

## 3. Self-Hosted Facilitator

**Implementation:** Faremeter framework
**Market Share:** <10% (niche)
**Status:** Production-ready

### Specifications

| Metric | Value |
|--------|-------|
| **Networks** | Any (plugin-based) |
| **Settlement** | Depends on chain |
| **Fees** | Gas only (no facilitator fee) |
| **Uptime** | Your responsibility |
| **Free Tier** | N/A (self-hosted) |

### Strengths
- ✅ **Full control** over infrastructure
- ✅ **No vendor lock-in**
- ✅ **Customizable** (modify source)
- ✅ **No facilitator fees** (just gas)
- ✅ **Open source** (LGPL-3.0)
- ✅ **Privacy** (no third party)

### Weaknesses
- ❌ **Maintenance burden** (DevOps required)
- ❌ **Complex setup** (not plug-and-play)
- ❌ **No SLA** (DIY reliability)
- ❌ **Scaling challenges** (handle yourself)

### When to Choose
- ✅ Need complete control
- ✅ Have DevOps resources
- ✅ Custom blockchain needs
- ✅ Privacy critical
- ✅ High volume (save on fees)
- ❌ Not for quick prototypes

---

## Feature Comparison

| Feature | Coinbase CDP | PayAI | Self-Hosted |
|---------|--------------|-------|-------------|
| **Market Share** | 77-80% | 14% | <10% |
| **Networks** | 4 chains | 7 chains | Unlimited |
| **Network Fees** | User pays | Covered | User pays |
| **Platform Fees** | Yes | No | No |
| **Settlement (Solana)** | N/A | <1s | <1s |
| **Settlement (Base)** | ~2s | ~2s | ~2s |
| **Free Tier** | No | 100K/mo | N/A |
| **API Keys Required** | Yes | No | No |
| **SLA** | 99.9% | None published | DIY |
| **Setup Complexity** | Easy | Easy | Hard |
| **Maintenance** | Managed | Managed | DIY |
| **Customization** | Limited | Limited | Full |
| **Open Source** | No | No | Yes |

---

## Performance Comparison

### Settlement Times

| Chain | Coinbase CDP | PayAI | Self-Hosted |
|-------|--------------|-------|-------------|
| **Solana** | N/A | <1s | <1s |
| **Base** | ~2s | ~2s | ~2s |
| **Polygon** | ~3s | ~3s | ~3s |
| **Ethereum** | ~15s | N/A | ~15s |
| **Avalanche** | N/A | ~3s | ~3s |

### Cost Comparison

**Example: 10,000 transactions @ $0.001 USDC each**

| Facilitator | Network Gas | Platform Fee | Total Cost |
|-------------|-------------|--------------|------------|
| **Coinbase CDP** | ~$10 | ~$20 | ~$30 |
| **PayAI** | $0 (covered) | $0 | $0 |
| **Self-Hosted** | ~$10 | $0 | ~$10 + DevOps |

---

## Pricing Models

### Coinbase CDP
```
Setup: Free
Per Transaction: Variable (gas + platform fee)
Monthly: Usage-based
Enterprise: Custom pricing
```

### PayAI
```
Setup: Free
FREE Tier: 100K settlements/month, 4 req/s
BASIC Tier: $1,500 PAYAI tokens, 500K/month, 10 req/s
PRO Tier: $2,800 PAYAI tokens, 1M/month, 25 req/s
Network Fees: Covered by PayAI
```

### Self-Hosted
```
Setup: Free (LGPL-3.0)
Infrastructure: Your servers
Gas Fees: You pay
DevOps: Your team
Total: Infrastructure + DevOps costs
```

---

## Testing Environments

### Coinbase CDP
```
Testnet: Base Sepolia
Faucet: https://www.coinbase.com/faucets
Cost: Free testnet tokens
```

### PayAI
```
Testnet: Supports all network testnets
Echo Merchant: FREE with guaranteed refunds
Cost: $0 (real transactions, full refunds)
```

### Self-Hosted
```
Testnet: Any supported network testnet
Faucet: Network-specific faucets
Cost: Free testnet tokens
```

---

## Decision Tree

```
Need Solana-first?
├─ YES → PayAI (fastest, fees covered)
└─ NO ↓

Need Base/Ethereum focus?
├─ YES → Coinbase CDP (market leader)
└─ NO ↓

Need complete control?
├─ YES → Self-Hosted (Faremeter)
└─ NO ↓

Need 7+ blockchains?
├─ YES → PayAI (7 chains vs 4)
└─ NO ↓

Need enterprise SLA?
├─ YES → Coinbase CDP (99.9%)
└─ NO ↓

Want fees covered?
└─ PayAI (unique feature)
```

---

## Configuration Examples

### Coinbase CDP
```typescript
import { fareMiddleware } from '@faremeter/middleware';

app.use(fareMiddleware({
  facilitator: 'https://cdp.coinbase.com/x402',  // CDP facilitator
  price: 0.001,
  merchantWallet: process.env.MERCHANT_WALLET,
  network: 'base'
}));
```

### PayAI
```typescript
app.use(fareMiddleware({
  facilitator: 'https://payai.network',  // PayAI facilitator
  price: 0.001,
  merchantWallet: process.env.MERCHANT_WALLET,
  network: 'solana'
}));
```

### Self-Hosted
```typescript
app.use(fareMiddleware({
  facilitator: 'https://your-facilitator.com',  // Your server
  price: 0.001,
  merchantWallet: process.env.MERCHANT_WALLET,
  network: 'solana'
}));
```

---

## Migration Between Facilitators

### Easy Migration (Just Change URL)

```typescript
// From CDP to PayAI
// Before:
const facilitator = 'https://cdp.coinbase.com/x402';

// After:
const facilitator = 'https://payai.network';

// That's it! No other code changes needed.
```

**x402 is designed for facilitator portability - switching is a 1-line change!**

---

**Related Docs:**
- [PayAI Tool Guide](../tools/payai-network-guide.md)
- [Corbits Tool Guide](../tools/corbits-faremeter-guide.md)
- [Integration Patterns](../guides/integration-patterns.md)
- [SDK Comparison](./sdk-comparison.md)
