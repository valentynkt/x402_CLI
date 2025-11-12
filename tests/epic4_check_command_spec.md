# Epic 4 Check Command - Test Specification

## Command Purpose
The `check` command validates HTTP 402 Payment Required responses and parses Lightning invoice information.

## Test Cases

### 1. HTTP 402 Status Detection
**Test**: `test_validate_402_status`
- **Given**: A URL that returns HTTP 402
- **When**: Running `x402 check <url>`
- **Then**: Should detect 402 status and report it
- **Expected Output**: "✓ HTTP 402 Payment Required detected"

### 2. WWW-Authenticate Header Validation
**Test**: `test_validate_www_authenticate_header`
- **Given**: A 402 response with WWW-Authenticate header
- **When**: Parsing the response
- **Then**: Should extract and validate the header
- **Expected Fields**:
  - `type`: "Lightning"
  - `invoice`: Valid BOLT11 invoice
  - `description`: Payment description

### 3. Invoice Parsing
**Test**: `test_parse_invoice`
- **Given**: A valid BOLT11 invoice in WWW-Authenticate header
- **When**: Parsing the invoice
- **Then**: Should extract:
  - Amount (in satoshis)
  - Timestamp
  - Payment hash
  - Description
  - Expiry time

### 4. Invalid URL Handling
**Test**: `test_invalid_url_handling`
- **Given**: An invalid URL
- **When**: Running check command
- **Then**: Should return clear error message
- **Expected**: "Error: Invalid URL format"

### 5. Network Error Handling
**Test**: `test_network_error_handling`
- **Given**: A URL that times out or is unreachable
- **When**: Running check command
- **Then**: Should handle gracefully
- **Expected**: "Error: Network request failed"

### 6. Non-402 Response
**Test**: `test_non_402_response`
- **Given**: A URL returning 200 OK
- **When**: Running check command
- **Then**: Should report no 402 found
- **Expected**: "✗ No HTTP 402 detected (received 200)"

### 7. Missing WWW-Authenticate Header
**Test**: `test_missing_www_authenticate`
- **Given**: A 402 response without WWW-Authenticate header
- **When**: Running check command
- **Then**: Should warn about missing header
- **Expected**: "⚠ HTTP 402 found but missing WWW-Authenticate header"

## Integration Tests

### Mock Server Test
**Test**: `test_check_command_with_mock_server`
- Start mock server with 402 endpoint
- Run check command against mock
- Verify correct parsing and output
- Cleanup mock server

## Expected Command Usage

```bash
# Basic check
x402 check https://api.example.com/resource

# Verbose output
x402 check https://api.example.com/resource --verbose

# JSON output
x402 check https://api.example.com/resource --json
```

## Expected JSON Output Format

```json
{
  "url": "https://api.example.com/resource",
  "status": 402,
  "www_authenticate": {
    "type": "Lightning",
    "invoice": "lnbc...",
    "amount_sats": 1000,
    "description": "API access",
    "expiry": 3600
  },
  "valid": true
}
```
