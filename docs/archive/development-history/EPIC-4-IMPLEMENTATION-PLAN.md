# Epic 4: Validation Tools (Simplified) - Implementation Plan

**Status:** 🔴 **NOT STARTED**
**Priority:** NICE TO HAVE (Can cut if timeline slips)
**Dependencies:** Epic 1 (Foundation) ✅ Complete

---

## 📋 Epic Overview

**Epic 4 Goal:** Provide quick validation and diagnostic tools for x402-dev setup and configuration

**Value Statement:** "Quick compliance checks and debugging"

**Scope:**
- **FR-3.5:** `check` command - Comprehensive API validation
- **FR-11:** `doctor` command - System diagnostics and setup validation

**Timeline:** Day 4 (November 8-9, 2024)
**Current Status:** Placeholder commands only - no implementation

---

## 🎯 Requirements Summary

### FR-3.5: Comprehensive API Check Command

**Command:** `x402-dev check <url>`

**Must Have:**
- ✅ Single-command comprehensive validation
- ✅ Validate headers (HTTP 402, WWW-Authenticate)
- ✅ Validate invoice structure (amount, recipient, memo)
- ✅ Validate protocol compliance
- ✅ Aggregate results with pass/fail summary
- ✅ Exit code 0 (all pass) or 1 (any fail)

**Should Have:**
- 🟡 Transaction status check (if real invoices)
- 🟡 Save results to file
- 🟡 JSON output format

**Acceptance Criteria:**
```bash
$ x402-dev check https://api.example.com
✅ HTTP 402 status code: PASS
✅ WWW-Authenticate header: PASS
✅ Invoice structure: PASS
  ├─ Amount: 0.01 USDC ✓
  ├─ Recipient: 7EqQdEUL...wJeK ✓
  └─ Memo: req_abc123 ✓
✅ Protocol compliance: PASS

Overall: ✅ ALL CHECKS PASSED
Exit code: 0
```

---

### FR-11: Doctor Command (System Diagnostics)

**Command:** `x402-dev doctor`

**Must Have:**
- ✅ Check environment validation (Rust toolchain - optional for binary users)
- ✅ Detect x402 packages (Corbits SDK, PayAI, CDP)
- ✅ Check port availability (3402 default)
- ✅ Validate configuration files (`.x402dev.yaml`)
- ✅ Visual indicators: ✅ (pass), ❌ (fail), ⚠️ (warning)
- ✅ Actionable fix suggestions
- ✅ Documentation links

**Should Have:**
- 🟡 Auto-fix for common issues (`--fix` flag)
- 🟡 SDK version compatibility checks
- 🟡 Tailored recommendations based on detected SDKs

**Acceptance Criteria:**
```bash
$ x402-dev doctor
x402-dev System Diagnostics
===========================

Environment:
  ✅ x402-dev binary: v0.1.0
  ⚠️ Rust toolchain: Not detected (optional for binary users)
  ✅ npm: v10.2.3

Configuration:
  ✅ Config file: .x402dev.yaml
  ✅ Port 8402: Available
  ✅ Config syntax: Valid

x402 Ecosystem:
  ❌ Corbits SDK: Not detected
  ❌ PayAI packages: Not detected
  ❌ CDP SDK: Not detected

💡 Suggestions:
  - Install Corbits SDK: npm install @corbits/sdk
  - See: https://docs.x402-dev.com/setup

Overall: ⚠️ WARNINGS DETECTED (not blocking)
Exit code: 0
```

---

## 🏗️ Implementation Design

### Architecture (KISS Principle)

**1. Create validation module:**
```
crates/x402-cli/src/commands/
├── check.rs      # Check command (FR-3.5)
└── doctor.rs     # Doctor command (FR-11)
```

**2. Reuse existing infrastructure:**
- ✅ Configuration system (Story 1.4)
- ✅ Error handling (Story 1.5)
- ✅ Colored output (colored crate)
- ✅ HTTP client (reqwest - for check command)
- ✅ Validation helpers (x402-domain/src/validation.rs)

**3. No new dependencies required:**
- ✅ Use existing: reqwest, colored, serde_yaml
- ✅ Optionally check npm with `which npm`

---

## 📝 Story Breakdown

### Story 4.1: Check Command Implementation

**Effort:** 3-4 hours

**Tasks:**
1. Create `crates/x402-cli/src/commands/check.rs`
2. Add `CheckArgs` with `url: String` parameter
3. Implement HTTP GET request to target URL
4. Validate HTTP 402 status code
5. Validate `WWW-Authenticate` header exists
6. Parse invoice from header
7. Validate invoice structure (reuse validation.rs helpers)
8. Display colored results with checkmarks
9. Return exit code 0 (pass) or 1 (fail)

**Acceptance:**
```bash
$ x402-dev check http://localhost:3402/api/data
✅ ALL CHECKS PASSED
Exit code: 0
```

---

### Story 4.2: Doctor Command Implementation

**Effort:** 4-5 hours

**Tasks:**
1. Create `crates/x402-cli/src/commands/doctor.rs`
2. Check x402-dev version (reuse version.rs)
3. Check Rust toolchain: `rustc --version` (optional, graceful failure)
4. Check npm availability: `which npm` / `npm --version`
5. Validate config file `.x402dev.yaml` (reuse config.rs)
6. Check port availability (use `TcpListener::bind`)
7. Detect x402 packages in `package.json` or `node_modules/`
8. Display results with ✅/❌/⚠️ indicators
9. Provide actionable suggestions for failures
10. Exit code 0 (always - diagnostics don't fail)

**Acceptance:**
```bash
$ x402-dev doctor
✅ x402-dev binary: v0.1.0
✅ Config file: .x402dev.yaml
⚠️ WARNINGS DETECTED (not blocking)
Exit code: 0
```

---

## 🧪 Testing Strategy

### Unit Tests

**Check command:**
- Test HTTP 402 detection
- Test header validation
- Test invoice parsing
- Test error handling (network timeout, invalid URL)

**Doctor command:**
- Test config validation
- Test port availability check
- Test package detection logic

### Integration Tests

**Manual testing:**
```bash
# Test check command against mock server
x402-dev mock &
x402-dev check http://localhost:3402/api/data

# Test doctor command
x402-dev doctor

# Test with invalid setup
rm .x402dev.yaml
x402-dev doctor  # Should show ❌ Config file not found
```

---

## ⏱️ Time Estimates

| Task | Effort | Status |
|------|--------|--------|
| Story 4.1: Check command | 3-4 hours | 🔴 Not started |
| Story 4.2: Doctor command | 4-5 hours | 🔴 Not started |
| Testing & docs | 1-2 hours | 🔴 Not started |
| **Total** | **8-11 hours** | **0% complete** |

---

## 🚨 Priority & Risk Assessment

### Priority Ranking

**Epic 4 is NICE TO HAVE:**
- ✅ Core demo works without it (Epics 1, 2, 5, 6)
- ✅ Can be cut if timeline slips
- ❌ Not essential for hackathon submission

**Cut Priority (if timeline slips):**
1. **First to cut:** Epic 4 entirely
2. **Alternative:** Implement only `doctor` command (more valuable than `check`)

### Dependencies

**Blocks:**
- ❌ Nothing - Epic 4 is standalone

**Blocked by:**
- ✅ Epic 1 (Foundation) - COMPLETE

**Parallel opportunities:**
- ✅ Can run in parallel with Epic 3 (Test Runner)
- ✅ Can run in parallel with Epic 5 (Policy Engine)

---

## 📊 Current Status

### What's Done

- ✅ CLI command placeholders (CheckArgs, DoctorArgs)
- ✅ Help text with examples and "SEE ALSO"
- ✅ Validation helpers in x402-domain/src/validation.rs

### What's Missing

- ❌ Check command implementation
- ❌ Doctor command implementation
- ❌ HTTP client for check command
- ❌ Package detection logic for doctor
- ❌ Tests for both commands

---

## 🎯 Next Steps

### If Implementing Epic 4

**Day 4 (Nov 8-9):**
1. Implement Story 4.2 (Doctor) first - more valuable
2. Implement Story 4.1 (Check) second - depends on Epic 2 mock server
3. Manual testing with both commands
4. Documentation and completion report

### If Cutting Epic 4

**Skip to Epic 5 (Policy Engine):**
- Policy Engine is critical for demo ("100 lines → 10 lines")
- Epic 4 can be post-hackathon enhancement

---

## 📚 Reference Documents

- **PRD:** `/docs/PRD.md` (FR-3.5, FR-11)
- **Epics:** `/docs/epics.md` (Epic 4: Validation Tools - Simplified)
- **Validation helpers:** `crates/x402-domain/src/validation.rs`
- **Config system:** `crates/x402-cli/src/config.rs`
- **Error handling:** `crates/x402-cli/src/errors.rs`

---

## ✅ Definition of Done

**Epic 4 is complete when:**
- ✅ `x402-dev check <url>` validates x402 API endpoints
- ✅ `x402-dev doctor` shows system diagnostics
- ✅ Both commands have colored output with ✅/❌/⚠️
- ✅ Both commands provide actionable suggestions
- ✅ Tests passing for validation logic
- ✅ Documentation updated

---

**Created:** 2025-11-12
**Status:** Planning phase - awaiting decision to implement or cut
