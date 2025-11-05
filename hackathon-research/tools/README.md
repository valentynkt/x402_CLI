# x402 Tools & SDKs Directory

Comprehensive index of all x402 protocol tools, SDKs, and implementations available for the Solana x402 AI Hackathon.

## Quick Reference Table

| Tool | Type | Solana | Best For | Difficulty | Guide |
|------|------|--------|----------|------------|-------|
| **[Corbits/Faremeter](#corbits--faremeter)** | SDK | ✅ Primary | Open-source, self-hosted | Medium | [Guide](./corbits-faremeter-guide.md) |
| **[PayAI Network](#payai-network)** | Facilitator | ✅ Primary | Quick start, 7-chain | Easy | [Guide](./payai-network-guide.md) |
| **[Coinbase CDP](#coinbase-cdp-sdk)** | SDK | ✅ Yes | Enterprise, official | Easy | [Guide](../guides/sponsors/cdp-wallets-integration.md) |
| **[Crossmint](#crossmint)** | Platform | ✅ Yes | Enterprise, 15+ chains | Medium | [Guide](./crossmint-enterprise-guide.md) |
| **[MCPay.tech](#mcpaytech)** | SDK | ✅ Yes | MCP monetization | Easy | [Guide](./mcpay-tech-guide.md) |
| **[x402-MCP](#x402-mcp)** | SDK | ✅ Agnostic | MCP protocol integration | Medium | [Guide](./x402-mcp-guide.md) |
| **[ACK Protocol](#ack-protocol)** | SDK | ✅ Primary | Identity + receipts | Medium | [Guide](./ack-protocol-guide.md) |
| **[Google A2A x402](#google-a2a-x402)** | SDK | ⏳ In Progress | Agent-to-agent commerce | Med-Adv | [Guide](./google-a2a-x402-guide.md) |
| **[Nexus (Thirdweb)](#nexus-thirdweb)** | Proxy | ⏳ Soon | Fastest integration, 26+ chains | Easy | [Guide](./nexus-thirdweb-guide.md) |
| **[Native Example](#native-example)** | Example | ✅ Primary | Learning, customization | Advanced | [Guide](./native-example-guide.md) |
| **[x402scan](#x402scan)** | Explorer | N/A | Transaction tracking | N/A | [Guide](./x402scan-explorer-guide.md) |

## Category Index

### By Use Case

**MCP (Model Context Protocol) Integration:**
- [MCPay.tech](#mcpaytech) - MCP server monetization with per-tool pricing
- [x402-MCP](#x402-mcp) - Multiple MCP integration implementations (TS, Go, Vercel)

**Agent-to-Agent Commerce:**
- [Google A2A x402](#google-a2a-x402) - Agent marketplace with multi-language support
- [ACK Protocol](#ack-protocol) - Verifiable identity and payment receipts

**General API Payments:**
- [Corbits/Faremeter](#corbits--faremeter) - Open-source, self-hostable framework
- [PayAI Network](#payai-network) - Hosted facilitator, 7-chain support
- [Nexus (Thirdweb)](#nexus-thirdweb) - Global edge proxy, 26+ chains

**Enterprise Solutions:**
- [Coinbase CDP](#coinbase-cdp-sdk) - Official Coinbase enterprise SDK
- [Crossmint](#crossmint) - Multi-protocol (x402 + Visa + Mastercard), 15+ chains

**Learning & Customization:**
- [Native Example](#native-example) - Bare-metal implementation for education

**Monitoring & Analytics:**
- [x402scan](#x402scan) - Explorer for tracking x402 transactions

### By Chain Support

**Solana Primary:**
- Corbits/Faremeter ✅
- PayAI Network ✅
- MCPay.tech ✅
- ACK Protocol ✅ (Solana only)
- Native Example ✅

**Multi-Chain (10+ chains):**
- Crossmint ✅ (15+ chains)
- Nexus (Thirdweb) ✅ (26+ chains)

**Blockchain Agnostic:**
- x402-MCP ✅ (adapter-based)

**Solana Coming Soon:**
- Google A2A x402 ⏳
- Nexus (Thirdweb) ⏳ (Target: Oct 30, 2025)

### By License Type

**Open Source:**
- Corbits/Faremeter (LGPL-3.0)
- MCPay.tech
- x402-MCP
- ACK Protocol
- Google A2A x402
- Native Example

**Proprietary/Commercial:**
- Coinbase CDP SDK
- PayAI Network
- Crossmint
- Nexus (Thirdweb)

---

## Corbits / Faremeter

**Type:** Open-source SDK framework
**Language:** TypeScript
**Chains:** Solana, Base, Polygon, + custom via plugins
**License:** LGPL-3.0

### Overview
The only fully open-source x402 framework with complete code control, self-hostable facilitator, and flexible integration options (wrappers, middleware, or proxies).

### Key Features
- 🎯 Three integration methods (client, server, proxy)
- 🔓 No vendor lock-in
- 🔌 Plugin architecture for any blockchain
- 💰 Access to premium APIs (Helius, DFlow, Triton, etc.)

### When to Use
- ✅ Want full control and customization
- ✅ Need self-hosting capability
- ✅ Prefer open-source solutions
- ✅ Solana-first project

[→ Full Corbits/Faremeter Guide](./corbits-faremeter-guide.md)

---

## PayAI Network

**Type:** Payment facilitator
**Language:** JavaScript SDK
**Chains:** Solana, Base, Polygon, Avalanche, Sei, IoTeX, Peaq (7 total)
**License:** Proprietary

### Overview
Multi-chain x402 facilitator with network fee coverage, free tier, and fastest integration for quick prototyping.

### Key Features
- 🌐 7-chain support (most of any facilitator)
- 💸 Network fees covered
- 🎁 Free tier available
- ⚡ Echo Merchant for testing
- 🤖 ElizaOS integration

### When to Use
- ✅ Need quick prototyping
- ✅ Want multi-chain support
- ✅ Prefer hosted solution
- ✅ Need fee coverage

[→ Full PayAI Network Guide](./payai-network-guide.md)

---

## Coinbase CDP SDK

**Type:** Enterprise SDK
**Language:** TypeScript, Python
**Chains:** Solana, Base, Ethereum, Arbitrum
**License:** Proprietary

### Overview
Official Coinbase x402 implementation with embedded wallets, enterprise support, and 77-80% market share.

### Key Features
- 🏢 Enterprise-grade support
- 👛 Embedded wallet infrastructure
- 📊 Market leader (77-80% share)
- 🔐 Coinbase security standards

### When to Use
- ✅ Need official Coinbase support
- ✅ Want embedded wallets
- ✅ Building enterprise application
- ✅ Base/Ethereum focus

[→ CDP Integration Guide](../guides/sponsors/cdp-wallets-integration.md)

---

## Crossmint

**Type:** Enterprise platform
**Language:** TypeScript
**Chains:** 15+ including Solana, Base, Ethereum, Polygon, etc.
**License:** Proprietary

### Overview
Multi-protocol enterprise solution supporting x402 + traditional payments (Visa, Mastercard) with deep commerce integrations (Amazon, Shopify).

### Key Features
- 💳 Multi-protocol (crypto + traditional)
- 🛒 E-commerce integrations
- 🌍 15+ blockchain support
- 🏢 Enterprise compliance
- 👛 Custodial and non-custodial wallets

### When to Use
- ✅ Need traditional + crypto payments
- ✅ E-commerce integration required
- ✅ Enterprise compliance needs
- ✅ Multi-chain requirements

[→ Full Crossmint Guide](./crossmint-enterprise-guide.md)

---

## MCPay.tech

**Type:** MCP SDK
**Language:** JavaScript
**Chains:** EVM + Solana
**License:** Open Source

### Overview
Payment infrastructure for Model Context Protocol servers, enabling micropayments for AI agent tool access with per-tool pricing.

### Key Features
- 🔧 MCP-native design
- 💰 Per-tool pricing (min $0.001)
- 🏆 Hackathon proven (1st place Coinbase Agents in Action)
- 🔄 Non-intrusive middleware
- 📚 Registry support (Smithery, KlavisAI, Composio)

### When to Use
- ✅ Building MCP servers
- ✅ Monetizing AI agent tools
- ✅ Need per-tool granular pricing
- ✅ Want MCP-specific features

[→ Full MCPay.tech Guide](./mcpay-tech-guide.md)

---

## x402-MCP

**Type:** MCP integration library (multiple implementations)
**Language:** TypeScript, Go
**Chains:** Blockchain agnostic (adapter-based)
**License:** Open Source (varies by implementation)

### Overview
Multiple independent implementations bridging Model Context Protocol with x402 payments, including official Coinbase example, Vercel AI SDK integration, and MCP-Go support.

### Key Features
- 🔄 Multiple implementations (ethanniser, Coinbase, Vercel, MCP-Go)
- 📦 Vercel AI SDK integration
- 🔧 Both client and server support
- 🌐 Multi-language (TypeScript, Go)

### When to Use
- ✅ Multiple implementation options needed
- ✅ Vercel AI SDK user
- ✅ Go language preference
- ✅ MCP protocol focus

[→ Full x402-MCP Guide](./x402-mcp-guide.md)

---

## ACK Protocol

**Type:** Identity-enhanced payment SDK
**Language:** TypeScript
**Chains:** Solana
**License:** Open Source

### Overview
Extends x402 with verifiable agent identity (W3C DIDs) and cryptographic payment receipts (Verifiable Credentials) for audit trails and reputation systems.

### Key Features
- 🆔 W3C DID-based agent identity
- 🧾 Verifiable Credential receipts
- 🔐 Cryptographic proofs
- 📊 Audit trail support
- 🌐 Live demo available (solana-paywal.vercel.app)

### When to Use
- ✅ Need agent identity verification
- ✅ Require payment receipts
- ✅ Building reputation systems
- ✅ Compliance/audit requirements
- ✅ Solana-first project

[→ Full ACK Protocol Guide](./ack-protocol-guide.md)

---

## Google A2A x402

**Type:** Agent-to-agent payment extension
**Language:** TypeScript, Python, Go
**Chains:** EVM chains (Solana in development)
**License:** Open Source

### Overview
Google-backed agent-to-agent payment extension, developed in collaboration with Coinbase, Ethereum Foundation, and MetaMask for decentralized agent commerce.

### Key Features
- 🤝 Google collaboration (Coinbase, Ethereum, MetaMask)
- 🌐 Multi-language SDKs (TS, Python, Go)
- 🤖 Agent-native protocol
- 📋 Production specification (v0.1 released)
- 🔄 Three-message payment flow

### When to Use
- ✅ Building agent marketplaces
- ✅ Want Google ecosystem integration
- ✅ Need multi-language support
- ✅ Enterprise credibility important
- ⚠️ Solana support in progress

[→ Full Google A2A x402 Guide](./google-a2a-x402-guide.md)

---

## Nexus (Thirdweb)

**Type:** Payment-gated edge proxy
**Language:** TypeScript
**Chains:** 26+ (Solana coming Oct 30, 2025)
**License:** Commercial (Free tier available)

### Overview
Global edge proxy by Thirdweb enabling x402 payments for any API without code changes, with broadest chain support (26+) and single-line integration.

### Key Features
- 🌍 Globally distributed edge network
- 🔗 26+ blockchain support (most in ecosystem)
- ⚡ Fastest integration (wrapFetchWithPayment)
- 👛 Embedded wallet support
- 🆓 Free tier available

### When to Use
- ✅ Want absolute fastest integration
- ✅ Need broadest chain support
- ✅ Prefer hosted proxy solution
- ✅ Want global edge performance
- ⚠️ Solana coming soon

[→ Full Nexus (Thirdweb) Guide](./nexus-thirdweb-guide.md)

---

## Native Example

**Type:** Reference implementation
**Language:** JavaScript (Express + Node)
**Chains:** Solana
**License:** Open Source

### Overview
Minimal x402 implementation without dependencies, demonstrating protocol internals using only Express and Solana Web3.js for learning and customization.

### Key Features
- 📚 Educational reference
- 🔧 Zero SDK dependencies
- 💡 Clear protocol demonstration
- ⚙️ Full customization capability
- 🎯 Production patterns shown

### When to Use
- ✅ Learning x402 protocol internals
- ✅ Need custom payment logic
- ✅ Want minimal dependencies
- ✅ Building educational material
- ❌ Not recommended for production (high risk)

[→ Full Native Example Guide](./native-example-guide.md)

---

## x402scan

**Type:** Blockchain explorer
**Chains:** All x402-enabled chains
**License:** N/A (Web service)

### Overview
Comprehensive explorer for x402/x420 ecosystem providing transaction tracking, merchant discovery, facilitator analytics, and network statistics.

### Key Features
- 📊 Real-time transaction tracking
- 🏪 Merchant discovery and verification
- 📈 Facilitator comparison
- 🔍 Resource registration tracking
- 📉 Ecosystem analytics

### When to Use
- ✅ Monitor payment transactions
- ✅ Discover active merchants
- ✅ Compare facilitators
- ✅ Track ecosystem growth
- ✅ Debug integration issues

[→ Full x402scan Guide](./x402scan-explorer-guide.md)

---

## Comparison Summary

### Best for Solana Hackathon

| Ranking | Tool | Reason |
|---------|------|--------|
| 🥇 | **Corbits/Faremeter** | Open-source, Solana-first, hackathon-friendly |
| 🥈 | **PayAI Network** | Fastest integration, 7-chain support |
| 🥉 | **MCPay.tech** | Unique MCP angle, hackathon winner |

**Special Categories:**
- **Most Chains:** Nexus (26+)
- **Best Identity:** ACK Protocol
- **Enterprise:** Crossmint or CDP SDK
- **Learning:** Native Example

### Integration Time Estimates

| Tool | Basic Setup | Production Ready |
|------|-------------|------------------|
| Nexus | 15-30 min | 2-4 hours |
| PayAI | 30-60 min | 2-4 hours |
| MCPay.tech | 30-60 min | 2-4 hours |
| CDP SDK | 1-2 hours | 4-8 hours |
| Faremeter | 1-2 hours | 6-12 hours |
| x402-MCP | 2-4 hours | 8-16 hours |
| ACK Protocol | 2-4 hours | 6-12 hours |
| A2A x402 | 2-4 hours | 8-16 hours |
| Crossmint | 2-4 hours | 8-16 hours |
| Native Example | 4-8 hours | 24+ hours |

### Prize Track Alignment

**Best x402 Agent Application ($10,000):**
- ✅✅ ACK Protocol (unique identity angle)
- ✅✅ Google A2A x402 (Google credibility)
- ✅✅ MCPay.tech (MCP-specific)
- ✅ All others

**Best Corbits Project ($5,000):**
- ✅✅ Corbits/Faremeter only

**Best Agent Money Protocol ($5,000):**
- ✅✅ MCPay.tech (tool monetization)
- ✅✅ Google A2A x402 (agent-native)
- ✅✅ ACK Protocol (receipts/identity)

**CDP Embedded Wallets ($5,000):**
- ✅✅ Coinbase CDP SDK
- ⚠️ Others (may conflict with Thirdweb)

---

## Decision Guide

### Choose based on your needs:

**🎯 Need fastest integration?**
→ Nexus (1 line) or PayAI (3 lines)

**🔓 Want open-source?**
→ Corbits/Faremeter (production) or Native (learning)

**🤖 Building MCP tools?**
→ MCPay.tech (simple) or x402-MCP (flexible)

**🆔 Need agent identity?**
→ ACK Protocol (Solana) or Google A2A (multi-chain)

**🌐 Need 15+ chains?**
→ Nexus (26+) or Crossmint (15+)

**🏢 Need enterprise support?**
→ CDP SDK or Crossmint

**📚 Learning x402?**
→ Native Example

---

**Navigation:**
- [← Back to Main README](../README.md)
- [SDK Comparison Reference](../reference/sdk-comparison.md)
- [Integration Patterns Guide](../guides/integration-patterns.md)
- [x402 Protocol Specification](../x402-protocol-specification.md)
