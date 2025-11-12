use actix_web::{web, HttpRequest, HttpResponse};
use std::time::Duration;

// Import configuration types from x402-core
// Note: These types need to be available from x402-core or passed as app data
use crate::server::{Config, InvoiceGenerator, PricingMatcher, SimulationMode};

// ============================================================================
// Payment Proof Detection (Two-Phase Flow)
// ============================================================================

/// Check if request has payment proof header
fn has_payment_proof(headers: &actix_web::http::header::HeaderMap) -> bool {
    headers
        .get("X-Payment-Proof")
        .and_then(|v| v.to_str().ok())
        .map(|s| !s.is_empty())
        .unwrap_or(false)
}

/// Extract payment proof from header
fn extract_payment_proof(headers: &actix_web::http::header::HeaderMap) -> Option<String> {
    headers
        .get("X-Payment-Proof")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

/// Get simulation mode from header override or global config
fn get_simulation_mode(
    headers: &actix_web::http::header::HeaderMap,
    config: &Config,
) -> SimulationMode {
    headers
        .get("X-Simulation-Mode")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| match s.to_lowercase().as_str() {
            "success" => Some(SimulationMode::Success),
            "failure" => Some(SimulationMode::Failure),
            "timeout" => Some(SimulationMode::Timeout),
            _ => None,
        })
        .unwrap_or(config.simulation_mode)
}

// ============================================================================
// Payment Verification Handlers
// ============================================================================

/// Verify payment with success simulation
async fn verify_payment_success(
    payment_proof: String,
    resource_path: String,
) -> HttpResponse {
    println!(
        "✅ Payment verification SUCCESS for proof: {} (resource: {})",
        payment_proof, resource_path
    );

    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .json(serde_json::json!({
            "status": "success",
            "payment_proof": payment_proof,
            "message": "Payment accepted",
            "resource": format!("Content for {}", resource_path)
        }))
}

/// Verify payment with failure simulation
async fn verify_payment_failure(payment_proof: String) -> HttpResponse {
    println!(
        "❌ Payment verification FAILURE for proof: {}",
        payment_proof
    );

    HttpResponse::PaymentRequired()
        .insert_header(("Content-Type", "application/json"))
        .json(serde_json::json!({
            "status": "failure",
            "payment_proof": payment_proof,
            "error": "Payment rejected",
            "message": "Payment verification failed - invalid or expired proof"
        }))
}

/// Verify payment with timeout simulation
async fn verify_payment_timeout(payment_proof: String, delay_ms: u64) -> HttpResponse {
    println!(
        "⏱️  Payment verification TIMEOUT for proof: {} (delay: {}ms)",
        payment_proof, delay_ms
    );

    // Simulate timeout delay using tokio
    tokio::time::sleep(Duration::from_millis(delay_ms)).await;

    HttpResponse::RequestTimeout()
        .insert_header(("Content-Type", "application/json"))
        .json(serde_json::json!({
            "status": "timeout",
            "payment_proof": payment_proof,
            "error": "Request timeout",
            "message": "Payment verification timed out"
        }))
}

// ============================================================================
// Main Request Handler (Two-Phase x402 Flow)
// ============================================================================

/// Handler that implements two-phase x402 payment flow
///
/// PHASE 1: Request without X-Payment-Proof → 402 with invoice
/// PHASE 2: Request with X-Payment-Proof → Simulate verification
pub async fn payment_required_handler(
    req: HttpRequest,
    pricing: web::Data<PricingMatcher>,
    generator: web::Data<InvoiceGenerator>,
    config: web::Data<Config>,
) -> HttpResponse {
    let path = req.path();
    let method = req.method();
    let headers = req.headers();

    // ============================================================================
    // PHASE 1: Check for payment proof
    // ============================================================================
    if !has_payment_proof(headers) {
        // No payment proof → Return 402 with invoice (Story 2.4)
        let amount = pricing.get_price_for_path(path);
        let invoice = generator.generate(amount, path);
        let invoice_header = invoice.format_www_authenticate();

        println!(
            "📨 {} {} -> 402 Payment Required (amount: {} SOL/USDC, recipient: {}, memo: {})",
            method, path, amount, invoice.recipient, invoice.memo
        );

        return HttpResponse::PaymentRequired()
            .insert_header(("WWW-Authenticate", invoice_header))
            .insert_header(("Content-Type", "application/json"))
            .json(serde_json::json!({
                "error": "Payment Required",
                "message": "Please complete payment to access this resource",
                "protocol": "x402-solana",
                "invoice": {
                    "recipient": invoice.recipient,
                    "amount": invoice.amount,
                    "currency": invoice.currency,
                    "memo": invoice.memo,
                    "network": invoice.network,
                    "timestamp": invoice.timestamp.to_rfc3339(),
                    "expires_at": invoice.expires_at.to_rfc3339(),
                    "resource_path": invoice.resource_path,
                },
                "path": path
            }));
    }

    // ============================================================================
    // PHASE 2: Verify payment proof
    // ============================================================================
    let payment_proof = match extract_payment_proof(headers) {
        Some(proof) => proof,
        None => {
            println!("⚠️  {} {} -> Invalid payment proof header", method, path);
            return HttpResponse::BadRequest()
                .insert_header(("Content-Type", "application/json"))
                .json(serde_json::json!({
                    "error": "Invalid payment proof",
                    "message": "X-Payment-Proof header is malformed"
                }));
        }
    };

    // Get simulation mode (header override or global config)
    let mode = get_simulation_mode(headers, &config);

    println!(
        "🔍 {} {} -> Verifying payment (mode: {:?}, proof: {})",
        method, path, mode, payment_proof
    );

    // Route to appropriate verification handler
    match mode {
        SimulationMode::Success => verify_payment_success(payment_proof, path.to_string()).await,
        SimulationMode::Failure => verify_payment_failure(payment_proof).await,
        SimulationMode::Timeout => {
            verify_payment_timeout(payment_proof, config.timeout_delay_ms).await
        }
    }
}
