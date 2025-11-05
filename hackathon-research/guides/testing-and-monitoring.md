# Testing and Monitoring Guide

Complete guide to testing x402 implementations and monitoring transactions.

## Testing Tools

### 1. x402 Echo Merchant (PayAI) ⭐ Best for Testing

**What it is:** Zero-cost testing environment with guaranteed full refunds

**Features:**
- ✅ Real x402 transaction flow
- ✅ All payments fully refunded
- ✅ Production facilitator (PayAI)
- ✅ No setup required
- ✅ Risk-free testing

**Access:** https://payai.network/echo

**How to Use:**
```typescript
// Point to PayAI facilitator
const config = {
  facilitator: 'https://payai.network',
  merchantWallet: 'YourWalletAddress',
  price: 0.001
};

// Make test transactions
// Funds automatically refunded
```

### 2. Solana Devnet

**Get Devnet SOL:**
```bash
# Via CLI
solana airdrop 2 --url https://api.devnet.solana.com

# Or use web faucet
https://faucet.solana.com/
```

**Get Devnet USDC:**
- Use Faremeter test tools
- Check Solana docs for devnet USDC faucet

**Devnet USDC Mint:**
```typescript
const USDC_DEVNET = new PublicKey('4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU');
```

### 3. Base Sepolia Testnet

**Get Testnet ETH:**
```bash
# Coinbase Faucet
https://www.coinbase.com/faucets/base-ethereum-goerli-faucet

# Or other Base Sepolia faucets
https://base-sepolia-faucet.com/
```

**Get Testnet USDC:**
- Use Coinbase Faucet for testnet USDC
- Mint from test contracts

---

## Monitoring Tools

### 1. x402scan (Primary) ⭐

**URL:** https://www.x402scan.com/

**Features:**
- Transaction explorer
- Real-time monitoring
- Facilitator performance tracking
- Resource discovery
- Embedded wallet for testing

**How to Use:**
1. Visit x402scan.com
2. Search by transaction hash
3. View transaction details
4. Verify payment amount and status
5. Check facilitator used

**Testing Flow:**
```typescript
// 1. Make x402 payment
const tx = await paidFetch(url, { wallet, maxAmount: 0.01 });

// 2. Get transaction hash from response
console.log('TX Hash:', tx.hash);

// 3. Search on x402scan
// https://www.x402scan.com/tx/{hash}

// 4. Verify:
// - Amount correct
// - Status: confirmed
// - Facilitator recorded
```

### 2. Solana Explorers

**Solana Explorer (Official):**
```bash
https://explorer.solana.com/

# Devnet
https://explorer.solana.com/?cluster=devnet

# Mainnet
https://explorer.solana.com/
```

**Solscan:**
```bash
https://solscan.io/

# More detailed transaction info
# Token balance history
# Account analytics
```

**Solana Beach:**
```bash
https://solanabeach.io/

# Network statistics
# Validator info
# Block explorer
```

### 3. Base Explorer

**BaseScan:**
```bash
https://basescan.org/

# Similar to Etherscan
# EVM transaction details
# Smart contract verification
```

---

## Testing Checklist

### Before Deployment

- [ ] Test on devnet/testnet first
- [ ] Verify wallet connection
- [ ] Test with insufficient balance
- [ ] Test with wrong recipient
- [ ] Test with amount too small
- [ ] Test with invalid transaction
- [ ] Verify error handling
- [ ] Check transaction confirmation
- [ ] Monitor gas costs
- [ ] Test with Echo Merchant (refunds)

### After Deployment

- [ ] Monitor first transactions on x402scan
- [ ] Verify payments arriving in merchant wallet
- [ ] Check transaction success rate
- [ ] Monitor settlement times
- [ ] Set up alerts for failures
- [ ] Track revenue metrics
- [ ] Monitor facilitator performance
- [ ] Check for security issues

---

## Development Frameworks

### Solana Development

```bash
# Anchor Framework (recommended)
npm install @project-serum/anchor

# Solana Web3.js
npm install @solana/web3.js

# Wallet Adapter
npm install @solana/wallet-adapter-react
npm install @solana/wallet-adapter-react-ui
npm install @solana/wallet-adapter-phantom
```

### Base/EVM Development

```bash
# Hardhat (smart contracts)
npm install hardhat

# Ethers.js (web3 library)
npm install ethers

# Wagmi (React hooks)
npm install wagmi

# Viem (modern web3)
npm install viem
```

---

## Monitoring Best Practices

### 1. Set Up Logging

```typescript
// Log all payments
app.use(fareMiddleware({
  price: 0.001,
  merchantWallet: process.env.MERCHANT_WALLET,
  onPayment: (payment) => {
    console.log('Payment received:', {
      txHash: payment.signature,
      amount: payment.amount,
      from: payment.payer,
      timestamp: new Date().toISOString()
    });

    // Save to database
    db.payments.insert(payment);
  }
}));
```

### 2. Track Metrics

```typescript
// Key metrics to track
const metrics = {
  totalTransactions: 0,
  successfulPayments: 0,
  failedPayments: 0,
  totalRevenue: 0,
  averagePayment: 0,
  uniqueCustomers: new Set()
};

// Update on each payment
function trackPayment(payment) {
  metrics.totalTransactions++;
  metrics.successfulPayments++;
  metrics.totalRevenue += payment.amount;
  metrics.uniqueCustomers.add(payment.payer);
  metrics.averagePayment = metrics.totalRevenue / metrics.successfulPayments;
}
```

### 3. Set Up Alerts

```typescript
// Alert on unusual activity
function checkForAnomalies(payment) {
  // Large payment alert
  if (payment.amount > 10.0) {
    sendAlert('Large payment detected', payment);
  }

  // Failure rate alert
  const failureRate = metrics.failedPayments / metrics.totalTransactions;
  if (failureRate > 0.1) { // 10%
    sendAlert('High failure rate detected', { failureRate });
  }

  // Revenue milestone alert
  if (metrics.totalRevenue >= nextMilestone) {
    sendAlert('Revenue milestone reached', {
      revenue: metrics.totalRevenue
    });
  }
}
```

---

## Testing Scenarios

### Scenario 1: Normal Payment Flow

```typescript
// Test successful payment
async function testNormalPayment() {
  const response = await paidFetch('http://localhost:3000/premium', {
    wallet: testWallet,
    maxAmount: 0.01
  });

  assert(response.ok, 'Payment should succeed');
  assert(response.data, 'Should receive content');
}
```

### Scenario 2: Insufficient Funds

```typescript
// Test with empty wallet
async function testInsufficientFunds() {
  const emptyWallet = createEmptyWallet();

  try {
    await paidFetch(url, {
      wallet: emptyWallet,
      maxAmount: 0.01
    });
    assert.fail('Should throw error');
  } catch (error) {
    assert(error.code === 'INSUFFICIENT_FUNDS');
  }
}
```

### Scenario 3: Wrong Amount

```typescript
// Test with amount below minimum
async function testWrongAmount() {
  // Server expects 0.01 USDC
  const lowPayment = createTransaction({
    amount: 0.001  // Too low
  });

  try {
    await submitPayment(lowPayment);
    assert.fail('Should reject payment');
  } catch (error) {
    assert(error.message.includes('Insufficient payment'));
  }
}
```

### Scenario 4: Network Failure

```typescript
// Test network interruption
async function testNetworkFailure() {
  // Simulate network down
  mockNetwork.simulateFailure();

  try {
    await paidFetch(url, { wallet, maxAmount: 0.01 });
    assert.fail('Should throw network error');
  } catch (error) {
    assert(error.code === 'NETWORK_ERROR');
  }
}
```

---

## Performance Testing

### Load Testing

```typescript
import { paidFetch } from '@faremeter/fetch';

async function loadTest(concurrentRequests: number) {
  const promises = [];

  for (let i = 0; i < concurrentRequests; i++) {
    const promise = paidFetch(url, {
      wallet: testWallets[i % testWallets.length],
      maxAmount: 0.01
    });
    promises.push(promise);
  }

  const start = Date.now();
  const results = await Promise.allSettled(promises);
  const duration = Date.now() - start;

  const succeeded = results.filter(r => r.status === 'fulfilled').length;
  const failed = results.filter(r => r.status === 'rejected').length;

  console.log(`Load Test Results:
    Concurrent Requests: ${concurrentRequests}
    Duration: ${duration}ms
    Succeeded: ${succeeded}
    Failed: ${failed}
    Success Rate: ${(succeeded/concurrentRequests * 100).toFixed(2)}%
    Avg Response Time: ${(duration/concurrentRequests).toFixed(0)}ms
  `);
}

// Run tests
await loadTest(10);   // 10 concurrent
await loadTest(50);   // 50 concurrent
await loadTest(100);  // 100 concurrent
```

---

## Debugging Tips

### 1. Enable Verbose Logging

```typescript
// Enable debug mode
process.env.X402_DEBUG = 'true';

// Or in code
const client = new X402Client({
  debug: true,
  logLevel: 'verbose'
});
```

### 2. Inspect Transactions

```typescript
// Get full transaction details
const txInfo = await connection.getTransaction(signature, {
  commitment: 'confirmed'
});

console.log('Transaction:', {
  slot: txInfo.slot,
  blockTime: txInfo.blockTime,
  fee: txInfo.meta.fee,
  status: txInfo.meta.err ? 'failed' : 'success',
  logs: txInfo.meta.logMessages
});
```

### 3. Check Balances

```typescript
// Check USDC balance
const balance = await connection.getTokenAccountBalance(tokenAccount);
console.log('USDC Balance:', balance.value.uiAmount);

// Check SOL balance (for gas)
const solBalance = await connection.getBalance(publicKey);
console.log('SOL Balance:', solBalance / 1e9);
```

---

## Quick Commands

```bash
# Solana
solana balance --url devnet
solana airdrop 2 --url devnet
spl-token accounts --url devnet

# Base/EVM
cast balance <address> --rpc-url <rpc>
cast send <address> --value <amount> --rpc-url <rpc>

# Testing
npm test
npm run test:integration
npm run test:e2e
```

---

**Related Docs:**
- [x402scan Explorer Guide](../tools/x402scan-explorer-guide.md)
- [Solana Implementation Guide](./solana-implementation.md)
- [Security Best Practices](./security-best-practices.md)
- [Integration Patterns](./integration-patterns.md)
