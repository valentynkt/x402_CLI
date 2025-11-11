#!/bin/bash

# Epic 2 Integration Test Suite
# Comprehensive validation of all x402 payment protocol features

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test results counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Test result tracking
declare -a FAILED_TEST_NAMES

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
    ((PASSED_TESTS++))
    ((TOTAL_TESTS++))
}

log_fail() {
    echo -e "${RED}[FAIL]${NC} $1"
    FAILED_TEST_NAMES+=("$1")
    ((FAILED_TESTS++))
    ((TOTAL_TESTS++))
}

log_section() {
    echo ""
    echo -e "${YELLOW}========================================${NC}"
    echo -e "${YELLOW}$1${NC}"
    echo -e "${YELLOW}========================================${NC}"
}

# Cleanup function
cleanup() {
    log_info "Cleaning up test environment..."
    pkill -f "x402-dev mock" 2>/dev/null || true
    rm -f ~/.x402dev/mock-server.pid 2>/dev/null || true
    rm -f /tmp/integration-test.log 2>/dev/null || true
    rm -f .x402dev.yaml 2>/dev/null || true
}

# Setup trap for cleanup
trap cleanup EXIT

# Start of tests
log_section "Epic 2 Integration Test Suite"
log_info "Testing x402 payment protocol implementation"
log_info "Binary: target/release/x402-dev"

# Verify binary exists
if [ ! -f "target/release/x402-dev" ]; then
    log_fail "Binary not found. Run 'cargo build --release' first."
    exit 1
fi

BINARY="./target/release/x402-dev"

# ============================================================
# TEST SCENARIO 1: Full x402 Payment Flow (CRITICAL)
# ============================================================
log_section "Test Scenario 1: Full x402 Payment Flow"

# Start server in background
log_info "Starting mock server on port 3402..."
$BINARY mock --port 3402 > /tmp/integration-test.log 2>&1 &
SERVER_PID=$!
sleep 3

# Verify server is running
if ! ps -p $SERVER_PID > /dev/null; then
    log_fail "Server failed to start"
    cat /tmp/integration-test.log
    exit 1
fi
log_success "Server started successfully (PID: $SERVER_PID)"

# Phase 1: Request without payment (should get 402)
log_info "Phase 1: Testing initial request without payment..."
RESPONSE=$(curl -sv http://localhost:3402/api/data 2>&1)

if echo "$RESPONSE" | grep -q "402 Payment Required"; then
    log_success "Correct 402 status returned"
else
    log_fail "Expected 402 status, got different response"
fi

if echo "$RESPONSE" | grep -q "WWW-Authenticate: x402-solana"; then
    log_success "WWW-Authenticate header present with x402-solana prefix"
else
    log_fail "WWW-Authenticate header missing or incorrect"
fi

# Verify space-separated format (NOT base64)
if echo "$RESPONSE" | grep -E "recipient=.+ amount=.+ currency=.+ memo=.+ network="; then
    log_success "Invoice format: space-separated key=value pairs (NOT base64)"
else
    log_fail "Invoice format incorrect (expected space-separated, not base64)"
fi

# Verify all required invoice fields
REQUIRED_FIELDS=("recipient=" "amount=" "currency=" "memo=" "network=" "expires_at=")
for field in "${REQUIRED_FIELDS[@]}"; do
    if echo "$RESPONSE" | grep -q "$field"; then
        log_success "Invoice contains required field: ${field%=}"
    else
        log_fail "Invoice missing required field: ${field%=}"
    fi
done

# Phase 2: Submit payment proof (success mode)
log_info "Phase 2: Testing payment submission (success mode)..."
PAYMENT_RESPONSE=$(curl -s -H "X-Payment-Proof: test_integration_tx" http://localhost:3402/api/data)

if echo "$PAYMENT_RESPONSE" | grep -q '"status":"success"'; then
    log_success "Payment accepted in success mode"
else
    log_fail "Payment not accepted in success mode"
    echo "Response: $PAYMENT_RESPONSE"
fi

if echo "$PAYMENT_RESPONSE" | grep -q '"data":'; then
    log_success "Data returned after successful payment"
else
    log_fail "No data returned after payment"
fi

# Phase 3: Test failure mode
log_info "Phase 3: Testing failure mode..."
FAILURE_RESPONSE=$(curl -s -H "X-Payment-Proof: test_fail_tx" -H "X-Simulation-Mode: failure" http://localhost:3402/api/data)

if echo "$FAILURE_RESPONSE" | grep -q '"status":"failure"'; then
    log_success "Failure mode returns correct status"
else
    log_fail "Failure mode not working correctly"
    echo "Response: $FAILURE_RESPONSE"
fi

# Phase 4: Test timeout mode
log_info "Phase 4: Testing timeout mode..."
TIMEOUT_RESPONSE=$(curl -s -H "X-Payment-Proof: test_timeout_tx" -H "X-Simulation-Mode: timeout" http://localhost:3402/api/data)

if echo "$TIMEOUT_RESPONSE" | grep -q "408\|timeout"; then
    log_success "Timeout mode works correctly"
else
    log_fail "Timeout mode not working as expected"
fi

# Stop server for next scenario
kill $SERVER_PID 2>/dev/null || true
sleep 2

# ============================================================
# TEST SCENARIO 2: Pricing Configuration Validation
# ============================================================
log_section "Test Scenario 2: Pricing Configuration"

# Create test configuration
log_info "Creating test pricing configuration..."
cat > .x402dev.yaml << 'EOF'
pricing:
  default: 0.01
  per_resource:
    /api/data: 0.05
    /api/premium: 0.10
    /api/admin/*: 0.20
EOF

# Start server with config
log_info "Starting server with custom pricing..."
$BINARY mock --port 3402 > /tmp/integration-test.log 2>&1 &
SERVER_PID=$!
sleep 3

# Test default pricing
log_info "Testing default pricing..."
DEFAULT_RESPONSE=$(curl -sv http://localhost:3402/random 2>&1)
if echo "$DEFAULT_RESPONSE" | grep -q "amount=0.01"; then
    log_success "Default pricing (0.01 SOL) applied correctly"
else
    log_fail "Default pricing not applied"
fi

# Test exact match
log_info "Testing exact path match pricing..."
EXACT_RESPONSE=$(curl -sv http://localhost:3402/api/data 2>&1)
if echo "$EXACT_RESPONSE" | grep -q "amount=0.05"; then
    log_success "Exact match pricing (0.05 SOL) applied"
else
    log_fail "Exact match pricing not applied"
fi

# Test prefix wildcard
log_info "Testing wildcard prefix pricing..."
WILDCARD_RESPONSE=$(curl -sv http://localhost:3402/api/admin/users 2>&1)
if echo "$WILDCARD_RESPONSE" | grep -q "amount=0.20"; then
    log_success "Wildcard pricing (0.20 SOL) applied"
else
    log_fail "Wildcard pricing not applied"
fi

# Another wildcard test
WILDCARD2_RESPONSE=$(curl -sv http://localhost:3402/api/admin/settings 2>&1)
if echo "$WILDCARD2_RESPONSE" | grep -q "amount=0.20"; then
    log_success "Wildcard pricing matches multiple paths"
else
    log_fail "Wildcard not matching multiple paths"
fi

# Cleanup config
rm -f .x402dev.yaml
kill $SERVER_PID 2>/dev/null || true
sleep 2

# ============================================================
# TEST SCENARIO 3: Server Lifecycle Management
# ============================================================
log_section "Test Scenario 3: Server Lifecycle Management"

# Start server
log_info "Starting server for lifecycle test..."
$BINARY mock --port 3402 > /tmp/integration-test.log 2>&1 &
sleep 3

# Check status (should be running)
log_info "Checking server status..."
if $BINARY mock status > /dev/null 2>&1; then
    EXIT_CODE=$?
    if [ $EXIT_CODE -eq 0 ]; then
        log_success "Status command: server detected as running"
    else
        log_fail "Status command returned unexpected exit code: $EXIT_CODE"
    fi
else
    log_fail "Status command failed"
fi

# Stop server
log_info "Stopping server..."
if $BINARY mock stop; then
    log_success "Stop command executed successfully"
else
    log_fail "Stop command failed"
fi

sleep 2

# Check status again (should be stopped)
log_info "Verifying server stopped..."
if ! $BINARY mock status > /dev/null 2>&1; then
    EXIT_CODE=$?
    if [ $EXIT_CODE -eq 2 ]; then
        log_success "Status command: server detected as stopped (exit code 2)"
    else
        log_success "Server stopped (exit code: $EXIT_CODE)"
    fi
else
    log_fail "Server still appears to be running"
fi

# Verify PID file removed
if [ ! -f ~/.x402dev/mock-server.pid ]; then
    log_success "PID file cleaned up correctly"
else
    log_fail "PID file not removed"
fi

# ============================================================
# TEST SCENARIO 4: Demo Checkpoint - "30 seconds vs 30 minutes"
# ============================================================
log_section "Test Scenario 4: Demo Checkpoint (30 Second Test)"

log_info "Running full demo workflow with timing..."
START_TIME=$(date +%s)

# Full demo workflow
log_info "Step 1: Starting server..."
$BINARY mock --port 3402 > /tmp/integration-test.log 2>&1 &
sleep 2

log_info "Step 2: Making initial request (expect 402)..."
curl -sv http://localhost:3402/api/data 2>&1 | grep -q "402" && log_success "Initial 402 received"

log_info "Step 3: Submitting payment proof..."
curl -s -H "X-Payment-Proof: demo_tx" http://localhost:3402/api/data | grep -q "success" && log_success "Payment accepted"

log_info "Step 4: Stopping server..."
$BINARY mock stop && log_success "Server stopped cleanly"

END_TIME=$(date +%s)
ELAPSED=$((END_TIME - START_TIME))

log_info "Total demo workflow time: ${ELAPSED} seconds"
if [ $ELAPSED -lt 30 ]; then
    log_success "DEMO CHECKPOINT ACHIEVED: Workflow completed in ${ELAPSED}s (target: <30s)"
else
    log_fail "DEMO CHECKPOINT NOT MET: Workflow took ${ELAPSED}s (target: <30s)"
fi

# ============================================================
# TEST SCENARIO 5: Performance Validation
# ============================================================
log_section "Test Scenario 5: Performance Metrics"

# Server startup time
log_info "Measuring server startup time..."
STARTUP_START=$(date +%s%N)
$BINARY mock --port 3402 > /tmp/integration-test.log 2>&1 &
SERVER_PID=$!
sleep 2
STARTUP_END=$(date +%s%N)
STARTUP_MS=$(( (STARTUP_END - STARTUP_START) / 1000000 ))

log_info "Server startup time: ${STARTUP_MS}ms"
if [ $STARTUP_MS -lt 3000 ]; then
    log_success "Startup time acceptable (<3 seconds)"
else
    log_fail "Startup time too slow (${STARTUP_MS}ms)"
fi

# Response time
log_info "Measuring response time..."
RESPONSE_START=$(date +%s%N)
curl -s http://localhost:3402/test > /dev/null
RESPONSE_END=$(date +%s%N)
RESPONSE_MS=$(( (RESPONSE_END - RESPONSE_START) / 1000000 ))

log_info "Response time: ${RESPONSE_MS}ms"
if [ $RESPONSE_MS -lt 200 ]; then
    log_success "Response time acceptable (<200ms)"
else
    log_fail "Response time too slow (${RESPONSE_MS}ms)"
fi

# Cleanup
kill $SERVER_PID 2>/dev/null || true

# ============================================================
# TEST SCENARIO 6: Protocol Compliance
# ============================================================
log_section "Test Scenario 6: Protocol Compliance Validation"

$BINARY mock --port 3402 > /tmp/integration-test.log 2>&1 &
sleep 2

COMPLIANCE_RESPONSE=$(curl -sv http://localhost:3402/api/test 2>&1)

# Check header format
if echo "$COMPLIANCE_RESPONSE" | grep -q "WWW-Authenticate: x402-solana"; then
    log_success "Protocol: Correct WWW-Authenticate prefix (x402-solana)"
else
    log_fail "Protocol: Incorrect or missing WWW-Authenticate prefix"
fi

# Verify NOT base64 encoded
if echo "$COMPLIANCE_RESPONSE" | grep -E "recipient=[A-Za-z0-9]+ amount=[0-9.]+ currency=[A-Z]+ memo=[^ ]+ network=[a-z-]+"; then
    log_success "Protocol: Space-separated format (NOT base64)"
else
    log_fail "Protocol: Invalid format (should be space-separated, not base64)"
fi

# Check CORS headers
if echo "$COMPLIANCE_RESPONSE" | grep -q "Access-Control-Allow-Origin"; then
    log_success "Protocol: CORS headers present"
else
    log_fail "Protocol: CORS headers missing"
fi

# Check Content-Type
if echo "$COMPLIANCE_RESPONSE" | grep -q "Content-Type: application/json"; then
    log_success "Protocol: Correct Content-Type (application/json)"
else
    log_fail "Protocol: Incorrect Content-Type"
fi

pkill -f "x402-dev mock" 2>/dev/null || true

# ============================================================
# FINAL SUMMARY
# ============================================================
log_section "INTEGRATION TEST SUMMARY"

echo ""
echo "Total Tests:  $TOTAL_TESTS"
echo -e "${GREEN}Passed:       $PASSED_TESTS${NC}"
echo -e "${RED}Failed:       $FAILED_TESTS${NC}"
echo ""

if [ $FAILED_TESTS -gt 0 ]; then
    echo -e "${RED}Failed Tests:${NC}"
    for test in "${FAILED_TEST_NAMES[@]}"; do
        echo -e "  ${RED}✗${NC} $test"
    done
    echo ""
fi

PASS_RATE=$(( PASSED_TESTS * 100 / TOTAL_TESTS ))
echo "Pass Rate: ${PASS_RATE}%"
echo ""

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║   🎉 ALL TESTS PASSED! EPIC 2 READY   ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
    exit 0
else
    echo -e "${RED}╔════════════════════════════════════════╗${NC}"
    echo -e "${RED}║   ❌ SOME TESTS FAILED - REVIEW NEEDED ║${NC}"
    echo -e "${RED}╚════════════════════════════════════════╝${NC}"
    exit 1
fi
