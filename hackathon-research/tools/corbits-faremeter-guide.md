# Corbits / Faremeter Guide

**Quick Links:** [Website](https://corbits.dev/) | [GitHub](https://github.com/faremeter) | [Docs](https://docs.corbits.dev)
**License:** LGPL-3.0 (Open Source) | **Integration Difficulty:** Medium

## Overview

Corbits is the **only fully open-source x402 framework**, built around **Faremeter** - a modular, self-hostable payment infrastructure for instant HTTP-based cryptocurrency transactions. Ideal for developers who want full control and customization.

## Key Features

- ✅ **Open Source:** Complete code access, self-hostable, extendable
- ✅ **Blockchain Agnostic:** Plugin system for any blockchain (Solana, Base, Polygon, custom EVMs)
- ✅ **Three Integration Methods:** Wrappers, middleware, or proxies
- ✅ **No Vendor Lock-in:** Host your own facilitator
- ✅ **Premium API Access:** Payment-proxied access to DFlow, Helius, Triton, Titan, Nansen, Yatori

## Technical Specifications

### Technology Stack
- **Language:** TypeScript (92.8%)
- **Architecture:** Modular plugin system
- **Package Manager:** pnpm workspace monorepo
- **Build System:** Turbo

### Repository Structure
```
faremeter/
├── packages/
│   ├── fetch/          # Client library (drop-in fetch wrapper)
│   ├── middleware/     # Server middleware (Express, Fastify)
│   └── facilitator/    # Self-hosted facilitator app
├── examples/           # Integration examples
└── docs/              # Comprehensive documentation
```

### Supported Networks

| Network | Support | Primary Token | Status |
|---------|---------|---------------|--------|
| **Solana** | Primary | USDC (SPL) | Production |
| **Base** | Secondary | USDC (ERC-20) | Production |
| **Polygon** | Secondary | USDC (ERC-20) | Production |
| **Custom EVMs** | Plugin-based | Any ERC-20 | Beta |

### Payment Method
- USDC micropayments (replaces API keys)
- HTTP 402 Payment Required protocol
- EIP-3009 gasless transfers (Base)
- SPL token transfers (Solana)

## Integration Methods

### 1. Client Wrapper (Easiest)
**Use Case:** Client apps, AI agents
**Effort:** Minimal - just replace `fetch()` with `paidFetch()`

```typescript
import { paidFetch } from '@faremeter/fetch';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';

const wallet = new PhantomWalletAdapter();
await wallet.connect();

const response = await paidFetch('https://api.example.com/data', {
  wallet,
  maxAmount: 0.01 // USDC
});
```

**Installation:**
```bash
npm install @faremeter/fetch @solana/wallet-adapter-phantom
```

### 2. Server Middleware (Flexible)
**Use Case:** API providers, merchants
**Effort:** Medium - add middleware to server

```typescript
import express from 'express';
import { fareMiddleware } from '@faremeter/middleware';

const app = express();

app.use(fareMiddleware({
  facilitator: 'https://payai.network',
  price: 0.001, // USDC per request
  merchantWallet: process.env.MERCHANT_WALLET
}));

app.get('/api/data', (req, res) => {
  // Payment already verified
  res.json({ data: 'your data' });
});
```

**Installation:**
```bash
npm install @faremeter/middleware express
```

### 3. Proxy (Zero Code Changes)
**Use Case:** Legacy systems, third-party APIs
**Effort:** Advanced - network-level integration

- Route traffic through payment gateway
- No modifications to existing services
- Requires facilitator deployment

## Wallet Integration

**Supported Wallets:**
- **Phantom** (Solana) - Pre-built adapter
- **Crossmint** - Pre-built adapter
- **Custom wallets** - Plugin architecture

```typescript
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';

const wallet = new PhantomWalletAdapter();
await wallet.connect();

// Use with any paidFetch call
const response = await paidFetch(url, { wallet });
```

## Partner APIs (Payment-Proxied)

Access premium APIs via Corbits proxy with micropayments:

| Service | Purpose | Traditional Pricing | x402 Pricing |
|---------|---------|---------------------|--------------|
| **DFlow** | Solana DEX routing | Subscription | Pay-per-request |
| **Helius** | Solana RPC | Tiered plans ($99-$499/mo) | ~$0.0001/request |
| **Triton** | High-performance RPC | Enterprise | Per-request |
| **Titan Exchange** | Trading infrastructure | API key | USDC payments |
| **Nansen** | Blockchain analytics | $150+/month | Query-based |
| **Yatori** | Solana token data | Subscription | Micropayments |

**Access Example:**
```typescript
const response = await paidFetch('https://api.corbits.dev/helius/rpc', {
  method: 'POST',
  body: JSON.stringify({ /* RPC request */ }),
  wallet: myWallet
});
```

## Framework Components

### 1. Client Library (`@faremeter/fetch`)
- Drop-in replacement for `fetch()`
- Automatic payment handling
- Wallet integration
- Configurable price limits

### 2. Server Middleware (`@faremeter/middleware`)
- Framework-agnostic (Express, Fastify, etc.)
- Payment verification
- Configurable pricing per endpoint
- Facilitator integration

### 3. Payment Facilitator
- Self-hosted facilitator application
- Payment verification
- Transaction broadcasting
- Settlement confirmation

### 4. Legacy Service Proxy
- Route existing APIs through payment gateway
- No code changes required
- Drop-in payment layer

## Pricing & Business Model

### Framework Costs
- **Framework:** FREE (LGPL-3.0)
- **Self-hosting:** FREE (your infrastructure)
- **No subscriptions:** Pay only blockchain gas
- **API Access:** Pay-per-use USDC micropayments

### Cost Comparison

| Model | Corbits/Faremeter | Traditional API |
|-------|------------------|-----------------|
| Setup | $0 | $0-$100 |
| Monthly | $0 | $10-$1,000 |
| Per Request | ~$0.0001-$0.01 | Included or ~$0.001 |
| Overage | Impossible (pay-per-use) | Expensive |

## Notable Projects

### Mallory (AI Chat App)
- **Built by:** Dark Research + Corbits
- **Stack:** React Native + Faremeter + Solana
- **Features:** x402-powered AI chat, pay-per-message
- **Status:** Open-source, production

### Switchboard Oracle Integration
- **Achievement:** First x402-compatible oracle (Oct 23, 2025)
- **Implementation:** Via Corbits SDK
- **Capability:** AI agents pay for oracle queries on-demand
- **Cost:** <$0.001 per query

## Documentation Quality: HIGH

**Available Resources:**
- **QUICKSTART.md** - Initial setup
- **DEV.md** - Development environment
- **ARCHITECTURE.md** - Design decisions
- **COMPATIBILITY.md** - Standards alignment
- **API Reference** - Complete API docs
- **Integration Guides** - Step-by-step tutorials

**GitHub:**
- URL: https://github.com/faremeter
- Stars: 36+ (growing)
- Active maintainers
- Regular updates

## Community & Support

**Channels:**
- **Telegram** - Primary community chat
- **Email:** support@corbits.dev
- **Twitter:** @corbits_dev
- **GitHub Issues** - Bug reports & features

**Response Time:** ~24 hours

## Integration Difficulty Breakdown

### Easy ✅
- Well-documented with clear examples
- Multiple integration paths
- Active community support
- Pre-built wallet adapters

### Medium ⚠️
- TypeScript/JavaScript required
- Blockchain wallet setup needed
- HTTP 402 protocol understanding
- Monorepo complexity for advanced use

### Advanced 🔧
- Custom facilitator deployment
- Blockchain plugin development
- Legacy system proxy setup
- Custom wallet adapters

## Unique Selling Points

1. **Only fully open-source x402 framework** - Complete code control
2. **Blockchain-agnostic** - Not locked to single chain
3. **Developer control** - Self-hostable, no vendor lock-in
4. **Partnership ecosystem** - Switchboard, Dark Research, premium APIs
5. **Flexible integration** - Wrappers, middleware, or proxies

## When to Choose Corbits

**✅ Choose Corbits if you:**
- Want full code control and customization
- Need blockchain flexibility
- Prefer self-hosting
- Want to avoid vendor lock-in
- Need advanced integration patterns
- Require legacy system integration

**❌ Consider alternatives if you:**
- Need fastest time-to-market (use PayAI)
- Want enterprise support (use Crossmint)
- Prefer hosted solutions only
- Don't want to manage infrastructure

## Quick Start Checklist

- [ ] Install `@faremeter/fetch` for client-side
- [ ] Install `@faremeter/middleware` for server-side
- [ ] Set up wallet adapter (Phantom/Crossmint)
- [ ] Choose facilitator (PayAI or self-hosted)
- [ ] Configure merchant wallet address
- [ ] Set pricing per endpoint
- [ ] Test with small amounts
- [ ] Review documentation on docs.corbits.dev

---

**Related Docs:**
- [Integration Patterns Guide](../guides/integration-patterns.md)
- [Wallet Integration Guide](../guides/wallet-integration-guide.md)
- [SDK Comparison Reference](../reference/sdk-comparison.md)
