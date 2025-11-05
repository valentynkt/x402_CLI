# ACK (Agent Commerce Kit) Protocol Guide

**Quick Links:** [GitHub](https://github.com/agentcommercekit/ack) | [Live Demo](https://solana-paywal.vercel.app/) | [Example Code](https://github.com/Woody4618/solana-paywal-x402)
**License:** Open Source | **Integration Difficulty:** Medium

## Overview

ACK (Agent Commerce Kit) extends the x402 protocol by adding **critical layers for the agent economy**: verifiable agent identity using W3C DIDs/VCs (Decentralized Identifiers / Verifiable Credentials) and cryptographically secure receipts. While x402 handles payments, ACK ensures **who paid** and **what was purchased** with cryptographic proof.

## Key Features

- ✅ **ACK-ID:** Verifiable agent identity using W3C DID/VC standards
- ✅ **ACK-Pay:** Cryptographically secure receipts as Verifiable Credentials
- ✅ **x402 Extension:** Builds on x402 with identity and receipt layers
- ✅ **Solana Native:** Built specifically for Solana blockchain
- ✅ **Multi-Use Case:** Image paywalling, jukebox, animatable APIs
- ✅ **Production Ready:** Live working examples available

## Technical Specifications

### Technology Stack
- **Base Protocol:** x402 (HTTP 402 Payment Required)
- **Identity Layer:** W3C Decentralized Identifiers (DIDs)
- **Receipt Layer:** W3C Verifiable Credentials (VCs)
- **Blockchain:** Solana
- **Token:** USDC (SPL)
- **Status:** In PR (x402 support not yet merged upstream)

### Repository Structure
```
ack/
├── ack-id/              # Verifiable agent identity
├── ack-pay/             # Payment receipts as VCs
├── x402-integration/    # x402 protocol extension
└── examples/            # Reference implementations
```

### Supported Networks

| Network | Support | Primary Token | Status |
|---------|---------|---------------|--------|
| **Solana** | Primary | USDC (SPL) | Production |
| **Others** | Planned | TBD | Roadmap |

### ACK Architecture Layers

1. **Base Layer (x402):** Payment required protocol
2. **Identity Layer (ACK-ID):** Verifiable agent identities
3. **Receipt Layer (ACK-Pay):** Cryptographic payment receipts
4. **Commerce Layer:** Business logic and policies

## Key Differentiators from Pure x402

### Identity Verification (ACK-ID)

**Problem:** x402 doesn't verify **who** is making the payment
**Solution:** W3C DIDs for cryptographic agent identity

```typescript
import { createAgentDID } from 'ack-id';

// Create verifiable agent identity
const agentDID = await createAgentDID({
  name: "My AI Agent",
  capabilities: ["data_access", "compute"],
  publicKey: agentPublicKey
});

// Sign requests with DID
const signedRequest = await agentDID.sign(paymentRequest);
```

### Payment Receipts (ACK-Pay)

**Problem:** x402 doesn't provide verifiable proof of what was purchased
**Solution:** Verifiable Credentials as cryptographic receipts

```typescript
import { issueReceipt } from 'ack-pay';

// Issue verifiable receipt after payment
const receipt = await issueReceipt({
  buyer: agentDID,
  seller: merchantDID,
  amount: 0.05,
  resource: "premium_api_access",
  timestamp: Date.now(),
  transactionHash: solanaTxHash
});

// Receipt can be verified by any party
const isValid = await verifyReceipt(receipt);
```

## Integration Methods

### Server-Side Integration

**Use Case:** API providers requiring identity verification
**Effort:** Medium - requires DID/VC understanding

```typescript
import { ackMiddleware } from 'ack-pay';
import express from 'express';

const app = express();

app.use(ackMiddleware({
  requireIdentity: true,
  issueReceipts: true,
  merchantDID: process.env.MERCHANT_DID,
  solanaRPC: process.env.RPC_URL
}));

app.get('/api/premium-data', async (req, res) => {
  // req.agentDID contains verified agent identity
  // req.paymentReceipt contains VC receipt

  const data = await getPremiumData(req.agentDID);
  res.json({ data, receipt: req.paymentReceipt });
});
```

### Client-Side Integration

**Use Case:** AI agents with persistent identity
**Effort:** Medium - setup DID and wallet

```typescript
import { ACKClient } from 'ack-pay';

const client = new ACKClient({
  agentDID: myAgentDID,
  wallet: solanaWallet,
  maxAmount: 0.1
});

// Make payment with identity
const response = await client.payForResource(
  'https://api.example.com/premium',
  {
    receiptRequired: true
  }
);

// Save receipt for audit trail
await saveReceipt(response.receipt);
```

## Use Cases for Hackathon

### 1. Image Paywalling (Live Example)
**Demo:** https://solana-paywal.vercel.app/
**Scenario:** AI agent pays to access premium images
**Implementation:** x402 payment + ACK receipt
**Revenue Model:** $0.01 per image

### 2. Music Jukebox (Live Example)
**Demo:** https://solana-paywal.vercel.app/
**Scenario:** Pay-per-play music streaming
**Implementation:** ACK-ID for user identity + x402 payments
**Revenue Model:** $0.05 per song

### 3. Animatable API (Live Example)
**Demo:** https://solana-paywal.vercel.app/
**Scenario:** Generate custom animations on-demand
**Implementation:** Agent identity + payment receipts
**Revenue Model:** $0.10 per animation

### 4. Audit Trail Commerce
**Scenario:** Enterprise AI agents need payment receipts
**Implementation:** Full ACK stack with VCs
**Revenue Model:** $0.50+ per high-value transaction

## Live Working Examples

### Solana Paywal Demo
**URL:** https://solana-paywal.vercel.app/
**Features:**
- Image paywalling with x402
- Music jukebox with ACK-Pay
- Animatable API with ACK-ID
- Live on Solana devnet/mainnet

**Source Code:** https://github.com/Woody4618/solana-paywal-x402

### Key Demo Features
1. **Wallet Connection:** Phantom/Solflare integration
2. **Payment Flow:** Complete x402 + ACK workflow
3. **Receipt Display:** VC visualization
4. **Identity Verification:** DID authentication

## W3C Standards Compliance

### DIDs (Decentralized Identifiers)

ACK-ID implements W3C DID specification:
- **Format:** `did:ack:solana:{publicKey}`
- **Resolution:** On-chain DID documents
- **Verification:** Ed25519 signatures
- **Portability:** Cross-platform agent identity

### VCs (Verifiable Credentials)

ACK-Pay implements W3C VC specification:
- **Format:** JSON-LD credentials
- **Signatures:** Solana cryptographic proofs
- **Validation:** Standard VC verification
- **Expiration:** Configurable TTL

## Integration Difficulty Breakdown

### Easy ✅
- Well-documented live examples
- Clear demo applications
- Standard W3C specs (if familiar)
- Solana wallet integration similar to others

### Medium ⚠️
- DID/VC concepts learning curve
- W3C standards understanding required
- More complex than pure x402
- Identity management overhead

### Advanced 🔧
- Custom DID methods
- VC schema design
- Cross-platform identity resolution
- Enterprise audit requirements

## Unique Selling Points

1. **Only x402 + Identity Solution** - Fills critical gap in pure x402
2. **Standards-Based:** Uses W3C DIDs/VCs, not proprietary
3. **Audit Trail:** Cryptographic receipts for compliance
4. **Agent Reputation:** DID enables reputation systems
5. **Live Production Examples:** Working demos on mainnet
6. **Solana-First:** Optimized for Solana's speed and cost

## When to Choose ACK

**✅ Choose ACK if you:**
- Need verifiable agent identity
- Require payment receipts for audit/compliance
- Building agent reputation systems
- Need proof of purchase for access control
- Want standards-based identity (DIDs/VCs)
- Target enterprise AI agent use cases

**❌ Consider alternatives if you:**
- Don't need identity verification (use pure x402)
- Want simplest integration (use PayAI)
- Prefer mature, merged protocol (wait for PR merge)
- Don't need receipts/audit trail
- Need multi-chain support (Solana only currently)

## Documentation Quality: MEDIUM

**Available Resources:**
- **GitHub Repository** - Source code and README
- **Live Demo** - Working examples
- **Example Implementation** - Woody4618/solana-paywal-x402
- **W3C Specs** - Standard DID/VC documentation

**GitHub:**
- URL: https://github.com/agentcommercekit/ack
- Status: Active development, PR pending
- Examples: Multiple live demos
- Community: Growing

**Gaps:**
- Comprehensive API documentation
- Integration tutorials
- Best practices guide
- Advanced patterns

## Community & Support

**Channels:**
- **GitHub Issues** - Primary support
- **Demo Site** - Live examples
- **Solana Discord** - Community discussions

**Response Time:** Community-driven, variable

## Current Status & Roadmap

### Current Status (November 2025)
- ✅ Core protocol implemented
- ✅ Live working examples
- ✅ Solana mainnet ready
- ⏳ x402 PR pending merge
- ⏳ Documentation expansion

### Roadmap
- Upstream x402 PR merge
- Multi-chain support
- Enhanced documentation
- Enterprise features
- Developer SDKs

## Pricing & Business Model

### Framework Costs
- **ACK Protocol:** FREE (open source)
- **Transaction Fees:** Solana gas only (~$0.00025)
- **No Platform Fees:** Direct peer-to-peer
- **DID/VC Issuance:** FREE (on-chain storage only)

### Cost Breakdown

| Component | Cost | Frequency |
|-----------|------|-----------|
| DID Creation | ~$0.01 | Once per agent |
| Payment | Gas + amount | Per transaction |
| Receipt Issuance | ~$0.0005 | Per transaction |
| DID Resolution | FREE | Unlimited |
| VC Verification | FREE | Unlimited |

## Quick Start Checklist

- [ ] Review W3C DID/VC specifications
- [ ] Set up Solana wallet (Phantom/Solflare)
- [ ] Explore live demo at solana-paywal.vercel.app
- [ ] Clone example repository (Woody4618/solana-paywal-x402)
- [ ] Create agent DID using ACK-ID
- [ ] Test payment flow with ACK-Pay
- [ ] Implement receipt verification
- [ ] Test on Solana devnet first
- [ ] Deploy to mainnet when ready

## Code Example: Complete Flow

```typescript
import { createAgentDID, ACKClient } from 'ack-protocol';
import { Connection, Keypair } from '@solana/web3.js';

// 1. Create agent identity
const agentKeypair = Keypair.generate();
const agentDID = await createAgentDID({
  keypair: agentKeypair,
  name: "My Trading Agent",
  capabilities: ["market_data", "trade_execution"]
});

// 2. Initialize ACK client
const client = new ACKClient({
  agentDID,
  wallet: agentKeypair,
  rpcUrl: process.env.SOLANA_RPC_URL,
  maxAmount: 1.0
});

// 3. Make payment with identity
const response = await client.fetch(
  'https://api.example.com/premium-data',
  {
    method: 'GET',
    receiptRequired: true
  }
);

// 4. Verify receipt
if (response.receipt) {
  const isValid = await response.receipt.verify();
  console.log('Receipt valid:', isValid);

  // Store for audit trail
  await storeReceipt(response.receipt);
}

// 5. Use purchased resource
const data = await response.json();
console.log('Premium data:', data);
```

## Hackathon Tips

### Prize Track Alignment
- **Best x402 Agent Application** ✅✅ (Strong fit - unique identity angle)
- **Best Agent Money Protocol** ✅✅ (Perfect fit - extends protocol)
- **Switchboard Integration** ✅ (Solana oracles + identity)

### Competitive Advantages
1. **Unique Value Prop:** Only solution with identity + payments
2. **Standards-Based:** W3C compliance appeals to judges
3. **Live Proof:** Working examples demonstrate maturity
4. **Enterprise Appeal:** Audit trails for compliance use cases
5. **Reputation Systems:** Enables agent marketplace evolution

### Integration Time
- **Basic Setup:** 2-4 hours (including DID/VC learning)
- **Production Ready:** 6-12 hours
- **Advanced Features:** 16+ hours

### Demo Strategy
1. Show live demo first (solana-paywal.vercel.app)
2. Explain identity verification advantage
3. Demonstrate receipt validation
4. Highlight audit trail for enterprise
5. Show agent reputation potential

---

**Related Docs:**
- [x402 Protocol Specification](../x402-protocol-specification.md)
- [Solana Integration Guide](../guides/solana-integration.md)
- [SDK Comparison Reference](../reference/sdk-comparison.md)
- [Code Repositories](../reference/code-repositories.md)
