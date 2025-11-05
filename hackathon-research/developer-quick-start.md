# Developer Quick Start

Your unified hub for x402 development. This document replaces the old ecosystem-tools-reference.md and technical-stack-reference.md with a modular, focused structure.

## Navigation

**Need to get started fast?** → [5-Minute Quick Start](#5-minute-quick-start)
**Choosing tools?** → [Decision Trees](#decision-trees)
**Building something?** → [Common Scenarios](#common-scenarios)

---

## 5-Minute Quick Start

### Option 1: AI Agent (Client)

```typescript
// Install
npm install @faremeter/fetch @solana/wallet-adapter-phantom

// Code (3 lines)
import { paidFetch } from '@faremeter/fetch';
const wallet = new PhantomWalletAdapter();
const data = await paidFetch(url, { wallet, maxAmount: 0.01 });
```

**Next Steps:** [Integration Patterns Guide](guides/integration-patterns.md)

### Option 2: API Provider (Server)

```typescript
// Install
npm install @faremeter/middleware express

// Code (4 lines)
import { fareMiddleware } from '@faremeter/middleware';
app.use(fareMiddleware({
  price: 0.001,
  merchantWallet: process.env.MERCHANT_WALLET
}));
```

**Next Steps:** [Integration Patterns Guide](guides/integration-patterns.md)

---

## Decision Trees

### 1. Which Tool Should I Use?

```
What are you building?

├─ AI Agent consuming APIs
│  → Use: Faremeter Fetch
│  → Guide: [Corbits Tool Guide](tools/corbits-faremeter-guide.md)
│
├─ API Provider monetizing endpoints
│  → Use: Faremeter Middleware
│  → Guide: [Corbits Tool Guide](tools/corbits-faremeter-guide.md)
│
├─ Multi-chain application (7+ chains)
│  → Use: PayAI or Crossmint
│  → Guides: [PayAI](tools/payai-network-guide.md) | [Crossmint](tools/crossmint-enterprise-guide.md)
│
├─ Enterprise application (Amazon/Shopify)
│  → Use: Crossmint
│  → Guide: [Crossmint Tool Guide](tools/crossmint-enterprise-guide.md)
│
├─ MCP Server monetization
│  → Use: x402-mcp
│  → Guide: [Integration Patterns](guides/integration-patterns.md#pattern-4-mcp-server-monetization)
│
└─ Testing/monitoring transactions
   → Use: x402scan
   → Guide: [x402scan Explorer Guide](tools/x402scan-explorer-guide.md)
```

### 2. Which Blockchain Should I Use?

```
What's your priority?

├─ Lowest cost (<$0.0001 per tx)
│  → Solana
│  → Guide: [Solana Implementation](guides/solana-implementation.md)
│
├─ Fastest settlement (<1s)
│  → Solana
│  → Guide: [Solana Implementation](guides/solana-implementation.md)
│
├─ EVM compatibility required
│  → Base (Coinbase L2)
│  → Reference: [Blockchain Networks](reference/blockchain-networks.md)
│
├─ Need 7+ blockchains
│  → PayAI or Crossmint
│  → Reference: [SDK Comparison](reference/sdk-comparison.md)
│
└─ Balance cost/speed
   → Base or Polygon
   → Reference: [Blockchain Networks](reference/blockchain-networks.md)
```

### 3. Which Wallet Should I Use?

```
Who's using the wallet?

├─ End users (browser apps)
│  → Phantom (Solana) or CDP Embedded (multi-chain)
│  → Guide: [Wallet Integration](guides/wallet-integration-guide.md)
│
├─ AI agents (autonomous)
│  → CDP Server Wallets
│  → Guide: [Wallet Integration](guides/wallet-integration-guide.md#3-cdp-server-wallets)
│
├─ Enterprise (compliance)
│  → Crossmint Wallets
│  → Guide: [Crossmint Tool Guide](tools/crossmint-enterprise-guide.md)
│
└─ Mobile apps
   → Phantom or CDP Embedded
   → Guide: [Wallet Integration](guides/wallet-integration-guide.md)
```

---

## Common Scenarios

### Scenario 1: "I want to build an AI agent that pays for APIs"

**Time:** 30 minutes
**Difficulty:** Easy ⭐

**Steps:**
1. Choose: Faremeter Fetch + Phantom Wallet
2. Read: [Integration Patterns - Pattern 1](guides/integration-patterns.md#pattern-1-simple-client-fetch-wrapper)
3. Install: `npm install @faremeter/fetch @solana/wallet-adapter-phantom`
4. Test on: [Echo Merchant](tools/payai-network-guide.md#testing-with-echo-merchant) (free refunds)
5. Deploy: Monitor on [x402scan](tools/x402scan-explorer-guide.md)

**Full Guide:** [Corbits Tool Guide](tools/corbits-faremeter-guide.md)

---

### Scenario 2: "I want to monetize my API"

**Time:** 1 hour
**Difficulty:** Easy ⭐

**Steps:**
1. Choose: Faremeter Middleware
2. Read: [Integration Patterns - Pattern 2](guides/integration-patterns.md#pattern-2-protected-api-middleware)
3. Install: `npm install @faremeter/middleware express`
4. Configure: Set price, merchant wallet, facilitator
5. Test: Use x402scan to verify transactions

**Full Guide:** [Integration Patterns Guide](guides/integration-patterns.md)

---

### Scenario 3: "I want to build on multiple blockchains"

**Time:** 2-4 hours
**Difficulty:** Medium ⭐⭐

**Steps:**
1. Choose: PayAI (7 chains) or Crossmint (15+ chains)
2. Read: [PayAI Tool Guide](tools/payai-network-guide.md) or [Crossmint](tools/crossmint-enterprise-guide.md)
3. Compare: [Facilitator Comparison](reference/facilitator-comparison.md)
4. Install SDK: See respective tool guide
5. Configure: Multi-chain setup

**Full Guides:**
- [PayAI Tool Guide](tools/payai-network-guide.md)
- [Crossmint Enterprise Guide](tools/crossmint-enterprise-guide.md)

---

### Scenario 4: "I want to monetize Claude AI tools (MCP)"

**Time:** 2-4 hours
**Difficulty:** Medium ⭐⭐

**Steps:**
1. Choose: x402-mcp
2. Read: [Integration Patterns - Pattern 4](guides/integration-patterns.md#pattern-4-mcp-server-monetization)
3. Install: `npm install x402-mcp @modelcontextprotocol/sdk`
4. Create: Paid MCP tools
5. Test: In Claude Desktop

**Full Guide:** [Integration Patterns Guide](guides/integration-patterns.md)

---

### Scenario 5: "I need enterprise features (Amazon, compliance)"

**Time:** 4-8 hours
**Difficulty:** Medium ⭐⭐

**Steps:**
1. Choose: Crossmint (only option for traditional commerce)
2. Read: [Crossmint Enterprise Guide](tools/crossmint-enterprise-guide.md)
3. Contact: Crossmint sales for API access
4. Integrate: Multi-protocol support (x402 + Visa + Mastercard)
5. Deploy: With enterprise SLA

**Full Guide:** [Crossmint Enterprise Guide](tools/crossmint-enterprise-guide.md)

---

## Documentation Structure

### 📦 Tools (Platform-Specific Guides)

Detailed guides for each major x402 platform:

- [Corbits/Faremeter Guide](tools/corbits-faremeter-guide.md) - Open-source framework
- [PayAI Network Guide](tools/payai-network-guide.md) - Multi-chain facilitator
- [x402scan Explorer Guide](tools/x402scan-explorer-guide.md) - Transaction explorer
- [Crossmint Enterprise Guide](tools/crossmint-enterprise-guide.md) - Enterprise platform

### 📚 Guides (How-To Documentation)

Step-by-step guides for common tasks:

- [Integration Patterns Guide](guides/integration-patterns.md) - 5 implementation patterns
- [Wallet Integration Guide](guides/wallet-integration-guide.md) - All wallet types
- [Solana Implementation Guide](guides/solana-implementation.md) - Solana-specific details
- [Testing and Monitoring Guide](guides/testing-and-monitoring.md) - Testing tools & practices
- [Security Best Practices Guide](guides/security-best-practices.md) - Security essentials

### 📖 Reference (Quick Lookup)

Quick reference documentation:

- [SDK Comparison Reference](reference/sdk-comparison.md) - Choose the right SDK
- [Facilitator Comparison Reference](reference/facilitator-comparison.md) - Compare facilitators
- [Blockchain Networks Reference](reference/blockchain-networks.md) - Network specs & config
- [Code Repositories Reference](reference/code-repositories.md) - All GitHub repos

---

## Quick Links by Role

### For AI Agent Developers
1. [Integration Patterns - Client](guides/integration-patterns.md#pattern-1-simple-client-fetch-wrapper)
2. [Wallet Integration - CDP Server](guides/wallet-integration-guide.md#3-cdp-server-wallets)
3. [Solana Implementation](guides/solana-implementation.md)

### For API Providers
1. [Integration Patterns - Server](guides/integration-patterns.md#pattern-2-protected-api-middleware)
2. [Security Best Practices](guides/security-best-practices.md)
3. [Testing and Monitoring](guides/testing-and-monitoring.md)

### For Enterprise Developers
1. [Crossmint Enterprise Guide](tools/crossmint-enterprise-guide.md)
2. [Facilitator Comparison](reference/facilitator-comparison.md)
3. [Security Best Practices](guides/security-best-practices.md)

### For MCP Server Developers
1. [Integration Patterns - MCP](guides/integration-patterns.md#pattern-4-mcp-server-monetization)
2. [Corbits Tool Guide](tools/corbits-faremeter-guide.md)
3. [Testing and Monitoring](guides/testing-and-monitoring.md)

---

## Comparison Tables

### Tools Comparison

| Tool | Type | License | Best For | Difficulty |
|------|------|---------|----------|------------|
| [Corbits](tools/corbits-faremeter-guide.md) | Framework | Open | Full control | Medium ⭐⭐ |
| [PayAI](tools/payai-network-guide.md) | Facilitator | Proprietary | Multi-chain | Easy ⭐ |
| [x402scan](tools/x402scan-explorer-guide.md) | Explorer | Open | Monitoring | Easy ⭐ |
| [Crossmint](tools/crossmint-enterprise-guide.md) | Platform | Proprietary | Enterprise | Medium ⭐⭐ |

**Detailed Comparison:** [SDK Comparison Reference](reference/sdk-comparison.md)

### Blockchain Comparison

| Network | Speed | Cost | x402 Status |
|---------|-------|------|-------------|
| Solana | <1s | <$0.0001 | ✅ Primary |
| Base | ~2s | ~$0.01 | ✅ Primary |
| Polygon | ~3s | ~$0.01 | ✅ Secondary |
| Ethereum | ~15s | $1-$10 | ⚠️ Limited |

**Detailed Comparison:** [Blockchain Networks Reference](reference/blockchain-networks.md)

### Facilitator Comparison

| Facilitator | Market Share | Networks | Fees | Best For |
|-------------|--------------|----------|------|----------|
| Coinbase CDP | 77-80% | 4 | User pays | Base/Enterprise |
| PayAI | 14% | 7 | Covered | Solana/Multi-chain |
| Self-Hosted | <10% | Custom | DIY | Full control |

**Detailed Comparison:** [Facilitator Comparison Reference](reference/facilitator-comparison.md)

---

## Installation Quick Reference

```bash
# Core x402
npm install @faremeter/fetch @faremeter/middleware

# Solana
npm install @solana/web3.js @solana/spl-token @solana/wallet-adapter-phantom

# Base/EVM
npm install viem ethers wagmi

# Wallets
npm install @coinbase/cdp-sdk @crossmint/client-sdk

# MCP
npm install x402-mcp @modelcontextprotocol/sdk
```

---

## Testing Resources

**Free Testing:**
- [Echo Merchant](tools/payai-network-guide.md#testing-with-echo-merchant) - Real transactions with full refunds
- [x402scan](tools/x402scan-explorer-guide.md) - Transaction verification

**Testnets:**
- Solana Devnet: https://api.devnet.solana.com
- Base Sepolia: https://sepolia.base.org

**Faucets:**
- Solana: https://faucet.solana.com
- Base: https://www.coinbase.com/faucets

---

## Support & Community

**Documentation:**
- x402 Protocol: https://docs.x402.org
- Corbits: https://docs.corbits.dev
- PayAI: https://docs.payai.network
- Coinbase CDP: https://docs.cdp.coinbase.com

**Community:**
- Corbits Discord: Check corbits.dev
- PayAI Discord: Check payai.network
- Solana Discord: https://discord.gg/solana

**GitHub:**
- [Code Repositories Reference](reference/code-repositories.md)

---

## Documentation Improvements

This restructured documentation provides:

- ✅ **70%+ token reduction** per file (easier AI context loading)
- ✅ **<10 second navigation** to any specific info
- ✅ **Modular structure** for team collaboration
- ✅ **Clear decision trees** for fast decisions
- ✅ **Scenario-based guidance** for common use cases
- ✅ **Comprehensive cross-links** between docs

**Previous Structure:**
- 2 files, 2,655 lines, 60KB

**New Structure:**
- 14 focused files, avg 200 lines each, 5-10KB per file

---

## Next Steps

1. **Choose your scenario** from [Common Scenarios](#common-scenarios)
2. **Read the relevant tool guide** from [Tools](#-tools-platform-specific-guides)
3. **Follow the integration pattern** from [Guides](#-guides-how-to-documentation)
4. **Test with Echo Merchant** (free refunds)
5. **Monitor on x402scan**
6. **Deploy to production**

**Questions?** Check the [Support & Community](#support--community) section.

---

**Pro Tip:** Bookmark this page - it's your single source of truth for x402 development!
