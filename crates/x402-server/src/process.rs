use anyhow::{anyhow, Context, Result};
use directories::BaseDirs;
use fs2::FileExt;
use std::fs::{self, File};
use std::path::PathBuf;
use std::time::Duration;
use sysinfo::{Pid, ProcessesToUpdate, System};

// ============================================================================
// Constants
// ============================================================================

/// Maximum wait time for graceful shutdown (seconds)
pub const SHUTDOWN_TIMEOUT_SECS: u64 = 5;

/// Poll interval for checking process shutdown (milliseconds)
pub const SHUTDOWN_POLL_INTERVAL_MS: u64 = 100;

// ============================================================================
// PID File Management
// ============================================================================

/// Get path to PID file using platform-specific home directory
pub fn get_pid_file_path() -> Result<PathBuf> {
    let base_dirs = BaseDirs::new()
        .ok_or_else(|| anyhow!("Cannot determine home directory"))?;

    Ok(base_dirs
        .home_dir()
        .join(".x402dev")
        .join("mock-server.pid"))
}

/// Write PID file with exclusive locking to prevent race conditions
pub fn write_pid_file(pid: u32) -> Result<()> {
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
pub fn read_pid_file() -> Option<u32> {
    let pid_path = get_pid_file_path().ok()?;
    fs::read_to_string(&pid_path)
        .ok()?
        .trim()
        .parse()
        .ok()
}

/// Delete PID file
pub fn delete_pid_file() -> Result<()> {
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
pub fn is_server_running(pid: u32) -> bool {
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
pub fn stop_server_process(pid: u32) -> Result<()> {
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

/// Process manager for coordinating server lifecycle
pub struct ProcessManager;

impl ProcessManager {
    /// Create a new process manager
    pub fn new() -> Self {
        Self
    }

    /// Check if server is currently running
    pub fn is_running(&self) -> bool {
        if let Some(pid) = read_pid_file() {
            is_server_running(pid)
        } else {
            false
        }
    }

    /// Get the current server PID if running
    pub fn get_pid(&self) -> Option<u32> {
        read_pid_file()
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}
