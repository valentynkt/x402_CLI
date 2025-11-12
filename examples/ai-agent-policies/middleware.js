/**
 * x402 Policy Enforcement Middleware for Express.js
 *
 * Auto-generated from policy.yaml using:
 * x402-dev policy generate policy.yaml --framework express
 *
 * This middleware demonstrates server-side policy enforcement.
 * It validates requests against the configured policies BEFORE
 * processing them, ensuring compliance at the API layer.
 */

const express = require('express');

/**
 * Policy configuration (loaded from policy.yaml)
 */
const POLICY_CONFIG = {
  version: "1.0",
  spending_cap: {
    max_amount: 10.0,
    currency: "USDC",
    period: "daily"
  },
  allowlist: {
    endpoints: ["/api/data", "/api/ai-query"]
  },
  rate_limit: {
    requests_per_minute: 10
  }
};

/**
 * In-memory tracking for spending and rate limits
 * In production, use Redis or database for distributed systems
 */
const policyState = {
  // Track spending per client/policy ID
  spending: new Map(),

  // Track request timestamps for rate limiting
  requestTimestamps: new Map(),

  // Track spending window start times
  windowStarts: new Map()
};

/**
 * Middleware: Enforce spending cap
 *
 * WHY: Prevents clients from exceeding their budget
 * WHEN: Before processing any payment-required request
 */
function enforceSpendingCap(req, res, next) {
  const policyId = req.headers['x-402-policy'] || 'default';
  const estimatedCost = parseFloat(req.headers['x-402-estimated-cost'] || '0');

  // Initialize tracking for this policy ID
  if (!policyState.spending.has(policyId)) {
    policyState.spending.set(policyId, 0);
    policyState.windowStarts.set(policyId, Date.now());
  }

  // Check if spending window needs reset
  const windowStart = policyState.windowStarts.get(policyId);
  const windowDuration = getWindowDuration(POLICY_CONFIG.spending_cap.period);

  if (Date.now() - windowStart > windowDuration) {
    // Reset for new period
    policyState.spending.set(policyId, 0);
    policyState.windowStarts.set(policyId, Date.now());
  }

  // Check if request would exceed cap
  const currentSpending = policyState.spending.get(policyId);
  if (currentSpending + estimatedCost > POLICY_CONFIG.spending_cap.max_amount) {
    return res.status(402).json({
      error: "Spending cap exceeded",
      current: currentSpending,
      requested: estimatedCost,
      limit: POLICY_CONFIG.spending_cap.max_amount,
      currency: POLICY_CONFIG.spending_cap.currency,
      period: POLICY_CONFIG.spending_cap.period
    });
  }

  // Record spending after successful request
  res.on('finish', () => {
    if (res.statusCode === 200) {
      const newTotal = policyState.spending.get(policyId) + estimatedCost;
      policyState.spending.set(policyId, newTotal);
      console.log(`Policy ${policyId}: Spent ${estimatedCost}, Total: ${newTotal}/${POLICY_CONFIG.spending_cap.max_amount}`);
    }
  });

  next();
}

/**
 * Middleware: Enforce endpoint allowlist
 *
 * WHY: Restricts access to approved endpoints only
 * WHEN: Before routing request to handlers
 */
function enforceAllowlist(req, res, next) {
  const requestPath = req.path;

  // Check if path is in allowlist
  const isAllowed = POLICY_CONFIG.allowlist.endpoints.some(allowed => {
    // Support wildcard matching
    if (allowed.endsWith('/*')) {
      const prefix = allowed.slice(0, -2);
      return requestPath.startsWith(prefix);
    }
    return requestPath === allowed;
  });

  if (!isAllowed) {
    return res.status(403).json({
      error: "Endpoint not in allowlist",
      requested: requestPath,
      allowed: POLICY_CONFIG.allowlist.endpoints
    });
  }

  next();
}

/**
 * Middleware: Enforce rate limiting
 *
 * WHY: Prevents abuse through request throttling
 * WHEN: Before processing every request
 */
function enforceRateLimit(req, res, next) {
  const clientId = req.headers['x-402-policy'] || req.ip;

  // Initialize tracking for this client
  if (!policyState.requestTimestamps.has(clientId)) {
    policyState.requestTimestamps.set(clientId, []);
  }

  const timestamps = policyState.requestTimestamps.get(clientId);
  const now = Date.now();
  const oneMinuteAgo = now - 60000;

  // Remove timestamps older than 1 minute
  const recentTimestamps = timestamps.filter(ts => ts > oneMinuteAgo);

  // Check if rate limit exceeded
  if (recentTimestamps.length >= POLICY_CONFIG.rate_limit.requests_per_minute) {
    const oldestTimestamp = recentTimestamps[0];
    const retryAfter = Math.ceil((oldestTimestamp + 60000 - now) / 1000);

    return res.status(429)
      .header('Retry-After', retryAfter)
      .json({
        error: "Rate limit exceeded",
        limit: POLICY_CONFIG.rate_limit.requests_per_minute,
        retryAfter: `${retryAfter} seconds`
      });
  }

  // Record this request
  recentTimestamps.push(now);
  policyState.requestTimestamps.set(clientId, recentTimestamps);

  next();
}

/**
 * Helper: Convert period string to milliseconds
 */
function getWindowDuration(period) {
  switch (period) {
    case 'hourly': return 3600000;
    case 'daily': return 86400000;
    case 'monthly': return 2592000000; // 30 days
    default: return 86400000; // default to daily
  }
}

/**
 * Complete x402 policy middleware stack
 * Apply all policy enforcement in order
 */
function x402PolicyMiddleware() {
  return [
    enforceRateLimit,      // 1. Check rate limits first (fastest)
    enforceAllowlist,      // 2. Validate endpoint is allowed
    enforceSpendingCap     // 3. Check budget (after other checks pass)
  ];
}

/**
 * Example Express.js application with policy enforcement
 */
function createApp() {
  const app = express();
  app.use(express.json());

  // Apply x402 policy middleware to all routes
  app.use(x402PolicyMiddleware());

  // Example endpoints (allowlisted in policy.yaml)
  app.get('/api/data', (req, res) => {
    res.json({
      message: "Data retrieved successfully",
      cost: 0.50
    });
  });

  app.post('/api/ai-query', (req, res) => {
    res.json({
      message: "AI query processed",
      result: "Example response",
      cost: 0.25
    });
  });

  // Example forbidden endpoint (NOT in allowlist)
  app.get('/api/forbidden', (req, res) => {
    // This will be blocked by allowlist middleware
    res.json({ message: "This should never be reached" });
  });

  return app;
}

// Export middleware for use in other applications
module.exports = {
  x402PolicyMiddleware,
  enforceSpendingCap,
  enforceAllowlist,
  enforceRateLimit,
  createApp
};

/**
 * Example usage:
 *
 * const express = require('express');
 * const { x402PolicyMiddleware } = require('./middleware');
 *
 * const app = express();
 * app.use(x402PolicyMiddleware());
 *
 * app.get('/api/data', (req, res) => {
 *   // Your endpoint logic
 * });
 *
 * app.listen(3000);
 */
