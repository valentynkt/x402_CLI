use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use anyhow::{anyhow, Context, Result};
use directories::BaseDirs;
use fs2::FileExt;
use std::fs::{self, File};
use std::path::PathBuf;
use std::time::Duration;
use sysinfo::{Pid, ProcessesToUpdate, System};

use crate::cli::{MockArgs, MockSubcommand};
use crate::config::{load_merged_config, CliOverrides, Config, PricingMatcher, SimulationMode};
use crate::commands::invoice::InvoiceGenerator;

// ============================================================================
// Constants
// ============================================================================

/// Maximum wait time for graceful shutdown (seconds)
const SHUTDOWN_TIMEOUT_SECS: u64 = 5;

/// Poll interval for checking process shutdown (milliseconds)
const SHUTDOWN_POLL_INTERVAL_MS: u64 = 100;

/// CORS max age for preflight requests (seconds)
const CORS_MAX_AGE_SECS: usize = 3600;

// ============================================================================
// PID File Management
// ============================================================================

/// Get path to PID file using platform-specific home directory
fn get_pid_file_path() -> Result<PathBuf> {
    let base_dirs = BaseDirs::new()
        .ok_or_else(|| anyhow!("Cannot determine home directory"))?;

    Ok(base_dirs
        .home_dir()
        .join(".x402dev")
        .join("mock-server.pid"))
}

/// Write PID file with exclusive locking to prevent race conditions
fn write_pid_file(pid: u32) -> Result<()> {
    let pid_path = get_pid_file_path()?;

    // Create parent directory if it doesn't exist
    if let Some(parent) = pid_path.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create .x402dev directory")?;
    }

    // Open file and acquire exclusive lock (prevents TOCTOU race condition)
    let file = File::create(&pid_path)
        .context("Failed to create PID file")?;

    file.try_lock_exclusive()
        .context("Server already running (cannot acquire PID file lock)")?;

    fs::write(&pid_path, pid.to_string())
        .context("Failed to write PID file")?;

    // Lock is automatically released when file handle is dropped
    Ok(())
}

/// Read PID file
fn read_pid_file() -> Option<u32> {
    let pid_path = get_pid_file_path().ok()?;
    fs::read_to_string(&pid_path)
        .ok()?
        .trim()
        .parse()
        .ok()
}

/// Delete PID file
fn delete_pid_file() -> Result<()> {
    let pid_path = get_pid_file_path()?;
    if pid_path.exists() {
        fs::remove_file(&pid_path)
            .context("Failed to remove PID file")?;
    }
    Ok(())
}

// ============================================================================
// Process Management
// ============================================================================

/// Check if server is running by PID
fn is_server_running(pid: u32) -> bool {
    let mut sys = System::new_all();
    sys.refresh_processes(ProcessesToUpdate::All);

    if let Some(process) = sys.process(Pid::from_u32(pid)) {
        // Check if it's actually our x402-dev process
        let name = process.name().to_string_lossy();
        name.contains("x402-dev") || name.contains("mock")
    } else {
        false
    }
}

/// Stop server process gracefully
fn stop_server(pid: u32) -> Result<()> {
    use nix::sys::signal::{kill, Signal};
    use nix::unistd::Pid as NixPid;

    // Send SIGTERM for graceful shutdown
    kill(NixPid::from_raw(pid as i32), Signal::SIGTERM)
        .context("Failed to send SIGTERM")?;

    // Wait up to SHUTDOWN_TIMEOUT_SECS for graceful shutdown
    let max_attempts = (SHUTDOWN_TIMEOUT_SECS * 1000) / SHUTDOWN_POLL_INTERVAL_MS;
    for _ in 0..max_attempts {
        if !is_server_running(pid) {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(SHUTDOWN_POLL_INTERVAL_MS));
    }

    // If still running after timeout, it's an error
    Err(anyhow!(
        "Server did not shut down gracefully within {} seconds",
        SHUTDOWN_TIMEOUT_SECS
    ))
}

// ============================================================================
// Command Handlers
// ============================================================================

/// Handle stop command
pub async fn handle_stop() -> Result<()> {
    let pid = read_pid_file()
        .ok_or_else(|| anyhow!("No PID file found. Server is not running."))?;

    if !is_server_running(pid) {
        delete_pid_file()?;
        println!("Server is not running (stale PID file removed)");
        std::process::exit(2); // Exit code 2: not running
    }

    println!("Stopping server (PID: {})...", pid);
    stop_server(pid)?;
    delete_pid_file()?;
    println!("Server stopped successfully");
    Ok(())
}

/// Handle status command
pub async fn handle_status() -> Result<()> {
    match read_pid_file() {
        Some(pid) => {
            if is_server_running(pid) {
                println!("Server is running (PID: {})", pid);
                std::process::exit(0);
            } else {
                delete_pid_file()?;
                println!("Server is not running (stale PID removed)");
                std::process::exit(2);
            }
        }
        None => {
            println!("Server is not running");
            std::process::exit(2);
        }
    }
}

/// Handle restart command
pub async fn handle_restart(args: &MockArgs) -> Result<()> {
    // Stop if running
    if let Some(pid) = read_pid_file() {
        if is_server_running(pid) {
            println!("Stopping server (PID: {})...", pid);
            stop_server(pid)?;
            delete_pid_file()?;
            println!("Server stopped");
        }
    }

    // Start server
    println!("Starting server...");
    start_server(args).await
}

// ============================================================================
// Server Lifecycle
// ============================================================================

/// Start the mock facilitator server
async fn start_server(args: &MockArgs) -> Result<()> {
    let port = args.port;

    // Check if already running
    if let Some(pid) = read_pid_file() {
        if is_server_running(pid) {
            eprintln!("Server already running (PID: {})", pid);
            std::process::exit(3); // Exit code 3: already running
        } else {
            // Clean up stale PID file
            delete_pid_file()?;
        }
    }

    // Load configuration with CLI overrides
    let cli_overrides = CliOverrides {
        port: None, // Port is handled separately
        solana_rpc: None,
        log_level: None,
        pricing: args.pricing,
    };

    let config = load_merged_config(Some(&cli_overrides))
        .context("Failed to load configuration")?;

    // Create pricing matcher
    let pricing_matcher = PricingMatcher::new(config.pricing.clone());

    // Write PID file
    let current_pid = std::process::id();
    write_pid_file(current_pid)?;

    println!("🚀 Starting x402 mock facilitator server on port {}", port);
    println!("📋 Server will respond with 402 Payment Required to all requests");
    println!("💰 Default pricing: {} SOL/USDC", config.pricing.default);
    println!("🎭 Simulation mode: {:?}", config.simulation_mode);
    println!("⏱️  Timeout delay: {}ms", config.timeout_delay_ms);

    if !config.pricing.per_resource.is_empty() {
        println!("📊 Per-resource pricing rules:");
        let mut rules: Vec<_> = config.pricing.per_resource.iter().collect();
        rules.sort_by_key(|(path, _)| *path);
        for (path, amount) in rules {
            println!("   {} → {} SOL/USDC", path, amount);
        }
    }

    println!("🌐 CORS enabled for frontend testing");
    println!("🔢 PID: {}", current_pid);
    println!();
    println!("Press Ctrl+C to stop the server");
    println!();

    // Start HTTP server
    let pricing_data = web::Data::new(pricing_matcher);
    let invoice_generator = web::Data::new(InvoiceGenerator::new());
    let config_data = web::Data::new(config.clone());

    let result = HttpServer::new(move || {
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
    .map_err(|e| {
        // Check if port is already in use (exit code 2 requirement)
        if e.kind() == std::io::ErrorKind::AddrInUse {
            eprintln!("❌ Error: Port {} is already in use", port);
            eprintln!("💡 Fix: Stop the process using this port or choose a different port");
            std::process::exit(2); // Exit code 2: port in use
        }
        e
    })
    .with_context(|| format!("Failed to bind to port {}", port))?
    .run()
    .await
    .context("HTTP server error");

    // Clean up PID file on shutdown
    delete_pid_file()?;

    result
}

/// Main entry point for mock command
pub async fn run(args: &MockArgs) -> Result<()> {
    // Handle subcommands
    match &args.command {
        Some(MockSubcommand::Stop) => handle_stop().await,
        Some(MockSubcommand::Status) => handle_status().await,
        Some(MockSubcommand::Restart) => handle_restart(args).await,
        None => start_server(args).await,
    }
}

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
async fn payment_required_handler(
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

// NOTE: generate_www_authenticate_header() function removed
// Invoice generation is now handled by the Invoice struct in invoice.rs module
// Tests for invoice functionality moved to invoice.rs
