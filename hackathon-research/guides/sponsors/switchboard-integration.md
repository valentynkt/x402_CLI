# Switchboard Oracle Integration Guide

**Prize:** $5,000
**Sponsor:** Switchboard Labs
**First x402 Compatible Oracle:** October 23, 2025
**Difficulty:** Medium
**Est. Integration Time:** 2-4 hours

---

## Overview

Switchboard is the first and only x402-compatible oracle network, providing AI agents with pay-per-query access to verified real-world data. Sub-100ms latency via Switchboard Surge, protecting $5B+ in assets across 50+ protocols, and micropayment pricing at <$0.001 per query.

### Key Benefits

- **First x402 Oracle:** Only oracle with native x402 payment support (Oct 23, 2025)
- **Ultra-Low Latency:** <100ms with Switchboard Surge real-time feeds
- **Micropayment Pricing:** Pay <$0.001 per query via USDC on Solana
- **Production Battle-Tested:** Securing $5B+ across 50+ DeFi protocols
- **Comprehensive Data:** Price feeds, randomness, custom data sources
- **Corbits SDK Ready:** First-class integration with Faremeter framework

### Why Use Switchboard?

$5,000 prize dedicated to Switchboard integration. Only x402-compatible oracle (competitive advantage). Quick 2-4 hour integration with excellent documentation. Multi-prize eligible: Combine with x402 Agent ($10k) + Corbits ($5k) = $20k potential.

---

## What is Switchboard?

AI agents need real-world data to make decisions, but traditional oracles require pre-funded subscriptions (not pay-per-use), human wallet management (no autonomous payments), API keys and authentication (friction for agents), and centralized access control (trust dependencies).

### The Switchboard Solution

Switchboard provides **x402-native oracle feeds** that enable:

1. **Pay-Per-Query:** Agents pay only for data they use (no subscriptions)
2. **Autonomous Payments:** Native USDC micropayments via x402 protocol
3. **Decentralized Verification:** Cryptographic proof of data accuracy
4. **Sub-Second Response:** Ultra-low latency for trading and real-time apps
5. **Broad Coverage:** 1000+ price feeds, VRF randomness, custom data

### Switchboard vs Traditional Oracles

| Feature | Switchboard | Chainlink | Pyth |
|---------|-------------|-----------|------|
| **x402 Support** | Yes (only oracle) | No | No |
| **Payment Model** | Pay-per-query | Subscription | Free (subsidized) |
| **Latency** | <100ms (Surge) | ~10s | ~400ms |
| **Solana Native** | Yes | Multi-chain | Solana + 50+ chains |
| **Cost per Query** | <$0.001 | Varies | Free |
| **Agent Autonomy** | Full (x402) | Requires setup | Requires setup |

**Best for hackathon:** Switchboard's x402 compatibility makes it the only oracle that enables truly autonomous AI agent data access.

---

## Technical Architecture

### Feed Types

**Price Feeds:**
- SOL/USD, BTC/USD, ETH/USD (1000+ pairs)
- $0.0008 per query
- 7+ data sources per feed

**VRF (Verifiable Random Function):**
- Cryptographically secure randomness
- On-chain proof of fairness
- $0.0005 per query

**Switchboard Surge (Real-Time):**
- Sub-100ms latency
- WebSocket streaming
- Monthly subscription model

### x402 Payment Flow

See [visa-tap-integration.md](./visa-tap-integration.md) for x402 payment flow description.

**Total time:** ~1.5 seconds (including on-chain verification)

---

## Developer Resources

### Official Resources (2025)

**Documentation:**
- Main Docs: https://docs.switchboard.xyz
- x402 Guide: https://docs.switchboard.xyz/x402
- API Reference: https://docs.switchboard.xyz/api
- Price Feeds List: https://app.switchboard.xyz/solana/mainnet

**Code & SDKs:**
- GitHub: https://github.com/switchboard-xyz/solana-sdk
- NPM: `@switchboard-xyz/solana.js`
- Corbits Integration: Via `@faremeter/fetch` (native support)

**Tools:**
- Explorer: https://app.switchboard.xyz
- Feed Monitor: https://app.switchboard.xyz/feeds
- Testnet Faucet: https://faucet.switchboard.xyz

**Community:**
- Discord: https://discord.gg/switchboardxyz
- Twitter: @switchboardxyz
- Email: support@switchboard.xyz

### SDK Installation

```bash
# Core Switchboard SDK
npm install @switchboard-xyz/solana.js

# x402 integration (recommended)
npm install @faremeter/fetch

# Solana dependencies
npm install @solana/web3.js @solana/spl-token
```

---

## Step-by-Step Integration

### Prerequisites

- Node.js 18+ development environment
- Solana wallet with USDC (devnet or mainnet)
- Basic understanding of Solana transactions
- Switchboard API key (optional for public feeds)

### Step 1: Get Switchboard Access

**For Public Feeds:** Most price feeds are publicly accessible via x402 payment without registration.

**For Custom Feeds or Higher Limits:** Visit https://app.switchboard.xyz and create API key.

### Step 2: Setup Solana Wallet

```javascript
const { Keypair } = require('@solana/web3.js');
const fs = require('fs');

// Generate new wallet
const wallet = Keypair.generate();
fs.writeFileSync('wallet.json', JSON.stringify(Array.from(wallet.secretKey)));

console.log('Wallet public key:', wallet.publicKey.toString());

// Fund wallet with USDC (testnet)
// Visit https://faucet.switchboard.xyz and request USDC
```

### Step 3: Query Price Feed with x402

```javascript
const { paidFetch, x402Config } = require('./config');

async function getSolPrice() {
  const feedUrl = 'https://api.switchboard.xyz/feeds/SOL-USD';

  try {
    const response = await paidFetch(feedUrl, {
      wallet: x402Config.wallet,
      connection: x402Config.connection,
      maxAmount: 0.001,  // Willing to pay up to 0.001 USDC
      method: 'GET',
      headers: { 'Accept': 'application/json' }
    });

    const data = await response.json();

    console.log('SOL Price:', data.price);
    console.log('Confidence:', data.confidence);
    console.log('Last Update:', new Date(data.timestamp * 1000));
    console.log('Payment:', response.headers.get('X-PAYMENT-AMOUNT'), 'USDC');

    return data;

  } catch (error) {
    console.error('Error fetching price:', error);
    throw error;
  }
}

getSolPrice();
```

### Step 4: Query Multiple Feeds

```javascript
async function getMultiplePrices(symbols) {
  const feeds = symbols.map(symbol => ({
    symbol: symbol,
    url: `https://api.switchboard.xyz/feeds/${symbol}-USD`
  }));

  const results = await Promise.all(
    feeds.map(async (feed) => {
      const response = await paidFetch(feed.url, {
        wallet: x402Config.wallet,
        connection: x402Config.connection,
        maxAmount: 0.001
      });

      const data = await response.json();

      return {
        symbol: feed.symbol,
        price: data.price,
        timestamp: data.timestamp,
        cost: parseFloat(response.headers.get('X-PAYMENT-AMOUNT'))
      };
    })
  );

  const totalCost = results.reduce((sum, r) => sum + r.cost, 0);

  console.log('Prices:');
  results.forEach(r => {
    console.log(`  ${r.symbol}: $${r.price} (cost: ${r.cost} USDC)`);
  });
  console.log(`Total cost: ${totalCost} USDC`);

  return results;
}

getMultiplePrices(['SOL', 'BTC', 'ETH', 'USDT']);
```

### Step 5: Use Switchboard Surge (Real-Time)

```javascript
const { SwitchboardSurge } = require('@switchboard-xyz/solana.js');

const surge = new SwitchboardSurge({
  apiKey: process.env.SWITCHBOARD_API_KEY,
  feeds: ['SOL/USD', 'BTC/USD', 'ETH/USD'],
  updateInterval: 100  // Update every 100ms
});

surge.on('update', (feed, data) => {
  console.log(`[${new Date().toISOString()}] ${feed}`);
  console.log(`  Price: $${data.price}`);
  console.log(`  Change: ${data.change_24h}%`);
  console.log(`  Latency: ${data.latency}ms`);
});

surge.on('error', (error) => {
  console.error('Surge error:', error);
});

surge.connect();
console.log('Connected to Switchboard Surge');

// Pricing note: Surge is billed monthly, not per-query
// Check https://docs.switchboard.xyz/surge/pricing
```

---

## Code Examples

### Trading Bot with Price Alerts

```javascript
const { paidFetch, x402Config } = require('./config');

class TradingBot {
  constructor(symbol, targetPrice, action) {
    this.symbol = symbol;
    this.targetPrice = targetPrice;
    this.action = action;
    this.checking = false;
  }

  async checkPrice() {
    if (this.checking) return;
    this.checking = true;

    try {
      const feedUrl = `https://api.switchboard.xyz/feeds/${this.symbol}-USD`;

      const response = await paidFetch(feedUrl, {
        wallet: x402Config.wallet,
        connection: x402Config.connection,
        maxAmount: 0.001
      });

      const data = await response.json();
      const currentPrice = data.price;

      console.log(`${this.symbol}: $${currentPrice} (target: $${this.targetPrice})`);

      if (this.action === 'buy' && currentPrice <= this.targetPrice) {
        console.log(`BUY signal! ${this.symbol} at $${currentPrice}`);
        await this.executeTrade('buy', currentPrice);
      } else if (this.action === 'sell' && currentPrice >= this.targetPrice) {
        console.log(`SELL signal! ${this.symbol} at $${currentPrice}`);
        await this.executeTrade('sell', currentPrice);
      }

    } catch (error) {
      console.error('Error checking price:', error);
    } finally {
      this.checking = false;
    }
  }

  async executeTrade(action, price) {
    console.log(`Executing ${action} at $${price}`);
    // Integrate with DEX here (Jupiter, Raydium, etc.)
  }

  start(intervalSeconds = 5) {
    console.log(`Starting bot: ${this.action} ${this.symbol} at $${this.targetPrice}`);
    this.interval = setInterval(() => this.checkPrice(), intervalSeconds * 1000);
  }

  stop() {
    if (this.interval) {
      clearInterval(this.interval);
      console.log('Bot stopped');
    }
  }
}

const bot = new TradingBot('SOL', 150.00, 'buy');
bot.start(10);  // Check every 10 seconds
```

---

## x402 Compatibility

### Why x402 Matters for Oracles

Traditional oracle access requires pre-funded accounts, API key management, manual payment setup, and human intervention for refills.

**x402 enables:**
- Pay-per-query (no subscriptions)
- Autonomous agent access (no human needed)
- Micropayments (<$0.001 per query)
- Instant settlement (400ms Solana finality)

### Switchboard x402 Implementation

Switchboard was the **first oracle to support x402** (October 23, 2025) via integration with Corbits SDK:

```javascript
const { paidFetch } = require('@faremeter/fetch');

// Automatic x402 handling:
// 1. First request → 402 response
// 2. Construct USDC payment
// 3. Retry with X-PAYMENT header
// 4. Receive data + on-chain verification

const response = await paidFetch(
  'https://api.switchboard.xyz/feeds/SOL-USD',
  {
    wallet: myWallet,
    connection: solanaConnection,
    maxAmount: 0.001
  }
);
```

---

## Use Cases for Hackathon

### 1. Autonomous Trading Agent

**Concept:** AI agent that monitors Switchboard price feeds and executes trades based on ML predictions

**Switchboard Integration:**
- Real-time price feeds via Surge (<100ms latency)
- Pay-per-query for cost efficiency
- Historical price data for backtesting

**Tech Stack:** Switchboard + x402 + Solana DEX + CDP Wallets

**Prize Potential:** Switchboard ($5k) + Best x402 Agent ($10k) + Corbits ($5k) = $20k

**Difficulty:** Medium (6-10 hours)

---

### 2. DeFi Risk Monitor

**Concept:** Agent that monitors liquidation risks across DeFi protocols using real-time oracle data

**Switchboard Integration:**
- Multi-asset price feeds
- Custom data feeds for TVL metrics
- Sub-second updates via Surge

**Tech Stack:** Switchboard + Solana + x402 + React Dashboard

**Prize Potential:** Switchboard ($5k) + Best x402 Agent ($10k) = $15k

**Difficulty:** Medium (8-12 hours)

---

### 3. Price Alert Bot

**Concept:** Simple agent that sends alerts when prices cross user-defined thresholds

**Switchboard Integration:**
- Periodic price queries via x402
- Pay only when checking prices
- Telegram/Discord notifications

**Tech Stack:** Switchboard + x402 + Node.js + Notification APIs

**Prize Potential:** Switchboard ($5k) + Best x402 Agent ($10k) = $15k

**Difficulty:** Easy (4-6 hours)

---

### 4. On-Chain Game with VRF

**Concept:** Fully on-chain game using Switchboard VRF for provably fair randomness

**Switchboard Integration:**
- VRF for random number generation
- Cryptographic proofs of fairness
- Pay-per-randomness via x402

**Tech Stack:** Switchboard VRF + Solana Program + x402 + React

**Prize Potential:** Switchboard ($5k) + Best x402 Application ($10k) = $15k

**Difficulty:** High (12-16 hours)

---

## Troubleshooting

See [common-troubleshooting.md](./common-troubleshooting.md) for generic issues.

### Switchboard-Specific Issues

#### Feed not found

**Solutions:**
```javascript
// Check available feeds
const feedList = await paidFetch(
  'https://api.switchboard.xyz/feeds',
  { wallet, connection, maxAmount: 0.0001 }
);

const feeds = await feedList.json();
console.log('Available feeds:', feeds.map(f => f.symbol));

// Use correct format (usually TOKEN-USD, not TOKEN-USDC)
const correctUrl = 'https://api.switchboard.xyz/feeds/SOL-USD';  // ✓
const wrongUrl = 'https://api.switchboard.xyz/feeds/SOL-USDC';   // ✗
```

#### Payment verification failed

**Solutions:**
```javascript
// Ensure transaction is confirmed before retrying
const response = await paidFetch(feedUrl, {
  wallet: wallet,
  connection: connection,
  maxAmount: 0.001,
  confirmOptions: {
    commitment: 'confirmed',
    preflightCommitment: 'confirmed'
  }
});

// Check transaction status manually
const signature = response.headers.get('X-PAYMENT-SIGNATURE');
const status = await connection.getSignatureStatus(signature);
console.log('Payment status:', status);
```

#### Rate limit exceeded

**Solutions:**
```javascript
// Add API key for higher limits
const response = await paidFetch(feedUrl, {
  wallet: wallet,
  connection: connection,
  maxAmount: 0.001,
  headers: {
    'X-API-KEY': process.env.SWITCHBOARD_API_KEY
  }
});
```

---

## Additional Resources

### Official Links
- Documentation: https://docs.switchboard.xyz
- Explorer: https://app.switchboard.xyz
- x402 Guide: https://docs.switchboard.xyz/x402
- GitHub: https://github.com/switchboard-xyz

### Community
- Discord: https://discord.gg/switchboardxyz
- Twitter: @switchboardxyz
- Email: support@switchboard.xyz

### Related Guides
- [x402 Protocol Specification](../../x402-protocol-specification.md)
- [Corbits Integration](../ecosystem/corbits-integration.md)
- [Visa TAP Integration](./visa-tap-integration.md)

---

**Last Updated:** November 4, 2025
**Hackathon Deadline:** November 11, 2025
**Integration Difficulty:** Medium (2-4 hours)

Switchboard is the **only x402-compatible oracle** - this is a unique competitive advantage!
