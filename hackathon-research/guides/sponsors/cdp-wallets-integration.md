# CDP Embedded Wallets Integration Guide

**Prize:** $5,000
**Sponsor:** Coinbase
**Total x402 Payments Processed:** 1.2M+ in 5 days (Nov 2025)
**Difficulty:** Easy-Medium
**Est. Integration Time:** 2-3 hours

---

## Overview

Coinbase Developer Platform (CDP) Embedded Wallets provide AI agents with secure, policy-controlled wallets requiring zero key management. 1.2M+ x402 payments processed in 5 days, production-ready compliance (KYT/OFAC), and official x402 facilitator support make CDP Wallets the industry standard for autonomous agent payments.

### Key Benefits

- **No Key Management:** Wallets managed by Coinbase infrastructure
- **Policy-Enforced Spending:** Programmable limits and restrictions
- **Production Ready:** 1.2M+ x402 payments (77-80% market share)
- **Compliance Built-In:** KYT (Know Your Transaction) and OFAC screening
- **x402 Facilitator:** Official facilitator at https://x402.org/facilitator
- **Multi-Chain Support:** Solana, Base, Ethereum, Polygon, and more

### Why Use CDP Wallets?

$5,000 prize for CDP integration. Industry standard with 77-80% of x402 implementations using CDP. Quick 2-3 hour setup with excellent documentation. Proven scale: Real production usage (Questflow: 130k+ transactions). Multi-prize eligible: Combine with x402 Agent ($10k) + others = $15k+ potential.

---

## What are CDP Embedded Wallets?

Traditional AI agent wallets face critical challenges: key management (agents need secure storage), no spending controls (once agent has key, unlimited access), compliance gaps (hard to implement KYT/OFAC), user trust issues (users fear giving agents full wallet access), and recovery issues (lost keys = lost funds).

### The CDP Wallet Solution

CDP provides **managed wallets with policy enforcement**:

1. **No Private Keys:** Coinbase manages keys in secure infrastructure
2. **Spending Policies:** Programmable limits on amount, frequency, recipients
3. **User Authorization:** Users grant specific permissions to agents
4. **Compliance:** Built-in transaction monitoring and screening
5. **Easy Recovery:** Users can revoke agent access anytime

### CDP vs Traditional Wallets

| Feature | CDP Wallets | Traditional Wallets |
|---------|-------------|---------------------|
| **Key Management** | Coinbase-managed | User-managed |
| **Policy Enforcement** | Built-in | Manual implementation |
| **x402 Support** | Native | Requires integration |
| **Compliance** | KYT/OFAC built-in | Manual |
| **Recovery** | Easy revocation | Lost key = lost funds |
| **Setup Time** | 15 minutes | Hours |

---

## Technical Architecture

CDP provides embedded wallets (user-owned, for consumer agents) and server wallets (app-owned, for backend services).

### x402 Payment Flow with CDP

See [visa-tap-integration.md](./visa-tap-integration.md) for detailed x402 payment flow.

**Total time:** ~2 seconds from 402 response to resource delivery

---

## Developer Resources

### Official Resources (2025)

**Documentation:**
- Main Docs: https://docs.cdp.coinbase.com
- x402 Guide: https://docs.cdp.coinbase.com/x402/welcome
- API Reference: https://docs.cdp.coinbase.com/wallet-api/reference
- Embedded Wallets: https://docs.cdp.coinbase.com/embedded-wallets

**Tools:**
- Developer Dashboard: https://portal.cdp.coinbase.com
- x402 Facilitator: https://x402.org/facilitator
- Facilitator Docs: https://x402.org/docs
- Sandbox: https://sandbox.cdp.coinbase.com

**SDKs:**
- NPM: `@coinbase/cdp-sdk`
- GitHub: https://github.com/coinbase/cdp-sdk
- Python: `coinbase-cdp`
- Ruby: `coinbase-cdp-ruby`

**Community:**
- Discord: Join via Coinbase Developer portal
- Forum: https://forums.coinbase.com/developer
- Support: developers@coinbase.com

### SDK Installation

```bash
# Core CDP SDK
npm install @coinbase/cdp-sdk

# x402 facilitator client
npm install @coinbase/x402-facilitator

# For Solana integration
npm install @solana/web3.js @solana/spl-token

# For Base integration
npm install ethers
```

---

## Step-by-Step Integration

### Prerequisites

- Coinbase Developer Platform account
- API credentials (API Key + API Secret)
- Node.js 18+ development environment
- Understanding of spending policies

### Step 1: Register with Coinbase

Sign up at https://portal.cdp.coinbase.com. Generate API credentials with permissions: `wallet:create`, `wallet:transfer`, `wallet:read`. Save credentials:

```bash
CDP_API_KEY_NAME=organizations/{org_id}/apiKeys/{key_id}
CDP_API_KEY_PRIVATE_KEY="-----BEGIN EC PRIVATE KEY-----\n...\n-----END EC PRIVATE KEY-----"
```

**Important:** Private key is shown only once. Store in `.env` file immediately.

### Step 2: Initialize CDP SDK

```javascript
const { Coinbase } = require('@coinbase/cdp-sdk');

const coinbase = new Coinbase({
  apiKeyName: process.env.CDP_API_KEY_NAME,
  privateKey: process.env.CDP_API_KEY_PRIVATE_KEY
});

async function verifyConnection() {
  try {
    const user = await coinbase.getDefaultUser();
    console.log('Connected to CDP as:', user.id);
    return true;
  } catch (error) {
    console.error('Connection failed:', error);
    return false;
  }
}

verifyConnection();
```

### Step 3: Create Embedded Wallet

```javascript
async function createWallet(userId, networkId = 'solana-devnet') {
  try {
    const wallet = await coinbase.createWallet({
      userId: userId,
      networkId: networkId
    });

    console.log('Wallet created!');
    console.log('Wallet ID:', wallet.id);
    console.log('Address:', wallet.defaultAddress.id);

    return wallet;

  } catch (error) {
    console.error('Error creating wallet:', error);
    throw error;
  }
}

createWallet('alice', 'solana-devnet');
```

### Step 4: Configure Spending Policy

```javascript
async function setSpendingPolicy() {
  try {
    const wallet = await coinbase.getWallet(walletConfig.walletId);

    const policy = {
      limits: {
        perTransaction: { amount: '1.0', currency: 'USDC' },
        perDay: { amount: '10.0', currency: 'USDC' },
        perMonth: { amount: '100.0', currency: 'USDC' }
      },
      allowedHours: { start: 0, end: 23 },
      requiresApproval: {
        threshold: { amount: '5.0', currency: 'USDC' }
      }
    };

    await wallet.setSpendingPolicy(policy);
    console.log('Spending policy configured:', policy);

    return policy;

  } catch (error) {
    console.error('Error setting policy:', error);
    throw error;
  }
}

setSpendingPolicy();
```

### Step 5: Fund Wallet

For testnet USDC, visit https://faucet.circle.com. For mainnet, transfer USDC to the wallet address.

### Step 6: Make x402 Payment

```javascript
const { X402Facilitator } = require('@coinbase/x402-facilitator');

const facilitator = new X402Facilitator({
  facilitatorUrl: 'https://x402.org/facilitator',
  coinbase: coinbase
});

async function makeX402Payment(resourceUrl, maxAmount = '1.0') {
  try {
    const initialResponse = await fetch(resourceUrl);

    if (initialResponse.status === 402) {
      console.log('Payment required (402 response)');

      const paymentDetails = {
        amount: initialResponse.headers.get('X-AMOUNT'),
        currency: initialResponse.headers.get('X-ACCEPT-PAYMENT')?.split('/')[0],
        recipient: initialResponse.headers.get('X-RECIPIENT'),
        network: initialResponse.headers.get('X-ACCEPT-PAYMENT')?.split('/')[1]
      };

      if (parseFloat(paymentDetails.amount) > parseFloat(maxAmount)) {
        throw new Error(`Amount ${paymentDetails.amount} exceeds max ${maxAmount}`);
      }

      const payment = await facilitator.createPayment({
        walletId: walletConfig.walletId,
        amount: paymentDetails.amount,
        currency: paymentDetails.currency,
        recipient: paymentDetails.recipient,
        network: paymentDetails.network
      });

      console.log('Payment created:', payment.signature);

      const paidResponse = await fetch(resourceUrl, {
        headers: {
          'X-PAYMENT': payment.signature,
          'X-PAYMENT-NETWORK': paymentDetails.network
        }
      });

      if (paidResponse.ok) {
        console.log('Payment successful! Resource received.');
        return await paidResponse.json();
      }
    } else if (initialResponse.ok) {
      console.log('Resource is free (no payment required)');
      return await initialResponse.json();
    }

  } catch (error) {
    console.error('x402 payment error:', error);
    throw error;
  }
}
```

---

## Code Examples

### Shopping Agent with CDP Wallet

```javascript
const { X402Facilitator } = require('@coinbase/x402-facilitator');

class ShoppingAgent {
  constructor(walletId, maxDailySpend = 20.0) {
    this.walletId = walletId;
    this.maxDailySpend = maxDailySpend;
    this.todaySpent = 0;
    this.facilitator = new X402Facilitator({
      facilitatorUrl: 'https://x402.org/facilitator',
      coinbase: coinbase
    });
  }

  async purchaseItem(merchantUrl, itemData) {
    try {
      console.log(`Attempting to purchase: ${itemData.name}`);

      if (this.todaySpent + itemData.price > this.maxDailySpend) {
        throw new Error('Daily spending limit would be exceeded');
      }

      const response = await fetch(merchantUrl, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(itemData)
      });

      if (response.status === 402) {
        const paymentDetails = this.parsePaymentHeaders(response);

        if (parseFloat(paymentDetails.amount) !== itemData.price) {
          throw new Error('Price mismatch');
        }

        const payment = await this.facilitator.createPayment({
          walletId: this.walletId,
          amount: paymentDetails.amount,
          currency: paymentDetails.currency,
          recipient: paymentDetails.recipient,
          network: paymentDetails.network
        });

        const paidResponse = await fetch(merchantUrl, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'X-PAYMENT': payment.signature,
            'X-PAYMENT-NETWORK': paymentDetails.network
          },
          body: JSON.stringify(itemData)
        });

        if (paidResponse.ok) {
          this.todaySpent += itemData.price;
          const order = await paidResponse.json();
          console.log('Purchase successful!', order);
          return order;
        }
      }

    } catch (error) {
      console.error('Purchase failed:', error);
      throw error;
    }
  }

  parsePaymentHeaders(response) {
    return {
      amount: response.headers.get('X-AMOUNT'),
      currency: response.headers.get('X-ACCEPT-PAYMENT')?.split('/')[0],
      recipient: response.headers.get('X-RECIPIENT'),
      network: response.headers.get('X-ACCEPT-PAYMENT')?.split('/')[1]
    };
  }

  async getSpendingSummary() {
    const wallet = await coinbase.getWallet(this.walletId);
    const balance = await wallet.getBalance('USDC');

    return {
      spentToday: this.todaySpent,
      remainingDaily: this.maxDailySpend - this.todaySpent,
      currentBalance: balance.amount
    };
  }
}

const agent = new ShoppingAgent('wallet-id-here', 50.0);
```

---

## Policy Configuration

### Policy Types

**1. Amount Limits:**
```javascript
{
  limits: {
    perTransaction: { amount: '5.0', currency: 'USDC' },
    perHour: { amount: '20.0', currency: 'USDC' },
    perDay: { amount: '100.0', currency: 'USDC' },
    perMonth: { amount: '1000.0', currency: 'USDC' }
  }
}
```

**2. Recipient Whitelist:**
```javascript
{
  allowedRecipients: [
    '0x1234...5678',
    '0xabcd...efgh'
  ]
  // Empty array = any recipient allowed
}
```

**3. Time Restrictions:**
```javascript
{
  allowedHours: { start: 9, end: 17 },  // 9 AM - 5 PM UTC
  allowedDays: [1, 2, 3, 4, 5]  // Monday-Friday (0=Sunday)
}
```

**4. Approval Requirements:**
```javascript
{
  requiresApproval: {
    threshold: { amount: '10.0', currency: 'USDC' },
    approvers: ['user@example.com'],
    timeout: 3600
  }
}
```

---

## x402 Facilitator Integration

### What is the x402 Facilitator?

The **Coinbase x402 Facilitator** (https://x402.org/facilitator) is the official payment verification service used by 77-80% of x402 implementations.

**Benefits:**
- Fast verification (<2 seconds)
- Production-ready infrastructure
- KYT/OFAC compliance
- Multi-chain support
- Free to use

### Using the Facilitator

```javascript
const facilitator = new X402Facilitator({
  facilitatorUrl: 'https://x402.org/facilitator',
  coinbase: coinbase
});

const payment = await facilitator.createPayment({
  walletId: 'your-wallet-id',
  amount: '0.50',
  currency: 'USDC',
  recipient: 'merchant-address',
  network: 'solana-mainnet'
});

// Merchant verifies via facilitator
const verification = await facilitator.verifyPayment({
  signature: payment.signature,
  network: 'solana-mainnet',
  expectedAmount: '0.50',
  expectedRecipient: 'merchant-address'
});
```

### Real-World Example: Questflow

Questflow processed **130,000+ autonomous microtransactions** using CDP Wallets + x402 Facilitator at $0.05 per task.

---

## Use Cases for Hackathon

### 1. Consumer Shopping Agent

**Concept:** Personal shopping assistant that finds deals and makes purchases autonomously

**CDP Integration:**
- Embedded wallet owned by user
- Spending policy limits daily purchases
- Policy approval for >$20 purchases

**Tech Stack:** CDP Wallets + x402 + Solana + React UI

**Prize Potential:** CDP Wallets ($5k) + Best x402 Agent ($10k) = $15k

**Difficulty:** Medium (6-8 hours)

---

### 2. Research Budget Agent

**Concept:** Academic research agent with strict spending controls

**CDP Integration:**
- Server wallet for university
- Policy enforces research budget
- Whitelist trusted data providers

**Tech Stack:** CDP Wallets + x402 + Multiple APIs

**Prize Potential:** CDP Wallets ($5k) + Best x402 Agent ($10k) = $15k

**Difficulty:** Easy-Medium (4-6 hours)

---

### 3. Subscription Manager

**Concept:** Agent that manages recurring payments and optimizes subscriptions

**CDP Integration:**
- Policy allows monthly payments
- Time restrictions (first week of month)
- Automatic payment to whitelisted services

**Tech Stack:** CDP Wallets + x402 + Visa TAP

**Prize Potential:** CDP Wallets ($5k) + Visa TAP ($10k) = $15k

**Difficulty:** Medium-High (10-12 hours)

---

### 4. Multi-Agent Marketplace

**Concept:** Platform where multiple agents provide services to each other

**CDP Integration:**
- Each agent has own CDP wallet
- Policies prevent overspending
- Facilitator handles all payments

**Tech Stack:** CDP Wallets + x402 + Solana + PostgreSQL

**Prize Potential:** CDP Wallets ($5k) + Best x402 Agent ($10k) + Corbits ($5k) = $20k

**Difficulty:** High (16-20 hours)

---

## Troubleshooting

See [common-troubleshooting.md](./common-troubleshooting.md) for generic issues.

### CDP-Specific Issues

#### Invalid API credentials

**Solutions:**
```javascript
// Verify credentials format
console.log('API Key Name:', process.env.CDP_API_KEY_NAME);
// Should be: organizations/{org_id}/apiKeys/{key_id}

// Check for newline issues
const privateKey = process.env.CDP_API_KEY_PRIVATE_KEY.replace(/\\n/g, '\n');
```

#### Policy violation

**Solutions:**
```javascript
// Check current spend
const wallet = await coinbase.getWallet(walletId);
const policy = await wallet.getSpendingPolicy();
const usage = await wallet.getPolicyUsage();

console.log('Policy:', policy);
console.log('Usage:', usage);

// Modify policy if needed
await wallet.updateSpendingPolicy({
  limits: {
    perDay: { amount: '100.0', currency: 'USDC' }
  }
});
```

#### Payment verification failed

**Solutions:**
```javascript
// Wait for confirmation
const payment = await facilitator.createPayment({...});

await facilitator.waitForConfirmation(payment.signature, {
  timeout: 30000,
  network: 'solana-mainnet'
});

// Then use in request
const response = await fetch(url, {
  headers: { 'X-PAYMENT': payment.signature }
});
```

---

## Additional Resources

### Official Links
- CDP Docs: https://docs.cdp.coinbase.com
- x402 Facilitator: https://x402.org/facilitator
- Developer Portal: https://portal.cdp.coinbase.com
- GitHub: https://github.com/coinbase/cdp-sdk

### Community
- Discord: Via Coinbase Developer portal
- Forum: https://forums.coinbase.com/developer
- Support: developers@coinbase.com

### Related Guides
- [x402 Protocol Specification](../../x402-protocol-specification.md)
- [Visa TAP Integration](./visa-tap-integration.md)
- [ATXP Integration](./atxp-integration.md)

---

**Last Updated:** November 4, 2025
**Hackathon Deadline:** November 11, 2025
**Integration Difficulty:** Easy-Medium (2-3 hours)

CDP Wallets power **77-80% of x402 implementations** - the industry standard!
