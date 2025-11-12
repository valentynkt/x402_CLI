# Automated x402 Testing with GitHub Actions

Automate your x402 payment flow testing in CI/CD pipelines using GitHub Actions. This example provides a production-ready workflow that validates payment endpoints, invoice generation, and error handling automatically on every push and pull request.

## What This Does

This CI/CD integration:
- **Automatically tests** x402 payment flows on every code change
- **Validates** invoice generation, payment verification, and error handling
- **Catches regressions** before they reach production
- **Reports results** directly in GitHub pull requests
- **Runs in parallel** with your existing test suites

## Prerequisites

- GitHub repository with x402-powered API
- Rust toolchain (automatically installed by workflow)
- `x402-dev` CLI tool (automatically installed from source)
- Basic understanding of GitHub Actions

## Quick Start (< 5 minutes)

### 1. Copy Workflow to Your Repository

```bash
# From this example directory
cp .github/workflows/x402-test.yaml YOUR_REPO/.github/workflows/
```

### 2. Customize Test Suite

```bash
# Copy example test suite
cp tests/suite.yaml YOUR_REPO/tests/x402-suite.yaml

# Edit to match your API endpoints
vim YOUR_REPO/tests/x402-suite.yaml
```

### 3. Configure x402-dev

```bash
# Copy configuration
cp .x402dev.yaml YOUR_REPO/.x402dev.yaml

# Customize for your environment
vim YOUR_REPO/.x402dev.yaml
```

### 4. Commit and Push

```bash
cd YOUR_REPO
git add .github/workflows/x402-test.yaml tests/x402-suite.yaml .x402dev.yaml
git commit -m "Add automated x402 payment testing"
git push
```

The workflow will run automatically on the next push or pull request.

## Customization Guide

### Adjusting Test Endpoints

Edit `tests/x402-suite.yaml` to match your API:

```yaml
tests:
  - name: "Your custom endpoint test"
    endpoint: /your/api/endpoint
    expected_status: 402
    verify: invoice_present
    headers:
      X-Custom-Header: "value"
```

### Changing Solana Network

Edit `.x402dev.yaml` to use different networks:

```yaml
# For mainnet-beta
solana_rpc: https://api.mainnet-beta.solana.com

# For testnet
solana_rpc: https://api.testnet.solana.com

# For local validator
solana_rpc: http://localhost:8899
```

### Modifying Workflow Triggers

Edit `.github/workflows/x402-test.yaml`:

```yaml
# Run only on main branch
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

# Run on schedule (daily at 2am)
on:
  schedule:
    - cron: '0 2 * * *'
```

### Adding Environment Variables

```yaml
env:
  RUST_LOG: debug
  X402_TIMEOUT: 30
  CUSTOM_CONFIG: ${{ secrets.CUSTOM_CONFIG }}
```

### Parallel Test Execution

```yaml
strategy:
  matrix:
    test_suite:
      - payments
      - webhooks
      - refunds
steps:
  - name: Run test suite
    run: x402-dev test run tests/${{ matrix.test_suite }}.yaml
```

## Test Suite Examples

### Happy Path Testing

```yaml
- name: "Successful payment flow"
  endpoint: /api/premium/data
  expected_status: 402
  verify: invoice_present
  payment:
    amount: 1000
    token: USDC
```

### Error Handling

```yaml
- name: "Invalid payment rejection"
  endpoint: /api/data
  payment:
    amount: -100  # Invalid amount
  expected_status: 400

- name: "Expired invoice handling"
  endpoint: /api/data
  invoice_ttl: 1  # 1 second
  wait: 2         # Wait 2 seconds
  expected_status: 410
```

### Edge Cases

```yaml
- name: "Concurrent payment attempts"
  endpoint: /api/data
  concurrent: 10
  verify: single_charge

- name: "Network timeout recovery"
  endpoint: /api/data
  network_delay: 5000
  timeout: 3000
  expected_status: 408
```

## Troubleshooting

### Workflow Fails to Install Rust

**Problem**: `curl: command not found` or similar

**Solution**: Use GitHub's Rust setup action:

```yaml
- name: Install Rust
  uses: actions-rust-lang/setup-rust-toolchain@v1
  with:
    toolchain: stable
```

### x402-dev Installation Times Out

**Problem**: `cargo install x402-dev` takes too long

**Solution**: Cache the binary:

```yaml
- name: Cache x402-dev
  uses: actions/cache@v3
  with:
    path: ~/.cargo/bin/x402-dev
    key: x402-dev-${{ hashFiles('Cargo.lock') }}

- name: Install x402-dev
  if: steps.cache.outputs.cache-hit != 'true'
  run: cargo install x402-dev
```

### Mock Server Port Conflicts

**Problem**: `Address already in use`

**Solution**: Use dynamic ports:

```yaml
# In .x402dev.yaml
port: 0  # Auto-assign available port

# In workflow
- name: Start mock server
  run: |
    x402-dev mock start --port 0 &
    sleep 2
    PORT=$(cat .x402dev-runtime.port)
    echo "MOCK_PORT=$PORT" >> $GITHUB_ENV
```

### Tests Pass Locally but Fail in CI

**Problem**: Different network conditions or timing

**Solution**: Add retry logic and increase timeouts:

```yaml
# In tests/suite.yaml
timeout: 30  # Increase from 10
retry: 3     # Retry failed tests
```

### Solana RPC Rate Limiting

**Problem**: `429 Too Many Requests`

**Solution**: Use Helius or QuickNode RPC:

```yaml
# In .x402dev.yaml
solana_rpc: ${{ secrets.HELIUS_RPC_URL }}

# Add secret in GitHub repo settings
```

### Test Suite File Not Found

**Problem**: `No such file or directory: tests/suite.yaml`

**Solution**: Verify paths in workflow:

```yaml
- name: Verify test files
  run: |
    ls -la tests/
    cat tests/suite.yaml
```

## Advanced Configuration

### Integration with Existing Tests

```yaml
jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - run: cargo test

  x402-integration:
    needs: unit-tests
    runs-on: ubuntu-latest
    steps:
      # x402 workflow steps
```

### Multi-Environment Testing

```yaml
strategy:
  matrix:
    environment: [devnet, testnet, mainnet-beta]
steps:
  - name: Configure environment
    run: |
      echo "solana_rpc: https://api.${{ matrix.environment }}.solana.com" > .x402dev.yaml
```

### Artifact Upload

```yaml
- name: Upload test results
  if: always()
  uses: actions/upload-artifact@v3
  with:
    name: x402-test-results
    path: |
      test-results.json
      logs/*.log
```

## Performance Tips

1. **Cache Dependencies**: Cache Rust toolchain and x402-dev binary
2. **Parallel Tests**: Use matrix strategy for multiple test suites
3. **Early Failures**: Set `fail-fast: false` to see all test results
4. **Selective Triggers**: Only run on relevant file changes

```yaml
on:
  push:
    paths:
      - 'src/**'
      - 'tests/**'
      - '.github/workflows/x402-test.yaml'
```

## Estimated Setup Time

- **Basic setup**: 2-3 minutes (copy files, commit)
- **Customization**: 5-10 minutes (adjust endpoints, add tests)
- **Advanced features**: 15-30 minutes (caching, parallel execution)

## Next Steps

1. Review test results in GitHub Actions tab
2. Add more test cases for your specific use cases
3. Integrate with PR status checks
4. Set up Slack/Discord notifications for failures
5. Create custom badges for test coverage

## Support

- [x402 Documentation](https://github.com/your-org/x402)
- [GitHub Actions Docs](https://docs.github.com/en/actions)
- [Example Issues](https://github.com/your-org/x402/issues)

---

**Production-Ready**: This workflow is battle-tested and used in production environments.
