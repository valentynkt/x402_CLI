# Epic 4 Doctor Command - Test Specification

## Command Purpose
The `doctor` command performs system diagnostics to verify x402 CLI environment and configuration.

## Test Cases

### 1. Environment Checking
**Test**: `test_check_environment`
- **Validates**:
  - Rust version compatibility (>= 1.70)
  - Cargo availability
  - Required system libraries
- **Expected Output**: "✓ Rust environment: OK"

### 2. Config File Validation
**Test**: `test_config_validation`
- **Given**: A valid x402.config.json file
- **When**: Running doctor command
- **Then**: Should validate:
  - Config file exists
  - Valid JSON structure
  - Required fields present
  - Wallet configuration valid
- **Expected Output**: "✓ Configuration: OK"

### 3. Missing Config Handling
**Test**: `test_missing_config_handling`
- **Given**: No config file exists
- **When**: Running doctor command
- **Then**: Should suggest initialization
- **Expected**: "⚠ No config file found. Run 'x402 init' to create one."

### 4. Port Availability Check
**Test**: `test_port_availability`
- **Given**: Default port 3402
- **When**: Running doctor command
- **Then**: Should check if port is available
- **Expected**: "✓ Port 3402: Available" or "⚠ Port 3402: In use"

### 5. Package Detection
**Test**: `test_package_detection`
- **Given**: A package.json file exists
- **When**: Running doctor command
- **Then**: Should detect and validate:
  - Package name
  - Version
  - x402 dependencies
- **Expected Output**: "✓ Package: x402-example@1.0.0"

### 6. Lightning Wallet Connection
**Test**: `test_wallet_connection`
- **Given**: Configured LND wallet
- **When**: Running doctor with --check-wallet flag
- **Then**: Should verify:
  - Wallet endpoint reachable
  - Authentication valid
  - Wallet synced
- **Expected**: "✓ Lightning wallet: Connected"

### 7. Network Connectivity
**Test**: `test_network_connectivity`
- **When**: Running doctor command
- **Then**: Should check:
  - Internet connectivity
  - DNS resolution
  - Required API endpoints reachable
- **Expected**: "✓ Network: OK"

### 8. Dependencies Check
**Test**: `test_dependencies_check`
- **Given**: A Cargo.toml file
- **When**: Running doctor command
- **Then**: Should verify:
  - Required crates installed
  - Version compatibility
  - No conflicting dependencies
- **Expected**: "✓ Dependencies: OK"

## Integration Tests

### Full Doctor Scan
**Test**: `test_doctor_command_full_scan`
- Run doctor command in test environment
- Verify all checks execute
- Validate summary report
- Check exit code (0 for all OK, 1 for warnings, 2 for errors)

### Docker Environment Test
**Test**: `test_doctor_in_container`
- Run doctor in isolated Docker container
- Verify detection of missing dependencies
- Validate helpful error messages

## Expected Command Usage

```bash
# Basic diagnostic
x402 doctor

# Detailed output
x402 doctor --verbose

# Check specific components
x402 doctor --check-wallet
x402 doctor --check-network

# JSON output for automation
x402 doctor --json
```

## Expected Output Format

### Console Output
```
🩺 x402 System Diagnostics
━━━━━━━━━━━━━━━━━━━━━━━━━━━

Environment:
  ✓ Rust 1.75.0
  ✓ Cargo 1.75.0
  ✓ System libraries OK

Configuration:
  ✓ Config file: x402.config.json
  ✓ Valid structure
  ✓ Wallet configured

Network:
  ✓ Internet connectivity
  ✓ DNS resolution
  ✓ Port 3402 available

Project:
  ✓ Package: my-api@1.0.0
  ✓ Dependencies up to date

━━━━━━━━━━━━━━━━━━━━━━━━━━━
Summary: All systems operational ✓
```

### JSON Output Format
```json
{
  "timestamp": "2025-01-12T01:30:00Z",
  "version": "0.1.0",
  "checks": {
    "environment": {
      "status": "ok",
      "rust_version": "1.75.0",
      "cargo_version": "1.75.0"
    },
    "configuration": {
      "status": "ok",
      "config_file": "x402.config.json",
      "wallet_configured": true
    },
    "network": {
      "status": "ok",
      "internet": true,
      "port_3402": "available"
    },
    "project": {
      "status": "ok",
      "package": "my-api@1.0.0",
      "dependencies": "ok"
    }
  },
  "summary": {
    "status": "ok",
    "total_checks": 4,
    "passed": 4,
    "warnings": 0,
    "errors": 0
  }
}
```

## Error Scenarios

### Missing Rust
```
✗ Rust not found
  Please install Rust from https://rustup.rs
```

### Port Conflict
```
⚠ Port 3402 is in use
  Run 'lsof -i :3402' to find the process
  Or configure a different port in x402.config.json
```

### Invalid Config
```
✗ Configuration error
  File: x402.config.json
  Error: Missing required field 'wallet.type'
  Run 'x402 init' to regenerate configuration
```
