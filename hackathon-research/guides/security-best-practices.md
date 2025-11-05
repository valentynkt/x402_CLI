# Security Best Practices Guide

Essential security practices for implementing x402 payments safely.

## Core Principles

### 1. Verify Everything

```typescript
// ✅ Always verify three things:
// 1. Amount is correct
// 2. Recipient is correct
// 3. Payment is confirmed on-chain
```

### 2. Never Trust Client Data

```typescript
// ❌ NEVER trust client-provided amounts
const amount = req.body.amount;  // NO!

// ✅ Always use server-defined prices
const PRICE = 0.001;  // Server decides price
```

### 3. Defense in Depth

```typescript
// Layer multiple security checks:
// 1. Input validation
// 2. Transaction simulation
// 3. On-chain verification
// 4. Balance confirmation
```

---

## Payment Verification

### Server-Side Verification (Critical)

```typescript
import { Connection, PublicKey } from '@solana/web3.js';

async function verifyPayment(
  transaction: Transaction,
  expectedAmount: number,
  merchantWallet: PublicKey
): Promise<boolean> {

  // 1. Verify transaction structure
  const transferIx = findTransferInstruction(transaction);
  if (!transferIx) {
    throw new Error('No transfer instruction found');
  }

  // 2. Verify recipient
  const recipient = transferIx.keys[1]?.pubkey;
  if (!recipient.equals(merchantWallet)) {
    throw new Error('Wrong recipient');
  }

  // 3. Verify amount
  const amount = transferIx.data.readBigUInt64LE(1);
  if (Number(amount) < expectedAmount) {
    throw new Error('Insufficient payment amount');
  }

  // 4. Simulate before submitting
  const simulation = await connection.simulateTransaction(transaction);
  if (simulation.value.err) {
    throw new Error(`Simulation failed: ${simulation.value.err}`);
  }

  // 5. Submit and confirm
  const signature = await connection.sendRawTransaction(
    transaction.serialize()
  );
  await connection.confirmTransaction(signature, 'confirmed');

  // 6. Verify on-chain balance change
  const confirmed = await verifyBalanceChange(signature, merchantWallet, expectedAmount);
  if (!confirmed) {
    throw new Error('Balance not updated on-chain');
  }

  return true;
}
```

### Balance Verification

```typescript
async function verifyBalanceChange(
  signature: string,
  merchantWallet: PublicKey,
  expectedAmount: number
): Promise<boolean> {

  const txInfo = await connection.getTransaction(signature, {
    commitment: 'confirmed'
  });

  if (!txInfo || !txInfo.meta) {
    return false;
  }

  // Check actual token balance changes
  const preBalances = txInfo.meta.preTokenBalances || [];
  const postBalances = txInfo.meta.postTokenBalances || [];

  for (const post of postBalances) {
    const pre = preBalances.find(p => p.accountIndex === post.accountIndex);

    if (post.owner === merchantWallet.toString()) {
      const preAmount = pre?.uiTokenAmount.amount || '0';
      const postAmount = post.uiTokenAmount.amount || '0';
      const change = BigInt(postAmount) - BigInt(preAmount);

      // Verify we actually received the funds
      return change >= BigInt(expectedAmount);
    }
  }

  return false;
}
```

---

## Input Validation

### Validate All Inputs

```typescript
// ✅ Validate transaction structure
function validateTransaction(tx: Transaction) {
  // Check signatures exist
  if (!tx.signatures || tx.signatures.length === 0) {
    throw new Error('Transaction not signed');
  }

  // Check recent blockhash
  if (!tx.recentBlockhash) {
    throw new Error('Missing recent blockhash');
  }

  // Check fee payer
  if (!tx.feePayer) {
    throw new Error('Missing fee payer');
  }

  // Check instructions
  if (!tx.instructions || tx.instructions.length === 0) {
    throw new Error('No instructions in transaction');
  }

  return true;
}
```

### Sanitize User Data

```typescript
// ✅ Sanitize and validate
function validateWalletAddress(address: string): PublicKey {
  try {
    const pubkey = new PublicKey(address);

    // Check it's on curve (valid Solana address)
    if (!PublicKey.isOnCurve(pubkey.toBytes())) {
      throw new Error('Invalid public key');
    }

    return pubkey;
  } catch (error) {
    throw new Error('Invalid wallet address');
  }
}
```

---

## Rate Limiting

### Implement Rate Limits

```typescript
import rateLimit from 'express-rate-limit';

// Per IP rate limiting
const limiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100, // 100 requests per window
  message: 'Too many requests, please try again later'
});

app.use('/api/', limiter);

// Per wallet rate limiting
const walletLimiter = new Map();

function checkWalletLimit(wallet: string): boolean {
  const now = Date.now();
  const limit = walletLimiter.get(wallet) || { count: 0, reset: now };

  // Reset if window expired
  if (now > limit.reset) {
    limit.count = 0;
    limit.reset = now + 15 * 60 * 1000; // 15 min window
  }

  limit.count++;
  walletLimiter.set(wallet, limit);

  return limit.count <= 100; // Max 100 per wallet per 15 min
}
```

---

## Amount Limits

### Set Transaction Limits

```typescript
// Client-side: maxAmount protection
const data = await paidFetch(url, {
  wallet,
  maxAmount: 0.01  // Never pay more than this
});

// Server-side: enforce limits
const LIMITS = {
  MIN_PAYMENT: 0.0001,  // $0.0001 minimum
  MAX_PAYMENT: 10.0,     // $10 maximum
  DAILY_LIMIT: 100.0     // $100 per wallet per day
};

function validatePaymentAmount(amount: number, wallet: string): boolean {
  // Check minimum
  if (amount < LIMITS.MIN_PAYMENT) {
    throw new Error('Payment amount too small');
  }

  // Check maximum
  if (amount > LIMITS.MAX_PAYMENT) {
    throw new Error('Payment amount too large');
  }

  // Check daily limit
  const dailyTotal = getDailyTotal(wallet);
  if (dailyTotal + amount > LIMITS.DAILY_LIMIT) {
    throw new Error('Daily limit exceeded');
  }

  return true;
}
```

---

## Wallet Security

### Private Key Management

```typescript
// ❌ NEVER expose private keys
const privateKey = 'your_private_key_here';  // NO!

// ❌ NEVER commit keys to git
const wallet = Keypair.fromSecretKey(
  Uint8Array.from([1, 2, 3, ...])  // NO!
);

// ✅ Use environment variables
const wallet = Keypair.fromSecretKey(
  Uint8Array.from(JSON.parse(process.env.SOLANA_PRIVATE_KEY))
);

// ✅ Use wallet adapters (no key exposure)
const phantom = new PhantomWalletAdapter();
await phantom.connect();

// ✅ Use managed wallets (CDP, Crossmint)
const wallet = await EmbeddedWallet.create({
  userId: 'user_123'
});
```

### Key Storage

```bash
# ✅ Use secure environment variables
export SOLANA_PRIVATE_KEY='[...]'
export MERCHANT_WALLET='...'

# ✅ Use secrets management
# AWS Secrets Manager
# Google Cloud Secret Manager
# HashiCorp Vault
# Azure Key Vault

# ❌ NEVER commit to git
# Add to .gitignore:
.env
.env.local
*.key
*.pem
```

---

## Recipient Validation

### Whitelist Trusted Recipients

```typescript
// Maintain whitelist of trusted merchants
const TRUSTED_MERCHANTS = new Set([
  'MerchantWallet1...',
  'MerchantWallet2...',
  'MerchantWallet3...'
]);

function validateRecipient(recipient: PublicKey): boolean {
  const recipientStr = recipient.toString();

  if (!TRUSTED_MERCHANTS.has(recipientStr)) {
    console.warn('Untrusted merchant:', recipientStr);
    return false;
  }

  return true;
}

// Use in client
const data = await paidFetch(url, {
  wallet,
  maxAmount: 0.01,
  onPaymentRequest: (details) => {
    // User confirms payment to recipient
    if (!validateRecipient(details.recipient)) {
      throw new Error('Untrusted recipient');
    }
  }
});
```

---

## Error Handling

### Secure Error Messages

```typescript
// ❌ Never expose internal details
catch (error) {
  res.status(500).json({
    error: error.message,          // NO! Leaks info
    stack: error.stack,            // NO! Security risk
    wallet: merchantWallet         // NO! Private info
  });
}

// ✅ Return generic errors to client
catch (error) {
  console.error('Payment error:', error);  // Log internally

  res.status(402).json({
    error: 'Payment verification failed',  // Generic message
    code: 'PAYMENT_FAILED'                 // Error code only
  });
}
```

### Log Security Events

```typescript
// Log suspicious activity
function logSecurityEvent(event: string, details: any) {
  console.log('[SECURITY]', {
    timestamp: new Date().toISOString(),
    event,
    details,
    ip: req.ip,
    userAgent: req.get('user-agent')
  });

  // Send to security monitoring
  securityMonitor.alert(event, details);
}

// Examples
logSecurityEvent('INVALID_PAYMENT', { wallet, reason });
logSecurityEvent('RATE_LIMIT_EXCEEDED', { wallet, count });
logSecurityEvent('LARGE_PAYMENT_DETECTED', { amount, wallet });
```

---

## Simulation Before Submission

### Always Simulate First

```typescript
async function submitPayment(transaction: Transaction) {
  // 1. Simulate first
  const simulation = await connection.simulateTransaction(transaction);

  // 2. Check for errors
  if (simulation.value.err) {
    console.error('Simulation failed:', simulation.value.err);
    console.error('Logs:', simulation.value.logs);
    throw new Error(`Transaction will fail: ${simulation.value.err}`);
  }

  // 3. Check compute units
  if (simulation.value.unitsConsumed > 1_000_000) {
    console.warn('High compute usage:', simulation.value.unitsConsumed);
  }

  // 4. Only then submit
  const signature = await connection.sendRawTransaction(
    transaction.serialize()
  );

  return signature;
}
```

---

## Monitoring & Alerts

### Set Up Alerts

```typescript
interface SecurityAlert {
  type: 'FAILED_PAYMENT' | 'LARGE_PAYMENT' | 'RATE_LIMIT' | 'SUSPICIOUS';
  severity: 'low' | 'medium' | 'high' | 'critical';
  details: any;
}

async function sendSecurityAlert(alert: SecurityAlert) {
  // Log to monitoring system
  console.error('[SECURITY ALERT]', alert);

  // Send email for critical alerts
  if (alert.severity === 'critical') {
    await sendEmail({
      to: 'security@example.com',
      subject: `CRITICAL: ${alert.type}`,
      body: JSON.stringify(alert, null, 2)
    });
  }

  // Send to Slack/Discord
  await notifyChannel(alert);

  // Store in database
  await db.securityAlerts.insert(alert);
}

// Monitor for suspicious activity
function monitorPayments(payment: Payment) {
  // Failed payment from same wallet repeatedly
  const failures = recentFailures.get(payment.wallet) || 0;
  if (failures > 5) {
    sendSecurityAlert({
      type: 'SUSPICIOUS',
      severity: 'high',
      details: { wallet: payment.wallet, failures }
    });
  }

  // Large payment
  if (payment.amount > 10.0) {
    sendSecurityAlert({
      type: 'LARGE_PAYMENT',
      severity: 'medium',
      details: payment
    });
  }
}
```

---

## API Security

### Authentication & Authorization

```typescript
// Add API key authentication for server-to-server
app.use('/api', (req, res, next) => {
  const apiKey = req.headers['x-api-key'];

  if (!apiKey || !isValidApiKey(apiKey)) {
    return res.status(401).json({ error: 'Unauthorized' });
  }

  next();
});

// Rate limit per API key
const apiKeyLimiter = new Map();

function checkApiKeyLimit(apiKey: string): boolean {
  // Similar to wallet limiter
  const limit = apiKeyLimiter.get(apiKey) || { count: 0, reset: Date.now() };

  if (Date.now() > limit.reset) {
    limit.count = 0;
    limit.reset = Date.now() + 60 * 60 * 1000; // 1 hour
  }

  limit.count++;
  apiKeyLimiter.set(apiKey, limit);

  return limit.count <= 1000; // 1000 requests per hour
}
```

---

## HTTPS/TLS

### Enforce HTTPS

```typescript
// Redirect HTTP to HTTPS
app.use((req, res, next) => {
  if (!req.secure && process.env.NODE_ENV === 'production') {
    return res.redirect(`https://${req.headers.host}${req.url}`);
  }
  next();
});

// Set security headers
import helmet from 'helmet';
app.use(helmet());

// HSTS (Force HTTPS)
app.use((req, res, next) => {
  res.setHeader(
    'Strict-Transport-Security',
    'max-age=31536000; includeSubDomains; preload'
  );
  next();
});
```

---

## Security Checklist

### Before Production

- [ ] All API keys in environment variables
- [ ] HTTPS/TLS enabled
- [ ] Rate limiting implemented
- [ ] Input validation on all endpoints
- [ ] Amount limits configured
- [ ] Payment verification complete (3 layers)
- [ ] Balance confirmation implemented
- [ ] Error messages sanitized
- [ ] Security logging enabled
- [ ] Alerts configured
- [ ] Wallet whitelist (if applicable)
- [ ] Simulation before submission
- [ ] No private keys in code
- [ ] Dependencies updated
- [ ] Security audit completed

### Ongoing

- [ ] Monitor security alerts
- [ ] Review logs daily
- [ ] Update dependencies monthly
- [ ] Rotate API keys quarterly
- [ ] Security audit annually
- [ ] Incident response plan tested
- [ ] Backup merchant wallet
- [ ] Test disaster recovery

---

## Common Vulnerabilities

### 1. Replay Attacks

```typescript
// ✅ Prevent replay attacks
const processedTxs = new Set();

function checkReplay(signature: string): boolean {
  if (processedTxs.has(signature)) {
    throw new Error('Transaction already processed');
  }

  processedTxs.add(signature);
  return true;
}
```

### 2. Race Conditions

```typescript
// ✅ Use locks for critical sections
const locks = new Map();

async function processPayment(signature: string) {
  // Acquire lock
  if (locks.has(signature)) {
    throw new Error('Payment already being processed');
  }

  locks.set(signature, true);

  try {
    // Process payment
    await verifyAndProcess(signature);
  } finally {
    // Release lock
    locks.delete(signature);
  }
}
```

### 3. Integer Overflow

```typescript
// ✅ Check for overflow
function safeAdd(a: bigint, b: bigint): bigint {
  const result = a + b;

  if (result < a || result < b) {
    throw new Error('Integer overflow');
  }

  return result;
}
```

---

**Related Docs:**
- [Testing and Monitoring Guide](./testing-and-monitoring.md)
- [Integration Patterns Guide](./integration-patterns.md)
- [Solana Implementation Guide](./solana-implementation.md)
- [Wallet Integration Guide](./wallet-integration-guide.md)
