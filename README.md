# x402-dev

A CLI tool for developing and testing x402 payment protocol integrations on Solana.

## Overview

x402-dev is a hybrid Rust + TypeScript command-line tool that enables developers to build, test, and deploy AI agents with autonomous payment capabilities using the x402 protocol (HTTP 402 + on-chain verification).

Built for the Solana x402 AI Hackathon.

## Features

- **Hybrid Architecture**: Rust core with embedded V8 runtime for JavaScript/TypeScript execution
- **x402 Protocol Support**: Native integration with HTTP 402 Payment Required standard
- **Solana Integration**: Built-in support for Solana blockchain and USDC stablecoin payments
- **Fast Build System**: Optimized TypeScript bundling with dual ESM/CJS output
- **Single Binary Distribution**: All-in-one executable with embedded runtime

## Installation

### From npm

```bash
npm install -g x402-dev
```

### From source

```bash
# Clone the repository
git clone <repository-url>
cd x402-dev

# Build the project
cargo build --release

# The binary will be at target/release/x402-dev
```

## Requirements

- **Node.js**: >= 18.0.0
- **Rust**: >= 1.70.0 (for building from source)
- **npm**: >= 9.0.0

## Usage

```bash
# Display help and available commands
x402-dev --help

# Get help for a specific command
x402-dev <command> --help

# Display version
x402-dev --version
```

### Available Commands

The following commands are available (implementations coming in future epics):

| Command | Description | Status |
|---------|-------------|--------|
| `mock` | Start mock facilitator server | Epic 2 |
| `test` | Run automated test suites | Epic 3 |
| `verify` | Verify x402 protocol compliance | Epic 3 |
| `check` | Check configuration and system health | Epic 4 |
| `monitor` | Monitor x402 transactions and performance | Epic 5 |
| `policy` | Manage payment policies and rules | Epic 5 |
| `examples` | Show example implementations and usage | Epic 6 |
| `doctor` | Diagnose issues and validate setup | Epic 4 |
| `init` | Initialize a new x402 project | Epic 6 |
| `version` | Display version and update information | Story 1.3 |

Example usage:

```bash
# Get help for the mock server command
x402-dev mock --help

# Start the mock facilitator server (Epic 2)
x402-dev mock --port 8080

# Run automated tests (Epic 3)
x402-dev test --suite integration

# Verify protocol compliance (Epic 3)
x402-dev verify --endpoint http://localhost:8080
```

**Note**: Most commands are placeholders and will be fully implemented in their respective epics.

## Project Structure

```
x402-dev/
├── crates/
│   ├── x402-cli/         # CLI binary entry point
│   ├── x402-core/        # Core library and runtime
│   └── xtask/            # Build automation
├── ts/                   # TypeScript runtime sources
│   ├── src/
│   │   └── runtime.ts    # JavaScript runtime entry point
│   └── dist/             # Bundled runtime (ESM + CJS)
└── package.json          # npm distribution manifest
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized for size)
cargo build --release
```

### Running Tests

```bash
# Run Rust tests
cargo test

# Run TypeScript tests
cd ts && npm test
```

### Package Size

```bash
# Create npm package
npm pack

# Current size: ~140KB (target: <10MB)
```

## Architecture

- **Rust Core**: High-performance CLI and system integration
- **V8 Runtime**: Embedded JavaScript runtime for Corbits SDK integration
- **TypeScript Bundling**: Compile-time bundling via tsup (ESM + CJS formats)
- **Size Optimization**: LTO, opt-level="z", symbol stripping

See [docs/architecture.md](docs/architecture.md) for detailed architecture documentation.

## Technology Stack

- **Blockchain**: Solana (devnet/mainnet)
- **Payment Protocol**: x402 (HTTP 402 + USDC)
- **Runtime**: V8 (via deno_core)
- **Language**: Rust + TypeScript
- **Build Tools**: Cargo, tsup

## Contributing

This is a hackathon project created for the Solana x402 AI Hackathon (October 28 - November 11, 2025).

## License

MIT - See LICENSE file for details

## Resources

- [x402 Protocol Documentation](https://docs.x402.org)
- [Solana Developer Docs](https://docs.solana.com)
- [Corbits Documentation](https://docs.corbits.ai)

## Acknowledgments

Built for the Solana x402 AI Hackathon with support from:
- Solana Foundation
- Visa TAP
- Coinbase CDP
- Switchboard
- Gradient Network
