# Blockchain Networks Reference

Quick reference for x402-supported blockchain networks, RPC endpoints, and token addresses.

## Supported Networks

| Network | Finality | Gas Cost | x402 Status | Best For |
|---------|----------|----------|-------------|----------|
| **Solana** | ~400ms | <$0.0001 | ✅ Primary | Micropayments, speed |
| **Base** | ~2s | ~$0.01 | ✅ Primary | EVM, Coinbase |
| **Polygon** | ~3s | ~$0.01 | ✅ Secondary | Balanced cost/speed |
| **Ethereum** | ~15s | $1-$10 | ⚠️ Limited | High value only |
| **Avalanche** | ~3s | ~$0.02 | ✅ Secondary | Fast EVM |
| **Arbitrum** | ~2s | ~$0.005 | ✅ Secondary | Cheap EVM L2 |
| **Optimism** | ~2s | ~$0.005 | ✅ Secondary | Cheap EVM L2 |

---

## 1. Solana

**Status:** Primary x402 network
**Recommended For:** Micropayments, highest speed, lowest cost

### Network Details

| Parameter | Mainnet | Devnet |
|-----------|---------|--------|
| **Finality** | ~400ms | ~400ms |
| **TPS** | 65,000+ | 65,000+ |
| **Gas Cost** | <$0.0001 | Free (faucet) |
| **Block Time** | 400ms | 400ms |

### RPC Endpoints

**Public (Rate Limited):**
```
Mainnet: https://api.mainnet-beta.solana.com
Devnet: https://api.devnet.solana.com
```

**Premium (Via Corbits Proxy):**
```
Helius: https://api.corbits.dev/helius/rpc
Triton: https://api.corbits.dev/triton/rpc
QuickNode: https://api.corbits.dev/quicknode/rpc
```

### USDC Token Addresses

```typescript
// Mainnet
const USDC_MINT = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');

// Devnet
const USDC_MINT_DEVNET = new PublicKey('4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU');
```

### Configuration

```typescript
import { Connection, clusterApiUrl } from '@solana/web3.js';

// Mainnet
const connection = new Connection('https://api.mainnet-beta.solana.com', 'confirmed');

// Devnet
const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
```

### Faucets
```
SOL Faucet: https://faucet.solana.com/
USDC Devnet: Use Faremeter test tools
```

---

## 2. Base

**Status:** Primary x402 network (Coinbase L2)
**Recommended For:** EVM compatibility, Coinbase ecosystem

### Network Details

| Parameter | Mainnet | Sepolia Testnet |
|-----------|---------|-----------------|
| **Finality** | ~2 seconds | ~2 seconds |
| **TPS** | 1,000+ | 1,000+ |
| **Gas Cost** | ~$0.01 | Free (faucet) |
| **Block Time** | 2s | 2s |

### RPC Endpoints

```
Mainnet: https://mainnet.base.org
Sepolia: https://sepolia.base.org
```

### USDC Token Address

```solidity
// Base Mainnet
USDC: 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913

// Base Sepolia
USDC: 0x036CbD53842c5426634e7929541eC2318f3dCF7e
```

### Configuration

```typescript
import { createPublicClient, http } from 'viem';
import { base } from 'viem/chains';

const client = createPublicClient({
  chain: base,
  transport: http()
});
```

### Faucets
```
ETH Faucet: https://www.coinbase.com/faucets/base-ethereum-goerli-faucet
USDC: Coinbase Faucet
```

---

## 3. Polygon

**Status:** Secondary x402 network
**Recommended For:** Balanced cost/speed, large ecosystem

### Network Details

| Parameter | Mainnet | Mumbai Testnet |
|-----------|---------|----------------|
| **Finality** | ~3 seconds | ~3 seconds |
| **TPS** | 7,000+ | 7,000+ |
| **Gas Cost** | ~$0.01 | Free (faucet) |
| **Block Time** | 2-3s | 2-3s |

### RPC Endpoints

```
Mainnet: https://polygon-rpc.com
Mumbai: https://rpc-mumbai.maticvigil.com
```

### USDC Token Address

```solidity
// Polygon Mainnet
USDC: 0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174

// Mumbai Testnet
USDC: 0x0FA8781a83E46826621b3BC094Ea2A0212e71B23
```

### Faucets
```
MATIC: https://faucet.polygon.technology/
USDC Mumbai: Polygon Faucet
```

---

## 4. Avalanche

**Status:** Supported (PayAI)
**Recommended For:** Fast EVM alternative

### Network Details

```
Mainnet (C-Chain): https://api.avax.network/ext/bc/C/rpc
Finality: ~3 seconds
Gas: ~$0.02
```

### USDC Token

```solidity
// Avalanche C-Chain
USDC: 0xB97EF9Ef8734C71904D8002F8b6Bc66Dd9c48a6E
```

---

## Network Selection Guide

### By Use Case

**Micropayments (<$0.01):**
- 🥇 Solana (gas <$0.0001)
- 🥈 Base (gas ~$0.01)
- 🥉 Arbitrum/Optimism (gas ~$0.005)

**Speed Priority:**
- 🥇 Solana (<1s settlement)
- 🥈 Base (~2s settlement)
- 🥉 Polygon (~3s settlement)

**EVM Compatibility:**
- 🥇 Base (Coinbase backing)
- 🥈 Polygon (mature ecosystem)
- 🥉 Arbitrum/Optimism (L2 benefits)

**Cost Optimization:**
- 🥇 Solana (<$0.0001)
- 🥈 Arbitrum/Optimism (~$0.005)
- 🥉 Base/Polygon (~$0.01)

---

## Code Examples

### Solana

```typescript
import { Connection, PublicKey } from '@solana/web3.js';

const connection = new Connection('https://api.mainnet-beta.solana.com');
const USDC_MINT = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');

// Get token account
const tokenAccount = await getAssociatedTokenAddress(
  USDC_MINT,
  walletPublicKey
);
```

### Base (Viem)

```typescript
import { createPublicClient, http } from 'viem';
import { base } from 'viem/chains';

const client = createPublicClient({
  chain: base,
  transport: http()
});

const USDC_ADDRESS = '0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913';

// Get balance
const balance = await client.readContract({
  address: USDC_ADDRESS,
  abi: ERC20_ABI,
  functionName: 'balanceOf',
  args: [walletAddress]
});
```

### Base (Ethers)

```typescript
import { ethers } from 'ethers';

const provider = new ethers.JsonRpcProvider('https://mainnet.base.org');
const USDC_ADDRESS = '0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913';

const usdcContract = new ethers.Contract(USDC_ADDRESS, ERC20_ABI, provider);
const balance = await usdcContract.balanceOf(walletAddress);
```

---

## Multi-Chain Configuration

```typescript
// Network configuration map
const NETWORKS = {
  solana: {
    rpc: 'https://api.mainnet-beta.solana.com',
    usdc: 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v',
    decimals: 6
  },
  base: {
    rpc: 'https://mainnet.base.org',
    usdc: '0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913',
    decimals: 6,
    chainId: 8453
  },
  polygon: {
    rpc: 'https://polygon-rpc.com',
    usdc: '0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174',
    decimals: 6,
    chainId: 137
  }
};

// Select network
const config = NETWORKS[selectedNetwork];
```

---

## Quick Commands

```bash
# Solana
solana balance --url mainnet-beta
solana airdrop 2 --url devnet

# Base
cast balance <address> --rpc-url https://mainnet.base.org
cast send <address> --value <amount> --rpc-url https://mainnet.base.org

# Check USDC balance (Solana)
spl-token balance EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
```

---

**Related Docs:**
- [Solana Implementation Guide](../guides/solana-implementation.md)
- [Facilitator Comparison](./facilitator-comparison.md)
- [SDK Comparison](./sdk-comparison.md)
