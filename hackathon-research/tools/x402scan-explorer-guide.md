# x402scan Explorer Guide

**Quick Links:** [Website](https://www.x402scan.com/) | [GitHub](https://github.com/meritsystems/x402scan)
**License:** Apache 2.0 (Open Source) | **Maintainer:** Merit Systems
**Integration Difficulty:** Easy (Passive) / Medium (Self-hosting)

## Overview

x402scan is the **ecosystem explorer and analytics platform** for the x402 protocol. Functions as "on-chain explorer + agent dashboard" providing real-time monitoring of all x402 payments, agent activities, transactions, and facilitator performance.

**Key Stat:** 182 GitHub stars, 102 forks, 24 contributors - most popular x402 explorer

## Key Features

- ✅ **Only Public x402 Explorer:** Comprehensive tracking of entire ecosystem
- ✅ **Embedded Wallet:** Test resources directly in browser, no setup
- ✅ **Resource Discovery:** Browse 50+ x402-enabled APIs
- ✅ **Facilitator Comparison:** Track performance across all facilitators
- ✅ **Real-time Monitoring:** Live transaction tracking
- ✅ **Open Source:** Self-host and extend (Apache 2.0)
- ✅ **Resource Registration:** List your service for free
- ✅ **Transaction Attribution:** Cross-facilitator tracking

## Technical Specifications

### Technology Stack

- **Language:** TypeScript (98.8%)
- **Framework:** Next.js (React)
- **Package Manager:** pnpm
- **Build Tool:** Turbo (monorepo orchestration)
- **Code Quality:** ESLint, Prettier
- **Background Sync:** Trigger.dev

### Repository Structure

```
x402scan/
├── workspaces/
│   ├── scan/          # Next.js web application (frontend)
│   ├── sync/          # Background service (Trigger.dev)
│   └── facilitators/  # Shared configuration layer
├── docs/              # Documentation
└── README.md          # Setup instructions
```

**Monorepo Benefits:**
- Shared code between frontend/backend
- Consistent TypeScript types
- Coordinated deployments

### Blockchain Integration

**Primary Support:**
- **Base chain** (Coinbase L2) - Primary focus
- USDC token tracking
- On-chain payment verification
- Transaction history indexing

**Data Sources:**
- Blockchain nodes
- Facilitator APIs
- On-chain events
- Transaction logs

### Development Setup

```bash
# Clone repository
git clone https://github.com/meritsystems/x402scan.git

# Install dependencies
pnpm install

# Run development servers
pnpm dev          # Frontend (Next.js)
pnpm dev:sync     # Background sync service

# Production build
pnpm build
```

## Features and Navigation

### Navigation Tabs

**1. Overview**
- Ecosystem summary statistics
- Recent transactions
- Active facilitators
- Growth metrics

**2. Composer** (New Feature)
- Payment composition tool
- Transaction builder
- Testing interface

**3. Marketplace**
- Resource discovery (50+ services)
- x402-enabled services
- Pricing comparison
- Service ratings

**4. Transactions**
- Real-time payment tracking
- Transaction history
- Advanced filtering and search
- Export capabilities

**5. Facilitators**
- Performance metrics
- Uptime tracking
- Fee comparison
- Network support

**6. Networks**
- Multi-chain statistics
- Per-network analytics
- Cross-chain flows

**7. Ecosystem**
- Overall health metrics
- Growth trends
- Adoption statistics

### Core Capabilities

**1. Resource Discovery**
- Browse x402-enabled APIs
- View transaction volumes
- Check pricing models
- Read service descriptions
- Filter by category

**2. Embedded Wallet**
- Test resources directly in browser
- No separate wallet setup needed
- One-click payments
- Instant results
- Zero friction testing

**3. Facilitator Tracking**
- Monitor all facilitators (Coinbase CDP, PayAI, etc.)
- Performance comparison
- Uptime statistics
- Transaction success rates
- Settlement time tracking

**4. Transaction Monitoring**
- Real-time payment tracking
- Search by hash, address, resource
- Filter by facilitator, network, status
- Export transaction data
- Historical analysis

**5. Resource Registration**
- Add new x402 services
- Submit for listing
- Automatic validation
- Community discovery
- Free listing

## Data Tracked

### Transaction Data Schema

```json
{
  "tx_hash": "0x...",
  "timestamp": "2025-11-04T10:30:00Z",
  "amount": "0.001",
  "token": "USDC",
  "buyer": "0x...",
  "merchant": "0x...",
  "resource": "https://api.example.com/data",
  "facilitator": "payai.network",
  "network": "base",
  "status": "confirmed"
}
```

### Facilitator Metrics

- Total transactions processed
- Transaction success rate (%)
- Average settlement time (seconds)
- Uptime percentage
- Supported networks
- Fee structure

### Ecosystem Metrics

- Total transactions (all-time)
- Transaction value (USD)
- Active resources (count)
- Active agents (count)
- Growth rates (7d, 30d, all-time)

## Recent Statistics (October 2025)

| Metric | Value | Growth (7d) |
|--------|-------|-------------|
| **Transaction Volume** | 163,600 | +701.7% |
| **Transaction Value** | $140,000 USD | +8,218.5% |
| **Active Resources** | 50+ | Growing |
| **Facilitators** | 8+ | Expanding |

**Insight:** Explosive growth indicating rapid ecosystem adoption

## Development Use Cases

### 1. Testing Validation ✅

**Purpose:** Verify your x402 implementation works correctly

**Process:**
1. Deploy your x402 service
2. Make a test transaction
3. Search for transaction on x402scan
4. Verify all details correct
5. Confirm settlement

**What to Check:**
- ✅ Transaction appears in search
- ✅ Amount matches expected
- ✅ Settlement time reasonable
- ✅ Status: confirmed
- ✅ Facilitator recorded correctly

### 2. Resource Discovery 🔍

**Purpose:** Find existing x402 APIs to integrate

**Process:**
1. Visit Marketplace tab
2. Browse available resources
3. Check pricing and transaction volume
4. Test with embedded wallet
5. Integrate into your application

**Use Cases:**
- Data feeds for AI agents
- AI model APIs
- Blockchain RPC access
- Analytics services
- Oracle services

### 3. Facilitator Selection ⚖️

**Purpose:** Compare facilitators before choosing

**Metrics to Compare:**
- Transaction success rate
- Average settlement time
- Supported networks
- Uptime percentage
- Fee structure
- Market share

**Example Comparison:**
```
Coinbase CDP: 99.9% success, 1.8s avg, 80% market share
PayAI: 99.8% success, 0.9s avg, 14% market share
```

### 4. Market Research 📊

**Purpose:** Understand x402 ecosystem landscape

**Insights Available:**
- Popular resource types
- Typical pricing models
- Transaction volume trends
- Growth rates
- Competitive analysis
- Revenue opportunities

### 5. Performance Monitoring 📈

**Purpose:** Track your deployed x402 service

**Monitored Metrics:**
- Transaction volume (daily/weekly/monthly)
- Revenue generated (USD)
- Success rate (%)
- User adoption (unique buyers)
- Ranking in marketplace
- Competitive positioning

## Resource Registration

### Method 1: Web Form (Easiest)

1. Visit https://www.x402scan.com/resources/register
2. Enter x402-enabled URL
3. Provide description
4. Submit for validation
5. Automatic listing upon approval

### Method 2: GitHub PR (Advanced)

```bash
# Fork repository
git clone https://github.com/YOUR_USERNAME/x402scan.git

# Modify facilitators configuration
# Edit workspaces/facilitators/config.ts

# Add your resource
{
  name: "Your Service Name",
  url: "https://your-service.com/api",
  description: "What your service does",
  pricing: "0.001 USDC per request",
  category: "data-feeds", // or ai-models, rpc, analytics, etc.
  networks: ["solana", "base"]
}

# Validate configuration
pnpm check:facilitators

# Submit pull request
```

**Required Information:**
- Service name
- x402-enabled URL
- Description (what it does)
- Pricing model
- Category
- Supported networks

## Facilitator Registration

**For Facilitator Providers:**

```typescript
// facilitators/config.ts
{
  id: "your-facilitator",
  name: "Your Facilitator Name",
  url: "https://your-facilitator.com",
  networks: ["solana", "base", "polygon"],
  status: "active"
}
```

**Validation:**
```bash
pnpm check:facilitators
```

## API Availability

### Current Status: LIMITED ⚠️

**No Public API Yet:**
- Web interface only
- No REST endpoints published
- No GraphQL endpoint
- No WebSocket streaming

**Data Access Methods:**
1. **Web interface** - Browse and search manually
2. **Self-hosting** - Clone and run locally (full access)
3. **GitHub** - Access source code directly

**Future Development:**
- Public API likely coming
- Developer access for analytics
- Programmatic data retrieval
- Webhook notifications

### Related Platform: x402station

**Note:** x402station is a separate analytics platform with API capabilities (distinct from x402scan)

## Integration Difficulty

### For Basic Use: EASY ✅

- **No integration required** - Web-based explorer
- Embedded wallet for testing
- Simple resource registration form
- Immediate access to all data
- Zero setup for browsing

### For Self-Hosting: MEDIUM ⚠️

**Requirements:**
- Next.js knowledge
- Environment configuration
- Trigger.dev account (background sync)
- Blockchain node access (optional)

**Setup Steps:**
```bash
git clone https://github.com/meritsystems/x402scan.git
cd x402scan
pnpm install
cp .env.example .env
# Configure environment variables
pnpm dev
```

**Environment Variables:**
- Database connection
- Trigger.dev API key
- Blockchain RPC endpoints
- Analytics keys (optional)

## Documentation Quality: MEDIUM-HIGH

### Strong Documentation ✅
- Clear GitHub README
- Setup instructions
- Configuration examples
- Contribution guidelines
- Architecture overview
- Monorepo structure explained

### Could Improve ⚠️
- API documentation (when available)
- Advanced usage tutorials
- Custom deployment guides
- Monitoring integration
- Data export formats
- Performance optimization tips

## Community & Statistics

**GitHub Activity:**
- **Stars:** 182 ⭐
- **Forks:** 102
- **Contributors:** 24
- **Commits:** 1,158
- **Status:** Active development

**Community:**
- Discord (active discussions)
- GitHub issues (responsive)
- Open to contributions
- Regular updates
- Merit Systems backing (credibility)

**License:** Apache 2.0 (permissive, commercial-friendly)

## Unique Selling Points

1. **Only public x402 explorer** with comprehensive tracking
2. **Embedded wallet** for immediate testing without setup
3. **Open-source** (can self-host and extend)
4. **Real-time monitoring** of entire ecosystem
5. **Automatic resource validation** for listings
6. **Merit Systems backing** (established credibility)
7. **Transaction attribution** across all facilitators
8. **Multi-facilitator comparison** in single interface

## Vision & Mission

> "Shed light on the activities happening over x402, build trust, and help standardize interaction patterns to grow the ecosystem massively."
> — x402scan Team

**Goals:**
- 🌐 Transparency for all x402 transactions
- 🤝 Trust building through visibility
- 📏 Standardization of best practices
- 📈 Ecosystem growth facilitation

## When to Use x402scan

**✅ Use x402scan for:**
- Testing your x402 implementation
- Discovering existing x402 APIs
- Comparing facilitators
- Market research and analysis
- Monitoring your service performance
- Listing your service for exposure
- Understanding ecosystem trends
- Validating transactions

**✅ Self-host x402scan if you:**
- Need programmatic data access
- Want custom analytics
- Require private deployment
- Need extended data retention
- Want to contribute features

## Quick Start Checklist

**For Users:**
- [ ] Visit https://www.x402scan.com/
- [ ] Browse Marketplace for available resources
- [ ] Test resources with embedded wallet
- [ ] Register your service (if provider)
- [ ] Monitor facilitator performance
- [ ] Track your transactions

**For Self-Hosting:**
- [ ] Clone GitHub repository
- [ ] Install dependencies (pnpm)
- [ ] Configure environment variables
- [ ] Set up Trigger.dev account
- [ ] Configure blockchain RPC endpoints
- [ ] Run dev servers
- [ ] Access local instance

## Tips & Best Practices

**For Service Providers:**
- Register early for marketplace visibility
- Monitor your service performance regularly
- Respond to community feedback
- Keep service description updated
- Optimize pricing based on market data

**For Developers:**
- Use embedded wallet for quick testing
- Validate all transactions on x402scan
- Compare facilitators before choosing
- Monitor ecosystem trends
- Contribute to open source

---

**Related Docs:**
- [Testing and Monitoring Guide](../guides/testing-and-monitoring.md)
- [Facilitator Comparison Reference](../reference/facilitator-comparison.md)
- [Integration Patterns Guide](../guides/integration-patterns.md)
