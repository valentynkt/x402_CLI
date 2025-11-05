# Sponsor Technologies - Overview

This document provides a high-level overview of all sponsor-specific technologies for the Solana x402 AI Hackathon. For detailed integration guides, see the [guides/sponsors/](./guides/sponsors/) directory.

---

## Quick Navigation

**Looking for detailed integration guides?** → [Integration Guides Directory](./guides/sponsors/README.md)

**Need multi-sponsor strategies?** → [Multi-Sponsor Integration Strategies](./guides/sponsors/multi-sponsor-strategies.md)

---

## Overview

| Technology | Sponsor | Prize | Difficulty | Integration Time | Guide |
|------------|---------|-------|------------|------------------|-------|
| **Visa TAP** | Visa | $10,000 | Medium | 4-8 hours | [visa-tap-integration.md](./guides/sponsors/visa-tap-integration.md) |
| **ATXP** | Circuit & Chisel | $10,000 (credits) | High | 8-16 hours | [atxp-integration.md](./guides/sponsors/atxp-integration.md) |
| **Switchboard** | Switchboard | $5,000 | Medium | 2-4 hours | [switchboard-integration.md](./guides/sponsors/switchboard-integration.md) |
| **CDP Embedded Wallets** | Coinbase | $5,000 | Easy-Medium | 2-3 hours | [cdp-wallets-integration.md](./guides/sponsors/cdp-wallets-integration.md) |
| **Gradient Parallax** | Gradient Network | $5,000 | High | 8-16 hours | [gradient-parallax-integration.md](./guides/sponsors/gradient-parallax-integration.md) |

**Total Prize Pool (Sponsor Technologies):** $40,000 + $10,000 credits

---

## Detailed Comparison

### Technical Specifications

| Feature | Visa TAP | ATXP | Switchboard | CDP Wallets | Gradient |
|---------|----------|------|-------------|-------------|----------|
| **Focus** | Agent verification | Multi-protocol orchestration | Oracle data feeds | Wallet infrastructure | Distributed AI |
| **Launch Date** | Oct 2025 | Sept 2025 | Established (x402: Oct 2025) | Established | 2025 |
| **Standard** | RFC 9421 | Proprietary | Oracle standard | Coinbase infrastructure | Distributed compute |
| **Blockchains** | Agnostic | Base, Solana, Polygon | Solana primary | Base, Solana, more | Solana |
| **Documentation** | Excellent | Limited (new) | Excellent | Excellent | Limited (new) |
| **Production Ready** | Yes | Early access | Yes | Yes | Development |
| **x402 Compatible** | Yes | Yes | Yes (first oracle!) | Yes | Potential |

### Integration Complexity

| Sponsor | Easy Aspects | Medium Aspects | Hard Aspects |
|---------|-------------|----------------|--------------|
| **Visa TAP** | SDK available, Clear docs | Signature implementation, Key management | Production deployment, Multi-merchant |
| **ATXP** | N/A | Service discovery | Multi-protocol, Nested transactions, Limited docs |
| **Switchboard** | Well-documented, Established | Oracle concepts, x402 integration | Query optimization, Cost management |
| **CDP Wallets** | Excellent docs, No key management | Policy configuration | Production security, Multi-wallet management |
| **Gradient** | N/A | N/A | Distributed systems, AI/ML expertise, New protocol |

### Market Position & Traction (2025)

| Sponsor | Key Metrics | Backing |
|---------|-------------|---------|
| **Visa TAP** | Launched Oct 2025, Cloudflare partnership, Industry standard | Visa Inc. + Cloudflare |
| **ATXP** | $19.2M seed (Sept 2025) | Primary VP, ParaFi, Stripe, Coinbase Ventures, Solana Ventures |
| **Switchboard** | $5B+ assets protected, 50+ protocols, First x402 oracle | Established Solana ecosystem |
| **CDP Wallets** | 1.2M+ payments in 5 days, 77-80% x402 market share | Coinbase Inc. |
| **Gradient** | $10M funding, 1.6B+ connections, 190+ regions | Pantera, Multicoin, Hack VC |

---

## Sponsor Technology Summaries

### 1. Visa Trusted Agent Protocol (TAP)

**What It Does:** Provides cryptographic verification of AI agent identity and purchase intent

**Key Features:**
- HTTP Message Signature standard (RFC 9421)
- Agent intent verification
- Consumer recognition data (loyalty, payment tokens)
- Device fingerprinting for fraud prevention
- Interoperable with x402 payment protocol

**Best For:**
- E-commerce agent platforms
- Merchant trust networks
- Agent commerce marketplaces
- Shopping assistants

**Official Resources:**
- Developer Portal: https://developer.visa.com/capabilities/trusted-agent-protocol
- GitHub: https://github.com/visa/trusted-agent-protocol
- Docs: https://developer.visa.com/capabilities/trusted-agent-protocol/docs

**Quick Start:**
```bash
npm install @visa/tap-sdk
```

**Detailed Guide:** [visa-tap-integration.md](./guides/sponsors/visa-tap-integration.md)

---

### 2. ATXP (Agentic Transaction Protocol)

**What It Does:** Universal protocol layer for AI agents to handle commerce lifecycle across multiple protocols and blockchains

**Key Features:**
- Real-time decision-making
- Nested transactions (atomic multi-step workflows)
- Autonomous tool discovery
- Multi-protocol support (x402, TAP, custom)
- Multi-chain (Base, Solana, Polygon)
- Policy-based spending controls

**Best For:**
- Cross-chain agent orchestration
- Multi-protocol payment routing
- Complex agent workflows
- Delegated spending platforms

**Official Resources:**
- Documentation: https://docs.atxp.ai
- Company: https://circuitandchisel.com
- Contact: developers@circuitandchisel.com

**Quick Start:**
```bash
npm install @atxp/sdk
```

**Detailed Guide:** [atxp-integration.md](./guides/sponsors/atxp-integration.md)

---

### 3. Switchboard Oracle

**What It Does:** Provides real-time oracle data feeds with x402 micropayment integration

**Key Features:**
- First and only x402-compatible oracle (Oct 23, 2025)
- Sub-100ms latency (Switchboard Surge)
- Pay-per-query pricing (<$0.001 per call)
- Price feeds, VRF, off-chain compute (TEE-based)
- Protects $5B+ in assets across 50+ protocols

**Best For:**
- Trading bots with real-time data
- DeFi strategy platforms
- Price alert systems
- Games needing verifiable randomness
- Data-driven AI agents

**Official Resources:**
- Documentation: https://docs.switchboard.xyz
- GitHub: https://github.com/switchboard-xyz/solana-sdk
- Surge Guide: https://docs.switchboard.xyz/switchboard-surge/surge

**Quick Start:**
```bash
npm install @switchboard-xyz/solana.js
```

**Detailed Guide:** [switchboard-integration.md](./guides/sponsors/switchboard-integration.md)

---

### 4. CDP Embedded Wallets

**What It Does:** Provides wallet infrastructure without key management for autonomous AI agents

**Key Features:**
- No private key management (Coinbase-hosted)
- Policy-enforced spending limits
- Embedded Wallets (consumer) + Server Wallets (agents)
- x402 facilitator integration
- Production-proven (1.2M+ payments in 5 days)
- Real-world example: Questflow (130k+ transactions)

**Best For:**
- Consumer-facing agent applications
- Autonomous workflow systems
- Subscription management agents
- Multi-agent platforms with budget controls

**Official Resources:**
- Documentation: https://docs.cdp.coinbase.com
- x402 Docs: https://docs.cdp.coinbase.com/x402/welcome
- Developer Platform: https://coinbase.com/developer-platform
- Facilitator: https://x402.org/facilitator

**Quick Start:**
```bash
npm install @coinbase/cdp-sdk
```

**Detailed Guide:** [cdp-wallets-integration.md](./guides/sponsors/cdp-wallets-integration.md)

---

### 5. Gradient Parallax

**What It Does:** Provides decentralized AI infrastructure with distributed inference (Parallax) and P2P communication (Lattica)

**Key Features:**
- Parallax: Distributed AI model inference across devices
- Lattica: Universal P2P data communication protocol
- Privacy-preserving computation (data stays local)
- 1.6B+ connections across 190+ regions
- Built on Solana blockchain
- $10M funding from Pantera, Multicoin, Hack VC

**Best For:**
- Distributed AI inference platforms
- Privacy-first AI applications
- Multi-agent collaboration systems
- Edge computing networks
- Federated learning

**Official Resources:**
- Documentation: https://docs.gradient.network
- Lattica Docs: https://docs.gradient.network/research/the-gradient-stack/lattica
- Parallax Docs: https://docs.gradient.network/the-open-intelligence-stack/parallax

**Quick Start:**
```bash
# Anticipated installation
npm install @gradient/parallax @gradient/lattica
```

**Detailed Guide:** [gradient-parallax-integration.md](./guides/sponsors/gradient-parallax-integration.md)

---

## Getting Started

### Step 1: Choose Your Sponsor Technologies

**Decision Framework:**

**For Beginners:**
- Start with **Switchboard + CDP Wallets** (easiest stack)
- Combine with x402 for payment integration
- Expected time: 4-6 hours
- Prize potential: $15-20k

**For Intermediate Developers:**
- Try **Visa TAP + CDP Wallets** (enterprise focus)
- Or **Switchboard + CDP + x402** (data + payments)
- Expected time: 8-12 hours
- Prize potential: $20-25k

**For Advanced Developers:**
- Tackle **ATXP + Gradient** (cutting edge)
- Or **ATXP + Visa TAP** (maximum complexity)
- Expected time: 16-24 hours
- Prize potential: $25-30k

**Selection Criteria:**
1. **Available Time:** 7-day hackathon (choose 2-3 sponsors max)
2. **Technical Skills:** Match complexity to your expertise
3. **Project Vision:** Which sponsors fit your idea?
4. **Prize Strategy:** See [Multi-Sponsor Strategies](./guides/sponsors/multi-sponsor-strategies.md)

### Step 2: Review Integration Guides

Visit the [Integration Guides Directory](./guides/sponsors/README.md) for:
- Step-by-step integration tutorials
- Complete code examples
- Use case recommendations
- Troubleshooting guides
- Best practices

### Step 3: Register with Developer Portals

**Required Registrations:**
- **Visa TAP:** https://developer.visa.com (API credentials)
- **ATXP:** Contact Circuit & Chisel for developer access
- **Switchboard:** https://docs.switchboard.xyz (no registration needed for devnet)
- **CDP Wallets:** https://coinbase.com/developer-platform (API keys)
- **Gradient:** https://docs.gradient.network (check for developer access)

**Timeline:** Allow 24-48 hours for API credential approval

### Step 4: Setup Development Environment

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Node.js dependencies
npm install @solana/web3.js @solana/spl-token

# Install sponsor SDKs (as needed)
npm install @visa/tap-sdk
npm install @atxp/sdk
npm install @switchboard-xyz/solana.js
npm install @coinbase/cdp-sdk
```

**Get Test Funds:**
```bash
# Get devnet SOL
solana airdrop 2

# Get devnet USDC from faucet
# Visit: https://faucet.circle.com (Base) or Solana faucet
```

### Step 5: Build & Test

**Development Workflow:**
1. **Day 1-2:** Setup infrastructure, test each SDK individually
2. **Day 3-5:** Build core integration, connect sponsors
3. **Day 6:** Polish, document, test thoroughly
4. **Day 7:** Deploy to mainnet (if required), record demo video, submit

**Testing Checklist:**
- [ ] Each sponsor SDK works independently
- [ ] x402 payment flow works end-to-end
- [ ] Sponsor integrations work together
- [ ] Error handling covers edge cases
- [ ] Works on devnet
- [ ] (Optional) Works on mainnet

---

## Integration Strategies

### Winning Multiple Bounties

Projects can win multiple sponsor prizes if they integrate technologies effectively. See [Multi-Sponsor Strategies](./guides/sponsors/multi-sponsor-strategies.md) for detailed combinations.

**Top Prize Combinations:**

| Stack | Technologies | Prize Potential | Time | Difficulty |
|-------|-------------|-----------------|------|------------|
| **Enterprise Commerce** | Visa TAP + CDP + x402 | $25,000 | 8-12h | Medium |
| **Decentralized System** | ATXP + Gradient + x402 | $25,000 | 16-20h | High |
| **High-Performance Trading** | Switchboard + CDP + x402 | $20,000 | 6-8h | Medium |
| **Cross-Chain Ecosystem** | ATXP + Visa TAP + x402 | $30,000 | 14-18h | High |

### Compatible Prize Tracks

Each sponsor technology is eligible for multiple prize tracks:

**Visa TAP:**
- Best use of Visa TAP ($10k)
- Best x402 Agent Application ($10k)
- Best Trustless Agent ($10k)

**ATXP:**
- Best Multi-Protocol Agent ($10k credits)
- Best x402 API Integration ($5k)
- Best Agent Money Protocol ($5k)

**Switchboard:**
- Best use of Switchboard ($5k)
- Best x402 Agent Application ($10k)

**CDP Wallets:**
- Best Usage of CDP Wallets ($5k)
- Best x402 Agent Application ($10k)
- Best Trustless Agent ($10k)

**Gradient:**
- Parallax Eco Track ($5k)
- Best x402 Agent Application ($10k)

---

## Hackathon Track Mapping

### Primary Sponsor Tracks

| Track | Sponsor | Prize | Requirement |
|-------|---------|-------|-------------|
| Best use of Visa TAP | Visa | $10,000 | Implement TAP signatures for agent verification |
| Best Multi-Protocol Agent | Circuit & Chisel | $10,000 credits | Use ATXP across multiple protocols/chains |
| Best use of Switchboard | Switchboard | $5,000 | Integrate x402-compatible oracle data |
| Best Usage of CDP Wallets | Coinbase | $5,000 | Use Embedded or Server Wallets with agents |
| Parallax Eco Track | Gradient | $5,000 | Use Parallax or Lattica protocols |

### Cross-Sponsor Tracks

These tracks accept multiple sponsor integrations:

- **Best x402 Agent Application** ($10,000)
- **Best Trustless Agent** ($10,000)
- **Best x402 API Integration** ($5,000)
- **Best Agent Money Protocol Hack** ($5,000)

**Strategy:** Target sponsor-specific track + cross-sponsor track for maximum prizes.

---

## Additional Resources

### Core Documentation

These documents provide foundation knowledge for all sponsor integrations:

- **x402 Protocol Specification:** [x402-protocol-specification.md](./x402-protocol-specification.md)
  - Complete x402 protocol details
  - HTTP 402 payment flow
  - On-chain verification process

- **Technical Stack Reference:** [technical-stack-reference.md](./technical-stack-reference.md)
  - SDK installation guides
  - Facilitator comparison
  - Wallet infrastructure
  - Code examples for common operations

- **Ecosystem Tools Reference:** [ecosystem-tools-reference.md](./ecosystem-tools-reference.md)
  - Corbits/Faremeter guide
  - PayAI Network details
  - x402scan explorer
  - Crossmint enterprise

- **Hackathon Rules & Tracks:** [hackathon-rules-and-tracks.md](./hackathon-rules-and-tracks.md)
  - All 13 prize tracks
  - Submission requirements
  - Judging criteria
  - Deadline: November 11, 2025

### Integration Guides

All detailed integration guides are in [guides/sponsors/](./guides/sponsors/):

- [README.md](./guides/sponsors/README.md) - Navigation hub
- [visa-tap-integration.md](./guides/sponsors/visa-tap-integration.md) - TAP implementation
- [atxp-integration.md](./guides/sponsors/atxp-integration.md) - ATXP multi-protocol
- [switchboard-integration.md](./guides/sponsors/switchboard-integration.md) - Oracle data
- [cdp-wallets-integration.md](./guides/sponsors/cdp-wallets-integration.md) - Wallet infrastructure
- [gradient-parallax-integration.md](./guides/sponsors/gradient-parallax-integration.md) - Distributed AI
- [multi-sponsor-strategies.md](./guides/sponsors/multi-sponsor-strategies.md) - Winning multiple bounties

### Implementation Guides

Supporting guides for common patterns:

- **Integration Patterns:** [guides/integration-patterns.md](./guides/integration-patterns.md)
- **Wallet Integration:** [guides/wallet-integration-guide.md](./guides/wallet-integration-guide.md)
- **Solana Implementation:** [guides/solana-implementation.md](./guides/solana-implementation.md)
- **Security Best Practices:** [guides/security-best-practices.md](./guides/security-best-practices.md)
- **Testing & Monitoring:** [guides/testing-and-monitoring.md](./guides/testing-and-monitoring.md)

---

## Quick Reference

### Installation Commands

```bash
# Sponsor SDKs
npm install @visa/tap-sdk                    # Visa TAP
npm install @atxp/sdk                         # ATXP
npm install @switchboard-xyz/solana.js        # Switchboard
npm install @coinbase/cdp-sdk                 # CDP Wallets
npm install @gradient/parallax @gradient/lattica  # Gradient (anticipated)

# Supporting libraries
npm install @faremeter/fetch                  # Corbits x402 client
npm install @solana/web3.js @solana/spl-token  # Solana
npm install @coinbase/coinbase-sdk            # Coinbase Base
```

### Official Links

| Sponsor | Documentation | GitHub | Developer Portal |
|---------|--------------|--------|------------------|
| **Visa TAP** | [developer.visa.com/tap](https://developer.visa.com/capabilities/trusted-agent-protocol) | [github.com/visa/trusted-agent-protocol](https://github.com/visa/trusted-agent-protocol) | developer.visa.com |
| **ATXP** | [docs.atxp.ai](https://docs.atxp.ai) | Contact for access | circuitandchisel.com |
| **Switchboard** | [docs.switchboard.xyz](https://docs.switchboard.xyz) | [github.com/switchboard-xyz/solana-sdk](https://github.com/switchboard-xyz/solana-sdk) | docs.switchboard.xyz |
| **CDP** | [docs.cdp.coinbase.com](https://docs.cdp.coinbase.com) | [github.com/coinbase/x402](https://github.com/coinbase/x402) | coinbase.com/developer-platform |
| **Gradient** | [docs.gradient.network](https://docs.gradient.network) | Check official site | docs.gradient.network |

### Support Channels

- **Solana Discord:** #x402-hackathon
- **Visa Developer Forums:** developer.visa.com/community
- **ATXP:** developers@circuitandchisel.com
- **Switchboard:** Discord via docs.switchboard.xyz
- **CDP:** Coinbase Developer Discord
- **Gradient:** Community channels via docs.gradient.network

---

## Success Criteria

### What Judges Look For

**Technical Excellence:**
- Correct implementation of sponsor protocols
- Clean, well-documented code
- Proper error handling
- Security best practices

**Integration Quality:**
- Genuine value from each sponsor integration
- Technologies work together seamlessly
- Not tokenistic or superficial

**Innovation:**
- Novel use cases
- Creative problem solving
- Real-world applicability

**Presentation:**
- Clear 3-minute demo video
- Comprehensive README
- Architecture diagrams
- Working deployment

### Submission Checklist

- [ ] Code deployed to GitHub (public repository)
- [ ] README with setup instructions
- [ ] Architecture diagram showing sponsor integrations
- [ ] 3-minute demo video
- [ ] Deployed to Solana devnet or mainnet (if required)
- [ ] All sponsor integrations working
- [ ] Submitted before November 11, 2025 deadline

---

## Timeline

**Hackathon Period:** October 28 - November 11, 2025

**Recommended Schedule:**
- **Day 1-2 (Oct 28-29):** Choose sponsors, register, setup environment
- **Day 3-5 (Oct 30-Nov 1):** Core development, integrate sponsors
- **Day 6-8 (Nov 2-4):** Polish, testing, bug fixes
- **Day 9-11 (Nov 5-7):** Documentation, demo video, final testing
- **Day 12-14 (Nov 8-10):** Deploy, submit, buffer for issues
- **Nov 11:** Final deadline

**Winners Announced:** November 17, 2025

---

## Frequently Asked Questions

**Q: Can I win multiple sponsor prizes?**
A: Yes! Projects that integrate multiple sponsors can win multiple bounties. See [Multi-Sponsor Strategies](./guides/sponsors/multi-sponsor-strategies.md).

**Q: Do I need to use all sponsors?**
A: No. Focus on 2-3 sponsors you can integrate deeply. Quality over quantity.

**Q: Where do I start?**
A: Read the [Integration Guides README](./guides/sponsors/README.md), choose 2-3 sponsors, and follow the step-by-step guides.

**Q: Can I use the same project for multiple tracks?**
A: Yes, if your project legitimately integrates the required technologies.

**Q: What if I can't get API credentials in time?**
A: Start with sponsors that don't require approval (Switchboard, CDP). Contact others immediately.

**Q: Is mainnet deployment required?**
A: Check hackathon rules. Devnet is typically acceptable, but mainnet may score higher.

---

**Last Updated:** November 4, 2025
**Version:** 2.0 (Refactored for clarity and conciseness)
**Hackathon Deadline:** November 11, 2025

---

**For detailed implementation guidance, visit:** [Integration Guides Directory](./guides/sponsors/README.md)
