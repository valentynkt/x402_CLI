# Code Repositories Reference

Comprehensive list of official repositories, examples, and resources for x402 development.

## Official x402 Repositories

### Protocol Specifications

```
x402 Spec (Coinbase): https://github.com/coinbase/x402
x402 Documentation: https://github.com/coinbase/x402-docs
Reference Implementation: https://github.com/coinbase/x402-reference
```

---

## SDK Repositories

### Faremeter (Corbits)

```
Main Framework: https://github.com/faremeter/faremeter
Fetch Library: https://github.com/faremeter/fetch
Middleware: https://github.com/faremeter/middleware
Facilitator: https://github.com/faremeter/facilitator
Examples: https://github.com/faremeter/examples
```

**Stars:** 36+ | **License:** LGPL-3.0

### Coinbase CDP SDK

```
CDP SDK: https://github.com/coinbase/cdp-sdk
CDP Docs: https://github.com/coinbase/cdp-docs
x402 Implementation: Part of CDP SDK
```

**Stars:** 1,000+ | **License:** Proprietary

### PayAI Network

```
SDK: https://github.com/PayAINetwork/sdk
Docs: https://github.com/PayAINetwork/docs
Examples: https://github.com/PayAINetwork/examples
```

**Stars:** Growing | **License:** Proprietary

### MCPay.tech

```
Main Repo: https://github.com/microchipgnu/MCPay
Website: https://mcpay.tech
Docs: https://docs.mcpay.tech
```

**Stars:** Growing | **License:** Open Source

### x402-MCP (Multiple Implementations)

```
ethanniser/x402-mcp: https://github.com/ethanniser/x402-mcp
Coinbase Example: https://github.com/coinbase/x402/tree/main/examples/typescript/mcp
Vercel Blog: https://vercel.com/blog/introducing-x402-mcp-open-protocol-payments-for-mcp-tools
mark3labs/mcp-go-x402: https://github.com/mark3labs/mcp-go-x402
FlowMCP Middleware: https://github.com/FlowMCP/x402-mcp-middleware
```

**Stars:** Various | **License:** Open Source (varies)

### ACK Protocol

```
Main Repo: https://github.com/agentcommercekit/ack
Live Demo: https://solana-paywal.vercel.app/
Example Code: https://github.com/Woody4618/solana-paywal-x402
```

**Stars:** Growing | **License:** Open Source

### Google A2A x402

```
Main Repo: https://github.com/google-agentic-commerce/a2a-x402
Organization: https://github.com/google-agentic-commerce
Releases: https://github.com/google-agentic-commerce/a2a-x402/releases
```

**Stars:** Growing | **License:** Open Source

### Nexus (Thirdweb)

```
Thirdweb SDK: https://github.com/thirdweb-dev/js
x402 Docs: https://portal.thirdweb.com/payments/x402
Nexus: https://nexus.thirdweb.com/
```

**Stars:** 1,400+ (thirdweb SDK) | **License:** Commercial

### Native x402 Example

```
Main Repo: https://github.com/Woody4618/x402-solana-examples
Solana Guide: https://solana.com/developers/guides/getstarted/intro-to-x402#native-implementation
```

**Stars:** Growing | **License:** Open Source

---

## Tools & Explorers

### x402scan

```
Explorer: https://github.com/meritsystems/x402scan
Frontend: https://github.com/meritsystems/x402scan/tree/main/workspaces/scan
Sync Service: https://github.com/meritsystems/x402scan/tree/main/workspaces/sync
```

**Stars:** 182 | **Forks:** 102 | **License:** Apache 2.0

---

## Example Projects

### MCP Servers

```
Anthropic MCP Examples: https://github.com/anthropics/mcp-examples
x402 MCP Wrapper: https://github.com/vercel/x402-mcp
MCP Official Spec: https://github.com/modelcontextprotocol/specification
MCPay.tech: https://mcpay.tech (check site for repo)
```

### AI Agent Frameworks

```
ElizaOS (ai16z - $2.6B): https://github.com/ai16z/eliza
LangChain: https://github.com/langchain-ai/langchain
AutoGPT: https://github.com/Significant-Gravitas/AutoGPT
```

### Notable Implementations

```
Mallory (AI Chat): Check Corbits org
Switchboard Oracle: https://github.com/switchboard-xyz
```

---

## Blockchain Repositories

### Solana

```
Solana Program Library: https://github.com/solana-labs/solana-program-library
Solana Web3.js: https://github.com/solana-labs/solana-web3.js
Wallet Adapter: https://github.com/solana-labs/wallet-adapter
Anchor Framework: https://github.com/coral-xyz/anchor
Anchor Examples: https://github.com/coral-xyz/anchor/tree/master/examples
```

### Base/EVM

```
Base Contracts: https://github.com/base-org/contracts
Viem (Web3 Library): https://github.com/wagmi-dev/viem
Wagmi (React Hooks): https://github.com/wagmi-dev/wagmi
Ethers.js: https://github.com/ethers-io/ethers.js
Hardhat: https://github.com/NomicFoundation/hardhat
```

---

## Quick Start Templates

### Faremeter Quickstart

```bash
# Clone template
git clone https://github.com/faremeter/quickstart-template
cd quickstart-template
pnpm install
pnpm dev
```

### x402 Client Template

```bash
# Create from scratch
mkdir x402-client && cd x402-client
npm init -y
npm install @faremeter/fetch @solana/wallet-adapter-phantom
```

### x402 Server Template

```bash
# Create from scratch
mkdir x402-server && cd x402-server
npm init -y
npm install @faremeter/middleware express
```

---

## Installation Commands

```bash
# Core x402 / Faremeter
npm install @faremeter/fetch @faremeter/middleware

# Solana
npm install @solana/web3.js @solana/spl-token
npm install @solana/wallet-adapter-react @solana/wallet-adapter-phantom

# Base/EVM
npm install viem wagmi ethers

# CDP
npm install @coinbase/cdp-sdk

# PayAI
npm install @payai/sdk

# Crossmint
npm install @crossmint/client-sdk

# MCPay.tech
npm install mcpay

# x402-MCP (TypeScript)
npm install x402-mcp @modelcontextprotocol/sdk

# x402-MCP (Go)
go get github.com/mark3labs/mcp-go-x402

# ACK Protocol
npm install ack-protocol

# Google A2A x402
npm install @google-agentic-commerce/a2a-x402

# Nexus (Thirdweb)
npm install thirdweb

# Native Example (clone repository)
git clone https://github.com/Woody4618/x402-solana-examples

# Development Tools
npm install -D typescript @types/node tsx
```

---

## Learning Resources

### Official Documentation

```
x402 Protocol: https://docs.x402.org
Solana x402 Guide: https://solana.com/developers/guides/getstarted/intro-to-x402
Corbits Docs: https://docs.corbits.dev
Coinbase CDP: https://docs.cdp.coinbase.com
PayAI Docs: https://docs.payai.network
```

### Tutorials & Guides

```
Solana Cookbook: https://solanacookbook.com
Solana Bootcamp: https://www.soldev.app
Base Docs: https://docs.base.org
Ethereum.org: https://ethereum.org/en/developers
```

### Community

```
Discord - Faremeter: Check corbits.dev for link
Discord - PayAI: Check payai.network for link
Discord - Solana: https://discord.gg/solana
Discord - Base: https://discord.gg/base
GitHub Discussions: On respective repos
```

---

## Testing Resources

### Testnets

```
Solana Devnet: https://api.devnet.solana.com
Base Sepolia: https://sepolia.base.org
Polygon Mumbai: https://rpc-mumbai.maticvigil.com
```

### Faucets

```
Solana: https://faucet.solana.com
Base: https://www.coinbase.com/faucets
Polygon: https://faucet.polygon.technology
```

### Explorers

```
x402scan: https://www.x402scan.com
Solana Explorer: https://explorer.solana.com
Solscan: https://solscan.io
BaseScan: https://basescan.org
PolygonScan: https://polygonscan.com
```

---

## Development Tools

### IDEs & Extensions

```
VS Code: https://code.visualstudio.com
Solana Extension: Search "Solana" in VS Code marketplace
Anchor Extension: Search "Anchor" in VS Code marketplace
```

### CLI Tools

```
Solana CLI: https://docs.solana.com/cli/install-solana-cli-tools
Anchor CLI: https://www.anchor-lang.com/docs/installation
Foundry (EVM): https://getfoundry.sh
Cast (EVM): Part of Foundry
```

### Monitoring

```
x402scan: https://www.x402scan.com
Solana FM: https://solana.fm
Helius Explorer: https://explorer.helius.dev
```

---

## Quick Clone Commands

```bash
# Clone all major repos
mkdir x402-dev && cd x402-dev

# Faremeter
git clone https://github.com/faremeter/faremeter

# x402scan
git clone https://github.com/meritsystems/x402scan

# x402 Spec
git clone https://github.com/coinbase/x402

# MCP Examples
git clone https://github.com/anthropics/mcp-examples

# Solana Program Library
git clone https://github.com/solana-labs/solana-program-library
```

---

**Related Docs:**
- [SDK Comparison](./sdk-comparison.md)
- [Integration Patterns](../guides/integration-patterns.md)
- [Testing and Monitoring](../guides/testing-and-monitoring.md)
