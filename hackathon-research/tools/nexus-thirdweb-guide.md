# Nexus by Thirdweb Guide

**Quick Links:** [Nexus](https://nexus.thirdweb.com/) | [x402 Docs](https://portal.thirdweb.com/payments/x402) | [Changelog](https://blog.thirdweb.com/changelog/x402-support/)
**License:** Commercial (Free tier available) | **Integration Difficulty:** Easy

## Overview

**Nexus** is a payment-gated edge proxy by Thirdweb that enables x402 payment protocol with any API. It unlocks AI agents to use APIs without creating accounts or managing API keys, while allowing API providers to get paid for requests. The proxy runs on globally distributed edge infrastructure.

**Note:** The protocol is **x402** (not x420). Earlier references to "x420" appear to be informal naming that has been standardized to x402.

## Key Features

- ✅ **Payment-Gated API Router:** Turn any API into a paid service
- ✅ **No API Keys Required:** Agents pay with crypto instead of managing keys
- ✅ **Global Edge Network:** Low-latency proxy infrastructure
- ✅ **Free to Set Up:** No upfront costs for API providers
- ✅ **26+ Chain Support:** Broadest multi-chain coverage in x402
- ✅ **SDK & API Integration:** Both code and REST API options

## Technical Specifications

### Technology Stack
- **Protocol:** x402 (HTTP 402 Payment Required)
- **Infrastructure:** Globally distributed edge proxy
- **Payment Rails:** USDC on 26+ chains
- **Client Libraries:** TypeScript SDK + REST API
- **Architecture:** Serverless edge functions

### Repository Structure
```
thirdweb-sdk/
├── packages/
│   └── thirdweb/
│       └── x402/          # x402 implementation
│           ├── client.ts  # Client SDK
│           ├── server.ts  # Server middleware
│           └── proxy.ts   # Nexus proxy utils
```

### Supported Networks

| Network | Support | Primary Token | Status |
|---------|---------|---------------|--------|
| **Solana** | Planned | USDC (SPL) | Target: Oct 30, 2025 |
| **Base** | Primary | USDC | Production |
| **Ethereum** | Primary | USDC | Production |
| **Polygon** | Primary | USDC | Production |
| **Arbitrum** | Secondary | USDC | Production |
| **Optimism** | Secondary | USDC | Production |
| **+20 More** | Secondary | Various | Production |

**Total: 26+ chains supported**

## Integration Methods

### 1. Client SDK (Recommended)

**Use Case:** AI agents calling paid APIs
**Effort:** Easy - wrap fetch with payment handler

```typescript
import { wrapFetchWithPayment } from "thirdweb/x402";
import { createThirdwebClient } from "thirdweb";

const client = createThirdwebClient({
  clientId: process.env.THIRDWEB_CLIENT_ID
});

// Wrap native fetch to handle 402 automatically
const paidFetch = wrapFetchWithPayment({
  client,
  wallet: myWallet,
  maxAmount: 0.10 // USDC spending limit
});

// Use like normal fetch - payments handled automatically
const response = await paidFetch('https://api.example.com/data');
const data = await response.json();
```

**Installation:**
```bash
npm install thirdweb
```

### 2. Server-Side (API Provider)

**Use Case:** Existing APIs wanting to add payment gates
**Effort:** Easy - configure Nexus proxy

```typescript
import { createNexusGateway } from "thirdweb/x402";

const gateway = createNexusGateway({
  upstreamAPI: 'https://my-existing-api.com',
  pricing: {
    '/data': 0.01,      // $0.01 per request
    '/premium': 0.10    // $0.10 per request
  },
  recipient: {
    address: process.env.MERCHANT_WALLET,
    chain: 'base'
  }
});

// Nexus handles:
// 1. Incoming requests
// 2. 402 Payment Required responses
// 3. Payment verification
// 4. Proxying to upstream API
```

### 3. REST API (No Code)

**Use Case:** Quick testing, non-JS environments
**Effort:** Minimal - just HTTP requests

```bash
# 1. Request resource
curl https://nexus.thirdweb.com/proxy/api-id/data

# 2. Receive 402 Payment Required with payment details
# Response includes: amount, recipient, chain, nonce

# 3. Sign payment authorization with wallet

# 4. Retry with payment header
curl https://nexus.thirdweb.com/proxy/api-id/data \
  -H "X-Payment-Authorization: <signed_payment>"
```

## Payment Flow

### Automatic Flow with SDK

```typescript
// 1. Initial request
const response = await paidFetch(url);

// SDK automatically handles:
// - 402 response detection
// - Payment amount validation (vs maxAmount)
// - Payment signature with wallet
// - Request retry with payment header
// - Response delivery

// Developer only sees final result
const data = await response.json();
```

### Manual Flow (Advanced)

```typescript
import { signPaymentAuthorization } from "thirdweb/x402";

// 1. Make initial request
const response1 = await fetch(url);

if (response1.status === 402) {
  // 2. Parse payment requirements
  const paymentInfo = await response1.json();

  // 3. Verify amount is acceptable
  if (paymentInfo.amount > maxAmount) {
    throw new Error('Price too high');
  }

  // 4. Sign payment authorization
  const signature = await signPaymentAuthorization({
    wallet: myWallet,
    amount: paymentInfo.amount,
    recipient: paymentInfo.recipient,
    chain: paymentInfo.chain,
    nonce: paymentInfo.nonce
  });

  // 5. Retry with payment
  const response2 = await fetch(url, {
    headers: {
      'X-Payment-Authorization': signature
    }
  });

  const data = await response2.json();
}
```

## Use Cases for Hackathon

### 1. API Key Replacement
**Scenario:** Eliminate API key management for AI agents
**Implementation:** Proxy existing APIs through Nexus
**Benefit:** Agents pay directly, no authentication needed

### 2. Multi-Chain Data Feeds
**Scenario:** Cross-chain data aggregation
**Implementation:** Accept payments on 26 chains
**Benefit:** Broadest chain support in ecosystem

### 3. Edge-Optimized Services
**Scenario:** Low-latency global AI agent access
**Implementation:** Leverage Nexus edge network
**Benefit:** <50ms additional latency worldwide

### 4. Legacy API Monetization
**Scenario:** Add payments to existing APIs without changes
**Implementation:** Point Nexus at upstream API
**Benefit:** Zero code changes to existing service

## Thirdweb Ecosystem Integration

### Embedded Wallets
Thirdweb's embedded wallet infrastructure integrates with x402:
- **Email Wallets:** Users sign in with email, pay with crypto
- **Social Wallets:** Google/Apple/Twitter-based payment wallets
- **Smart Wallets:** Account abstraction for gas sponsorship

```typescript
import { inAppWallet } from "thirdweb/wallets";

const wallet = inAppWallet();
await wallet.connect({
  client,
  strategy: "email",
  email: "agent@example.com"
});

// Use for x402 payments
const paidFetch = wrapFetchWithPayment({ client, wallet });
```

### Additional Thirdweb Features
- **Connect SDK:** Multi-wallet connection
- **Storage:** IPFS/Arweave integration
- **Auth:** On-chain authentication
- **Engine:** Backend infrastructure

## Pricing & Business Model

### Nexus Costs
- **Setup:** FREE
- **Proxy Service:** FREE tier available
- **Transaction Fees:** Network gas only
- **Premium Features:** Pay-as-you-go pricing

### Thirdweb Pricing Tiers

| Tier | Monthly | Included | Best For |
|------|---------|----------|----------|
| **Starter** | FREE | 1M requests | Testing, MVPs |
| **Growth** | $99 | 10M requests | Small production |
| **Pro** | $499 | 50M requests | Scale-ups |
| **Enterprise** | Custom | Unlimited | Large enterprises |

### Cost Comparison

| Model | Nexus | Traditional API |
|-------|-------|-----------------|
| Setup | $0 | $0-$500 |
| Monthly | $0-$499 | $50-$5,000 |
| Per Request | Gas + amount | Included/overage |
| Multi-Chain | 26+ chains | Single chain |

## Chain Support Details

### Primary Chains (Full Support)
- Ethereum Mainnet
- Base
- Polygon
- Arbitrum One
- Optimism
- BNB Smart Chain

### Secondary Chains (Full Support)
- Avalanche C-Chain
- Fantom Opera
- Gnosis Chain
- Celo
- Aurora
- Moonbeam
- Moonriver
- Cronos
- Harmony One
- Evmos
- Kava EVM
- Metis
- Boba Network
- Syscoin
- +more

### Coming Soon
- **Solana** (Target: October 30, 2025)

## Integration Difficulty Breakdown

### Easy ✅
- Thirdweb SDK well-documented
- `wrapFetchWithPayment()` is drop-in
- Free tier for testing
- No server infrastructure needed
- Multi-wallet support built-in

### Medium ⚠️
- Thirdweb account required
- Client ID setup
- Wallet integration
- Understanding payment flow
- Chain selection considerations

### Advanced 🔧
- Custom pricing policies
- Advanced error handling
- Multi-chain recipient management
- Enterprise SLA negotiation

## Unique Selling Points

1. **Broadest Chain Support:** 26+ chains, more than any competitor
2. **Edge Infrastructure:** Globally distributed, low-latency
3. **No API Keys:** Eliminates key management for agents
4. **Free to Start:** No upfront costs
5. **Ecosystem Integration:** Embedded wallets, storage, auth
6. **Thirdweb Credibility:** Established Web3 infrastructure provider

## When to Choose Nexus

**✅ Choose Nexus if you:**
- Need multi-chain support (26+ chains)
- Want fastest integration (SDK wrapper)
- Prefer hosted solution (no infrastructure)
- Need global edge performance
- Want embedded wallet support
- Are already using Thirdweb ecosystem

**❌ Consider alternatives if you:**
- Solana-first project (wait for Solana support)
- Want open-source self-hosted (use Corbits)
- Need MCP integration (use MCPay)
- Prefer zero vendor lock-in
- Need sub-$0.001 pricing

## Documentation Quality: HIGH

**Available Resources:**
- **Portal Docs:** https://portal.thirdweb.com/payments/x402
- **Blog/Changelog:** Regular updates on features
- **SDK Reference:** Complete TypeScript docs
- **Code Examples:** Integration patterns
- **Video Tutorials:** Thirdweb YouTube channel

**GitHub:**
- URL: https://github.com/thirdweb-dev/js
- Package: `thirdweb` (npm)
- Stars: 1,400+
- Active maintenance
- Community support

## Community & Support

**Channels:**
- **Discord:** https://discord.gg/thirdweb (Primary support)
- **GitHub Issues:** Bug reports & features
- **Twitter/X:** @thirdweb
- **Email:** support@thirdweb.com

**Response Time:**
- Discord: <24 hours
- Support tickets: <48 hours
- Enterprise: SLA available

## Current Status & Roadmap

### Current Status (November 2025)
- ✅ 26+ EVM chains supported
- ✅ SDK integration complete
- ✅ Edge infrastructure live
- ✅ Free tier available
- ⏳ Solana support (target Oct 30, 2025)

### Roadmap
- Solana integration completion
- Additional chain support
- Enhanced analytics dashboard
- Webhook notifications
- Advanced pricing rules

## Quick Start Checklist

- [ ] Create Thirdweb account at thirdweb.com
- [ ] Get client ID from dashboard
- [ ] Install `thirdweb` package
- [ ] Set up wallet (embedded or external)
- [ ] Choose target chain(s)
- [ ] Wrap fetch with `wrapFetchWithPayment()`
- [ ] Test with small amounts
- [ ] Configure pricing for your API (if provider)
- [ ] Deploy to production

## Code Example: Complete Integration

```typescript
import { createThirdwebClient } from "thirdweb";
import { wrapFetchWithPayment } from "thirdweb/x402";
import { inAppWallet } from "thirdweb/wallets";
import { base } from "thirdweb/chains";

// === CLIENT SETUP ===
const client = createThirdwebClient({
  clientId: process.env.THIRDWEB_CLIENT_ID
});

// Set up embedded wallet (email-based)
const wallet = inAppWallet();
await wallet.connect({
  client,
  chain: base,
  strategy: "email",
  email: "agent@example.com"
});

// Wrap fetch with automatic payment handling
const paidFetch = wrapFetchWithPayment({
  client,
  wallet,
  chain: base,
  maxAmount: 0.25 // Max $0.25 USDC per request
});

// === USE LIKE NORMAL FETCH ===
async function getMarketData(symbol: string) {
  try {
    const response = await paidFetch(
      `https://nexus.thirdweb.com/proxy/market-api/${symbol}`
    );

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }

    const data = await response.json();
    console.log('Market data:', data);
    console.log('Payment tx:', response.headers.get('X-Payment-Tx'));

    return data;
  } catch (error) {
    console.error('Payment or request failed:', error);
    throw error;
  }
}

// Payment automatically handled on 402 responses
const solPrice = await getMarketData('SOL');
const ethPrice = await getMarketData('ETH');
```

## Hackathon Tips

### Prize Track Alignment
- **Best x402 Agent Application** ✅ (Strong SDK support)
- **Best Agent Money Protocol** ✅ (API key elimination angle)
- **CDP Integration** ⚠️ (Competitor to Coinbase, might conflict)

### Competitive Advantages
1. **Fastest Integration:** Literally one function call (`wrapFetchWithPayment`)
2. **26+ Chains:** Broadest support shows technical capability
3. **Edge Infrastructure:** Performance story for judges
4. **Embedded Wallets:** UX advantage for non-crypto users
5. **Established Company:** Thirdweb credibility reduces risk perception

### Integration Time
- **Basic Setup:** 15-30 minutes (fastest in ecosystem)
- **Production Ready:** 2-4 hours
- **Advanced Features:** 8+ hours

### Demo Strategy
1. Show one-line integration (`wrapFetchWithPayment`)
2. Demonstrate multi-chain payment in live demo
3. Highlight global edge performance (latency metrics)
4. Show email wallet UX (non-crypto-native friendly)
5. Compare setup time vs competitors

---

**Related Docs:**
- [x402 Protocol Specification](../x402-protocol-specification.md)
- [Multi-Chain Integration Guide](../guides/multi-chain-guide.md)
- [SDK Comparison Reference](../reference/sdk-comparison.md)
- [Code Repositories](../reference/code-repositories.md)
