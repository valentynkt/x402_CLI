# Common Troubleshooting Guide

**Applies to:** All sponsor integrations (Visa TAP, ATXP, Switchboard, CDP Wallets, Gradient)

This guide covers generic issues that apply across all sponsor technologies. For sponsor-specific problems, see the relevant integration guide.

---

## SDK & Authentication Issues

### Issue: Invalid API Credentials

**Symptoms:**
```
401 Unauthorized
{"error": "Invalid API key"}
```

**Common Causes:**
- Incorrect API key format
- Expired credentials
- Environment variable not loaded
- Whitespace in .env file

**Solutions:**
```bash
# Verify .env file format (no quotes, no spaces)
API_KEY=abc123xyz456

# Load environment variables
source .env
# or
export $(cat .env | xargs)

# Test key format
echo $API_KEY | wc -c  # Check length
```

---

### Issue: Network Timeout

**Symptoms:**
```
Error: Request timeout after 30000ms
```

**Solutions:**
```javascript
// Increase timeout
const response = await fetch(url, {
  timeout: 60000  // 60 seconds
});

// Add retry logic
async function fetchWithRetry(url, options, maxRetries = 3) {
  for (let i = 0; i < maxRetries; i++) {
    try {
      return await fetch(url, options);
    } catch (error) {
      if (i === maxRetries - 1) throw error;
      await new Promise(resolve => setTimeout(resolve, 1000 * Math.pow(2, i)));
    }
  }
}
```

---

## Solana Blockchain Errors

### Issue: Insufficient SOL for Transaction Fees

**Symptoms:**
```
Error: Transaction simulation failed - insufficient funds
```

**Solutions:**
```bash
# Check SOL balance
solana balance <YOUR_WALLET> --url devnet

# Get testnet SOL
solana airdrop 1 <YOUR_WALLET> --url devnet

# Or visit faucet
# https://faucet.solana.com
```

**Minimum Required:**
- Devnet: ~0.1 SOL for testing
- Mainnet: ~0.01 SOL for transaction fees

---

### Issue: Transaction Failed

**Symptoms:**
```
Error: Transaction failed with error code 0x1
```

**Common Causes:**
- Insufficient balance (SOL or USDC)
- Account not initialized
- Invalid recipient address
- Program error
- RPC rate limiting

**Debugging Steps:**
```javascript
// Get detailed error
const tx = await connection.sendTransaction(transaction, [wallet]);
const confirmation = await connection.confirmTransaction(tx);
console.log('Transaction details:', confirmation);

// Check account info
const accountInfo = await connection.getAccountInfo(wallet.publicKey);
console.log('Account balance:', accountInfo.lamports / 1e9, 'SOL');

// Verify recipient exists
const recipientInfo = await connection.getAccountInfo(recipientAddress);
if (!recipientInfo) {
  console.error('Recipient account does not exist');
}
```

---

### Issue: RPC Rate Limiting

**Symptoms:**
```
429 Too Many Requests
{"error": "Rate limit exceeded"}
```

**Solutions:**
```javascript
// Use paid RPC endpoint (recommended for production)
const connection = new Connection('https://api.mainnet-beta.solana.com', {
  commitment: 'confirmed',
  httpHeaders: {
    'Authorization': `Bearer ${RPC_API_KEY}`
  }
});

// Or implement rate limiting
const rateLimiter = {
  calls: [],
  maxCalls: 100,
  windowMs: 60000,

  async wait() {
    const now = Date.now();
    this.calls = this.calls.filter(time => now - time < this.windowMs);

    if (this.calls.length >= this.maxCalls) {
      const oldestCall = Math.min(...this.calls);
      const waitTime = this.windowMs - (now - oldestCall);
      await new Promise(resolve => setTimeout(resolve, waitTime));
    }

    this.calls.push(now);
  }
};

// Use before each RPC call
await rateLimiter.wait();
const result = await connection.getBalance(publicKey);
```

---

## x402 Payment Issues

### Issue: Payment Verification Failed

**Symptoms:**
```
Error: Payment signature invalid or not found on-chain
```

**Common Causes:**
- Transaction not confirmed yet
- Wrong network (devnet vs mainnet)
- Facilitator can't find transaction
- Transaction failed

**Solutions:**
```javascript
// Wait for transaction confirmation
const signature = await sendPaymentTransaction();
await connection.confirmTransaction(signature, 'confirmed');

// Verify signature exists on-chain
const status = await connection.getSignatureStatus(signature);
if (!status.value) {
  throw new Error('Transaction not found');
}
if (status.value.err) {
  throw new Error(`Transaction failed: ${status.value.err}`);
}

// Then retry request with payment
const response = await fetch(url, {
  headers: { 'X-PAYMENT': signature }
});
```

---

### Issue: Facilitator Error

**Symptoms:**
```
402 Payment Required
X-ERROR: facilitator_unavailable
```

**Solutions:**
```javascript
// Try alternative facilitator
const facilitators = [
  'https://x402.org/facilitator',
  'https://payai.network',
  'https://self-hosted-facilitator.com'
];

for (const facilitatorUrl of facilitators) {
  try {
    const response = await paidFetch(url, {
      wallet,
      connection,
      facilitator: facilitatorUrl,
      maxAmount: 1.0
    });
    return response;
  } catch (error) {
    console.warn(`Facilitator ${facilitatorUrl} failed:`, error);
  }
}

throw new Error('All facilitators unavailable');
```

---

## Development Best Practices

### Logging

**Always log critical information:**
```javascript
function logTransaction(action, details) {
  console.log(JSON.stringify({
    timestamp: new Date().toISOString(),
    action: action,
    wallet: details.wallet?.substring(0, 8),
    amount: details.amount,
    signature: details.signature?.substring(0, 16),
    status: details.status,
    error: details.error?.message
  }, null, 2));
}

// Usage
logTransaction('payment_attempt', {
  wallet: wallet.publicKey.toString(),
  amount: 0.5,
  signature: null,
  status: 'pending'
});
```

---

### Testing in Sandbox

**Always test in sandbox/devnet first:**
```javascript
const config = {
  network: process.env.NODE_ENV === 'production' ? 'mainnet' : 'devnet',
  rpcUrl: process.env.NODE_ENV === 'production'
    ? 'https://api.mainnet-beta.solana.com'
    : 'https://api.devnet.solana.com',
  usdc: process.env.NODE_ENV === 'production'
    ? 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v'  // Mainnet USDC
    : '4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU'  // Devnet USDC
};
```

---

### Error Recovery

**Implement comprehensive error handling:**
```javascript
async function safeApiCall(apiFunction, fallbackValue = null) {
  try {
    return await apiFunction();
  } catch (error) {
    console.error('API call failed:', error);

    // Log to monitoring service
    if (process.env.NODE_ENV === 'production') {
      await logToMonitoring(error);
    }

    // Return fallback or rethrow
    if (fallbackValue !== null) {
      return fallbackValue;
    }
    throw error;
  }
}

// Usage
const price = await safeApiCall(
  () => oracleAPI.getPrice('SOL'),
  { price: null, error: true }
);
```

---

## Debugging Techniques

### Enable Debug Mode

Most SDKs support debug mode:
```javascript
// Visa TAP
const tapClient = new TAPClient({
  debug: true,
  logLevel: 'verbose'
});

// ATXP
const atxp = new ATXPClient({
  debug: true
});

// Switchboard
process.env.DEBUG = 'switchboard:*';

// CDP
const cdp = new Coinbase({
  debug: true
});
```

---

### Check Sandbox Status

Verify sandbox environments are operational:
```javascript
async function checkSandboxHealth() {
  const checks = {
    solana: await fetch('https://api.devnet.solana.com', {
      method: 'POST',
      body: JSON.stringify({
        jsonrpc: '2.0',
        id: 1,
        method: 'getHealth'
      })
    }),
    facilitator: await fetch('https://x402.org/facilitator/health'),
  };

  for (const [service, response] of Object.entries(checks)) {
    console.log(`${service}: ${response.ok ? 'OK' : 'DEGRADED'}`);
  }
}
```

---

## Getting Help

### Check Status Pages

- Solana: https://status.solana.com
- Coinbase: https://status.coinbase.com
- x402 Facilitator: https://x402.org/status

### Community Support

- Solana Discord: https://discord.gg/solana
- Hackathon Discord: Check official hackathon website
- GitHub Issues: Check each SDK's GitHub repo

### Documentation

- Solana Docs: https://docs.solana.com
- x402 Spec: https://docs.x402.org
- Refer to specific sponsor integration guides for detailed troubleshooting

---

**Last Updated:** November 4, 2025
