use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use anyhow::{Context, Result};

// Re-export types needed by handlers and lifecycle
pub use crate::handlers::payment_required_handler;

// Import from CLI crate (temporary - will move to x402-core later)
// For now, we need to access these from the calling code
pub use serde::{Deserialize, Serialize};
pub use std::collections::HashMap;

// Re-export configuration types that handlers need
// These should eventually live in x402-core
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum SimulationMode {
    #[default]
    Success,
    Failure,
    Timeout,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub solana_rpc: String,
    pub log_level: String,
    pub pricing: PricingConfig,
    pub simulation_mode: SimulationMode,
    pub timeout_delay_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingConfig {
    pub default: f64,
    pub per_resource: HashMap<String, f64>,
}

pub struct PricingMatcher {
    config: PricingConfig,
}

impl PricingMatcher {
    pub fn new(config: PricingConfig) -> Self {
        PricingMatcher { config }
    }

    pub fn get_price_for_path(&self, path: &str) -> f64 {
        // Priority 1: Exact match
        if let Some(&amount) = self.config.per_resource.get(path) {
            return amount;
        }

        // Priority 2: Prefix match (wildcard patterns)
        let mut matches: Vec<(&str, f64)> = Vec::new();
        for (pattern, &amount) in &self.config.per_resource {
            if pattern.ends_with("/*") {
                let prefix = &pattern[..pattern.len() - 2];
                if path.starts_with(prefix) {
                    matches.push((prefix, amount));
                }
            }
        }

        // If multiple wildcards match, use the longest (most specific) prefix
        if !matches.is_empty() {
            matches.sort_by_key(|(prefix, _)| std::cmp::Reverse(prefix.len()));
            return matches[0].1;
        }

        // Priority 3: Default pricing
        self.config.default
    }
}

// Re-export Invoice types (these will move to x402-core in future)
pub use chrono::{DateTime, Duration as ChronoDuration, Utc};
pub use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub recipient: String,
    pub amount: f64,
    pub currency: String,
    pub memo: String,
    pub network: String,
    pub timestamp: DateTime<Utc>,
    pub resource_path: String,
    pub expires_at: DateTime<Utc>,
}

impl Invoice {
    pub fn new(amount: f64, resource_path: &str, recipient: String) -> Self {
        let now = Utc::now();
        let expires_at = now + ChronoDuration::minutes(5);

        Self {
            recipient,
            amount,
            currency: "USDC".to_string(),
            memo: format!("req-{}", Uuid::new_v4()),
            network: "devnet".to_string(),
            timestamp: now,
            resource_path: resource_path.to_string(),
            expires_at,
        }
    }

    pub fn format_www_authenticate(&self) -> String {
        format!(
            "x402-solana recipient={} amount={} currency={} memo={} network={}",
            self.recipient,
            self.amount,
            self.currency,
            self.memo,
            self.network
        )
    }
}

use std::sync::atomic::{AtomicUsize, Ordering};

pub const TEST_ADDRESSES: &[&str] = &[
    "GXk8vTest1111111111111111111111111111qPz9",
    "HYn9xTest2222222222222222222222222222rAb3",
    "JZp4yTest3333333333333333333333333333sCd7",
    "KAq5zTest4444444444444444444444444444tDe8",
    "MBr6ATest5555555555555555555555555555uEf9",
    "NCs7BTest6666666666666666666666666666vFg1",
    "PDt8CTest7777777777777777777777777777wGh2",
    "QEu9DTest8888888888888888888888888888xHi3",
    "RFv1ETest9999999999999999999999999999yJk4",
    "SGw2FTestAAAAAAAAAAAAAAAAAAAAAAAAAAAAzKm5",
    "THx3GTestBBBBBBBBBBBBBBBBBBBBBBBBBBB1Mn6",
    "UJy4HTestCCCCCCCCCCCCCCCCCCCCCCCCCCC2Np7",
    "VKz5JTestDDDDDDDDDDDDDDDDDDDDDDDDDDD3Pq8",
    "WMa6KTestEEEEEEEEEEEEEEEEEEEEEEEEEEE4Qr9",
    "XNb7MTestFFFFFFFFFFFFFFFFFFFFFFFFFFFF5Rs1",
    "YPc8NTestGGGGGGGGGGGGGGGGGGGGGGGGGGG6St2",
    "ZQd9PTestHHHHHHHHHHHHHHHHHHHHHHHHHHH7Tu3",
    "ARe1QTestJJJJJJJJJJJJJJJJJJJJJJJJJJJ8Uv4",
    "BSf2RTestKKKKKKKKKKKKKKKKKKKKKKKKKKK9Vw5",
    "CTg3STestMMMMMMMMMMMMMMMMMMMMMMMMMMM1Wx6",
];

pub struct InvoiceGenerator {
    address_index: AtomicUsize,
}

impl InvoiceGenerator {
    pub fn new() -> Self {
        Self {
            address_index: AtomicUsize::new(0),
        }
    }

    pub fn generate(&self, amount: f64, resource_path: &str) -> Invoice {
        let idx = self.address_index.fetch_add(1, Ordering::SeqCst);
        let recipient = TEST_ADDRESSES[idx % TEST_ADDRESSES.len()].to_string();
        Invoice::new(amount, resource_path, recipient)
    }
}

impl Default for InvoiceGenerator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Server Configuration
// ============================================================================

/// Configuration for the mock server
pub struct MockServerConfig {
    pub port: u16,
    pub pricing_matcher: PricingMatcher,
    pub invoice_generator: InvoiceGenerator,
    pub config: Config,
}

/// CORS max age for preflight requests (seconds)
const CORS_MAX_AGE_SECS: usize = 3600;

// ============================================================================
// Server Setup
// ============================================================================

/// Start the HTTP server with the given configuration
pub async fn start_http_server(server_config: MockServerConfig) -> Result<()> {
    let port = server_config.port;

    let pricing_data = web::Data::new(server_config.pricing_matcher);
    let invoice_generator = web::Data::new(server_config.invoice_generator);
    let config_data = web::Data::new(server_config.config);

    HttpServer::new(move || {
        App::new()
            // CORS middleware - allow all origins, methods, and headers for testing
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(CORS_MAX_AGE_SECS),
            )
            // Share pricing matcher, invoice generator, and config across all handlers
            .app_data(pricing_data.clone())
            .app_data(invoice_generator.clone())
            .app_data(config_data.clone())
            // Wildcard route handler - matches all paths and methods
            .default_service(web::route().to(payment_required_handler))
    })
    .bind(("127.0.0.1", port))
    .inspect_err(|e| {
        // Check if port is already in use (exit code 2 requirement)
        if e.kind() == std::io::ErrorKind::AddrInUse {
            eprintln!("❌ Error: Port {} is already in use", port);
            eprintln!("💡 Fix: Stop the process using this port or choose a different port");
            std::process::exit(2); // Exit code 2: port in use
        }
    })
    .with_context(|| format!("Failed to bind to port {}", port))?
    .run()
    .await
    .context("HTTP server error")
}

/// Mock server instance
pub struct MockServer {
    config: MockServerConfig,
}

impl MockServer {
    /// Create a new mock server with the given configuration
    pub fn new(config: MockServerConfig) -> Self {
        Self { config }
    }

    /// Start the server (blocking)
    pub async fn run(self) -> Result<()> {
        start_http_server(self.config).await
    }
}
