# MCP Server with x402 Payments - Quick Start

A minimal Model Context Protocol (MCP) server with x402 payment integration. Get a working paid API endpoint running in under 2 minutes.

## Prerequisites

- Rust 1.75 or higher (`rustc --version`)
- x402-dev CLI installed (`cargo install x402-dev`)
- Solana devnet access (free)

## Quick Start

```bash
# 1. Initialize the example (creates files in current directory)
x402-dev examples init mcp-server-starter
cd mcp-server-starter

# 2. Start the x402-dev daemon (handles payments)
x402-dev start

# 3. Run the MCP server
cargo run

# 4. Test the paid endpoint
curl http://localhost:8402/data
# Returns: 402 Payment Required with x402 invoice
```

**Total time**: ~90 seconds from init to running server

## How It Works

### x402 Payment Protocol Flow

```
Client Request → Server (402 + Invoice) → Client Pays → Server (200 + Data)
```

1. **Client makes request**: `GET /data`
2. **Server returns 402**: HTTP 402 Payment Required with x402 invoice header
3. **x402-dev daemon**: Automatically generates Solana devnet invoice
4. **Client pays**: Submits payment to Solana (handled by x402-dev)
5. **Payment verified**: x402-dev validates on-chain transaction
6. **Server returns data**: HTTP 200 with protected content

### Key Concepts

- **402 Status Code**: HTTP standard for "Payment Required"
- **x402 Protocol**: Adds `X-402-Invoice` header with payment details
- **Solana Integration**: Fast, low-cost payments on devnet/mainnet
- **Automatic Verification**: x402-dev handles all payment validation

## What You Get

- ✅ Working HTTP server with payment protection
- ✅ Automatic invoice generation
- ✅ Payment verification without custom code
- ✅ Production-ready error handling
- ✅ Clear inline documentation

## Project Structure

```
mcp-server-starter/
├── src/
│   └── main.rs          # HTTP server with x402 integration
├── Cargo.toml           # Rust dependencies
├── .x402dev.yaml        # x402-dev configuration
└── README.md            # This file
```

## Example Response

### Before Payment (402)
```http
HTTP/1.1 402 Payment Required
X-402-Invoice: {"amount":1000,"currency":"USDC","address":"7xKX..."}
Content-Type: application/json

{
  "error": "Payment required",
  "amount_lamports": 1000,
  "message": "Pay 0.000001 SOL to access this endpoint"
}
```

### After Payment (200)
```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "data": "Protected content accessed successfully",
  "timestamp": "2025-11-12T10:30:45Z"
}
```

## Configuration

Edit `.x402dev.yaml` to customize:

```yaml
port: 8402                                    # Server port
solana_rpc: https://api.devnet.solana.com    # Solana network
log_level: info                               # Logging verbosity
payment_amount: 1000                          # Default payment (lamports)
```

## Next Steps

### Production Deployment
1. Switch to Solana mainnet RPC endpoint
2. Configure custom payment amounts per endpoint
3. Add authentication/API keys for paid users
4. Implement rate limiting and caching

### Advanced Features
- **Multiple pricing tiers**: Different amounts per endpoint
- **Subscription model**: Time-based access with x402
- **Metered billing**: Pay-per-use with automatic invoicing
- **Webhook integration**: Real-time payment notifications

### Full Documentation
- [x402-dev CLI Documentation](../../docs/x402-dev-cli.md)
- [x402 Protocol Specification](../../docs/x402-protocol.md)
- [Solana Integration Guide](../../docs/solana-integration.md)
- [Production Best Practices](../../docs/production-guide.md)

## Troubleshooting

### Server won't start
```bash
# Check if port 8402 is available
lsof -i :8402

# Use different port
# Edit .x402dev.yaml and change port value
```

### Payment verification fails
```bash
# Check x402-dev daemon status
x402-dev status

# Restart daemon
x402-dev restart

# Check logs
x402-dev logs
```

### Solana RPC issues
```bash
# Test RPC connection
curl https://api.devnet.solana.com -X POST \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}'

# Use alternative devnet RPC
# Edit .x402dev.yaml: solana_rpc: https://devnet.helius-rpc.com
```

## Contributing

Found a bug or want to improve this example?
- Open an issue: [GitHub Issues](https://github.com/yourusername/x402-dev/issues)
- Submit a PR: [Contributing Guide](../../CONTRIBUTING.md)

## License

MIT License - See [LICENSE](../../LICENSE) for details

---

**Built with**: Rust, Actix-web, x402-dev, Solana
**Estimated setup time**: < 2 minutes
**Production ready**: Yes (with mainnet configuration)
