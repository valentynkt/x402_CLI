# x402-MCP Integration Guide

**Quick Links:** [ethanniser/x402-mcp](https://github.com/ethanniser/x402-mcp) | [Coinbase Example](https://github.com/coinbase/x402/tree/main/examples/typescript/mcp) | [Vercel Blog](https://vercel.com/blog/introducing-x402-mcp-open-protocol-payments-for-mcp-tools) | [mark3labs/mcp-go-x402](https://github.com/mark3labs/mcp-go-x402)
**License:** Open Source (varies by implementation) | **Integration Difficulty:** Medium

## Overview

**x402-MCP** bridges the Model Context Protocol (MCP) with x402 payments, enabling AI agents to autonomously pay for MCP tool access using cryptocurrency micropayments. Multiple implementations exist across TypeScript, Go, and specialized frameworks, making MCP tools monetizable through open web protocols rather than vendor-specific APIs.

## Key Features

- ✅ **MCP-Native:** Direct integration with Model Context Protocol
- ✅ **Multiple Implementations:** TypeScript, Go, middleware options
- ✅ **Vercel AI SDK Support:** Works with Vercel's AI tooling
- ✅ **Client & Server:** Both sides of MCP payment flow
- ✅ **Blockchain Agnostic:** Supports multiple chains via adapters
- ✅ **Open Protocol:** Vendor-neutral payment standard

## Technical Specifications

### Technology Stack
- **Base Protocol:** MCP (Model Context Protocol)
- **Payment Layer:** x402
- **Languages:** TypeScript, Go
- **Frameworks:** Vercel AI SDK, Express, MCP-Go
- **Blockchain Support:** Configurable (Solana, EVM)

### Available Implementations

| Implementation | Language | Use Case | Status |
|----------------|----------|----------|--------|
| **ethanniser/x402-mcp** | TypeScript | Full client/server | Active |
| **Coinbase Official** | TypeScript | Reference implementation | Production |
| **Vercel x402-mcp** | TypeScript | Vercel AI SDK integration | Production |
| **mark3labs/mcp-go-x402** | Go | MCP-Go transport | Active |
| **FlowMCP middleware** | TypeScript | Express middleware | Active |

## Implementation 1: ethanniser/x402-mcp

### Overview
Complete TypeScript implementation with both MCP server and client for creating paid MCP tools.

### Server Setup

```typescript
import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import { x402Server } from 'x402-mcp';

// Create MCP server with x402 integration
const server = new Server(
  {
    name: 'paid-tools-server',
    version: '1.0.0',
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

// Add paid tool
server.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [
    {
      name: 'get_market_data',
      description: 'Get real-time market data (costs 0.01 USDC)',
      inputSchema: {
        type: 'object',
        properties: {
          symbol: { type: 'string' }
        },
        required: ['symbol']
      },
      x402: {
        price: 0.01,
        currency: 'USDC',
        recipient: process.env.MERCHANT_WALLET
      }
    }
  ]
}));

// Handle tool execution with payment verification
server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args, x402Payment } = request.params;

  if (name === 'get_market_data') {
    // Verify payment
    const paymentValid = await verifyPayment(x402Payment);
    if (!paymentValid) {
      throw new Error('Payment required or invalid');
    }

    // Execute tool
    const data = await fetchMarketData(args.symbol);
    return {
      content: [{
        type: 'text',
        text: JSON.stringify(data)
      }]
    };
  }
});

const transport = new StdioServerTransport();
await server.connect(transport);
```

### Client Setup

```typescript
import { Client } from '@modelcontextprotocol/sdk/client/index.js';
import { x402Client } from 'x402-mcp';

const client = new Client({
  name: 'trading-agent',
  version: '1.0.0'
}, {
  capabilities: {}
});

// Configure x402 payment
const paidClient = x402Client(client, {
  wallet: myWallet,
  maxAmount: 0.10
});

// List available tools (includes pricing info)
const { tools } = await paidClient.listTools();

tools.forEach(tool => {
  if (tool.x402) {
    console.log(`${tool.name}: ${tool.x402.price} ${tool.x402.currency}`);
  }
});

// Call paid tool - payment handled automatically
const result = await paidClient.callTool({
  name: 'get_market_data',
  arguments: { symbol: 'SOL' }
});

console.log('Market data:', result.content[0].text);
```

**Installation:**
```bash
npm install x402-mcp @modelcontextprotocol/sdk
```

## Implementation 2: Coinbase Official Example

### Overview
Reference implementation from Coinbase x402 repository demonstrating MCP integration patterns.

### Key Features
- **Official Coinbase Code:** Battle-tested patterns
- **CDP Integration:** Works with Coinbase Developer Platform
- **Complete Examples:** Server and client code
- **SVM Tests:** 6 Solana Virtual Machine test scenarios

### Usage Example

```typescript
import { createMCPServer } from '@coinbase/x402-mcp';

const server = createMCPServer({
  tools: [
    {
      name: 'premium_tool',
      description: 'Paid tool example',
      price: {
        amount: '0.05',
        currency: 'USDC',
        chain: 'solana'
      },
      handler: async (params, paymentProof) => {
        // paymentProof automatically verified
        return processRequest(params);
      }
    }
  ],
  merchantWallet: process.env.MERCHANT_WALLET
});

await server.listen();
```

**Installation:**
```bash
npm install @coinbase/x402-mcp
```

## Implementation 3: Vercel x402-mcp Package

### Overview
Integration with Vercel AI SDK, enabling paid MCP tools in AI applications.

### Key Features
- **Vercel AI SDK Integration:** Seamless with AI SDK
- **Edge Function Ready:** Works in Vercel Edge Runtime
- **Developer Experience:** Simple API design
- **Production Ready:** Used by Vercel projects

### Usage Example

```typescript
import { x402 } from 'x402-mcp';
import { openai } from '@ai-sdk/openai';
import { generateText } from 'ai';

// Define paid MCP tool
const paidTool = x402.tool({
  name: 'analyze_code',
  description: 'AI code analysis (costs 0.10 USDC)',
  price: 0.10,
  currency: 'USDC',
  parameters: z.object({
    code: z.string()
  }),
  execute: async ({ code }, payment) => {
    // Payment already verified by middleware
    return await analyzeCode(code);
  }
});

// Use with Vercel AI SDK
const { text } = await generateText({
  model: openai('gpt-4'),
  tools: {
    analyzeCode: paidTool
  },
  prompt: 'Analyze this code: function foo() { ... }'
});
```

**Installation:**
```bash
npm install x402-mcp ai @ai-sdk/openai
```

## Implementation 4: mark3labs/mcp-go-x402

### Overview
Go implementation for MCP-Go clients and servers with x402 transport layer.

### Usage Example

```go
package main

import (
    "github.com/mark3labs/mcp-go/mcp"
    "github.com/mark3labs/mcp-go-x402/transport"
)

func main() {
    // Create MCP server with x402 transport
    server := mcp.NewServer(
        mcp.ServerInfo{
            Name:    "paid-tools",
            Version: "1.0.0",
        },
    )

    // Add paid tool
    server.AddTool(mcp.Tool{
        Name:        "get_data",
        Description: "Get data (0.01 USDC)",
        InputSchema: map[string]interface{}{
            "type": "object",
            "properties": map[string]interface{}{
                "query": map[string]string{"type": "string"},
            },
        },
        X402: &mcp.X402Config{
            Price:     0.01,
            Currency:  "USDC",
            Recipient: merchantWallet,
        },
        Handler: func(params map[string]interface{}, payment *mcp.Payment) (interface{}, error) {
            // Process paid request
            return processData(params["query"].(string)), nil
        },
    })

    // Start server with x402 transport
    transport := transport.NewX402Transport(
        transport.WithWallet(wallet),
    )

    server.Serve(transport)
}
```

**Installation:**
```bash
go get github.com/mark3labs/mcp-go-x402
```

## Implementation 5: FlowMCP x402 Middleware

### Overview
Express-compatible middleware for protecting MCP endpoints with x402.

### Usage Example

```typescript
import express from 'express';
import { x402Middleware } from '@flowmcp/x402-mcp-middleware';

const app = express();

// Apply x402 protection to MCP endpoints
app.use('/mcp', x402Middleware({
  pricing: {
    'tools/analyze': 0.05,
    'tools/generate': 0.10,
  },
  merchantWallet: process.env.MERCHANT_WALLET,
  chain: 'solana'
}));

// MCP endpoints automatically protected
app.post('/mcp/tools/analyze', (req, res) => {
  // req.x402Payment contains verified payment
  const result = analyze(req.body.params);
  res.json({ result });
});

app.listen(3000);
```

**Installation:**
```bash
npm install @flowmcp/x402-mcp-middleware
```

## Use Cases for Hackathon

### 1. Paid AI Agent Tools Marketplace
**Scenario:** Monetize specialized MCP tools
**Implementation:** Use ethanniser/x402-mcp for tool server
**Revenue Model:** $0.01-$1.00 per tool invocation

### 2. Premium Data Integration
**Scenario:** AI agents access paid data sources
**Implementation:** Vercel x402-mcp + AI SDK
**Revenue Model:** Query-based pricing

### 3. Compute-as-a-Service
**Scenario:** GPU/compute time for AI inference
**Implementation:** MCP-Go x402 for performance
**Revenue Model:** Per-second compute pricing

### 4. Multi-Tool Workflows
**Scenario:** Complex workflows using multiple paid tools
**Implementation:** MCP orchestrator with x402
**Revenue Model:** Workflow-based pricing

## Integration Difficulty Breakdown

### Easy ✅
- Vercel x402-mcp (if using AI SDK)
- FlowMCP middleware (Express apps)
- Clear documentation across implementations
- MCP protocol is well-standardized

### Medium ⚠️
- ethanniser implementation (full control)
- Coinbase official (more configuration)
- Understanding MCP protocol
- Payment verification logic

### Advanced 🔧
- MCP-Go implementation
- Custom transport layers
- Multi-chain support
- Advanced error handling

## Unique Selling Points

1. **MCP-Native:** Only x402 solution purpose-built for MCP
2. **Multiple Languages:** TypeScript and Go options
3. **Framework Integration:** Vercel AI SDK, Express, MCP-Go
4. **Open Protocol:** Vendor-neutral vs proprietary payment APIs
5. **Production Ready:** Multiple battle-tested implementations
6. **Active Ecosystem:** Growing MCP + x402 community

## When to Choose x402-MCP

**✅ Choose x402-MCP if you:**
- Building with Model Context Protocol
- Want to monetize MCP tools
- Using Vercel AI SDK
- Need Go implementation (high performance)
- Want vendor-neutral payments
- Building AI agent tool marketplaces

**❌ Consider alternatives if you:**
- Not using MCP (use standard x402)
- Want simpler HTTP API payments (use Corbits)
- Need fastest integration (use PayAI)
- Prefer all-in-one solution (use MCPay.tech)

## Documentation Quality: MEDIUM-HIGH

**Available Resources:**
- **ethanniser/x402-mcp:** GitHub README, code examples
- **Coinbase Example:** Complete reference implementation
- **Vercel Blog:** Integration guide, best practices
- **MCP-Go:** Go package documentation
- **FlowMCP:** Middleware usage guide

**GitHub:**
- Multiple active repositories
- Code examples available
- Community-driven support
- Regular updates

## Community & Support

**Channels:**
- **MCP Discord:** #payments channel
- **GitHub Issues:** Per-implementation support
- **Vercel Community:** For Vercel AI SDK integration
- **Model Context Protocol:** https://modelcontextprotocol.io

**Response Time:** Community-driven, varies by repo

## Quick Start Checklist

- [ ] Choose implementation (TypeScript, Go, or Vercel)
- [ ] Review MCP protocol basics
- [ ] Set up wallet (Solana or EVM)
- [ ] Install chosen package
- [ ] Define MCP tools with pricing
- [ ] Implement payment verification
- [ ] Test with MCP client (Claude Desktop, etc.)
- [ ] Deploy MCP server
- [ ] Register in MCP directory (optional)

## Code Example: Complete MCP Paid Tool

```typescript
import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import { verifyX402Payment } from 'x402-mcp';

// === SERVER: Paid MCP Tool ===
const server = new Server({
  name: 'market-data-pro',
  version: '1.0.0',
}, {
  capabilities: { tools: {} },
});

// List tools with pricing
server.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [
    {
      name: 'get_price',
      description: 'Real-time price (0.01 USDC)',
      inputSchema: {
        type: 'object',
        properties: {
          symbol: { type: 'string' }
        }
      },
      x402: {
        price: 0.01,
        currency: 'USDC',
        chain: 'solana',
        recipient: process.env.MERCHANT_WALLET
      }
    },
    {
      name: 'get_analysis',
      description: 'AI market analysis (0.10 USDC)',
      inputSchema: {
        type: 'object',
        properties: {
          symbol: { type: 'string' },
          timeframe: { type: 'string' }
        }
      },
      x402: {
        price: 0.10,
        currency: 'USDC',
        chain: 'solana',
        recipient: process.env.MERCHANT_WALLET
      }
    }
  ]
}));

// Execute tools with payment verification
server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args, x402Payment } = request.params;

  // Verify payment on-chain
  const payment = await verifyX402Payment(x402Payment);
  if (!payment.verified) {
    throw new Error('Payment verification failed');
  }

  // Check payment matches tool price
  const tool = (await server.getTools()).find(t => t.name === name);
  if (payment.amount < tool.x402.price) {
    throw new Error('Insufficient payment');
  }

  // Execute tool
  if (name === 'get_price') {
    const price = await fetchPrice(args.symbol);
    return {
      content: [{
        type: 'text',
        text: JSON.stringify({ symbol: args.symbol, price })
      }]
    };
  }

  if (name === 'get_analysis') {
    const analysis = await generateAnalysis(args.symbol, args.timeframe);
    return {
      content: [{
        type: 'text',
        text: analysis
      }]
    };
  }
});

await server.connect(new StdioServerTransport());

// === CLIENT: AI Agent Using Paid Tools ===
import { Client } from '@modelcontextprotocol/sdk/client/index.js';
import { createX402Client } from 'x402-mcp';

const client = new Client({
  name: 'trading-bot',
  version: '1.0.0'
}, {
  capabilities: {}
});

// Wrap with x402 payment capability
const paidClient = createX402Client(client, {
  wallet: botWallet,
  maxAmount: 0.50, // Max $0.50 per tool call
  chain: 'solana'
});

// Discover tools
const { tools } = await paidClient.listTools();

console.log('Available paid tools:');
tools.forEach(tool => {
  console.log(`- ${tool.name}: $${tool.x402.price} ${tool.x402.currency}`);
});

// Call paid tools
const priceData = await paidClient.callTool({
  name: 'get_price',
  arguments: { symbol: 'SOL' }
});

const analysis = await paidClient.callTool({
  name: 'get_analysis',
  arguments: { symbol: 'SOL', timeframe: '1d' }
});

console.log('Price:', JSON.parse(priceData.content[0].text));
console.log('Analysis:', analysis.content[0].text);
```

## Hackathon Tips

### Prize Track Alignment
- **Best x402 Agent Application** ✅✅ (MCP angle is unique)
- **Best Agent Money Protocol** ✅✅ (Tool marketplace narrative)
- **CDP Integration** ✅ (Coinbase implementation available)

### Competitive Advantages
1. **MCP Ecosystem:** Tap into growing tool protocol
2. **Claude Desktop Compatible:** Works with Claude's MCP support
3. **Multiple Implementations:** Show technical depth
4. **AI Agent Focus:** Perfect narrative for agent hackathon
5. **Open Protocol:** Standards-based approach

### Integration Time
- **Vercel AI SDK:** 2-4 hours (fastest)
- **ethanniser/x402-mcp:** 4-8 hours
- **MCP-Go:** 6-12 hours
- **Custom Implementation:** 12+ hours

### Demo Strategy
1. Show MCP tool discovery with pricing
2. Demonstrate Claude Desktop integration (if possible)
3. Highlight multiple paid tools in workflow
4. Emphasize open protocol vs vendor lock-in
5. Show tool marketplace potential

---

**Related Docs:**
- [x402 Protocol Specification](../x402-protocol-specification.md)
- [MCP Protocol Documentation](https://modelcontextprotocol.io)
- [SDK Comparison Reference](../reference/sdk-comparison.md)
- [Code Repositories](../reference/code-repositories.md)
