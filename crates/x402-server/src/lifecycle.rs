use anyhow::{anyhow, Result};
use crate::process::{
    delete_pid_file, is_server_running, read_pid_file, stop_server_process, write_pid_file,
};
use crate::server::{MockServer, MockServerConfig};

// ============================================================================
// Command Handlers
// ============================================================================

/// Handle stop command
pub async fn stop_server() -> Result<()> {
    let pid = read_pid_file()
        .ok_or_else(|| anyhow!("No PID file found. Server is not running."))?;

    if !is_server_running(pid) {
        delete_pid_file()?;
        println!("Server is not running (stale PID file removed)");
        std::process::exit(2); // Exit code 2: not running
    }

    println!("Stopping server (PID: {})...", pid);
    stop_server_process(pid)?;
    delete_pid_file()?;
    println!("Server stopped successfully");
    Ok(())
}

/// Handle status command
pub async fn server_status() -> Result<()> {
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
pub async fn restart_server(config: MockServerConfig) -> Result<()> {
    // Stop if running
    if let Some(pid) = read_pid_file() {
        if is_server_running(pid) {
            println!("Stopping server (PID: {})...", pid);
            stop_server_process(pid)?;
            delete_pid_file()?;
            println!("Server stopped");
        }
    }

    // Start server
    println!("Starting server...");
    start_server(config).await
}

// ============================================================================
// Server Lifecycle
// ============================================================================

/// Start the mock facilitator server
pub async fn start_server(server_config: MockServerConfig) -> Result<()> {
    let port = server_config.port;

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

    // Write PID file
    let current_pid = std::process::id();
    write_pid_file(current_pid)?;

    println!("🚀 Starting x402 mock facilitator server on port {}", port);
    println!("📋 Server will respond with 402 Payment Required to all requests");
    println!("💰 Default pricing: {} SOL/USDC", server_config.config.pricing.default);
    println!("🎭 Simulation mode: {:?}", server_config.config.simulation_mode);
    println!("⏱️  Timeout delay: {}ms", server_config.config.timeout_delay_ms);

    if !server_config.config.pricing.per_resource.is_empty() {
        println!("📊 Per-resource pricing rules:");
        let mut rules: Vec<_> = server_config.config.pricing.per_resource.iter().collect();
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
    let server = MockServer::new(server_config);
    let result = server.run().await;

    // Clean up PID file on shutdown
    delete_pid_file()?;

    result
}
