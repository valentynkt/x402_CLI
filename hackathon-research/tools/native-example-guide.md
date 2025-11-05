# Native x402 Example Guide

**Quick Links:** [GitHub](https://github.com/Woody4618/x402-solana-examples) | [Solana Guide](https://solana.com/developers/guides/getstarted/intro-to-x402#native-implementation)
**License:** Open Source | **Integration Difficulty:** Advanced

## Overview

The **Native x402 Example** demonstrates minimal x402 implementation **without dependencies** on SDKs or facilitators. This bare-metal approach shows the core protocol mechanics using Express (server) and Node (client), implementing x402 from first principles on Solana.

## Key Features

- ✅ **Zero Dependencies:** No x402 SDK, no facilitator services
- ✅ **Educational:** Learn x402 protocol internals
- ✅ **Customizable:** Full control over every step
- ✅ **Solana Native:** Direct Solana blockchain interaction
- ✅ **Complete Flow:** Server, client, and payment verification
- ✅ **Production Pattern:** Real-world implementation reference

## Technical Specifications

### Technology Stack
- **Server:** Express.js (minimal HTTP server)
- **Client:** Node.js (fetch API)
- **Blockchain:** Solana
- **Payment Token:** USDC (SPL)
- **Dependencies:** Only Solana Web3.js and SPL Token libraries
- **Protocol:** Pure x402 HTTP 402 implementation

### Repository Structure
```
x402-solana-examples/
├── server/
│   ├── index.js           # Express server with 402 handling
│   ├── verify.js          # On-chain payment verification
│   └── wallet.js          # Merchant wallet setup
├── client/
│   ├── index.js           # Client with payment flow
│   ├── sign.js            # Transaction signing
│   └── wallet.js          # Client wallet setup
├── shared/
│   └── constants.js       # Network configs, addresses
└── README.md              # Flow documentation
```

### Supported Networks

| Network | Support | Primary Token | Status |
|---------|---------|---------------|--------|
| **Solana Mainnet** | Primary | USDC (SPL) | Production |
| **Solana Devnet** | Testing | Devnet USDC | Testing |

## Protocol Implementation

### x402 Flow Overview

The native example implements the complete x402 flow:

```
1. Client → Server: GET /api/data
2. Server → Client: 402 Payment Required + payment details
3. Client: Construct USDC transfer transaction
4. Client: Sign transaction with wallet
5. Client → Server: POST /api/data + payment proof
6. Server: Verify transaction on-chain
7. Server → Client: 200 OK + requested data
```

### Minimal Server Implementation

```javascript
// server/index.js
const express = require('express');
const { verifyPayment } = require('./verify');
const app = express();

// Protected endpoint
app.get('/api/data', async (req, res) => {
  const paymentHeader = req.headers['x-payment-signature'];

  if (!paymentHeader) {
    // Step 1: No payment → Return 402
    return res.status(402).json({
      error: 'Payment Required',
      amount: 0.01, // USDC
      recipient: process.env.MERCHANT_WALLET,
      currency: 'USDC',
      chain: 'solana',
      nonce: generateNonce()
    });
  }

  // Step 2: Verify payment on-chain
  try {
    const isValid = await verifyPayment(paymentHeader);

    if (!isValid) {
      return res.status(403).json({ error: 'Invalid payment' });
    }

    // Step 3: Payment verified → Return data
    res.json({
      data: 'Your premium data',
      timestamp: Date.now()
    });
  } catch (error) {
    res.status(500).json({ error: 'Verification failed' });
  }
});

app.listen(3000);
```

### Payment Verification (On-Chain)

```javascript
// server/verify.js
const { Connection, PublicKey } = require('@solana/web3.js');
const { getAccount } = require('@solana/spl-token');

async function verifyPayment(signature) {
  const connection = new Connection(process.env.SOLANA_RPC_URL);

  // 1. Fetch transaction from blockchain
  const tx = await connection.getTransaction(signature, {
    commitment: 'confirmed'
  });

  if (!tx) {
    return false;
  }

  // 2. Parse transaction details
  const { meta, transaction } = tx;

  // 3. Verify recipient
  const recipientAccount = transaction.message.accountKeys[1];
  if (recipientAccount.toBase58() !== process.env.MERCHANT_WALLET) {
    return false;
  }

  // 4. Verify amount (parse from instruction data)
  const transferAmount = parseTransferAmount(transaction);
  if (transferAmount < 0.01) { // USDC has 6 decimals
    return false;
  }

  // 5. Verify token is USDC
  const tokenMint = parseTokenMint(transaction);
  const USDC_MINT = 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v';
  if (tokenMint !== USDC_MINT) {
    return false;
  }

  return true;
}

module.exports = { verifyPayment };
```

### Minimal Client Implementation

```javascript
// client/index.js
const {
  Connection,
  PublicKey,
  Transaction,
  sendAndConfirmTransaction
} = require('@solana/web3.js');
const {
  createTransferInstruction,
  getAssociatedTokenAddress
} = require('@solana/spl-token');

async function fetchPaidResource(url) {
  const connection = new Connection(process.env.SOLANA_RPC_URL);

  // 1. Initial request (no payment)
  const response1 = await fetch(url);

  if (response1.status !== 402) {
    // No payment required
    return await response1.json();
  }

  // 2. Parse payment requirements
  const paymentInfo = await response1.json();
  console.log('Payment required:', paymentInfo);

  // 3. Construct USDC transfer transaction
  const USDC_MINT = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');
  const merchantPubkey = new PublicKey(paymentInfo.recipient);
  const walletPubkey = myWallet.publicKey;

  const senderATA = await getAssociatedTokenAddress(
    USDC_MINT,
    walletPubkey
  );

  const recipientATA = await getAssociatedTokenAddress(
    USDC_MINT,
    merchantPubkey
  );

  const transferInstruction = createTransferInstruction(
    senderATA,
    recipientATA,
    walletPubkey,
    paymentInfo.amount * 1_000_000, // USDC has 6 decimals
    [],
    TOKEN_PROGRAM_ID
  );

  const transaction = new Transaction().add(transferInstruction);

  // 4. Send transaction
  const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [myWallet]
  );

  console.log('Payment sent:', signature);

  // 5. Retry request with payment proof
  const response2 = await fetch(url, {
    headers: {
      'X-Payment-Signature': signature
    }
  });

  return await response2.json();
}

module.exports = { fetchPaidResource };
```

## Use Cases for Hackathon

### 1. Learning x402 Internals
**Scenario:** Deep understanding of protocol mechanics
**Benefit:** Build custom implementations
**Audience:** Advanced developers, protocol researchers

### 2. Custom Payment Logic
**Scenario:** Non-standard payment requirements
**Implementation:** Modify native example for escrow, multisig, etc.
**Benefit:** Flexibility beyond SDK limitations

### 3. Minimalist Deployment
**Scenario:** Embedded systems, serverless functions
**Implementation:** Native example has minimal footprint
**Benefit:** Smallest possible bundle size

### 4. Educational Demos
**Scenario:** Teaching x402 protocol
**Implementation:** Step-by-step native implementation
**Benefit:** Clear understanding without SDK magic

## Integration Difficulty Breakdown

### Easy ✅
- Well-commented code
- Clear flow documentation
- Standard Node.js/Express patterns
- Minimal dependencies

### Medium ⚠️
- Understanding Solana transactions
- SPL token mechanics
- On-chain verification logic
- Error handling

### Advanced 🔧
- Custom payment verification
- Security hardening
- Production deployment
- Scaling considerations
- Replay attack prevention

## Advantages of Native Implementation

### Full Control
- **Customization:** Modify any step of the flow
- **Optimization:** Remove unnecessary abstractions
- **Debugging:** Understand every line of code
- **Security:** No SDK vulnerabilities

### Learning Benefits
- **Protocol Understanding:** See x402 mechanics directly
- **Blockchain Skills:** Learn Solana transaction construction
- **Best Practices:** Production-ready patterns
- **Foundation:** Build your own SDK if needed

### Minimal Footprint
- **Small Bundle:** No large SDK dependencies
- **Fast Startup:** Minimal initialization
- **Low Memory:** Efficient resource usage
- **Serverless-Friendly:** Works in constrained environments

## Disadvantages vs SDKs

### More Code to Write
- Manual transaction construction
- Manual verification logic
- Manual error handling
- No high-level abstractions

### Maintenance Burden
- Keep up with protocol changes
- Implement new features manually
- Security updates self-managed
- No community SDK support

### Higher Risk
- More chance for security bugs
- Manual replay protection needed
- More testing required
- No SDK-level optimizations

## When to Choose Native Implementation

**✅ Choose Native if you:**
- Want deep protocol understanding
- Need custom payment logic
- Require minimal dependencies
- Building educational material
- Have advanced Solana knowledge
- Need maximum control/flexibility

**❌ Use SDK instead if you:**
- Want fastest development (use Corbits/PayAI)
- Prefer battle-tested code
- Need community support
- Don't need customization
- Want automatic protocol updates

## Quick Start Checklist

- [ ] Review x402 protocol specification
- [ ] Understand Solana transaction structure
- [ ] Study SPL token transfer mechanics
- [ ] Clone example repository
- [ ] Set up Solana devnet wallet
- [ ] Get devnet USDC from faucet
- [ ] Run server locally (port 3000)
- [ ] Test client payment flow
- [ ] Verify transaction on explorer
- [ ] Study verification logic
- [ ] Customize for your use case
- [ ] Add error handling
- [ ] Implement replay protection
- [ ] Test on devnet extensively
- [ ] Deploy to mainnet

## Code Example: Complete Minimal Flow

```javascript
// === COMPLETE MINIMAL SERVER ===
// server.js (50 lines)
const express = require('express');
const { Connection, PublicKey } = require('@solana/web3.js');

const app = express();
const connection = new Connection(process.env.RPC_URL);
const MERCHANT_WALLET = process.env.MERCHANT_WALLET;
const PRICE = 0.01; // USDC

app.get('/api/data', async (req, res) => {
  const sig = req.headers['x-payment-signature'];

  if (!sig) {
    return res.status(402).json({
      price: PRICE,
      recipient: MERCHANT_WALLET,
      currency: 'USDC',
      chain: 'solana'
    });
  }

  // Verify payment
  try {
    const tx = await connection.getTransaction(sig);
    if (!tx) {
      return res.status(403).json({ error: 'Transaction not found' });
    }

    // TODO: Parse and verify amount, recipient, token
    // (Implementation details in repository)

    res.json({ data: 'Your premium data' });
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
});

app.listen(3000);

// === COMPLETE MINIMAL CLIENT ===
// client.js (60 lines)
const fetch = require('node-fetch');
const {
  Connection,
  Transaction,
  sendAndConfirmTransaction
} = require('@solana/web3.js');
const { createTransferInstruction } = require('@solana/spl-token');

async function callPaidAPI(url, wallet) {
  const connection = new Connection(process.env.RPC_URL);

  // Initial request
  const res1 = await fetch(url);

  if (res1.status === 402) {
    const payment = await res1.json();

    // Construct payment (USDC transfer)
    // TODO: Build transaction, get ATAs, create instruction
    // (Implementation details in repository)

    const tx = new Transaction().add(transferInstruction);
    const sig = await sendAndConfirmTransaction(connection, tx, [wallet]);

    // Retry with payment
    const res2 = await fetch(url, {
      headers: { 'X-Payment-Signature': sig }
    });

    return await res2.json();
  }

  return await res1.json();
}
```

## Security Considerations

### Replay Protection
```javascript
// Track used transaction signatures
const usedSignatures = new Set();

function verifyPayment(signature) {
  if (usedSignatures.has(signature)) {
    throw new Error('Payment already used');
  }

  // Verify transaction...

  usedSignatures.add(signature);
  return true;
}
```

### Amount Verification
```javascript
// Always verify exact amount
function parseAmount(transaction) {
  const amount = /* parse from instruction data */;
  const MINIMUM_AMOUNT = 0.01 * 1_000_000; // USDC decimals

  if (amount < MINIMUM_AMOUNT) {
    throw new Error('Insufficient payment');
  }

  return amount;
}
```

### Rate Limiting
```javascript
// Prevent abuse
const rateLimit = require('express-rate-limit');

app.use('/api', rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100 // Max 100 requests per window
}));
```

## Documentation Quality: MEDIUM

**Available Resources:**
- **GitHub Repository** - Complete working example
- **README** - Flow documentation
- **Code Comments** - Inline explanations
- **Solana Guide** - Official documentation reference

**GitHub:**
- URL: https://github.com/Woody4618/x402-solana-examples
- Code: Well-commented JavaScript
- Examples: Server + client implementations
- Status: Working, maintained

**Gaps:**
- No video tutorials
- Limited error handling examples
- Basic security patterns only
- No advanced features documented

## Community & Support

**Channels:**
- **GitHub Issues** - Questions and bugs
- **Solana Discord** - #x402 channel
- **Code Review** - Learn from repository

**Response Time:** Community-driven, variable

## Hackathon Tips

### Prize Track Alignment
- **Best x402 Agent Application** ⚠️ (Higher risk but more impressive)
- **Best Corbits Project** ❌ (Not Corbits-based)
- **Educational Submissions** ✅ (Teach-the-judges angle)

### Competitive Advantages
1. **Deep Understanding:** Judges appreciate protocol mastery
2. **Custom Features:** Implement unique payment logic
3. **Minimal Footprint:** Performance story
4. **Educational Value:** Can explain how x402 works

### Competitive Disadvantages
1. **Higher Risk:** More bugs, security issues
2. **Development Time:** Slower than using SDK
3. **Less Polish:** Won't look as refined as SDK apps

### Integration Time
- **Study Time:** 4-8 hours (learning protocol)
- **Basic Implementation:** 8-16 hours
- **Production Hardening:** 24+ hours

### Demo Strategy
1. Show side-by-side code comparison (native vs SDK)
2. Explain protocol internals during demo
3. Highlight custom features SDKs can't do
4. Emphasize learning/educational value
5. Be prepared for technical questions

## Production Deployment Checklist

- [ ] Implement comprehensive replay protection
- [ ] Add transaction expiry checking
- [ ] Verify token mint address
- [ ] Validate recipient address
- [ ] Check transaction confirmation status
- [ ] Implement proper error handling
- [ ] Add logging and monitoring
- [ ] Set up rate limiting
- [ ] Test edge cases thoroughly
- [ ] Security audit verification logic
- [ ] Load testing
- [ ] Disaster recovery plan

---

**Related Docs:**
- [x402 Protocol Specification](../x402-protocol-specification.md)
- [Solana Integration Guide](../guides/solana-integration.md)
- [SDK Comparison Reference](../reference/sdk-comparison.md)
- [Security Best Practices](../guides/security-best-practices.md)
