# Gradient Parallax Integration Guide

**Prize:** $5,000
**Sponsor:** Gradient Network
**Funding:** $10M from Pantera, Multicoin, HSG
**Network Scale:** 1.6B+ connections across 190+ regions
**Difficulty:** High
**Est. Integration Time:** 8-16 hours

---

## Overview

Gradient Parallax is a decentralized AI inference and communication protocol enabling AI agents to perform distributed computation across a global network of 1.6B+ connections. Built on Solana blockchain with $10M funding from top crypto VCs, Gradient combines **Parallax** (distributed AI inference) and **Lattica** (P2P agent communication).

### Key Benefits

- **Distributed AI Inference:** Run large models across decentralized network
- **Privacy-Preserving:** Compute without exposing raw data
- **Massive Scale:** 1.6B+ connections in 190+ regions worldwide
- **Solana Native:** Built on Solana for fast, cheap transactions
- **Dual Protocols:** Parallax (compute) + Lattica (communication)
- **Strong Backing:** $10M from Pantera, Multicoin, Hack VC

### Why Use Gradient?

$5,000 prize for Gradient integration. Cutting edge tech (2025) with minimal competition. Most teams won't attempt distributed AI—this is your differentiator. Multi-prize eligible with Solana track prizes. Decentralized AI is the next frontier.

---

## What is Gradient Parallax?

Current AI agent limitations: centralized compute (agents rely on single cloud providers), privacy risks (send sensitive data to third-party APIs), cost at scale (large model inference is expensive), no collaboration (agents can't easily communicate peer-to-peer), and censorship (centralized providers can block agents).

### The Gradient Solution

Gradient provides **two complementary protocols**:

#### 1. Parallax (Distributed AI Inference)

Split AI model inference across decentralized nodes:
- **Model Sharding:** Large models split across multiple nodes
- **Privacy-Preserving:** Each node sees only fragments
- **Cost Reduction:** Pay only for resources used
- **Censorship Resistance:** No single point of control

#### 2. Lattica (P2P Agent Communication)

Enable direct agent-to-agent communication:
- **Peer Discovery:** Find other agents offering services
- **Message Routing:** Efficient P2P message delivery
- **Encrypted Channels:** Secure agent collaboration
- **Reputation System:** Trust scoring for agent interactions

### Gradient vs Traditional AI Infrastructure

| Feature | Gradient | OpenAI API | Centralized Hosting |
|---------|----------|------------|---------------------|
| **Decentralization** | Fully distributed | Centralized | Centralized |
| **Privacy** | Fragment-based | Full data sent | Full data sent |
| **Censorship** | Resistant | Vulnerable | Vulnerable |
| **Cost** | Pay-per-compute | Fixed pricing | Fixed pricing |
| **Agent P2P** | Native (Lattica) | No | No |
| **Blockchain** | Solana-native | None | None |

---

## Technical Architecture

### Parallax: Distributed AI Inference

**How It Works:** Large models (e.g., LLaMA 70B) are sharded into parts, each shard processed by different nodes across regions, partial computations aggregated to produce final output.

**Benefits:**
- Run 70B parameter models without 70B GPU
- Privacy: No single node sees full input/output
- Cost: Pay only nodes actually used
- Speed: Parallel processing across nodes

### Lattica: P2P Agent Communication

**Communication Patterns:**

**1. Direct Messaging:**
```javascript
await lattica.send({
  to: 'agent-xyz-123',
  message: {
    type: 'data-request',
    payload: { symbol: 'SOL' }
  },
  encrypted: true
});
```

**2. Broadcast:**
```javascript
await lattica.broadcast({
  channel: 'trading-signals',
  message: { signal: 'buy', confidence: 0.9 }
});
```

**3. Request-Response:**
```javascript
const response = await lattica.request({
  to: 'oracle-agent',
  request: { action: 'get-price', symbol: 'SOL' },
  timeout: 5000
});
```

### Solana Integration

Gradient uses Solana for payment settlement, proof of computation, reputation tracking, and agent registry.

---

## Developer Resources

### Official Resources (2025)

**Documentation:**
- Main Docs: https://docs.gradient.network
- Parallax Guide: https://docs.gradient.network/parallax
- Lattica Guide: https://docs.gradient.network/lattica
- API Reference: https://docs.gradient.network/api

**Company:**
- Website: https://gradient.network
- GitHub: https://github.com/gradient-network
- Email: developers@gradient.network

**Community:**
- Discord: https://discord.gg/gradient-network
- Twitter: @gradientnetwork

**Network Status:**
- Explorer: https://explorer.gradient.network
- Stats: https://stats.gradient.network

### SDK Installation

```bash
# Core Gradient SDK
npm install @gradient/sdk

# Parallax (distributed compute)
npm install @gradient/parallax

# Lattica (P2P messaging)
npm install @gradient/lattica

# Solana integration
npm install @solana/web3.js @solana/spl-token
```

---

## Step-by-Step Integration

### Prerequisites

- Node.js 18+ development environment
- Solana wallet with SOL/USDC
- Understanding of distributed systems
- Gradient Network account (register at gradient.network)

### Step 1: Register with Gradient

Contact Gradient Network for developer access:

```bash
GRADIENT_API_KEY=gn_api_abc123xyz
GRADIENT_API_SECRET=gn_secret_def456uvw
```

### Step 2: Initialize Gradient SDK

```javascript
const { Gradient } = require('@gradient/sdk');
const { Connection, Keypair } = require('@solana/web3.js');
const fs = require('fs');

const secretKey = JSON.parse(fs.readFileSync('wallet.json'));
const wallet = Keypair.fromSecretKey(Uint8Array.from(secretKey));

const connection = new Connection(
  process.env.SOLANA_RPC_URL || 'https://api.mainnet-beta.solana.com',
  'confirmed'
);

const gradient = new Gradient({
  apiKey: process.env.GRADIENT_API_KEY,
  apiSecret: process.env.GRADIENT_API_SECRET,
  wallet: wallet,
  connection: connection,
  network: 'mainnet'
});

async function verifyConnection() {
  try {
    const status = await gradient.getNetworkStatus();
    console.log('Connected to Gradient Network');
    console.log('Active nodes:', status.activeNodes);
    console.log('Total connections:', status.totalConnections);
    return true;
  } catch (error) {
    console.error('Connection failed:', error);
    return false;
  }
}

verifyConnection();
```

### Step 3: Setup Parallax

```javascript
const { Parallax } = require('@gradient/parallax');

const parallax = new Parallax({
  gradient: gradient,
  wallet: wallet,
  connection: connection,
  preferences: {
    minNodes: 4,
    maxNodes: 16,
    preferredRegions: ['us-west', 'eu-central', 'asia-pacific'],
    privacy: 'high',
    maxCostPerInference: 0.10
  }
});

async function testInference() {
  try {
    const result = await parallax.infer({
      model: 'llama-3-8b',
      prompt: 'What is the capital of France?',
      maxTokens: 50
    });

    console.log('Inference result:', result.output);
    console.log('Cost:', result.cost, 'USDC');
    console.log('Nodes used:', result.nodesUsed);
    console.log('Latency:', result.latency, 'ms');

  } catch (error) {
    console.error('Inference failed:', error);
  }
}

testInference();
```

### Step 4: Setup Lattica

```javascript
const { Lattica } = require('@gradient/lattica');

const lattica = new Lattica({
  gradient: gradient,
  wallet: wallet,
  agentId: 'my-agent-123',
  agentName: 'Trading Agent Alpha',
  agentType: 'trading',
  capabilities: ['market-analysis', 'price-alerts', 'trading-signals'],
  trackReputation: true
});

lattica.on('message', (from, message) => {
  console.log(`Message from ${from}:`, message);
});

async function connectLattica() {
  try {
    await lattica.connect();
    console.log('Connected to Lattica network');
    console.log('Agent ID:', lattica.agentId);
    console.log('Peer count:', lattica.peerCount);
  } catch (error) {
    console.error('Connection failed:', error);
  }
}

connectLattica();
```

### Step 5: Distributed Inference Example

```javascript
async function analyzeMarketData(marketData) {
  console.log('Starting distributed market analysis...');

  try {
    const result = await parallax.infer({
      model: 'llama-70b',
      prompt: `
        Analyze the following market data and provide insights:
        ${JSON.stringify(marketData, null, 2)}

        Provide trend analysis, key patterns, risk assessment, and trading recommendations.
      `,
      maxTokens: 500,
      temperature: 0.7,
      shards: 8,  // Split across 8 nodes
      privacy: 'high',
      maxCost: 0.15
    });

    console.log('\n=== Analysis Results ===');
    console.log(result.output);
    console.log('\n=== Compute Details ===');
    console.log('Nodes used:', result.nodesUsed);
    console.log('Regions:', result.regions);
    console.log('Cost:', result.cost, 'USDC');
    console.log('Latency:', result.latency, 'ms');
    console.log('Privacy score:', result.privacyScore);

    return result;

  } catch (error) {
    console.error('Analysis failed:', error);
    throw error;
  }
}
```

### Step 6: P2P Communication Example

```javascript
async function setupAgentCommunication() {
  console.log('Discovering other agents...');

  const agents = await lattica.discover({
    type: 'oracle',
    capabilities: ['price-feeds'],
    minReputation: 0.8,
    maxDistance: 3
  });

  console.log(`Found ${agents.length} oracle agents`);

  if (agents.length > 0) {
    const oracleAgent = agents[0];
    console.log(`\nRequesting price from ${oracleAgent.id}...`);

    const response = await lattica.request({
      to: oracleAgent.id,
      request: {
        type: 'get-price',
        symbol: 'SOL',
        timestamp: Date.now()
      },
      timeout: 10000,
      retries: 3
    });

    console.log('Oracle response:', response);

    // Subscribe to trading signals channel
    await lattica.subscribe({
      channel: 'trading-signals',
      filter: {
        symbol: 'SOL',
        confidence: { $gte: 0.85 }
      }
    });

    lattica.on('channel-message', (channel, message) => {
      if (channel === 'trading-signals') {
        console.log('Trading signal received:', message);
      }
    });
  }
}

setupAgentCommunication();
```

---

## Code Examples

### Multi-Agent Trading System

```javascript
const { parallax } = require('./parallax-setup');
const { lattica } = require('./lattica-setup');

class DistributedTradingSystem {
  constructor() {
    this.signals = [];
    this.setupLattica();
  }

  setupLattica() {
    lattica.subscribe({ channel: 'market-data' });

    lattica.on('channel-message', async (channel, message) => {
      if (channel === 'market-data') {
        await this.analyzeMarketData(message);
      }
    });
  }

  async analyzeMarketData(data) {
    console.log(`Analyzing market data for ${data.symbol}...`);

    const analysis = await parallax.infer({
      model: 'llama-70b',
      prompt: `Given this market data: ${JSON.stringify(data)}\n\nShould I buy, sell, or hold? Provide reasoning and confidence score.`,
      maxTokens: 200,
      shards: 6,
      privacy: 'high'
    });

    console.log('AI Analysis:', analysis.output);

    const decision = this.parseDecision(analysis.output);

    if (decision.confidence > 0.85) {
      await lattica.publish({
        channel: 'trading-signals',
        message: {
          symbol: data.symbol,
          action: decision.action,
          confidence: decision.confidence,
          reasoning: decision.reasoning,
          timestamp: Date.now(),
          computeCost: analysis.cost
        }
      });

      console.log(`Published ${decision.action} signal with ${decision.confidence} confidence`);
    }
  }

  parseDecision(text) {
    const action = text.toLowerCase().includes('buy') ? 'buy' :
                   text.toLowerCase().includes('sell') ? 'sell' : 'hold';

    const confidenceMatch = text.match(/confidence[:\s]+(\d+\.?\d*)/i);
    const confidence = confidenceMatch ? parseFloat(confidenceMatch[1]) : 0.5;

    return { action, confidence, reasoning: text };
  }

  async start() {
    console.log('Distributed trading system started');
    await lattica.connect();

    const peers = await lattica.discover({
      type: 'trading',
      minReputation: 0.7
    });

    console.log(`Connected to ${peers.length} peer trading agents`);
  }
}

const system = new DistributedTradingSystem();
system.start();
```

---

## Lattica P2P Communication

### Agent Discovery

```javascript
// Discover agents by type
const tradingAgents = await lattica.discover({
  type: 'trading',
  capabilities: ['signal-generation'],
  minReputation: 0.8,
  online: true
});

// Discover nearby agents (low latency)
const nearbyAgents = await lattica.discover({
  maxDistance: 2,
  preferredRegions: ['us-west']
});
```

### Reputation System

```javascript
// Rate an agent
await lattica.rate({
  agent: 'oracle-123',
  rating: 5,
  category: 'accuracy',
  comment: 'Very accurate price data'
});

// Get agent reputation
const reputation = await lattica.getReputation('oracle-123');
console.log('Overall score:', reputation.score);  // 0-1
console.log('Total ratings:', reputation.count);
```

---

## Solana Integration

### Payment for Compute

```javascript
async function payForCompute(taskId, amount) {
  const transaction = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: new PublicKey(gradientTreasuryAddress),
      lamports: amount * 1e9
    })
  );

  transaction.add(
    new TransactionInstruction({
      keys: [],
      programId: new PublicKey(MEMO_PROGRAM_ID),
      data: Buffer.from(`gradient-compute-${taskId}`)
    })
  );

  const signature = await connection.sendTransaction(transaction, [wallet]);
  await connection.confirmTransaction(signature);

  console.log('Payment confirmed:', signature);
  return signature;
}

const result = await parallax.infer({
  model: 'llama-70b',
  prompt: 'Analyze...',
  paymentSignature: await payForCompute('task-123', 0.05)
});
```

---

## Use Cases for Hackathon

### 1. Distributed AI Trading Network

**Concept:** Network of agents that collaboratively analyze markets using distributed AI

**Gradient Integration:**
- Parallax for large model inference (market analysis)
- Lattica for agent communication (share signals)
- Solana for payments and reputation
- Privacy-preserving data sharing

**Tech Stack:** Gradient + Solana + x402 + Switchboard

**Prize Potential:** Gradient ($5k) + Best x402 Agent ($10k) + Switchboard ($5k) = $20k

**Difficulty:** High (14-20 hours)

---

### 2. Privacy-Preserving Research Agent

**Concept:** Agent that analyzes sensitive data without exposing it to any single node

**Gradient Integration:**
- Parallax with high privacy settings
- Fragment-based computation
- Encrypted results

**Tech Stack:** Gradient + Solana + CDP Wallets

**Prize Potential:** Gradient ($5k) + Best x402 Agent ($10k) = $15k

**Difficulty:** High (12-16 hours)

---

### 3. Multi-Agent Collaboration Platform

**Concept:** Platform where specialized agents discover and collaborate with each other

**Gradient Integration:**
- Lattica for agent discovery
- P2P messaging between agents
- Reputation system for trust

**Tech Stack:** Gradient + Solana + React + PostgreSQL

**Prize Potential:** Gradient ($5k) + Best x402 Application ($10k) = $15k

**Difficulty:** Very High (16-24 hours)

---

## Troubleshooting

See [common-troubleshooting.md](./common-troubleshooting.md) for generic issues.

### Gradient-Specific Issues

#### Insufficient nodes available

**Solutions:**
```javascript
// Reduce shard count
const result = await parallax.infer({
  model: 'llama-70b',
  prompt: '...',
  shards: 2,
  fallbackToLess: true
});

// Or use smaller model
const result = await parallax.infer({
  model: 'llama-8b',
  prompt: '...'
});
```

#### Agent discovery returns empty

**Solutions:**
```javascript
// Broaden search criteria
const agents = await lattica.discover({
  type: 'trading',
  online: true
  // Remove minReputation and capability requirements initially
});

// Register your agent as discoverable
await lattica.updateProfile({
  discoverable: true,
  type: 'oracle',
  capabilities: ['price-feeds']
});
```

#### High compute costs

**Solutions:**
```javascript
// Use smaller model
const result = await parallax.infer({
  model: 'llama-8b',
  prompt: '...'
});

// Set strict cost limit
const result = await parallax.infer({
  model: 'llama-70b',
  prompt: '...',
  maxCost: 0.10,
  throwOnOvercost: true
});
```

---

## Additional Resources

### Official Links
- Documentation: https://docs.gradient.network
- Website: https://gradient.network
- Explorer: https://explorer.gradient.network
- GitHub: https://github.com/gradient-network

### Community
- Discord: https://discord.gg/gradient-network
- Twitter: @gradientnetwork
- Email: developers@gradient.network

### Research Papers
- Parallax Whitepaper: https://gradient.network/papers/parallax
- Lattica Protocol: https://gradient.network/papers/lattica

### Related Guides
- [x402 Protocol Specification](../../x402-protocol-specification.md)
- [Solana Development Guide](../ecosystem/solana-guide.md)

---

**Last Updated:** November 4, 2025
**Hackathon Deadline:** November 11, 2025
**Integration Difficulty:** High (8-16 hours)

Gradient represents the **future of decentralized AI** - challenging but extremely impressive!
