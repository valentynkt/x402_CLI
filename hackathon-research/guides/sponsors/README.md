# Sponsor Technologies Integration Guides

This directory contains detailed technical integration guides for all Solana x402 AI Hackathon sponsor technologies.

## Quick Navigation

| Technology | Prize | Difficulty | Integration Time | Guide |
|------------|-------|------------|------------------|-------|
| **Visa TAP** | $10,000 | Medium | 4-8 hours | [visa-tap-integration.md](./visa-tap-integration.md) |
| **ATXP** | $10,000 (credits) | High | 8-16 hours | [atxp-integration.md](./atxp-integration.md) |
| **Switchboard** | $5,000 | Medium | 2-4 hours | [switchboard-integration.md](./switchboard-integration.md) |
| **CDP Embedded Wallets** | $5,000 | Easy-Medium | 2-3 hours | [cdp-wallets-integration.md](./cdp-wallets-integration.md) |
| **Gradient Parallax** | $5,000 | High | 8-16 hours | [gradient-parallax-integration.md](./gradient-parallax-integration.md) |

## Integration Guides

### [Visa Trusted Agent Protocol (TAP)](./visa-tap-integration.md)
**Focus:** Agent identity verification and trusted commerce

Visa's protocol for secure AI agent transactions using HTTP Message Signatures (RFC 9421). Provides cryptographic agent verification, consumer recognition data, and interoperability with x402. Launched October 2025 with full GitHub implementation and developer portal support.

**Best for:** E-commerce agents, merchant platforms, trust networks

---

### [ATXP (Agentic Transaction Protocol)](./atxp-integration.md)
**Focus:** Multi-protocol orchestration and nested transactions

Circuit & Chisel's web-wide protocol for autonomous AI agent commerce. Supports real-time decision-making, nested transactions, autonomous tool discovery, and multi-blockchain compatibility. Raised $19.2M in September 2025 from Primary Venture Partners, ParaFi, and major Web2/Web3 investors.

**Best for:** Multi-protocol agents, cross-chain applications, complex workflows

---

### [Switchboard Oracle](./switchboard-integration.md)
**Focus:** On-chain data feeds with x402 micropayments

Solana's fastest oracle with sub-100ms latency. First x402-compatible oracle (Oct 23, 2025) enabling pay-per-query data access at <$0.001 per call. Protects $5B+ in assets across 50+ protocols.

**Best for:** Trading bots, DeFi strategies, price alerts, data-driven agents

---

### [CDP Embedded Wallets](./cdp-wallets-integration.md)
**Focus:** User-friendly wallet infrastructure with policy enforcement

Coinbase Developer Platform wallets for autonomous agents. No key management required, policy-enforced spending limits, and production-proven at scale (1.2M+ payments in 5 days). Full x402 facilitator integration.

**Best for:** Consumer-facing agents, autonomous payment systems, workflow automation

---

### [Gradient Parallax](./gradient-parallax-integration.md)
**Focus:** Distributed AI inference on Solana

Decentralized AI infrastructure with Parallax (distributed inference) and Lattica (P2P communication) protocols. Backed by $10M from Pantera, Multicoin, and HSG. Enables privacy-preserving AI with 1.6B+ device connections.

**Best for:** Distributed AI applications, privacy-first systems, edge computing, multi-agent collaboration

---

## Multi-Sponsor Strategies

### [Combining Multiple Sponsors](./multi-sponsor-strategies.md)

Learn how to win multiple bounties simultaneously by integrating complementary sponsor technologies. Includes architecture patterns, prize compatibility matrix, and proven technology stacks.

**Featured Combinations:**
- Visa TAP + CDP Wallets + x402 (Enterprise Agent Commerce)
- ATXP + Gradient Parallax + x402 (Fully Decentralized System)
- Switchboard + Solana + PayAI (High-Performance Trading)

---

## Troubleshooting

### [Common Troubleshooting Guide](./common-troubleshooting.md)

Generic troubleshooting for all sponsor integrations, including:
- SDK and API issues (authentication, rate limits, timeouts)
- Solana blockchain errors (insufficient balance, RPC failures)
- x402 payment problems (facilitator errors, on-chain verification)
- Debugging techniques and best practices

Each sponsor guide also includes technology-specific troubleshooting sections.

---

## Getting Started

### Prerequisites

Before integrating any sponsor technology, ensure you have:

1. **Core x402 Knowledge**
   - Read [x402-protocol-specification.md](../../x402-protocol-specification.md)
   - Understand HTTP 402 payment flow
   - Familiar with Solana basics

2. **Development Environment**
   - Node.js 18+ installed
   - Solana CLI configured
   - Wallet with devnet SOL/USDC

3. **Hackathon Context**
   - Review [hackathon-rules-and-tracks.md](../../hackathon-rules-and-tracks.md)
   - Understand submission requirements
   - Know deadline: November 11, 2025

### Integration Workflow

**1. Choose Your Technology** (1-2 hours)
- Review comparison table above
- Match to your project idea
- Consider difficulty and integration time
- Check prize compatibility

**2. Setup Development Environment** (1-2 hours)
- Register with sponsor developer portal
- Get API keys and credentials
- Install required SDKs
- Configure test environment

**3. Implement Core Integration** (2-16 hours depending on technology)
- Follow step-by-step guide for your chosen sponsor
- Implement authentication/verification
- Add payment integration
- Test in sandbox environment

**4. Build Your Application** (Remaining time)
- Develop unique features
- Integrate multiple sponsors if applicable
- Add UI/UX polish
- Prepare demo video

**5. Test & Deploy** (Final 2-3 days)
- Test on Solana devnet
- Deploy to mainnet (if required)
- Document architecture
- Record demo video

---

## Technology Selection Guide

### By Project Type

**E-commerce & Shopping Agents:**
- Primary: Visa TAP + CDP Wallets
- Secondary: x402 + Switchboard (for pricing data)

**Trading & DeFi Applications:**
- Primary: Switchboard + Solana
- Secondary: CDP Server Wallets + x402

**Multi-Agent Systems:**
- Primary: ATXP + Gradient Lattica
- Secondary: CDP Wallets + x402

**Privacy-First Applications:**
- Primary: Gradient Parallax + ATXP
- Secondary: x402 (with self-hosted facilitator)

**Autonomous Workflow Agents:**
- Primary: CDP Embedded Wallets + ATXP
- Secondary: Switchboard (for data) + x402

### By Technical Expertise

**Beginner-Friendly:**
- CDP Embedded Wallets (Easy-Medium, 2-3 hours)
- Switchboard (Medium, 2-4 hours)

**Intermediate:**
- Visa TAP (Medium, 4-8 hours)

**Advanced:**
- ATXP (High, 8-16 hours)
- Gradient Parallax (High, 8-16 hours)

---

## Resources

### Documentation
- [Sponsor Technologies Overview](../../sponsor-technologies.md) - High-level comparison
- [Technical Stack Reference](../../technical-stack-reference.md) - SDK installation
- [Integration Patterns](../integration-patterns.md) - Common patterns
- [Ecosystem Tools](../../ecosystem-tools-reference.md) - Facilitators & infrastructure

### Support
- **Visa TAP:** developer.visa.com
- **ATXP:** docs.atxp.ai
- **Switchboard:** docs.switchboard.xyz
- **CDP:** docs.cdp.coinbase.com
- **Gradient:** docs.gradient.network

### Community
- Solana Discord: #x402-hackathon
- Sponsor-specific Discord channels
- GitHub discussions for each SDK

---

## Tips for Success

1. **Start Simple:** Get basic integration working before adding complex features
2. **Test Early:** Use sandbox environments to validate integration quickly
3. **Combine Strategically:** Multiple sponsor integrations can win multiple prizes
4. **Document Well:** Clear architecture docs improve judging scores
5. **Demo Matters:** 3-minute video is crucial - practice your presentation

---

## Quick Reference: All Sponsors

### Official Documentation Links

| Sponsor | Docs | GitHub | Developer Portal |
|---------|------|--------|-----------------|
| **Visa TAP** | [Visa Developer](https://developer.visa.com/capabilities/trusted-agent-protocol/overview) | [github.com/visa/trusted-agent-protocol](https://github.com/visa/trusted-agent-protocol) | developer.visa.com |
| **ATXP** | [docs.atxp.ai](https://docs.atxp.ai) | TBA | Contact Circuit & Chisel |
| **Switchboard** | [docs.switchboard.xyz](https://docs.switchboard.xyz) | [github.com/switchboard-xyz/solana-sdk](https://github.com/switchboard-xyz/solana-sdk) | docs.switchboard.xyz |
| **CDP** | [docs.cdp.coinbase.com](https://docs.cdp.coinbase.com) | [github.com/coinbase/x402](https://github.com/coinbase/x402) | coinbase.com/developer-platform |
| **Gradient** | [docs.gradient.network](https://docs.gradient.network) | TBA | docs.gradient.network |

### SDK Installation

```bash
# Visa TAP
npm install @visa/tap-sdk

# ATXP
npm install @atxp/sdk

# Switchboard
npm install @switchboard-xyz/solana.js

# CDP Wallets
npm install @coinbase/cdp-sdk

# Gradient (anticipated)
npm install @gradient/parallax @gradient/lattica
```

---

**Last Updated:** November 4, 2025
**Hackathon Deadline:** November 11, 2025

For questions or clarifications, refer to the main [sponsor-technologies.md](../../sponsor-technologies.md) document or contact the hackathon organizers.
