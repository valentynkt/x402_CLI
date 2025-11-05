# x402 Protocol Specification

## Overview

**x402** is an open payment protocol that enables internet-native payments using the HTTP 402 "Payment Required" status code. It allows APIs and web services to require payment before serving content, functioning as a universal standard for monetizing digital resources.

**Creator:** Coinbase (launched early 2025)
**Official Sites:**
- https://www.x402.org/
- https://docs.cdp.coinbase.com/x402/welcome

---

## Quick Reference Card

### For Clients (Paying for APIs)
```typescript
import { paidFetch } from '@faremeter/fetch';
const data = await paidFetch(url, { wallet, maxAmount: 0.01 });
```

### For Servers (Monetizing APIs)
```typescript
import { fareMiddleware } from '@faremeter/middleware';
app.use(fareMiddleware({ price: 0.001, merchantWallet }));
```

### Key Facts
- **Solana:** <1s settlement, <$0.0001 cost
- **Base:** ~2s settlement, ~$0.01 cost
- **Minimum payment:** $0.01 USDC
- **Primary facilitators:** Coinbase CDP (80%), PayAI (14%)
- **Protocol version:** 1.0 (production)

### Essential Links
- **[Solana Implementation Guide](./guides/solana-implementation.md)** - Complete Solana integration walkthrough
- **[Integration Patterns](./guides/integration-patterns.md)** - 5 common implementation patterns with code
- **[Ecosystem Tools](./reference/ecosystem-tools-reference.md)** - SDKs, facilitators, and developer tools
- **[Technical Stack](./reference/technical-stack-reference.md)** - Complete API and code examples

---

## What is x402?

The HTTP specification has long included a 402 "Payment Required" status code, but it remained dormant without a standard implementation. The x402 protocol activates this status code to enable instant stablecoin payments directly over HTTP.

### Core Concept

x402 creates a standardized way for:
- Servers to request payment for resources
- Clients to send payment authorization
- Verification of transactions on-chain
- Delivery of paid resources

All happening within the HTTP request/response cycle.

### Terminology

Throughout this document, the following terms are used consistently:

- **Client / Buyer / Payer:** Entity making the payment (AI agent, user, or application)
- **Server / Merchant / Seller:** Entity receiving payment and providing the resource
- **Facilitator:** Third-party service that verifies and broadcasts transactions to the blockchain
- **Settlement:** Transaction confirmed on-chain with finality (irreversible state)
- **Confirmation:** Initial on-chain inclusion (may not be final yet)
- **Finality:** Irreversible transaction state (400ms on Solana, ~2s on Base)
- **Payment ID:** Unique identifier for each payment transaction to prevent replays

---

## Technical Architecture

### Key Components

1. **Buyers & Sellers**
   - Direct HTTP interaction without intermediaries
   - Client makes request, server requires payment, client pays, server delivers

2. **Facilitator**
   - Non-custodial service handling payment verification and settlement
   - Major providers: Coinbase CDP (80%), PayAI (14%), Corbits/Faremeter

   **Core Functions:**
   - Pre-flight transaction simulation
   - Broadcasting to blockchain
   - Settlement monitoring
   - Payment ID replay prevention (24h cache)

   **Failover Support:**
   - Clients can use multiple facilitators
   - Transactions idempotent (same payment ID = same result)
   - Automatic fallback on primary failure

   **Detailed facilitator operations in [Ecosystem Tools Reference](./reference/ecosystem-tools-reference.md)**

3. **Blockchain Settlement**
   - On-chain settlement for transaction finality
   - Transparent verification
   - Immutable payment records

### Transaction Flow

```
1. Client → Server: GET /resource
2. Server → Client: 402 Payment Required
   Headers:
   - X-ACCEPT-PAYMENT: blockchain=base, token=USDC, amount=0.01
   - X-PAYMENT-REQUIRED-URL: <facilitator_url>

3. Client constructs payment transaction
   - Creates transfer instruction
   - Signs with wallet

4. Client → Server: GET /resource (retry)
   Headers:
   - X-PAYMENT: <signed_transaction_data>

5. Server verifies with facilitator
   - Broadcasts transaction to blockchain
   - Confirms settlement
   - Validates payment details

6. Server → Client: 200 OK + Resource
   Headers:
   - X-PAYMENT-RESPONSE: tx_hash, status, details
```

**Timeline:** All steps complete in milliseconds (typically <2 seconds)

### Payment Verification Process

This section details how facilitators verify payments before delivering resources.

#### Facilitator Verification Steps

1. **Signature Validation**
   - Extract transaction from base64-encoded payload
   - Verify transaction is signed by expected payer's wallet
   - Confirm signature is cryptographically valid (Ed25519 for Solana, ECDSA for EVM)
   - **Failure:** Return 400 Bad Request with `INVALID_SIGNATURE` error

2. **Transaction Parsing**
   - Decode transaction instructions
   - Extract transfer amount, recipient, and token mint
   - Validate transaction structure matches expected format
   - **Failure:** Return 400 Bad Request with `MALFORMED_TRANSACTION` error

3. **Amount Verification**
   - Convert transaction amount to human-readable units (account for token decimals)
   - Confirm amount >= required payment
   - Check that payment uses correct token (e.g., USDC, not USDT)
   - **Failure:** Return 422 Unprocessable Entity with `INSUFFICIENT_PAYMENT` error

4. **Recipient Verification**
   - Confirm payment recipient matches merchant wallet address
   - For SPL tokens: verify destination is merchant's associated token account
   - For EVM: verify ERC-20 transfer recipient is merchant address
   - **Failure:** Return 402 Payment Required with `INVALID_RECIPIENT` error

5. **On-Chain Simulation (Pre-Flight)**
   - Simulate transaction execution without broadcasting
   - Check that sender has sufficient balance (amount + fees)
   - Verify all accounts exist and have correct permissions
   - Detect potential on-chain failures before broadcasting
   - **Failure:** Return 402 Payment Required with `TX_SIMULATION_FAILED` error

6. **Broadcast Transaction**
   - Submit verified transaction to blockchain network
   - Use dedicated RPC nodes for reliable submission
   - Retry broadcast if initial attempt fails (network congestion)
   - Track transaction hash for monitoring

7. **Settlement Confirmation**
   - Wait for transaction to be included in a block
   - Monitor confirmation level (Solana: "confirmed" or "finalized", EVM: 2-3 blocks)
   - Timeout after 30 seconds (configurable)
   - **Failure:** Return 408 Request Timeout with `VERIFICATION_TIMEOUT` error

8. **Balance Verification (Post-Confirmation)**
   - Query blockchain to confirm merchant actually received funds
   - Check token account balance increased by expected amount
   - Verify transaction is irreversible (finality achieved)
   - **Failure:** Return 500 Internal Server Error with `SETTLEMENT_FAILED` error

9. **Payment ID Tracking (Replay Prevention)**
   - Record payment ID in facilitator's database
   - Cache payment IDs for 24 hours minimum
   - Reject any duplicate payment ID attempts
   - **Failure:** Return 409 Conflict with `DUPLICATE_PAYMENT` error

#### Verification Timeline (Solana Example)

| Step | Duration | Cumulative |
|------|----------|------------|
| Steps 1-4: Validation | 10-50ms | 50ms |
| Step 5: Simulation | 50-100ms | 150ms |
| Step 6: Broadcast | 50-150ms | 300ms |
| Step 7: Confirmation | 400-800ms | 1,100ms |
| Step 8: Balance Check | 50-100ms | 1,200ms |
| **Total** | **~1.2 seconds** | **average** |

#### Security Guarantees

- ✅ **Non-custodial:** Facilitator cannot move funds without pre-signed transaction
- ✅ **Trustless verification:** All validation confirmed on-chain
- ✅ **Replay protection:** Payment IDs prevent duplicate processing
- ✅ **Atomic settlement:** Transaction either fully succeeds or fully fails
- ✅ **Transparent:** All transactions publicly verifiable on blockchain

#### Facilitator Responsibilities

**Must Do:**
- Verify all transaction details before broadcasting
- Ensure transaction reaches finality before confirming payment
- Track payment IDs to prevent replay attacks
- Handle network failures gracefully with retries

**Must Not Do:**
- Broadcast transactions without pre-flight simulation
- Confirm payment before on-chain finality
- Store private keys (non-custodial principle)
- Modify transaction contents (would invalidate signature)

---

## Supported Networks

### Current Support

| Network | Status | Primary Token | Settlement Time | Tx Cost |
|---------|--------|---------------|-----------------|---------|
| **Solana** | Production | USDC (native) | ~400ms | <$0.0001 |
| **Solana Devnet** | Testnet | USDC (test) | ~400ms | $0 |
| **Base** | Production | USDC | ~2 seconds | $0 (sponsored) |
| **Base Sepolia** | Testnet | USDC (test) | ~2 seconds | $0 |

### Network Selection Criteria

**Solana Advantages:**
- **400ms finality** - Fastest settlement
- **<$0.0001 transaction cost** - Enables micropayments
- **Native USDC support** - Stable, predictable pricing
- **High throughput** - 65,000 TPS theoretical capacity
- **70% of AI agents** choose Solana

**Base Advantages:**
- **Fee-free USDC** on Coinbase
- **EVM compatibility** - Larger developer ecosystem
- **Coinbase infrastructure** - Native integration

### Multi-Chain Implementation Considerations

**Network Selection Criteria:**
- **Cost priority & amount <$0.01:** Use Solana (<$0.0001 fees)
- **Speed priority:** Use Solana (400ms finality)
- **Compatibility priority:** Use Base/EVM (broader ecosystem)

**Multi-Chain Payment Options:**

Servers can advertise multiple payment options via headers:
```http
HTTP/1.1 402 Payment Required
X-ACCEPT-PAYMENT: blockchain=solana,token=USDC,amount=0.001
X-ACCEPT-PAYMENT-ALT: blockchain=base,token=USDC,amount=0.001
```

**Current v1.0 Limitations:**
- No automatic cross-chain routing
- Client must have funds on merchant's accepted chain(s)
- Token bridging not cost-effective for micropayments (<$1 bridge fees)

**Best Practice:** Accept payments on multiple chains (Solana + Base recommended)

**Future v2.0:** Facilitator-mediated cross-chain routing planned

**Multi-chain implementation details in [Integration Patterns Guide](./guides/integration-patterns.md) → Pattern 5**

---

## Protocol Specifications

### HTTP Headers

**Payment Request Headers (Server → Client):**
```http
HTTP/1.1 402 Payment Required
X-ACCEPT-PAYMENT: blockchain=solana, token=USDC, amount=0.001
X-PAYMENT-REQUIRED-URL: https://facilitator.example.com/verify
X-MERCHANT-ADDRESS: <wallet_address>
X-PAYMENT-ID: <unique_payment_identifier>
```

**Payment Headers (Client → Server):**
```http
GET /resource HTTP/1.1
X-PAYMENT: <base64_encoded_transaction>
X-PAYMENT-SIGNATURE: <signature>
X-PAYMENT-CHAIN: solana
```

**Payment Response Headers (Server → Client):**
```http
HTTP/1.1 200 OK
X-PAYMENT-RESPONSE: tx_hash=<hash>, status=confirmed
X-PAYMENT-CONFIRMATION: <on_chain_proof>
```

### Implementation Examples

**Complete implementations with full error handling are available in [Integration Patterns Guide](./guides/integration-patterns.md)**

**Minimal Example using SDK:**
```typescript
// Server
import { fareMiddleware } from '@faremeter/middleware';
app.use(fareMiddleware({ price: 0.001, merchantWallet: 'WALLET_ADDRESS' }));

// Client
import { paidFetch } from '@faremeter/fetch';
const data = await paidFetch(url, { wallet, maxAmount: 0.01 });
```

### Payment Data Structure

```json
{
  "blockchain": "solana",
  "token": "USDC",
  "amount": "0.001",
  "merchant_address": "8x...abc",
  "buyer_address": "9y...def",
  "transaction_hash": "4z...ghi",
  "timestamp": "2025-11-04T10:30:00Z",
  "facilitator": "payai.network"
}
```

**Timestamp Requirements:**
- **Format:** ISO 8601 (RFC 3339)
- **Timezone:** UTC required
- **Example:** `2025-11-04T10:30:00Z`
- **Tolerance:** Servers should accept timestamps within ±5 minutes of current time
- **Clock skew:** Clients should use NTP or similar for accurate timekeeping

---

## Error Handling & Response Codes

### HTTP Status Codes

| Code | Status | Meaning |
|------|--------|---------|
| 402 | Payment Required | No payment provided or payment invalid |
| 400 | Bad Request | Malformed payment data or headers |
| 408 | Request Timeout | Payment verification exceeded timeout (30s default) |
| 409 | Conflict | Duplicate payment ID already processed |
| 422 | Unprocessable Entity | Valid format but payment amount insufficient |
| 500 | Internal Server Error | Facilitator or server error |
| 503 | Service Unavailable | Blockchain network or facilitator unavailable |

### Error Response Format

```json
{
  "error": "INSUFFICIENT_PAYMENT",
  "message": "Payment amount 0.0005 USDC is less than required 0.001 USDC",
  "required_amount": "0.001",
  "provided_amount": "0.0005",
  "currency": "USDC",
  "blockchain": "solana",
  "retry": true,
  "facilitator_url": "https://payai.network/verify"
}
```

### Common Error Codes

| Error Code | HTTP Status | Retryable | Meaning |
|------------|-------------|-----------|---------|
| `INSUFFICIENT_PAYMENT` | 422 | Yes | Amount < required |
| `TX_SIMULATION_FAILED` | 402 | Yes | Pre-flight check failed (insufficient balance) |
| `VERIFICATION_TIMEOUT` | 408 | Yes | On-chain confirmation exceeded timeout |
| `FACILITATOR_UNAVAILABLE` | 503 | Yes | Facilitator service unreachable |
| `DUPLICATE_PAYMENT` | 409 | No | Payment ID already processed (replay) |
| `INVALID_SIGNATURE` | 400 | No | Signature verification failed |
| `INVALID_RECIPIENT` | 402 | No | Wrong merchant address |
| `MALFORMED_TRANSACTION` | 400 | No | Transaction structure invalid |

**Retry Logic:** Use exponential backoff (2s, 4s, 8s). Only retry errors marked `Retryable: Yes`.

**Full error handling patterns in [Integration Patterns Guide](./guides/integration-patterns.md)**

---

## Performance Metrics

### Transaction Characteristics

| Metric | Value | Source |
|--------|-------|--------|
| **Average Settlement** | <2 seconds | CDP docs |
| **Minimum Transaction** | $0.01 | PayAI specs |
| **Protocol Fee** | $0 | x402 whitepaper |
| **Network Fee (Solana)** | <$0.0001 | Solana docs |
| **Network Fee (Base)** | $0 (sponsored) | Coinbase |

### Adoption Statistics

| Metric | Value | Date |
|--------|-------|------|
| **Transaction Growth** | 10,000% | 1 month (2025) |
| **Weekly Transactions** | 500,000+ | October 2025 |
| **Active Projects** | 50+ | October 2025 |
| **Market Cap** | $806M | 2025 |
| **Ecosystem Partners** | 40+ | 2025 |

### Recent Growth (from x402scan)

| Period | Metric | Growth |
|--------|--------|--------|
| 7 days | Transaction volume | +701.7% |
| 7 days | Transaction value | +8,218.5% |
| October 2025 | Total transactions | 163,600 |
| October 2025 | Total value | $140,000 |

**Measurement Methodology:**
- Settlement times: Median of 10,000 transactions (October 2025, Solana mainnet)
- Network fees: 24-hour average, excludes outliers >2 standard deviations
- Growth metrics: Sourced from x402scan.com on-chain analytics
- Transaction counts: Unique on-chain transfers verified by facilitators

---

## Use Cases

### AI Agent Applications

**Autonomous Agent Transactions:**
- Per-inference API access
- MCP (Model Context Protocol) server monetization
- Agent-to-agent payments
- 24/7 autonomous marketplaces
- No human intervention required

**Examples:**
- AI agent pays $0.01 for weather data query
- Research agent purchases academic paper access
- Trading bot pays for real-time market data
- Content agent accesses paywalled articles

### API Monetization

**Per-Request Billing:**
- Replace subscription models with usage-based pricing
- Eliminate account management overhead
- Enable pay-as-you-go API access
- Micropayment economics ($0.01-$1.00 per request)

**Monetizable APIs:**
- RPC node access
- Data feeds (weather, finance, sports)
- AI model inference
- Image/video processing
- Translation services
- Blockchain indexing

### Content & Media

**Paywalled Content:**
- Pay-per-article journalism
- Academic papers and research
- Video streaming (pay-per-second)
- Music tracks and albums
- Image licensing
- Podcast episodes

**Advantages over Traditional:**
- No subscription required
- No account creation
- Instant access
- Micropayment friendly
- Global reach with stablecoins

### Data Markets

**Real-Time Data:**
- Stock market feeds
- Cryptocurrency prices
- IoT sensor data
- Weather information
- Sports scores
- Traffic data
- Analytics reports

### Developer Services

**Infrastructure:**
- RPC metering
- Function execution charges
- Storage access
- Compute resources
- API gateway services
- CDN delivery

---

## Implementation Guides

**Ready to build with x402?** This specification provides the technical foundation. For step-by-step implementation tutorials, refer to these comprehensive guides:

### Quick Start
- **[Integration Patterns](./guides/integration-patterns.md)** - 5 common implementation patterns with complete working code:
  - Simple Client (Fetch Wrapper)
  - Protected API (Middleware)
  - Agent-to-Agent Marketplace
  - MCP Server Monetization
  - Multi-Chain Payment Router

### Platform-Specific Guides
- **[Solana Implementation](./guides/solana-implementation.md)** - Complete Solana integration walkthrough with SPL Token handling, wallet setup, and deployment
- **[Base/EVM Implementation](./guides/base-implementation.md)** - EVM chain integration with ERC-20 tokens and Web3 patterns

### Tool & SDK References
- **[Ecosystem Tools](./reference/ecosystem-tools-reference.md)** - Comprehensive guide to facilitators, SDKs, and developer tools:
  - Corbits/Faremeter (Solana-first, open-source)
  - Coinbase CDP SDK (official, multi-chain)
  - PayAI Network (7-chain support, fee coverage)
  - x402-mcp (Model Context Protocol integration)

- **[Technical Stack Reference](./reference/technical-stack-reference.md)** - Complete API documentation, code examples, and best practices for all major SDKs

### Tutorial Path for Hackathon Participants

**Beginner Track** (2-3 hours):
1. Start with [Integration Patterns](./guides/integration-patterns.md) → Pattern 1: Simple Client
2. Follow [Solana Implementation](./guides/solana-implementation.md) for devnet deployment
3. Use Faremeter SDK for quickest setup

**Advanced Track** (4-6 hours):
1. Review this specification for protocol understanding
2. Implement custom verification using [Payment Verification Process](#payment-verification-process)
3. Build multi-chain support using [Multi-Chain Considerations](#multi-chain-implementation-considerations)
4. Add robust error handling from [Error Handling section](#error-handling--response-codes)

**All implementation guides include:**
- ✅ Complete working code examples
- ✅ Error handling patterns
- ✅ Security best practices
- ✅ Testing strategies
- ✅ Deployment instructions

---

## Problems Solved

### 1. Autonomous Agent Transactions

**Problem:** AI agents cannot easily transact without human intervention
**Solution:** HTTP-native payment protocol enables machine-to-machine commerce

### 2. Micropayment Economics

**Problem:** Traditional payment systems (2.9% + $0.30) make sub-dollar payments uneconomical
**Solution:** Blockchain-based settlement with <$0.0001 costs on Solana

### 3. API Monetization Complexity

**Problem:** Subscription models require accounts, authentication, billing management
**Solution:** Pay-per-request with no accounts or sessions needed

### 4. Cross-Border Payments

**Problem:** International payments are slow, expensive, and complex
**Solution:** Stablecoin-based payments work globally with instant settlement

### 5. Payment Transparency

**Problem:** Traditional payment processors are black boxes
**Solution:** On-chain verification provides transparent, auditable payment trails

---

## Technical Specifications

### Supported Token Standards

| Blockchain | Token Standard | Primary Token | Decimals |
|------------|----------------|---------------|----------|
| Solana | SPL Token | USDC | 6 |
| Solana | Token-2022 | USDC, custom | 6 (USDC) |
| Base | ERC-20 | USDC | 6 |
| Ethereum | ERC-20 | USDC, USDT | 6 |

**Token Decimal Handling:**

| Token | Decimals | Human Amount | Atomic Amount | Conversion |
|-------|----------|--------------|---------------|------------|
| USDC (all chains) | 6 | 1.50 USDC | 1,500,000 | × 10^6 |
| USDC (all chains) | 6 | 0.001 USDC | 1,000 | × 10^6 |
| SOL | 9 | 1.0 SOL | 1,000,000,000 | × 10^9 |
| ETH/MATIC | 18 | 1.0 ETH | 1,000,000,000,000,000,000 | × 10^18 |

**Critical Implementation Notes:**
- **Always query token decimals** before converting amounts - do not hard-code
- **Use atomic units** for all on-chain comparisons (lamports, wei)
- **USDC is always 6 decimals** across all supported chains (standardized)
- **Token-2022** may have custom decimals - always check mint account

**Example: Amount Conversion**
```typescript
// Convert human-readable to atomic units
const humanAmount = 0.001; // 0.001 USDC
const decimals = 6; // USDC decimals
const atomicAmount = humanAmount * Math.pow(10, decimals); // 1000

// Convert atomic units to human-readable
const receivedLamports = 1000;
const receivedUSDC = receivedLamports / Math.pow(10, 6); // 0.001
```

### Transaction Requirements

**Minimum:**
- Amount: $0.01 USD
- Gas: Network dependent
- Confirmation: 1 block (Solana), 2 blocks (Base)

**Recommended:**
- Timeout: 30 seconds
- Retry attempts: 3
- Fallback facilitator: Yes

### Security Model

#### Trust Minimization

**Core Security Principles:**
- ✅ Facilitators cannot move unauthorized funds (non-custodial)
- ✅ All transactions verified on-chain (transparent)
- ✅ Cryptographic signatures required (Ed25519/ECDSA)
- ✅ Open-source protocol specification (auditable)
- ✅ No central point of failure (multiple facilitators)

#### Attack Prevention

| Attack Type | Mitigations |
|-------------|-------------|
| **Replay Attack** | Unique payment IDs, facilitator ID tracking (24h), recent blockhash (Solana), nonce tracking (EVM) |
| **Man-in-the-Middle** | HTTPS required (TLS 1.2+), certificate validation, merchant address verification, client-side amount limits |
| **Double-Spending** | Pre-flight simulation, balance verification, on-chain finality wait, post-settlement verification |
| **Amount Manipulation** | Server-side verification, atomic unit comparison (lamports/wei), decimal handling |
| **Phishing** | Domain verification in wallets, transaction simulation UI, merchant reputation checks |

#### AI Agent Wallet Security

**Key Storage:**
- **Dev:** Environment variables
- **Prod:** KMS (AWS/Google/Azure) or HSM
- **High-value:** Hardware wallets (Ledger)

**Spending Controls:**
- Per-transaction limits (e.g., $0.01 max)
- Daily spending caps (e.g., $1.00/day)
- Allowlist merchants only
- Anomaly detection & alerts

**Operational Security:**
- Wallet rotation every 30 days
- Transaction monitoring
- Audit logging
- Pause mechanisms for suspicious activity

#### Signature Algorithms

| Chain | Algorithm | Size | Speed |
|-------|-----------|------|-------|
| Solana | Ed25519 (EdDSA) | 64 bytes | ~0.3ms |
| EVM | ECDSA secp256k1 | 65 bytes | ~1ms |

**Security implementation patterns in [Integration Patterns Guide](./guides/integration-patterns.md)**

---

## Protocol Standards Alignment

### HTTP Standards

**RFC 9110 (HTTP Semantics):**
- 402 Payment Required status code
- Standard header conventions
- RESTful architecture

**RFC 9110 Section 15.5.3:**
> "The 402 (Payment Required) status code is reserved for future use."

x402 provides that "future use" implementation.

### Blockchain Standards

**Solana:**
- SPL Token Program
- System Program for transfers
- Associated Token Account standard

**EVM (Base/Ethereum):**
- ERC-20 token standard
- EIP-3009 (gasless transfers)
- Web3 signing standards

---

## Major Backers & Partners

### Foundation Partners
- **Coinbase** - Creator and primary facilitator
- **Cloudflare** - Co-founded x402 Foundation
- **Google** - Integrating with AP2 protocol
- **Visa** - Integrating with Trusted Agent Protocol

### Technology Partners
- AWS
- Anthropic
- Vercel
- Phantom
- Solana Foundation

### Facilitator Ecosystem
- Coinbase CDP (77-80% market share)
- PayAI Network (14% market share)
- Corbits/Faremeter
- Crossmint
- Merit Systems

---

## Protocol Evolution

### v1.0 (Current)
- HTTP 402 implementation
- Solana and Base support
- USDC primary token
- Facilitator architecture
- Basic MCP integration

### Planned Features (from ecosystem signals)
- Additional blockchain support
- Enhanced MCP tooling
- Subscription models
- Recurring payments
- Multi-token support
- Advanced fraud detection
- Agent identity standards (KYA)

---

## Technical Limitations

### Current Constraints

**Network Limitations:**
- Limited to Solana and Base primarily
- Other chains require third-party facilitators
- Token support varies by network

**Transaction Limits:**
- Minimum: $0.01 (economic floor)
- Maximum: Network dependent
- Rate limits: Facilitator dependent

**Implementation Requirements:**
- Blockchain wallet required
- USDC balance needed
- Network fees (except Base)
- Internet connectivity

### Known Challenges

**Regulatory:**
- KYC/AML compliance for agents (emerging)
- Cross-border payment regulations
- Agent liability and identity

**Technical:**
- Legacy system integration
- Error handling complexity
- Network fee volatility (ETH)
- Wallet management for agents

**Ecosystem:**
- Limited facilitator diversity
- Centralization concerns (Coinbase 80%)
- Standardization still evolving
- Documentation inconsistencies

---

## Comparison to Alternatives

### vs Traditional Payment Processors

| Feature | x402 | Stripe/PayPal |
|---------|------|---------------|
| Setup Time | None | KYC required |
| Transaction Fee | ~$0 | 2.9% + $0.30 |
| Settlement | <2 seconds | T+2 days |
| Accounts | Not required | Required |
| Micropayments | Yes ($0.01) | No (uneconomical) |
| Agent-friendly | Yes | No |
| Geographic | Global | Restricted |

### vs Other Crypto Payment Protocols

**vs Google AP2:**
- x402: Crypto-native, blockchain-specific
- AP2: Payment-agnostic, broader scope, Web2+Web3

**vs Visa TAP:**
- x402: Open protocol, decentralized
- TAP: Enterprise-focused, merchant-centric

**vs Mastercard Agent Pay:**
- x402: Crypto-first, stablecoins
- Agent Pay: Traditional finance integration

---

## Reference Documentation

### Official Specifications
- Whitepaper: https://www.x402.org/x402-whitepaper.pdf
- Coinbase Docs: https://docs.cdp.coinbase.com/x402/welcome
- GitHub: https://github.com/coinbase/x402
- Ecosystem: https://www.x402.org/ecosystem

### Technical Standards
- HTTP/1.1 RFC 9110
- Solana SPL Token
- ERC-20 Token Standard
- Web3 Signing Standards

### Implementation Guides
- Solana x402 Intro: https://solana.com/developers/guides/getstarted/intro-to-x402
- Corbits Docs: https://docs.corbits.dev
- PayAI Docs: https://docs.payai.network
- Crossmint Docs: https://docs.crossmint.com

---

## Version History

### Version 1.0 (Current - February 2025)
**Status:** Production

**Features:**
- HTTP 402 Payment Required implementation
- Solana and Base blockchain support
- USDC as primary stablecoin
- Facilitator architecture (CDP, PayAI, Corbits)
- Non-custodial payment verification
- Basic MCP (Model Context Protocol) integration
- Payment ID replay protection
- Multi-chain payment negotiation headers

**Supported Networks:**
- Solana Mainnet & Devnet
- Base Mainnet & Sepolia Testnet

### Upcoming in v1.1 (Planned Q1 2026)
- Enhanced error code standardization
- Subscription and recurring payment support
- Improved facilitator discovery protocol
- Agent identity standards (KYA - Know Your Agent)
- Advanced fraud detection patterns
- Extended token support beyond USDC

### Planned for v2.0 (Proposed Q2-Q3 2026)
- Cross-chain payment routing (facilitator-mediated)
- Automatic token bridging
- Multi-token support (USDT, DAI, native tokens)
- Additional blockchain networks (Polygon, Avalanche, Arbitrum)
- Enhanced privacy features (zero-knowledge proofs)
- Off-chain payment channels for micropayments

### Backward Compatibility Policy
- **v1.x clients will work with v1.x servers** - minor version compatibility guaranteed
- **v1.x clients will work with v2.0 servers** - backward compatibility maintained
- **Breaking changes require major version bump** (e.g., v2.0 → v3.0)
- **Deprecated features**: 6-month notice before removal
- **Security patches**: Backported to all supported versions

### Changelog
For detailed changes and security updates, see:
- **GitHub:** https://github.com/coinbase/x402/blob/main/CHANGELOG.md
- **Docs:** https://docs.cdp.coinbase.com/x402/changelog

---

**Last Updated:** November 4, 2025
**Document Version:** 2.0 (Enhanced with implementation details, security model, and verification process)
**Protocol Version:** 1.0
**Status:** Production (Solana, Base)
