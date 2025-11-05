# SDK Comparison Reference

Quick reference guide for choosing the right SDK/library for your x402 implementation.

## Comparison Matrix

| SDK/Tool | License | Language | Chains | Best For | Difficulty |
|----------|---------|----------|--------|----------|------------|
| **Faremeter** | LGPL-3.0 | TypeScript | 3+ | Open-source, control | Medium ⭐⭐ |
| **CDP SDK** | Proprietary | TS, Python | 4 | Enterprise, official | Easy ⭐ |
| **PayAI SDK** | Proprietary | JavaScript | 7 | Quick start, multi-chain | Easy ⭐ |
| **Crossmint** | Proprietary | TypeScript | 15+ | Enterprise commerce | Medium ⭐⭐ |
| **MCPay.tech** | Open Source | JavaScript | EVM+Solana | MCP monetization | Easy ⭐ |
| **x402-mcp** | Open Source | TS, Go | Agnostic | MCP protocol integration | Medium ⭐⭐ |
| **ACK Protocol** | Open Source | TypeScript | Solana | Identity + receipts | Medium ⭐⭐ |
| **Google A2A x402** | Open Source | TS, Python, Go | EVM (Solana dev) | Agent-to-agent commerce | Medium-Advanced ⭐⭐⭐ |
| **Nexus (Thirdweb)** | Commercial | TypeScript | 26+ | Fastest integration | Easy ⭐ |
| **Native Example** | Open Source | JavaScript | Solana | Learning, customization | Advanced ⭐⭐⭐ |

---

## 1. Faremeter (Corbits)

**GitHub:** https://github.com/faremeter
**Docs:** https://docs.corbits.dev
**License:** LGPL-3.0 (Open Source)

### Quick Install
```bash
npm install @faremeter/fetch @faremeter/middleware
```

### Client Usage
```typescript
import { paidFetch } from '@faremeter/fetch';
const data = await paidFetch(url, { wallet, maxAmount: 0.01 });
```

### Server Usage
```typescript
import { fareMiddleware } from '@faremeter/middleware';
app.use(fareMiddleware({ price: 0.001, merchantWallet }));
```

### When to Choose
- ✅ Want open-source solution
- ✅ Need full code control
- ✅ Self-hosting preferred
- ✅ Solana-first
- ❌ Not for quick prototypes

---

## 2. Coinbase CDP SDK

**Docs:** https://docs.cdp.coinbase.com/
**GitHub:** https://github.com/coinbase/cdp-sdk
**License:** Proprietary

### Quick Install
```bash
npm install @coinbase/cdp-sdk
```

### Usage
```typescript
import { Coinbase, Wallet } from '@coinbase/cdp-sdk';
const wallet = await Wallet.create();
const payment = await wallet.createX402Payment({
  amount: '0.001',
  token: 'USDC',
  network: 'base'
});
```

### When to Choose
- ✅ Want official Coinbase SDK
- ✅ Base/Ethereum focus
- ✅ Enterprise support needed
- ✅ Embedded wallets
- ❌ Not for Solana-first

---

## 3. PayAI SDK

**Docs:** https://docs.payai.network/
**GitHub:** https://github.com/PayAINetwork
**License:** Proprietary

### Quick Install
```bash
npm install @payai/sdk
```

### Usage
```typescript
import { PayAIClient } from '@payai/sdk';
const client = new PayAIClient({ network: 'solana' });
const result = await client.pay({ amount: '0.001', recipient });
```

### When to Choose
- ✅ Need fastest integration
- ✅ Multi-chain support (7 chains)
- ✅ Network fees covered
- ✅ Free tier available
- ❌ Not for full customization

---

## 4. Crossmint SDK

**Docs:** https://docs.crossmint.com
**License:** Proprietary

### Quick Install
```bash
npm install @crossmint/client-sdk
```

### Usage
```typescript
import { CrossmintClient } from '@crossmint/client-sdk';
const crossmint = new CrossmintClient({ apiKey });
const wallet = await crossmint.wallets.create({ chain: 'solana' });
```

### When to Choose
- ✅ Enterprise applications
- ✅ Need 15+ chains
- ✅ Traditional commerce (Amazon, Shopify)
- ✅ Multi-protocol (x402 + Visa + Mastercard)
- ❌ Not for simple projects

---

## 5. MCPay.tech

**Website:** https://mcpay.tech
**GitHub:** https://github.com/microchipgnu/MCPay
**License:** Open Source

### Quick Install
```bash
npm install mcpay
```

### Server Usage
```javascript
import { createMcpPaidHandler } from "mcpay/handler";
export const paidMcp = createMcpPaidHandler(
  async (server) => {
    server.paidTool("hello", "pay for hello", "$0.001", {},
      async ({}) => ({ content: [{type: 'text', text: 'Hello!'}] })
    );
  },
  { recipient: { evm: { address: '0x...' } } }
);
```

### When to Choose
- ✅ Building MCP servers
- ✅ Want per-tool pricing
- ✅ Need EVM + Solana support
- ✅ Hackathon winner tech
- ❌ Not for non-MCP APIs

---

## 6. x402-mcp (Multiple Implementations)

**ethanniser:** https://github.com/ethanniser/x402-mcp
**Coinbase Example:** https://github.com/coinbase/x402/tree/main/examples/typescript/mcp
**Vercel Package:** https://vercel.com/blog/introducing-x402-mcp-open-protocol-payments-for-mcp-tools
**MCP-Go:** https://github.com/mark3labs/mcp-go-x402
**License:** Open Source (varies)

### Quick Install
```bash
# TypeScript
npm install x402-mcp

# Go
go get github.com/mark3labs/mcp-go-x402

# Vercel AI SDK
npm install x402-mcp ai
```

### Usage (ethanniser)
```typescript
import { createX402Client } from 'x402-mcp';
const paidClient = createX402Client(mcpClient, {
  wallet: myWallet,
  maxAmount: 0.50
});
const result = await paidClient.callTool({
  name: 'get_price',
  arguments: { symbol: 'SOL' }
});
```

### When to Choose
- ✅ Multiple implementation options
- ✅ TypeScript or Go
- ✅ Vercel AI SDK integration
- ✅ MCP protocol focus
- ❌ Not for simple HTTP APIs

---

## 7. ACK Protocol

**GitHub:** https://github.com/agentcommercekit/ack
**Live Demo:** https://solana-paywal.vercel.app/
**License:** Open Source

### Overview
Extends x402 with verifiable agent identity (DIDs) and cryptographic receipts (VCs).

### Usage
```typescript
import { createAgentDID, ACKClient } from 'ack-protocol';

// Create agent identity
const agentDID = await createAgentDID({
  name: "My Trading Agent",
  capabilities: ["market_data"]
});

// Make payment with identity
const client = new ACKClient({ agentDID, wallet });
const response = await client.fetch(url, { receiptRequired: true });
```

### When to Choose
- ✅ Need agent identity verification
- ✅ Require payment receipts
- ✅ Building reputation systems
- ✅ Compliance/audit trails
- ✅ Solana-first
- ❌ Not if identity overkill

---

## 8. Google A2A x402

**GitHub:** https://github.com/google-agentic-commerce/a2a-x402
**License:** Open Source
**Partners:** Coinbase, Ethereum Foundation, MetaMask

### Overview
Agent-to-Agent payment extension for Google's A2A protocol.

### Quick Install
```bash
npm install @google-agentic-commerce/a2a-x402
```

### Usage
```typescript
import { A2AClient } from '@google-agentic-commerce/a2a-x402';

const client = new A2AClient({
  agentId: 'my-trading-agent',
  wallet: solanaKeypair
});

// Call another agent's service (payment automatic)
const response = await client.callAgent({
  agentId: 'market-data-agent',
  method: 'getPriceData',
  params: { symbol: 'SOL' }
});
```

### When to Choose
- ✅ Agent-to-agent marketplace
- ✅ Google ecosystem integration
- ✅ Multi-language support (TS/Python/Go)
- ✅ Enterprise credibility
- ⚠️ Solana support in progress
- ❌ Not for simple APIs

---

## 9. Nexus (Thirdweb)

**Website:** https://nexus.thirdweb.com/
**Docs:** https://portal.thirdweb.com/payments/x402
**License:** Commercial (Free tier)

### Quick Install
```bash
npm install thirdweb
```

### Client Usage
```typescript
import { wrapFetchWithPayment } from "thirdweb/x402";
import { createThirdwebClient } from "thirdweb";

const client = createThirdwebClient({ clientId: THIRDWEB_CLIENT_ID });
const paidFetch = wrapFetchWithPayment({ client, wallet, maxAmount: 0.25 });

// Use like normal fetch - 402 payments automatic
const response = await paidFetch('https://api.example.com/data');
```

### When to Choose
- ✅ Fastest integration (1 function call)
- ✅ Broadest chain support (26+)
- ✅ Global edge infrastructure
- ✅ Embedded wallet support
- ✅ No server infrastructure needed
- ⚠️ Solana coming (target Oct 30, 2025)
- ❌ Not for self-hosting

---

## 10. Native Example

**GitHub:** https://github.com/Woody4618/x402-solana-examples
**License:** Open Source

### Overview
Minimal x402 implementation without SDKs, using only Express and Solana Web3.js.

### Server (Minimal)
```javascript
app.get('/api/data', async (req, res) => {
  const sig = req.headers['x-payment-signature'];
  if (!sig) {
    return res.status(402).json({
      price: 0.01,
      recipient: MERCHANT_WALLET,
      currency: 'USDC'
    });
  }
  const tx = await connection.getTransaction(sig);
  // Verify transaction...
  res.json({ data: 'Your data' });
});
```

### Client (Minimal)
```javascript
const res1 = await fetch(url);
if (res1.status === 402) {
  const payment = await res1.json();
  // Construct USDC transfer...
  const sig = await sendAndConfirmTransaction(connection, tx, [wallet]);
  const res2 = await fetch(url, {
    headers: { 'X-Payment-Signature': sig }
  });
}
```

### When to Choose
- ✅ Learning x402 internals
- ✅ Need full control/customization
- ✅ Minimal dependencies
- ✅ Custom payment logic
- ❌ Not for production (high risk)
- ❌ Not for fast development

---

## Feature Comparison

| Feature | Faremeter | CDP | PayAI | Crossmint | MCPay | x402-mcp | ACK | A2A | Nexus | Native |
|---------|-----------|-----|-------|-----------|-------|----------|-----|-----|-------|--------|
| **Solana** | ✅ Primary | ✅ Yes | ✅ Primary | ✅ Yes | ✅ Yes | ✅ Agnostic | ✅ Primary | ⏳ Dev | ⏳ Soon | ✅ Primary |
| **Base** | ✅ Yes | ✅ Primary | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Agnostic | ❌ No | ✅ Yes | ✅ Yes | ❌ No |
| **10+ Chains** | ❌ No | ❌ No | ✅ Yes | ✅ Yes | ❌ No | ✅ Agnostic | ❌ No | ⚠️ EVM | ✅ 26+ | ❌ No |
| **Open Source** | ✅ Yes | ❌ No | ❌ No | ❌ No | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ❌ No | ✅ Yes |
| **Free Tier** | ✅ Yes | ⚠️ Usage | ✅ Yes | ❌ No | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |
| **Self-Host** | ✅ Yes | ❌ No | ❌ No | ❌ No | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ❌ No | ✅ Yes |
| **MCP Native** | ❌ No | ❌ No | ❌ No | ❌ No | ✅ Yes | ✅ Yes | ❌ No | ❌ No | ❌ No | ❌ No |
| **Identity/Receipts** | ❌ No | ❌ No | ❌ No | ❌ No | ❌ No | ❌ No | ✅ Yes | ⚠️ Partial | ❌ No | ❌ No |
| **Multi-Language** | ❌ TS only | ✅ TS/Py | ❌ JS only | ❌ TS only | ❌ JS only | ✅ TS/Go | ❌ TS only | ✅ TS/Py/Go | ❌ TS only | ❌ JS only |
| **Embedded Wallets** | ❌ No | ✅ Yes | ❌ No | ✅ Yes | ❌ No | ❌ No | ❌ No | ❌ No | ✅ Yes | ❌ No |

---

## Network Support

| Network | Faremeter | CDP | PayAI | Crossmint | MCPay | ACK | A2A | Nexus | Native |
|---------|-----------|-----|-------|-----------|-------|-----|-----|-------|--------|
| **Solana** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ⏳ | ⏳ | ✅ |
| **Base** | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ❌ |
| **Polygon** | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ❌ |
| **Ethereum** | ❌ | ✅ | ❌ | ✅ | ✅ | ❌ | ✅ | ✅ | ❌ |
| **Avalanche** | ❌ | ❌ | ✅ | ✅ | ⚠️ | ❌ | ⚠️ | ✅ | ❌ |
| **Arbitrum** | ❌ | ✅ | ❌ | ✅ | ⚠️ | ❌ | ✅ | ✅ | ❌ |
| **Optimism** | ❌ | ❌ | ❌ | ✅ | ⚠️ | ❌ | ✅ | ✅ | ❌ |
| **Sei** | ❌ | ❌ | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **IoTeX** | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **Peaq** | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **20+ Chains** | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ | ✅ 26+ | ❌ |
| **Others** | Via plugins | ❌ | ❌ | 5+ more | EVM | ❌ | EVM | Many | Custom |

---

## Decision Tree

```
Learning x402 protocol internals?
├─ YES → Native Example (educational)
└─ NO ↓

Building MCP servers?
├─ YES → MCPay.tech or x402-mcp
│   ├─ Want per-tool pricing → MCPay.tech
│   └─ Want multiple implementations → x402-mcp
└─ NO ↓

Need agent identity + payment receipts?
├─ YES → ACK Protocol (Solana)
└─ NO ↓

Building agent-to-agent marketplace?
├─ YES → Google A2A x402
└─ NO ↓

Need 20+ blockchains?
├─ YES → Nexus (26+) or Crossmint (15+)
│   ├─ Want fastest integration → Nexus
│   └─ Need enterprise + multi-protocol → Crossmint
└─ NO ↓

Need open-source/self-hosted?
├─ YES → Faremeter (production) or Native (custom)
└─ NO ↓

Solana-first project?
├─ YES → PayAI or Faremeter
│   ├─ Want hosted solution → PayAI
│   └─ Want self-hosted → Faremeter
└─ NO ↓

Need official Coinbase support?
├─ YES → CDP SDK
└─ NO ↓

Want absolute fastest integration?
└─ Nexus (1 line: wrapFetchWithPayment)
```

---

## Installation Commands

```bash
# Faremeter
npm install @faremeter/fetch @faremeter/middleware

# CDP SDK
npm install @coinbase/cdp-sdk

# PayAI
npm install @payai/sdk

# Crossmint
npm install @crossmint/client-sdk

# MCPay.tech
npm install mcpay

# x402-mcp (TypeScript)
npm install x402-mcp

# x402-mcp (Go)
go get github.com/mark3labs/mcp-go-x402

# ACK Protocol
npm install ack-protocol

# Google A2A x402
npm install @google-agentic-commerce/a2a-x402

# Nexus (Thirdweb)
npm install thirdweb

# Native Example (clone repository)
git clone https://github.com/Woody4618/x402-solana-examples

# Common wallet adapters
npm install @solana/wallet-adapter-phantom
npm install @solana/web3.js @solana/spl-token
```

---

## Quick Start Comparison

### Faremeter (3 lines)
```typescript
import { paidFetch } from '@faremeter/fetch';
const wallet = new PhantomWalletAdapter();
const data = await paidFetch(url, { wallet, maxAmount: 0.01 });
```

### CDP SDK (4 lines)
```typescript
import { Coinbase, Wallet } from '@coinbase/cdp-sdk';
const coinbase = new Coinbase({ apiKeyName, privateKey });
const wallet = await Wallet.create();
const payment = await wallet.createX402Payment({ amount, recipient });
```

### PayAI (3 lines)
```typescript
import { PayAIClient } from '@payai/sdk';
const client = new PayAIClient({ network: 'solana' });
const result = await client.pay({ amount, recipient });
```

### Crossmint (4 lines)
```typescript
import { CrossmintClient } from '@crossmint/client-sdk';
const crossmint = new CrossmintClient({ apiKey });
const wallet = await crossmint.wallets.create({ chain: 'solana' });
const tx = await wallet.send({ to, amount });
```

---

**Related Docs:**

**Tool Guides:**
- [Corbits/Faremeter Guide](../tools/corbits-faremeter-guide.md)
- [PayAI Network Guide](../tools/payai-network-guide.md)
- [Crossmint Enterprise Guide](../tools/crossmint-enterprise-guide.md)
- [MCPay.tech Guide](../tools/mcpay-tech-guide.md)
- [x402-MCP Integration Guide](../tools/x402-mcp-guide.md)
- [ACK Protocol Guide](../tools/ack-protocol-guide.md)
- [Google A2A x402 Guide](../tools/google-a2a-x402-guide.md)
- [Nexus (Thirdweb) Guide](../tools/nexus-thirdweb-guide.md)
- [Native Example Guide](../tools/native-example-guide.md)
- [x402scan Explorer Guide](../tools/x402scan-explorer-guide.md)

**Other References:**
- [Integration Patterns](../guides/integration-patterns.md)
- [x402 Protocol Specification](../x402-protocol-specification.md)
- [Code Repositories](./code-repositories.md)
