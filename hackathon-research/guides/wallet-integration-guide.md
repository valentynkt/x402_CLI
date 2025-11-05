# Wallet Integration Guide

Complete guide to integrating wallets for x402 payments across all platforms and use cases.

## Quick Selection Matrix

| Wallet Type | Best For | Difficulty | Chains | Cost |
|-------------|----------|------------|--------|------|
| **Phantom** | Solana user apps | Easy ⭐ | Solana | Free |
| **CDP Embedded** | Seamless user UX | Easy ⭐ | Multi-chain | Usage-based |
| **CDP Server** | AI agents/backends | Medium ⭐⭐ | Multi-chain | Usage-based |
| **Crossmint** | Enterprise apps | Medium ⭐⭐ | 15+ chains | Enterprise |

---

## 1. Phantom Wallet (Solana)

**Best For:** User-facing Solana applications, browser extensions
**Integration Difficulty:** Easy ⭐
**Chains:** Solana only
**Website:** https://phantom.app/

### Installation

```bash
npm install @solana/wallet-adapter-phantom @solana/web3.js
```

### Basic Integration

```typescript
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';
import { Connection, clusterApiUrl } from '@solana/web3.js';

// Initialize wallet
const phantom = new PhantomWalletAdapter();

// Connect wallet
await phantom.connect();
console.log('Connected:', phantom.publicKey?.toString());

// Get public key
const publicKey = phantom.publicKey?.toString();

// Sign transaction
const signedTx = await phantom.signTransaction(transaction);

// Disconnect
await phantom.disconnect();
```

### React Integration

```typescript
import { WalletProvider } from '@solana/wallet-adapter-react';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui';

function App() {
  const wallets = [new PhantomWalletAdapter()];

  return (
    <WalletProvider wallets={wallets} autoConnect>
      <WalletModalProvider>
        <WalletMultiButton />
        <YourApp />
      </WalletModalProvider>
    </WalletProvider>
  );
}
```

### Using with x402

```typescript
import { paidFetch } from '@faremeter/fetch';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';

const wallet = new PhantomWalletAdapter();
await wallet.connect();

// Make paid request
const data = await paidFetch('https://api.example.com/data', {
  wallet,
  maxAmount: 0.01  // USDC
});
```

### Features

- ✅ Most popular Solana wallet (millions of users)
- ✅ Mobile + browser support
- ✅ Built-in token swap
- ✅ Excellent UX
- ✅ Hardware wallet support
- ✅ Free to use

### When to Use

- ✅ Building on Solana
- ✅ Need established user base
- ✅ Mobile apps
- ✅ Browser extensions
- ❌ Not for server-side/agents
- ❌ Not for multi-chain

---

## 2. CDP Embedded Wallets

**Best For:** Seamless user experience without key management
**Integration Difficulty:** Easy ⭐
**Chains:** Base, Ethereum, Polygon, Arbitrum
**Provider:** Coinbase

### Installation

```bash
npm install @coinbase/cdp-sdk
```

### Create User Wallet

```typescript
import { EmbeddedWallet } from '@coinbase/cdp-sdk';

// Create wallet for user (no keys exposed to user)
const wallet = await EmbeddedWallet.create({
  userId: 'user_123',
  email: 'user@example.com',
  chain: 'base'
});

console.log('Wallet created:', wallet.address);

// Send transaction
const tx = await wallet.send({
  to: 'recipient_address',
  amount: '0.001',
  token: 'USDC'
});

console.log('Transaction:', tx.hash);
```

### Policy Enforcement

```typescript
const wallet = await EmbeddedWallet.create({
  userId: 'user_123',
  policies: {
    maxTransactionAmount: '10.00',
    dailyLimit: '100.00',
    allowedTokens: ['USDC', 'ETH'],
    requiresApproval: false
  }
});
```

### Recovery Setup

```typescript
// Enable social recovery
await wallet.enableRecovery({
  method: 'email',
  backup: 'user@example.com'
});

// Recovery process
const recovered = await EmbeddedWallet.recover({
  userId: 'user_123',
  verificationCode: '123456'
});
```

### Features

- ✅ No key management for users
- ✅ User-friendly onboarding
- ✅ Policy enforcement
- ✅ Social recovery mechanisms
- ✅ Multi-chain support
- ✅ Autonomous agent support

### When to Use

- ✅ Consumer-facing apps
- ✅ Want seamless UX
- ✅ Need policy controls
- ✅ Multi-chain required
- ❌ Not for full decentralization
- ❌ Requires Coinbase integration

---

## 3. CDP Server Wallets

**Best For:** AI agents and backend autonomous operations
**Integration Difficulty:** Medium ⭐⭐
**Chains:** Base, Ethereum, Polygon, Arbitrum
**Provider:** Coinbase

### Installation

```bash
npm install @coinbase/cdp-sdk
```

### Create Agent Wallet

```typescript
import { ServerWallet } from '@coinbase/cdp-sdk';

// Create wallet for AI agent
const agentWallet = await ServerWallet.create({
  agentId: 'agent_456',
  chain: 'base',
  policies: {
    maxTransactionAmount: '1.00',
    allowedRecipients: ['verified_merchants'],
    requiresApproval: false
  }
});

console.log('Agent wallet:', agentWallet.address);

// Enable autonomous payments
await agentWallet.enableAutonomousPayments();
```

### Agent Payment Flow

```typescript
import { paidFetch } from '@faremeter/fetch';

// Agent makes autonomous payment
const data = await paidFetch('https://api.example.com/data', {
  wallet: agentWallet,
  maxAmount: 0.01
});

// Payment happens automatically without human intervention
```

### Policy Configuration

```typescript
const agentWallet = await ServerWallet.create({
  agentId: 'trading_agent',
  policies: {
    // Safety limits
    maxTransactionAmount: '5.00',
    dailyLimit: '50.00',

    // Allowed operations
    allowedTokens: ['USDC', 'USDT'],
    allowedRecipients: [
      'verified_merchant_1',
      'verified_merchant_2'
    ],

    // Approval settings
    requiresApproval: false,
    notifyOnTransaction: true,

    // Time restrictions
    allowedHours: { start: 9, end: 17 },
    allowedDays: ['mon', 'tue', 'wed', 'thu', 'fri']
  }
});
```

### Audit Trails

```typescript
// Get transaction history
const transactions = await agentWallet.getTransactionHistory({
  startDate: '2025-11-01',
  endDate: '2025-11-04',
  limit: 100
});

// Export for compliance
await agentWallet.exportAuditLog({
  format: 'csv',
  destination: 's3://bucket/audit-logs/'
});
```

### Features

- ✅ Trusted sender capabilities
- ✅ Policy-enforced safety
- ✅ Complete audit trails
- ✅ No manual intervention needed
- ✅ Autonomous operation
- ✅ Compliance-ready

### When to Use

- ✅ AI agents
- ✅ Backend services
- ✅ Automated workflows
- ✅ Server-side operations
- ❌ Not for user-facing wallets
- ❌ Requires CDP account

---

## 4. Crossmint Wallets

**Best For:** Enterprise multi-chain applications
**Integration Difficulty:** Medium ⭐⭐
**Chains:** 15+ networks (Solana, Base, Ethereum, etc.)
**Provider:** Crossmint

### Installation

```bash
npm install @crossmint/client-sdk
```

### Create Wallet

```typescript
import { CrossmintClient } from '@crossmint/client-sdk';

const crossmint = new CrossmintClient({
  apiKey: process.env.CROSSMINT_API_KEY
});

// Create non-custodial wallet
const wallet = await crossmint.wallets.create({
  type: 'non-custodial',
  chain: 'solana',  // or 'base', 'ethereum', 'polygon', etc.
  userId: 'user_789'
});

console.log('Wallet address:', wallet.address);
```

### Multi-Chain Support

```typescript
// Create wallets on multiple chains
const solanaWallet = await crossmint.wallets.create({
  type: 'non-custodial',
  chain: 'solana',
  userId: 'user_789'
});

const baseWallet = await crossmint.wallets.create({
  type: 'non-custodial',
  chain: 'base',
  userId: 'user_789'
});

// Use appropriate wallet per chain
const dataSolana = await paidFetch(solanaApiUrl, {
  wallet: solanaWallet,
  maxAmount: 0.01
});

const dataBase = await paidFetch(baseApiUrl, {
  wallet: baseWallet,
  maxAmount: 0.01
});
```

### Enterprise Features

```typescript
// Enterprise wallet with compliance
const enterpriseWallet = await crossmint.wallets.create({
  type: 'enterprise',
  chain: 'base',
  organizationId: 'org_123',
  policies: {
    requiresMultiSig: true,
    signers: 3,
    threshold: 2,
    complianceLevel: 'high'
  }
});
```

### Features

- ✅ 15+ blockchain networks
- ✅ Non-custodial smart wallets
- ✅ Enterprise compliance (VASP, SOC2)
- ✅ Traditional payment integration
- ✅ Multi-signature support
- ✅ Treasury management

### When to Use

- ✅ Enterprise applications
- ✅ Multi-chain requirements
- ✅ Compliance needed (VASP, SOC2)
- ✅ Traditional + crypto payments
- ❌ Not for simple prototypes
- ❌ Higher cost than alternatives

---

## Wallet Comparison

### Feature Matrix

| Feature | Phantom | CDP Embedded | CDP Server | Crossmint |
|---------|---------|--------------|------------|-----------|
| **Chains** | Solana | 4 chains | 4 chains | 15+ chains |
| **User UX** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | N/A | ⭐⭐⭐⭐ |
| **Agent Support** | ❌ | ⚠️ Limited | ✅ Best | ✅ Yes |
| **Key Management** | User | Managed | Managed | Managed |
| **Policies** | ❌ | ✅ | ✅ | ✅ |
| **Audit Trails** | ❌ | ✅ | ✅ | ✅ |
| **Cost** | Free | Usage | Usage | Enterprise |
| **Compliance** | Basic | Good | Good | Enterprise |
| **Mobile** | ✅ | ✅ | ❌ | ✅ |

### Cost Comparison

| Wallet | Setup | Per Transaction | Monthly |
|--------|-------|-----------------|---------|
| **Phantom** | Free | Gas only (~$0.0001) | $0 |
| **CDP Embedded** | Free | CDP fees + gas | Usage-based |
| **CDP Server** | Free | CDP fees + gas | Usage-based |
| **Crossmint** | Contact sales | Variable | Contact sales |

---

## Integration Patterns

### Pattern 1: Browser App with Phantom

```typescript
// For user-facing Solana apps
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';
import { paidFetch } from '@faremeter/fetch';

const wallet = new PhantomWalletAdapter();
await wallet.connect();

const data = await paidFetch(url, { wallet, maxAmount: 0.01 });
```

### Pattern 2: Seamless UX with CDP Embedded

```typescript
// For apps wanting zero-friction onboarding
import { EmbeddedWallet } from '@coinbase/cdp-sdk';

const wallet = await EmbeddedWallet.create({
  userId: req.user.id,
  email: req.user.email
});

const data = await paidFetch(url, { wallet, maxAmount: 0.01 });
```

### Pattern 3: AI Agent with CDP Server

```typescript
// For autonomous agents
import { ServerWallet } from '@coinbase/cdp-sdk';

const agentWallet = await ServerWallet.create({
  agentId: 'agent_123',
  policies: { maxTransactionAmount: '1.00' }
});

await agentWallet.enableAutonomousPayments();

// Agent pays automatically
const data = await paidFetch(url, { wallet: agentWallet });
```

### Pattern 4: Multi-Chain with Crossmint

```typescript
// For enterprise multi-chain apps
import { CrossmintClient } from '@crossmint/client-sdk';

const crossmint = new CrossmintClient({ apiKey: API_KEY });

const wallets = {
  solana: await crossmint.wallets.create({ chain: 'solana' }),
  base: await crossmint.wallets.create({ chain: 'base' })
};

// Use appropriate wallet per chain
const data = await paidFetch(url, {
  wallet: wallets[selectedChain],
  maxAmount: 0.01
});
```

---

## Security Best Practices

### 1. Never Expose Private Keys

```typescript
// ❌ NEVER DO THIS
const privateKey = 'your_private_key_here';

// ✅ DO THIS - use wallet adapters
const wallet = new PhantomWalletAdapter();
await wallet.connect();
```

### 2. Set Transaction Limits

```typescript
// Always set maxAmount
const data = await paidFetch(url, {
  wallet,
  maxAmount: 0.01  // Never pay more than this
});
```

### 3. Validate Recipients

```typescript
// Validate merchant before paying
const trustedMerchants = ['Merchant1Address', 'Merchant2Address'];

if (!trustedMerchants.includes(merchantAddress)) {
  throw new Error('Untrusted merchant');
}
```

### 4. Implement Policies

```typescript
// For server wallets, always use policies
const wallet = await ServerWallet.create({
  policies: {
    maxTransactionAmount: '5.00',
    dailyLimit: '50.00',
    allowedRecipients: trustedMerchants,
    requiresApproval: false
  }
});
```

### 5. Monitor and Alert

```typescript
// Set up monitoring
wallet.on('transaction', async (tx) => {
  console.log('Transaction:', tx.hash);

  if (tx.amount > 1.0) {
    await sendAlert('Large transaction detected', tx);
  }
});
```

---

## Troubleshooting

### Common Issues

**Phantom: "Wallet not detected"**
```typescript
if (!window.phantom?.solana) {
  alert('Please install Phantom wallet');
  window.open('https://phantom.app/', '_blank');
}
```

**CDP: "Insufficient funds"**
```typescript
// Check balance before transaction
const balance = await wallet.getBalance();
if (balance < amount) {
  throw new Error(`Insufficient balance: ${balance} USDC`);
}
```

**Crossmint: "API key invalid"**
```typescript
// Ensure API key is set
if (!process.env.CROSSMINT_API_KEY) {
  throw new Error('CROSSMINT_API_KEY not set');
}
```

---

## Quick Start Commands

```bash
# Phantom (Solana)
npm install @solana/wallet-adapter-phantom @solana/web3.js

# CDP Embedded/Server
npm install @coinbase/cdp-sdk

# Crossmint
npm install @crossmint/client-sdk

# x402 Integration
npm install @faremeter/fetch
```

---

**Related Docs:**
- [Integration Patterns Guide](./integration-patterns.md)
- [Solana Implementation Guide](./solana-implementation.md)
- [Security Best Practices](./security-best-practices.md)
- [Corbits Tool Guide](../tools/corbits-faremeter-guide.md)
