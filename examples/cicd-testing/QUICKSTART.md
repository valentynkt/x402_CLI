# Quick Start Guide - x402 CI/CD Testing

Get automated x402 payment testing running in your GitHub repository in under 5 minutes.

## Prerequisites

- GitHub repository
- Basic knowledge of GitHub Actions

## Installation (3 steps)

### Step 1: Copy Workflow File (30 seconds)

```bash
# Clone this example or copy the workflow file
cp .github/workflows/x402-test.yaml YOUR_REPO/.github/workflows/
```

### Step 2: Copy Test Suite (1 minute)

```bash
# Copy and customize the test suite
cp tests/suite.yaml YOUR_REPO/tests/x402-suite.yaml
cp .x402dev.yaml YOUR_REPO/.x402dev.yaml

# Edit endpoints to match your API
vim YOUR_REPO/tests/x402-suite.yaml
```

### Step 3: Commit and Push (30 seconds)

```bash
cd YOUR_REPO
git add .github/workflows/x402-test.yaml tests/x402-suite.yaml .x402dev.yaml
git commit -m "Add automated x402 payment testing"
git push
```

Done! The workflow will run automatically on your next push or pull request.

## Verification

1. Go to your repository on GitHub
2. Click the "Actions" tab
3. See your workflow running
4. View test results in the workflow logs

## What Happens Next

Every time you push code or create a pull request:

1. GitHub Actions checks out your code
2. Installs Rust and x402-dev
3. Starts a mock payment server
4. Runs your test suite
5. Reports results
6. Comments on your PR with test summary

## Customization

### Change Test Endpoints

Edit `tests/x402-suite.yaml`:

```yaml
tests:
  - name: "Test my endpoint"
    endpoint: /api/my/endpoint  # <- Change this
    expected_status: 402
```

### Use Different Solana Network

Edit `.x402dev.yaml`:

```yaml
solana:
  rpc_url: "https://api.mainnet-beta.solana.com"  # <- Change this
```

### Add More Tests

Add to `tests/x402-suite.yaml`:

```yaml
tests:
  # ... existing tests ...

  - name: "My new test"
    endpoint: /api/new
    expected_status: 402
    verify:
      - invoice_present
```

## Troubleshooting

### Workflow Doesn't Run

**Problem**: No Actions tab or workflow not triggering

**Solution**: Enable GitHub Actions in repository settings

### Tests Fail Locally But Pass in CI

**Problem**: Different environment behavior

**Solution**: Run locally with same configuration:

```bash
x402-dev test run --suite tests/x402-suite.yaml --config .x402dev.yaml
```

### Need Help?

- Check [README.md](README.md) for detailed documentation
- Review [VALIDATION.md](VALIDATION.md) for examples
- Open an issue in the repository

## Next Steps

- Add more test cases for your specific use cases
- Set up branch protection requiring tests to pass
- Configure Slack/Discord notifications
- Add performance benchmarks

## Example Output

When tests pass, you'll see:

```
✅ x402 Payment Tests

Summary:
- Total: 14
- Passed: 14 ✅
- Failed: 0 ❌

All tests passed successfully!
```

When tests fail, you'll see:

```
❌ x402 Payment Tests

Summary:
- Total: 14
- Passed: 12 ✅
- Failed: 2 ❌

Failed Tests:
- Invalid payment amount rejection
- Expired invoice handling

View full results: https://github.com/...
```

## Time Investment

- **Setup**: 3 minutes
- **First customization**: 5 minutes
- **Ongoing maintenance**: Minimal

**Total**: Less than 10 minutes to production-ready automated testing

---

**Ready to go!** Your x402 payment flows are now automatically tested on every code change.
