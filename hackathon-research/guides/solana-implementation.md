# Solana Implementation Guide

Complete guide to implementing x402 payments on Solana with SPL tokens (USDC).

## Why Solana for x402?

| Advantage | Value | Impact |
|-----------|-------|--------|
| **Speed** | <1 second settlement | Real-time payments |
| **Cost** | <$0.0001 per transaction | True micropayments |
| **Finality** | ~400ms | No chargebacks |
| **Ecosystem** | Largest x402 adoption | Most tools/support |

---

## Protocol Overview

x402 on Solana uses HTTP 402 Payment Required status codes with SPL token transfers.

### Payment Flow (5 Steps)

```
1. Client → Server: GET /premium
2. Server → Client: 402 Payment Required
   {
     "amount": "100",
     "token": "USDC",
     "recipient": "TokenAccountAddress",
     "mint": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
   }

3. Client: Creates signed SPL token transfer transaction
4. Client → Server: GET /premium
   Header: X-PAYMENT: {base64-encoded transaction}
5. Server: Verifies, submits, confirms
   Server → Client: 200 OK + content
```

---

## Server Implementation

### Express.js Server with SPL Tokens

```typescript
import express from 'express';
import {
  Connection,
  Transaction,
  PublicKey,
  clusterApiUrl
} from '@solana/web3.js';
import {
  getAssociatedTokenAddress,
  TOKEN_PROGRAM_ID
} from '@solana/spl-token';

const app = express();
const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

// Configuration
const USDC_MINT_DEVNET = new PublicKey('4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU');
const USDC_MINT_MAINNET = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');
const MERCHANT_WALLET = new PublicKey('YourSolanaWalletAddress');
const PRICE_USDC = 100; // 0.0001 USDC (6 decimals)

// Get merchant's token account
const merchantTokenAccount = await getAssociatedTokenAddress(
  USDC_MINT_DEVNET,
  MERCHANT_WALLET
);

// Protected endpoint
app.get('/premium', async (req, res) => {
  const paymentHeader = req.headers['x-payment'];

  if (!paymentHeader) {
    // No payment - return 402
    return res.status(402).json({
      error: 'Payment required',
      amount: PRICE_USDC.toString(),
      token: 'USDC',
      recipient: merchantTokenAccount.toString(),
      mint: USDC_MINT_DEVNET.toString(),
      network: 'solana-devnet'
    });
  }

  try {
    // Parse payment header
    const payment = JSON.parse(paymentHeader);
    const txBuffer = Buffer.from(payment.payload.serializedTransaction, 'base64');
    const transaction = Transaction.from(txBuffer);

    // Verify transaction
    const isValid = await verifyTransaction(transaction);

    if (!isValid) {
      return res.status(402).json({ error: 'Invalid payment' });
    }

    // Submit transaction
    const signature = await connection.sendRawTransaction(
      transaction.serialize()
    );

    // Await confirmation
    await connection.confirmTransaction(signature, 'confirmed');

    // Verify balance change
    const confirmed = await verifyBalanceChange(signature);

    if (!confirmed) {
      return res.status(402).json({ error: 'Payment not confirmed' });
    }

    // Payment confirmed - return content
    res.json({
      success: true,
      content: 'Premium data here',
      txSignature: signature
    });

  } catch (error) {
    console.error('Payment error:', error);
    res.status(402).json({ error: 'Payment verification failed' });
  }
});

// Verify transaction structure
async function verifyTransaction(tx: Transaction): Promise<boolean> {
  try {
    // Check instructions
    for (const ix of tx.instructions) {
      // Check if it's a Token Program instruction
      if (ix.programId.equals(TOKEN_PROGRAM_ID)) {
        // Instruction type byte (3 = Transfer)
        const instructionType = ix.data[0];

        if (instructionType === 3) { // Transfer
          // Read amount (u64 little-endian at offset 1)
          const amount = ix.data.readBigUInt64LE(1);

          // Verify amount
          if (Number(amount) < PRICE_USDC) {
            return false;
          }

          // Verify destination (3rd account in transfer)
          const destination = ix.keys[1]?.pubkey;
          if (!destination?.equals(merchantTokenAccount)) {
            return false;
          }

          return true;
        }
      }
    }

    return false;
  } catch (error) {
    console.error('Verification error:', error);
    return false;
  }
}

// Verify balance actually changed on-chain
async function verifyBalanceChange(signature: string): Promise<boolean> {
  try {
    const txInfo = await connection.getTransaction(signature, {
      commitment: 'confirmed'
    });

    if (!txInfo || !txInfo.meta) {
      return false;
    }

    // Check token balance changes
    const preBalances = txInfo.meta.preTokenBalances || [];
    const postBalances = txInfo.meta.postTokenBalances || [];

    // Find merchant token account
    for (let i = 0; i < postBalances.length; i++) {
      const post = postBalances[i];
      const pre = preBalances.find(p => p.accountIndex === post.accountIndex);

      if (post.owner === MERCHANT_WALLET.toString()) {
        const preAmount = pre?.uiTokenAmount.amount || '0';
        const postAmount = post.uiTokenAmount.amount || '0';
        const change = BigInt(postAmount) - BigInt(preAmount);

        // Verify we received at least the required amount
        return change >= BigInt(PRICE_USDC);
      }
    }

    return false;
  } catch (error) {
    console.error('Balance verification error:', error);
    return false;
  }
}

app.listen(3000, () => {
  console.log('Solana x402 server running on port 3000');
  console.log('Merchant token account:', merchantTokenAccount.toString());
});
```

---

## Client Implementation

### Node.js Client

```typescript
import {
  Connection,
  PublicKey,
  Transaction,
  SystemProgram,
  clusterApiUrl,
  Keypair
} from '@solana/web3.js';
import {
  getOrCreateAssociatedTokenAccount,
  createTransferInstruction,
  TOKEN_PROGRAM_ID
} from '@solana/spl-token';
import fetch from 'node-fetch';

const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

// Your wallet
const payer = Keypair.fromSecretKey(
  Uint8Array.from(JSON.parse(process.env.SOLANA_PRIVATE_KEY))
);

async function buyPremiumContent() {
  // Step 1: Request without payment
  const response402 = await fetch('http://localhost:3000/premium');

  if (response402.status !== 402) {
    throw new Error('Expected 402 Payment Required');
  }

  const paymentDetails = await response402.json();
  console.log('Payment required:', paymentDetails);

  // Step 2: Create token accounts if needed
  const mintPubkey = new PublicKey(paymentDetails.mint);
  const recipientPubkey = new PublicKey(paymentDetails.recipient);

  // Get or create sender's token account
  const senderTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    mintPubkey,
    payer.publicKey
  );

  // Step 3: Create transfer instruction
  const transferIx = createTransferInstruction(
    senderTokenAccount.address,  // Source
    recipientPubkey,              // Destination
    payer.publicKey,              // Owner
    parseInt(paymentDetails.amount), // Amount
    [],                           // Multi-signers
    TOKEN_PROGRAM_ID
  );

  // Step 4: Build transaction
  const transaction = new Transaction().add(transferIx);
  transaction.recentBlockhash = (
    await connection.getLatestBlockhash()
  ).blockhash;
  transaction.feePayer = payer.publicKey;

  // Step 5: Sign transaction
  transaction.sign(payer);

  // Step 6: Serialize to base64
  const serialized = transaction.serialize().toString('base64');

  // Step 7: Retry request with payment
  const paymentHeader = JSON.stringify({
    x402Version: 1,
    scheme: 'exact',
    network: 'solana-devnet',
    payload: {
      serializedTransaction: serialized
    }
  });

  const response200 = await fetch('http://localhost:3000/premium', {
    headers: {
      'X-PAYMENT': paymentHeader
    }
  });

  if (response200.status === 200) {
    const content = await response200.json();
    console.log('Success! Content:', content);
    return content;
  } else {
    const error = await response200.json();
    throw new Error(`Payment failed: ${error.error}`);
  }
}

// Run
buyPremiumContent()
  .then(() => console.log('Complete'))
  .catch(console.error);
```

---

## Using Faremeter SDK (Easier)

### Server with Faremeter Middleware

```typescript
import express from 'express';
import { fareMiddleware } from '@faremeter/middleware';

const app = express();

app.use(fareMiddleware({
  facilitator: 'https://payai.network',
  price: 0.001,  // USDC
  merchantWallet: process.env.MERCHANT_WALLET,
  network: 'solana'
}));

app.get('/premium', (req, res) => {
  // Payment automatically verified by middleware
  res.json({ content: 'Premium data' });
});

app.listen(3000);
```

### Client with Faremeter Fetch

```typescript
import { paidFetch } from '@faremeter/fetch';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';

const wallet = new PhantomWalletAdapter();
await wallet.connect();

// Automatic payment handling
const content = await paidFetch('http://localhost:3000/premium', {
  wallet,
  maxAmount: 0.01
});

console.log('Content:', content);
```

---

## Network Configuration

### Devnet (Testing)

```typescript
const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
const USDC_MINT = new PublicKey('4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU');
```

**Devnet Faucets:**
- SOL: https://faucet.solana.com/
- USDC Devnet: Use Faremeter test tools

### Mainnet (Production)

```typescript
const connection = new Connection(
  'https://api.mainnet-beta.solana.com',
  'confirmed'
);
const USDC_MINT = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');
```

**Mainnet RPC Providers:**
- **Helius:** https://helius.dev (via Corbits proxy)
- **Triton:** https://triton.one (via Corbits proxy)
- **QuickNode:** https://quicknode.com
- **Public:** https://api.mainnet-beta.solana.com (rate-limited)

---

## Token Configuration

### USDC Addresses

| Network | Mint Address | Decimals |
|---------|--------------|----------|
| **Mainnet** | `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v` | 6 |
| **Devnet** | `4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU` | 6 |

### Amount Calculation

```typescript
// USDC has 6 decimals
const usdcAmount = 1.50;  // $1.50
const lamports = usdcAmount * 1_000_000;  // 1,500,000

// Example: 0.0001 USDC = 100 lamports
const microPayment = 0.0001 * 1_000_000;  // 100
```

---

## Security Best Practices

### 1. Always Verify Three Things

```typescript
// ✅ 1. Verify amount
if (amount < PRICE_USDC) {
  throw new Error('Insufficient payment');
}

// ✅ 2. Verify recipient
if (!destination.equals(merchantTokenAccount)) {
  throw new Error('Wrong recipient');
}

// ✅ 3. Verify on-chain confirmation
const confirmed = await verifyBalanceChange(signature);
if (!confirmed) {
  throw new Error('Payment not confirmed on-chain');
}
```

### 2. Simulate Before Submitting

```typescript
// Simulate transaction before submitting
const simulation = await connection.simulateTransaction(transaction);

if (simulation.value.err) {
  throw new Error(`Simulation failed: ${simulation.value.err}`);
}

// Only then submit
const signature = await connection.sendRawTransaction(transaction.serialize());
```

### 3. Handle Token Account Creation

```typescript
// Check if recipient token account exists
const recipientTokenAccount = await connection.getAccountInfo(recipientPubkey);

if (!recipientTokenAccount) {
  // Create associated token account instruction
  const createATAIx = createAssociatedTokenAccountInstruction(
    payer.publicKey,           // Payer
    recipientTokenAccount,     // ATA address
    MERCHANT_WALLET,           // Owner
    USDC_MINT                  // Mint
  );

  transaction.add(createATAIx);
}

transaction.add(transferIx);
```

---

## Error Handling

### Common Errors

**"Account does not exist"**
```typescript
// Create associated token account first
const merchantTokenAccount = await getOrCreateAssociatedTokenAccount(
  connection,
  payer,
  USDC_MINT,
  MERCHANT_WALLET
);
```

**"Insufficient funds"**
```typescript
// Check balance before payment
const balance = await connection.getTokenAccountBalance(senderTokenAccount);
if (balance.value.amount < PRICE_USDC.toString()) {
  throw new Error('Insufficient USDC balance');
}
```

**"Transaction simulation failed"**
```typescript
// Common causes:
// 1. Wrong mint address
// 2. Token account doesn't exist
// 3. Insufficient SOL for gas
// 4. Invalid instruction data

// Always simulate first
const result = await connection.simulateTransaction(tx);
console.log('Simulation logs:', result.value.logs);
```

---

## Performance Optimization

### 1. Use Confirmations Wisely

```typescript
// Fast: 'processed' (not recommended for payments)
// Balanced: 'confirmed' (recommended)
// Secure: 'finalized' (slowest, most secure)

const signature = await connection.sendRawTransaction(tx);
await connection.confirmTransaction(signature, 'confirmed');
```

### 2. Batch Requests

```typescript
// Get multiple accounts in one call
const accounts = await connection.getMultipleAccountsInfo([
  merchantTokenAccount,
  customerTokenAccount
]);
```

### 3. Use WebSocket for Real-time

```typescript
// Subscribe to account changes
const subscriptionId = connection.onAccountChange(
  merchantTokenAccount,
  (accountInfo) => {
    console.log('Balance updated:', accountInfo.lamports);
  },
  'confirmed'
);
```

---

## Testing Checklist

- [ ] Test on devnet first
- [ ] Verify token account creation
- [ ] Test with insufficient balance
- [ ] Test with wrong recipient
- [ ] Test with amount too small
- [ ] Simulate all transactions
- [ ] Verify balance changes on-chain
- [ ] Test error handling
- [ ] Monitor gas costs
- [ ] Test with actual Phantom wallet

---

## Quick Start Commands

```bash
# Install dependencies
npm install @solana/web3.js @solana/spl-token express

# Or use Faremeter SDK
npm install @faremeter/fetch @faremeter/middleware

# Get devnet SOL
solana airdrop 2 YourAddress --url devnet

# Check USDC balance
spl-token balance EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
```

---

## Resources

**Official Solana Docs:**
- x402 Guide: https://solana.com/developers/guides/getstarted/intro-to-x402
- SPL Token: https://spl.solana.com/token
- Web3.js: https://solana-labs.github.io/solana-web3.js/

**Development Tools:**
- Solana Explorer (Devnet): https://explorer.solana.com/?cluster=devnet
- Solana Explorer (Mainnet): https://explorer.solana.com/
- Transaction Inspector: https://www.x402scan.com/

**Related Docs:**
- [Integration Patterns Guide](./integration-patterns.md)
- [Wallet Integration Guide](./wallet-integration-guide.md)
- [Blockchain Networks Reference](../reference/blockchain-networks.md)
- [PayAI Network Tool Guide](../tools/payai-network-guide.md)
