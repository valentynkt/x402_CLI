# Integration Patterns Guide

A practical guide to implementing x402 payments in your application. Choose the pattern that matches your use case, copy the code, and ship in hours.

## Quick Selection Guide

| Pattern | Use Case | Complexity | Time | Best For |
|---------|----------|------------|------|----------|
| **1. Simple Client** | Consuming paid APIs | Low | 30min | AI agents, clients |
| **2. Protected API** | Monetizing your API | Low | 1hr | API providers, merchants |
| **3. Agent Marketplace** | Agent-to-agent commerce | Medium | 4-8hr | Service marketplaces |
| **4. MCP Server** | Paid AI tools | Medium | 2-4hr | Claude AI tools |
| **5. Multi-Chain Router** | Network optimization | High | 8-16hr | High-volume apps |

---

## Pattern 1: Simple Client (Fetch Wrapper)

**Use Case:** AI agents or clients consuming paid APIs
**Complexity:** Low ⭐
**Time to Implement:** 30 minutes

### Overview

The simplest pattern - replace `fetch()` with `paidFetch()` to add payment handling to any HTTP request.

### Implementation

```typescript
import { paidFetch } from '@faremeter/fetch';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';

// Set up wallet (once)
const wallet = new PhantomWalletAdapter();
await wallet.connect();

// Make paid request (anywhere)
const data = await paidFetch('https://api.example.com/data', {
  wallet,
  maxAmount: 0.01  // Safety limit in USDC
});

console.log('Received data:', data);
```

### Installation

```bash
npm install @faremeter/fetch @solana/wallet-adapter-phantom
```

### How It Works

1. **Request:** Client sends request to paid API
2. **402 Response:** Server returns 402 Payment Required with payment details
3. **Auto-Payment:** `paidFetch` automatically constructs and sends payment
4. **Verification:** Server verifies payment on-chain
5. **Response:** Server returns requested data with 200 OK

### Error Handling

```typescript
try {
  const data = await paidFetch(url, {
    wallet,
    maxAmount: 0.01
  });
} catch (error) {
  if (error.code === 'INSUFFICIENT_FUNDS') {
    console.error('Not enough USDC in wallet');
  } else if (error.code === 'PAYMENT_REJECTED') {
    console.error('Server rejected payment');
  } else if (error.code === 'PRICE_TOO_HIGH') {
    console.error('Price exceeds maxAmount');
  }
}
```

### When to Use

- ✅ Building AI agents that consume APIs
- ✅ Client applications needing data
- ✅ Quick prototypes
- ✅ Browser-based apps
- ✅ CLI tools

### Complete Example

```typescript
// ai-agent.ts
import { paidFetch } from '@faremeter/fetch';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';

class AIAgent {
  private wallet: PhantomWalletAdapter;

  async init() {
    this.wallet = new PhantomWalletAdapter();
    await this.wallet.connect();
    console.log('Agent wallet connected');
  }

  async fetchData(query: string) {
    const data = await paidFetch('https://api.example.com/search', {
      method: 'POST',
      body: JSON.stringify({ query }),
      wallet: this.wallet,
      maxAmount: 0.005
    });

    return data;
  }
}

// Usage
const agent = new AIAgent();
await agent.init();
const results = await agent.fetchData('crypto prices');
```

---

## Pattern 2: Protected API (Middleware)

**Use Case:** Monetizing your API endpoints
**Complexity:** Low ⭐
**Time to Implement:** 1 hour

### Overview

Add payment protection to your existing API with middleware. Works with Express, Fastify, or any Node.js framework.

### Implementation

```typescript
import express from 'express';
import { fareMiddleware } from '@faremeter/middleware';

const app = express();

// Apply to all routes
app.use(fareMiddleware({
  facilitator: 'https://payai.network',
  price: 0.001,  // USDC per request
  merchantWallet: process.env.MERCHANT_WALLET,
  network: 'solana'
}));

// Your API endpoints
app.get('/api/data', (req, res) => {
  // Payment already verified by middleware
  res.json({ data: 'premium data' });
});

app.listen(3000);
```

### Installation

```bash
npm install @faremeter/middleware express
```

### Advanced Configuration

```typescript
// Different prices per endpoint
app.get('/api/basic',
  fareMiddleware({ price: 0.001 }),
  (req, res) => res.json({ tier: 'basic' })
);

app.get('/api/premium',
  fareMiddleware({ price: 0.01 }),
  (req, res) => res.json({ tier: 'premium' })
);

// Optional free endpoints
app.get('/api/free', (req, res) => {
  res.json({ message: 'No payment required' });
});
```

### Environment Configuration

```bash
# .env
MERCHANT_WALLET=YourSolanaWalletAddress
X402_FACILITATOR=https://payai.network
X402_NETWORK=solana
X402_DEFAULT_PRICE=0.001
```

### How It Works

1. **Incoming Request:** Client sends request to your API
2. **Middleware Check:** fareMiddleware checks for payment header
3. **No Payment:** Returns 402 with payment details
4. **With Payment:** Verifies transaction on-chain
5. **Valid Payment:** Calls next() to continue to handler
6. **Response:** Your handler returns data

### Error Responses

```typescript
// Automatic error handling by middleware
{
  "error": "Payment required",
  "amount": "0.001",
  "token": "USDC",
  "recipient": "YourWalletAddress",
  "facilitator": "https://payai.network",
  "network": "solana"
}
```

### When to Use

- ✅ Monetizing existing APIs
- ✅ Data providers
- ✅ AI model endpoints
- ✅ Premium features
- ✅ Rate limiting via payment

### Complete Example

```typescript
// api-server.ts
import express from 'express';
import { fareMiddleware } from '@faremeter/middleware';

const app = express();
app.use(express.json());

// Configuration
const fareConfig = {
  facilitator: 'https://payai.network',
  merchantWallet: process.env.MERCHANT_WALLET,
  network: 'solana'
};

// Free endpoint
app.get('/api/status', (req, res) => {
  res.json({ status: 'online', paid: false });
});

// Paid endpoints with different prices
app.get('/api/search',
  fareMiddleware({ ...fareConfig, price: 0.001 }),
  async (req, res) => {
    const { q } = req.query;
    const results = await searchDatabase(q);
    res.json({ results, paid: true });
  }
);

app.post('/api/analyze',
  fareMiddleware({ ...fareConfig, price: 0.01 }),
  async (req, res) => {
    const analysis = await runExpensiveAnalysis(req.body);
    res.json({ analysis, paid: true });
  }
);

// Start server
app.listen(3000, () => {
  console.log('Paid API running on port 3000');
  console.log(`Merchant wallet: ${process.env.MERCHANT_WALLET}`);
});
```

---

## Pattern 3: Agent-to-Agent Marketplace

**Use Case:** Agents buying and selling services from each other
**Complexity:** Medium ⭐⭐
**Time to Implement:** 4-8 hours

### Overview

Create a marketplace where AI agents can autonomously discover, purchase, and provide services.

### Seller Agent

```typescript
import express from 'express';
import { fareMiddleware } from '@faremeter/middleware';

class SellerAgent {
  private app = express();
  private services = new Map();

  constructor(private wallet: string) {
    this.setupServices();
  }

  addService(name: string, price: number, handler: Function) {
    this.services.set(name, { price, handler });

    this.app.post(`/services/${name}`,
      fareMiddleware({
        facilitator: 'https://payai.network',
        price,
        merchantWallet: this.wallet,
        network: 'solana'
      }),
      async (req, res) => {
        const result = await handler(req.body);
        res.json({ result, service: name });
      }
    );
  }

  start(port: number) {
    this.app.listen(port, () => {
      console.log(`Seller agent listening on port ${port}`);
    });
  }
}

// Usage
const seller = new SellerAgent('SellerWalletAddress');

seller.addService('translate', 0.001, async (data) => {
  return await translate(data.text, data.targetLang);
});

seller.addService('analyze', 0.01, async (data) => {
  return await deepAnalysis(data.content);
});

seller.start(3000);
```

### Buyer Agent

```typescript
import { paidFetch } from '@faremeter/fetch';

class BuyerAgent {
  constructor(private wallet: any) {}

  async discoverServices(marketplaceUrl: string) {
    // Free discovery endpoint
    const services = await fetch(`${marketplaceUrl}/list`);
    return services.json();
  }

  async purchaseService(serviceUrl: string, params: any) {
    const result = await paidFetch(serviceUrl, {
      method: 'POST',
      body: JSON.stringify(params),
      wallet: this.wallet,
      maxAmount: 0.1  // Safety limit
    });

    return result;
  }
}

// Usage
const buyer = new BuyerAgent(buyerWallet);
const services = await buyer.discoverServices('https://marketplace.com');
const result = await buyer.purchaseService(
  services[0].url,
  { text: 'Hello', targetLang: 'es' }
);
```

### Service Registry

```typescript
// marketplace-registry.ts
interface Service {
  id: string;
  name: string;
  description: string;
  price: number;
  url: string;
  provider: string;
  rating: number;
}

class ServiceRegistry {
  private services: Map<string, Service> = new Map();

  register(service: Service) {
    this.services.set(service.id, service);
  }

  search(query: string) {
    return Array.from(this.services.values())
      .filter(s => s.name.includes(query) || s.description.includes(query))
      .sort((a, b) => b.rating - a.rating);
  }

  getService(id: string) {
    return this.services.get(id);
  }
}
```

### When to Use

- ✅ Agent marketplaces
- ✅ Decentralized service networks
- ✅ Peer-to-peer agent commerce
- ✅ Dynamic service discovery
- ✅ Reputation-based systems

---

## Pattern 4: MCP Server Monetization

**Use Case:** Monetizing Claude AI tools via MCP protocol
**Complexity:** Medium ⭐⭐
**Time to Implement:** 2-4 hours

### Overview

Create paid tools for Claude Desktop using the Model Context Protocol (MCP) with x402 payments.

### Implementation

```typescript
import { createPaidTool } from 'x402-mcp';
import { MCPServer } from '@modelcontextprotocol/sdk';

const server = new MCPServer({
  name: 'premium-tools',
  version: '1.0.0'
});

// Free tool
server.addTool({
  name: 'basic_search',
  description: 'Free basic search',
  handler: async (query) => {
    return await basicSearch(query);
  }
});

// Paid tool
const premiumSearch = createPaidTool({
  name: 'premium_search',
  description: 'Advanced search with AI analysis',
  price: 0.005,  // USDC per query
  facilitator: 'https://payai.network',
  merchantWallet: process.env.MERCHANT_WALLET,
  handler: async (query) => {
    const results = await advancedSearch(query);
    const analysis = await aiAnalyze(results);
    return { results, analysis };
  }
});

server.addTool(premiumSearch);

server.start();
```

### Installation

```bash
npm install x402-mcp @modelcontextprotocol/sdk
```

### MCP Configuration

```json
{
  "mcpServers": {
    "premium-tools": {
      "command": "node",
      "args": ["dist/server.js"],
      "env": {
        "MERCHANT_WALLET": "YourWalletAddress"
      }
    }
  }
}
```

### When to Use

- ✅ Paid Claude tools
- ✅ Premium AI capabilities
- ✅ Monetizing AI workflows
- ✅ High-value AI services

---

## Pattern 5: Multi-Chain Payment Router

**Use Case:** Optimizing payments across multiple blockchains
**Complexity:** High ⭐⭐⭐
**Time to Implement:** 8-16 hours

### Overview

Intelligent routing of payments based on cost, speed, and availability across Solana, Base, and Polygon.

### Implementation

```typescript
class PaymentRouter {
  private clients = {
    solana: new SolanaClient(),
    base: new BaseClient(),
    polygon: new PolygonClient()
  };

  async selectOptimalChain(params: {
    amount: number,
    priority: 'cost' | 'speed' | 'balanced'
  }) {
    // Get current gas prices
    const costs = await Promise.all([
      this.clients.solana.estimateCost(params.amount),
      this.clients.base.estimateCost(params.amount),
      this.clients.polygon.estimateCost(params.amount)
    ]);

    if (params.priority === 'cost') {
      // Solana usually cheapest
      return costs[0].total < 0.001 ? 'solana' : 'base';
    } else if (params.priority === 'speed') {
      // Solana fastest
      return 'solana';
    } else {
      // Balanced: Base for EVM, Solana for micro
      return params.amount < 0.01 ? 'solana' : 'base';
    }
  }

  async pay(chain: string, params: PaymentParams) {
    const client = this.clients[chain];

    try {
      const tx = await client.pay(params);
      console.log(`Paid on ${chain}:`, tx.hash);
      return tx;
    } catch (error) {
      // Fallback to another chain
      console.warn(`Failed on ${chain}, trying fallback`);
      return this.payWithFallback(chain, params);
    }
  }

  private async payWithFallback(failedChain: string, params: PaymentParams) {
    const chains = Object.keys(this.clients).filter(c => c !== failedChain);

    for (const chain of chains) {
      try {
        return await this.clients[chain].pay(params);
      } catch (error) {
        continue;
      }
    }

    throw new Error('All chains failed');
  }
}

// Usage
const router = new PaymentRouter();

const chain = await router.selectOptimalChain({
  amount: 0.005,
  priority: 'cost'
});

const tx = await router.pay(chain, {
  recipient: 'MerchantAddress',
  amount: 0.005,
  token: 'USDC'
});
```

### When to Use

- ✅ High-volume applications
- ✅ Cost optimization critical
- ✅ Multi-chain support needed
- ✅ Redundancy required
- ✅ Global user base

---

## Best Practices

### Network Selection

| Criteria | Solana | Base | Polygon |
|----------|--------|------|---------|
| **Micropayments (<$0.01)** | ✅ Best | ❌ Too expensive | ❌ Too expensive |
| **EVM Compatibility** | ❌ No | ✅ Yes | ✅ Yes |
| **Speed** | ✅ <1s | ✅ ~2s | ⚠️ ~3s |
| **Gas Fees** | ✅ <$0.0001 | ⚠️ ~$0.01 | ⚠️ ~$0.01 |

### Error Handling

```typescript
// Always implement proper error handling
async function safePaidFetch(url: string, options: any) {
  try {
    return await paidFetch(url, options);
  } catch (error) {
    // Log for debugging
    console.error('Payment failed:', error);

    // User-friendly error
    if (error.code === 'INSUFFICIENT_FUNDS') {
      throw new Error('Please add USDC to your wallet');
    } else if (error.code === 'PAYMENT_TIMEOUT') {
      throw new Error('Payment took too long, please try again');
    } else {
      throw new Error('Payment failed, please contact support');
    }
  }
}
```

### Security

```typescript
// Always set maxAmount
const data = await paidFetch(url, {
  wallet,
  maxAmount: 0.01  // Never pay more than this
});

// Verify merchant wallet before paying
const trustedMerchants = ['Wallet1...', 'Wallet2...'];
if (!trustedMerchants.includes(merchantWallet)) {
  throw new Error('Untrusted merchant');
}

// Rate limiting for paid endpoints
app.use(fareMiddleware({
  price: 0.001,
  merchantWallet: process.env.MERCHANT_WALLET,
  rateLimit: {
    windowMs: 60000,  // 1 minute
    max: 100  // 100 requests per minute
  }
}));
```

---

## Quick Start Commands

```bash
# Pattern 1: Simple Client
npm install @faremeter/fetch @solana/wallet-adapter-phantom

# Pattern 2: Protected API
npm install @faremeter/middleware express

# Pattern 3: Agent Marketplace
npm install @faremeter/fetch @faremeter/middleware express

# Pattern 4: MCP Server
npm install x402-mcp @modelcontextprotocol/sdk

# Pattern 5: Multi-Chain Router
npm install @faremeter/fetch @solana/web3.js ethers
```

---

**Related Docs:**
- [Corbits/Faremeter Tool Guide](../tools/corbits-faremeter-guide.md)
- [PayAI Network Tool Guide](../tools/payai-network-guide.md)
- [Wallet Integration Guide](./wallet-integration-guide.md)
- [Security Best Practices](./security-best-practices.md)
