# Visa Trusted Agent Protocol (TAP) Integration Guide

**Prize:** $10,000
**Sponsor:** Visa
**Launch Date:** October 14, 2025
**Difficulty:** Medium
**Est. Integration Time:** 4-8 hours

---

## Overview

Visa TAP is an ecosystem-led framework for AI commerce providing cryptographic verification of agent identity and intent. Developed with Cloudflare and aligned with Coinbase's x402 standard.

### Key Benefits

- **Agent Verification:** Cryptographic proof of agent identity
- **Consumer Recognition:** Secure transmission of loyalty and payment data
- **Fraud Prevention:** Device fingerprinting and risk scoring
- **Interoperability:** Works seamlessly with x402
- **Production Ready:** Launched October 2025 with enterprise backing

### Why Use TAP?

**$10,000 Prize:** One of the highest single-sponsor prizes. Well-documented framework compatible with x402, CDP Wallets, and Solana. Few teams will implement agent verification correctly—this is your competitive advantage.

---

## What is Visa TAP?

Visa TAP solves the critical challenge: **How do you trust an AI agent?** Traditional web commerce relies on human verification (CAPTCHAs, 2FA). AI agents need programmatic, cryptographic verification.

**TAP provides:**
1. Agent authentication (prove identity)
2. Intent verification (prove action is legitimate)
3. Consumer recognition (securely pass user preferences)
4. Payment authorization (prove permission to spend)

### TAP vs Other Protocols

| Feature | Visa TAP | x402 | ATXP |
|---------|----------|------|------|
| **Focus** | Agent trust & identity | Payment mechanics | Multi-protocol orchestration |
| **Layer** | Verification | Settlement | Coordination |
| **Standard** | RFC 9421 (HTTP Signatures) | HTTP 402 (Payment Required) | Proprietary |
| **Launch** | October 2025 | May 2025 | September 2025 |
| **Best Use** | Agent-to-merchant trust | Blockchain payments | Complex workflows |

**Complementary, not competitive:** TAP + x402 = Trusted payments

---

## Technical Architecture

### Core Standards

#### HTTP Message Signature (RFC 9421)

```http
Signature: keyId="agent-123",
           algorithm="rsa-sha256",
           created=1730630400,
           headers="(request-target) host date digest content-type",
           signature="Base64(digital_signature)"
```

**Components:**
- `keyId`: Identifies agent's public key
- `algorithm`: Cryptographic algorithm (rsa-sha256, ecdsa-p256-sha256)
- `created`: Unix timestamp of signature creation
- `headers`: Which HTTP headers are signed
- `signature`: Base64-encoded digital signature

#### TAP-Specific Headers

```http
Agent-ID: agent_vsa_123abc456def
Intent-Hash: SHA-256=hash_of_purchase_intent
Consumer-Token: encrypted_consumer_data
Device-Fingerprint: device_xyz_789
```

### Protocol Flow

TAP verifies agent identity, then x402 handles payment settlement:

```
1. Agent → Merchant: Signed request with TAP
2. Merchant: Verify TAP signature ✓
3. Merchant → Agent: 402 Payment Required (x402)
4. Agent: Create payment transaction (Solana)
5. Agent → Merchant: Retry with X-PAYMENT header
6. Merchant: Verify payment on-chain ✓
7. Merchant → Agent: 200 OK + Resource
```

**Total time:** ~2 seconds (including on-chain verification)

---

## Developer Resources

### Official Resources (2025)

**Visa Developer Portal:**
- URL: https://developer.visa.com/capabilities/trusted-agent-protocol/overview
- Documentation: Full technical specifications
- API Reference: Complete endpoint documentation
- Sandbox: Free testing environment

**GitHub Repository:**
- URL: https://github.com/visa/trusted-agent-protocol
- Sample Code: Reference implementation (TypeScript/JavaScript)
- Examples: Agent and merchant integration examples

**Related Documentation:**
- RFC 9421: https://www.rfc-editor.org/rfc/rfc9421.html
- x402 Integration: See [x402-protocol-specification.md](../../x402-protocol-specification.md)

### SDK Installation

```bash
# Official Visa TAP SDK
npm install @visa/tap-sdk

# Required dependencies
npm install crypto node-fetch
```

---

## Step-by-Step Integration

### Prerequisites

- Visa Developer Center account
- API credentials from developer portal
- Node.js 18+ development environment
- RSA or ECDSA key pair for your agent

### Step 1: Register with Visa

Register at [developer.visa.com](https://developer.visa.com). Receive API credentials within 24 hours:

```bash
VISA_API_KEY_NAME=vdp_api_key_123abc
VISA_API_KEY=secret_key_456def
```

### Step 2: Generate Agent Key Pair

Use `crypto.generateKeyPairSync('rsa', { modulusLength: 4096 })` to create keys:

```javascript
const crypto = require('crypto');
const fs = require('fs');

const { publicKey, privateKey } = crypto.generateKeyPairSync('rsa', {
  modulusLength: 4096,
  publicKeyEncoding: { type: 'spki', format: 'pem' },
  privateKeyEncoding: { type: 'pkcs8', format: 'pem' }
});

fs.writeFileSync('agent-private-key.pem', privateKey);
fs.writeFileSync('agent-public-key.pem', publicKey);
```

**Key Management:**
- Store private key in environment variable (never commit to git)
- Register public key with Visa Developer Portal
- Rotate keys every 90 days

### Step 3: Register Agent Public Key

Upload public key to Visa to receive `key_id` for use in Signature header.

### Step 4: Implement Signature Creation

```javascript
// tap-signer.js
const crypto = require('crypto');
const fs = require('fs');

class TAPSigner {
  constructor(keyId, privateKeyPath) {
    this.keyId = keyId;
    this.privateKey = fs.readFileSync(privateKeyPath, 'utf8');
  }

  sign(request) {
    const { method, url, headers, body } = request;
    const urlObj = new URL(url);

    const digest = `SHA-256=${crypto
      .createHash('sha256')
      .update(body || '')
      .digest('base64')}`;

    const signingString = [
      `(request-target): ${method.toLowerCase()} ${urlObj.pathname}`,
      `host: ${urlObj.host}`,
      `date: ${headers.date || new Date().toUTCString()}`,
      `digest: ${digest}`,
      `content-type: ${headers['content-type'] || 'application/json'}`
    ].join('\n');

    const signature = crypto
      .createSign('RSA-SHA256')
      .update(signingString)
      .sign(this.privateKey, 'base64');

    return {
      keyId: this.keyId,
      algorithm: 'rsa-sha256',
      headers: '(request-target) host date digest content-type',
      signature: signature
    };
  }

  formatSignatureHeader(sig) {
    return `keyId="${sig.keyId}",algorithm="${sig.algorithm}",headers="${sig.headers}",signature="${sig.signature}"`;
  }
}

module.exports = TAPSigner;
```

### Step 5: Make TAP-Signed Request

```javascript
// agent.js
const TAPSigner = require('./tap-signer');
const crypto = require('crypto');

const signer = new TAPSigner(
  process.env.VISA_KEY_ID,
  './agent-private-key.pem'
);

async function purchaseWithTAP(merchantUrl, purchaseData) {
  const date = new Date().toUTCString();
  const body = JSON.stringify(purchaseData);

  const intent = {
    action: 'purchase',
    item_id: purchaseData.item_id,
    amount: purchaseData.amount,
    timestamp: Date.now()
  };

  const intentHash = crypto
    .createHash('sha256')
    .update(JSON.stringify(intent))
    .digest('base64');

  const request = {
    method: 'POST',
    url: merchantUrl,
    headers: { 'date': date, 'content-type': 'application/json' },
    body: body
  };

  const signature = signer.sign(request);

  const response = await fetch(merchantUrl, {
    method: 'POST',
    headers: {
      'Date': date,
      'Content-Type': 'application/json',
      'Signature': signer.formatSignatureHeader(signature),
      'Agent-ID': process.env.VISA_AGENT_ID,
      'Intent-Hash': `SHA-256=${intentHash}`
    },
    body: body
  });

  return response;
}
```

---

## Interoperability with x402

### TAP + x402 Integration

```javascript
const { TAPClient } = require('@visa/tap-sdk');
const { paidFetch } = require('@faremeter/fetch');

const agent = new TAPClient({
  keyId: process.env.VISA_KEY_ID,
  privateKey: process.env.VISA_PRIVATE_KEY
});

async function securePurchase(url, data) {
  const signedRequest = await agent.createSignedRequest({
    method: 'POST',
    url: url,
    body: data
  });

  // Make request with x402 payment capability
  const response = await paidFetch(url, {
    method: 'POST',
    headers: signedRequest.headers,  // Includes TAP signature
    body: JSON.stringify(data),
    wallet: myWallet,
    maxAmount: data.amount
  });

  return response;
}
```

**Benefits:**
- TAP proves "this is a real, authorized agent"
- x402 proves "payment has been made"
- Together: Complete trust and settlement

---

## Use Cases for Hackathon

### 1. E-commerce Agent Platform

**Concept:** AI shopping agents that browse and purchase on behalf of users

**TAP Integration:**
- Agent identity verification
- Consumer loyalty data transmission
- Purchase intent verification
- Fraud prevention via device fingerprints

**Tech Stack:** Visa TAP + x402 + CDP Wallets + Solana

**Prize Potential:** Visa TAP ($10k) + Best x402 Agent ($10k) = $20k

---

### 2. Merchant Trust Network

**Concept:** Platform for merchants to verify and trust AI agents

**TAP Integration:**
- Agent reputation system using TAP signatures
- Historical purchase verification
- Dispute resolution via signed intent logs

**Tech Stack:** Visa TAP + Solana (on-chain reputation) + x402

**Prize Potential:** Visa TAP ($10k) + Best Trustless Agent ($10k) = $20k

---

### 3. Autonomous Subscription Manager

**Concept:** Agent that manages recurring subscriptions and optimizes costs

**TAP Integration:**
- Monthly payment authorization via TAP
- Service discovery with verified merchants
- Automatic renewal with signed intent

**Tech Stack:** Visa TAP + CDP Wallets (policies) + x402

**Prize Potential:** Visa TAP ($10k) + CDP Wallets ($5k) = $15k

---

### 4. Agent Commerce Marketplace

**Concept:** Two-sided marketplace connecting agents and merchants

**TAP Integration:**
- Agent onboarding with TAP registration
- Merchant verification portal
- Transaction monitoring dashboard

**Tech Stack:** Visa TAP + x402 + Solana + React

**Prize Potential:** Visa TAP ($10k) + Best x402 Agent ($10k) = $20k

---

## Troubleshooting

See [common-troubleshooting.md](./common-troubleshooting.md) for generic issues.

### TAP-Specific Issues

#### Issue: "Invalid TAP signature"

**Possible Causes:**
- Incorrect signing string construction
- Wrong headers included in signature
- Date header too old (>5 minutes)
- Public key not registered

**Solutions:**
```javascript
// Ensure correct header order
const headers = '(request-target) host date digest content-type';

// Ensure fresh date
const date = new Date().toUTCString();

// Verify key is registered
curl https://developer.visa.com/api/tap/agents/$KEY_ID \
  -H "Authorization: Bearer $VISA_API_KEY"
```

---

#### Issue: "Intent hash mismatch"

**Solutions:**
```javascript
// Use deterministic JSON serialization
const intent = {
  action: 'purchase',
  item_id: req.body.item_id,
  amount: req.body.amount,
  timestamp: req.body.timestamp  // Use exact timestamp from request
};

// Sort keys before hashing
const intentStr = JSON.stringify(intent, Object.keys(intent).sort());
const hash = crypto.createHash('sha256').update(intentStr).digest('base64');
```

---

#### Issue: "Agent not found"

**Solutions:**
```bash
# Check agent status
curl https://developer.visa.com/api/tap/agents/$AGENT_ID \
  -H "Authorization: Bearer $VISA_API_KEY"

# Re-register if needed (see Step 3)
```

---

## Additional Resources

### Official Links
- Developer Portal: https://developer.visa.com/capabilities/trusted-agent-protocol
- GitHub: https://github.com/visa/trusted-agent-protocol
- RFC 9421: https://www.rfc-editor.org/rfc/rfc9421.html
- Cloudflare Blog: https://blog.cloudflare.com/secure-agentic-commerce

### Community
- Visa Developer Forums: https://developer.visa.com/community
- Stack Overflow: Tag `visa-tap`
- Discord: #visa-tap (Solana x402 Hackathon server)

### Related Guides
- [x402 Protocol Specification](../../x402-protocol-specification.md)
- [CDP Wallets Integration](./cdp-wallets-integration.md)
- [Multi-Sponsor Strategies](./multi-sponsor-strategies.md)

---

**Last Updated:** November 4, 2025
**Hackathon Deadline:** November 11, 2025
**Integration Difficulty:** Medium (4-8 hours)
