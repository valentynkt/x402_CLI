# Story 2.6: Server Lifecycle Management

Status: pending

## Story

As a developer,
I want to start, stop, and check status of the mock server,
So that I can manage server lifecycle without manual process killing.

## Acceptance Criteria

1. **Given** the mock server CLI is available
   **When** I run `x402-dev mock`
   **Then** server starts on configured port

2. **And** `x402-dev mock stop` stops the running server

3. **And** `x402-dev mock status` shows if server is running

4. **And** `x402-dev mock restart` restarts the server

5. **And** server PID is tracked for stop/restart operations

6. **And** appropriate exit codes are returned (0=success, 1=not found, 2=port in use, 3=other error)

## Tasks / Subtasks

- [ ] Task 1: Add sysinfo dependency (AC: #5)
  - [ ] Add `sysinfo = "0.31"` to workspace dependencies in Cargo.toml
  - [ ] Add `sysinfo = { workspace = true }` to crates/x402-mock/Cargo.toml

- [ ] Task 2: Implement PID file management (AC: #2, #5)
  - [ ] Create `~/.x402dev/mock-server.pid` file on server start
  - [ ] Write server PID to file
  - [ ] Create helper functions: write_pid(), read_pid(), delete_pid()
  - [ ] Ensure directory exists before writing PID

- [ ] Task 3: Implement process existence check (AC: #2, #3)
  - [ ] Use sysinfo to check if PID process exists
  - [ ] Create is_server_running() function
  - [ ] Handle stale PID file cleanup

- [ ] Task 4: Implement start command (AC: #1, #6)
  - [ ] Check if server already running before start
  - [ ] Return exit code 2 if port in use
  - [ ] Write PID file after successful start
  - [ ] Setup log file output to `~/.x402dev/logs/mock-server.log`

- [ ] Task 5: Implement stop command (AC: #2, #6)
  - [ ] Read PID from file
  - [ ] Check if process exists
  - [ ] Send SIGTERM signal to process
  - [ ] Delete PID file after stop
  - [ ] Return exit code 1 if server not found

- [ ] Task 6: Implement status command (AC: #3, #6)
  - [ ] Check PID file existence
  - [ ] Verify process is running
  - [ ] Display server status (running/stopped)
  - [ ] Show port, PID if running
  - [ ] Return appropriate exit code

- [ ] Task 7: Implement restart command (AC: #4)
  - [ ] Call stop command
  - [ ] Wait for process to terminate
  - [ ] Call start command
  - [ ] Handle case when server not running (just start)

- [ ] Task 8: Add logging infrastructure (AC: #6)
  - [ ] Create `~/.x402dev/logs/` directory
  - [ ] Configure file logging to mock-server.log
  - [ ] Add rotation or size limits (optional)
  - [ ] Log server start/stop events

- [ ] Task 9: Integrate with CLI (AC: #1-4)
  - [ ] Update Commands enum with Start, Stop, Status, Restart variants
  - [ ] Wire commands to lifecycle functions
  - [ ] Add help text for each command
  - [ ] Update main.rs to handle new commands

- [ ] Task 10: Test lifecycle management (AC: #1-6)
  - [ ] Test start command creates PID file
  - [ ] Test stop command removes PID file and terminates process
  - [ ] Test status command shows correct state
  - [ ] Test restart command sequence
  - [ ] Test error cases (port in use, not running, etc.)
  - [ ] Verify exit codes

## Dev Notes

### Architecture Constraints

- **Pure Rust Implementation** (ADR-001): Use sysinfo crate for process management
- **PID File Location**: `~/.x402dev/mock-server.pid`
- **Log File Location**: `~/.x402dev/logs/mock-server.log`
- **Error Handling**: Use anyhow::Result with context messages
- **Exit Codes**: 0=success, 1=not found, 2=port in use, 3=other error

### Project Structure

```
crates/x402-mock/src/
├── lifecycle.rs      # NEW: Lifecycle management (start, stop, status, restart)
├── pid.rs            # NEW: PID file management
├── server.rs         # Existing HTTP server (Story 2.1)
└── lib.rs            # Module exports
```

**File Locations:**
- PID file: `~/.x402dev/mock-server.pid`
- Log file: `~/.x402dev/logs/mock-server.log`

### Key Implementation Details

**PID File Management:**
```rust
use std::fs;
use std::path::PathBuf;

fn get_pid_file_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".x402dev").join("mock-server.pid")
}

fn write_pid(pid: u32) -> anyhow::Result<()> {
    let path = get_pid_file_path();
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, pid.to_string())?;
    Ok(())
}

fn read_pid() -> anyhow::Result<Option<u32>> {
    let path = get_pid_file_path();
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&path)?;
    Ok(content.trim().parse().ok())
}

fn delete_pid() -> anyhow::Result<()> {
    let path = get_pid_file_path();
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(())
}
```

**Process Existence Check:**
```rust
use sysinfo::{System, Pid};

fn is_process_running(pid: u32) -> bool {
    let mut sys = System::new();
    sys.refresh_processes();
    sys.process(Pid::from(pid as usize)).is_some()
}

fn is_server_running() -> anyhow::Result<Option<u32>> {
    if let Some(pid) = read_pid()? {
        if is_process_running(pid) {
            return Ok(Some(pid));
        } else {
            // Stale PID file, clean it up
            delete_pid()?;
        }
    }
    Ok(None)
}
```

**Start Command:**
```rust
pub fn start() -> anyhow::Result<()> {
    // Check if already running
    if let Some(pid) = is_server_running()? {
        anyhow::bail!("Server already running with PID {}", pid);
    }

    // Start server
    let config = Config::load()?;
    let pid = std::process::id();

    // Write PID file
    write_pid(pid)?;

    // Setup logging
    setup_file_logging()?;

    // Start HTTP server (Story 2.1)
    server::run(config)?;

    Ok(())
}
```

**Stop Command:**
```rust
pub fn stop() -> anyhow::Result<()> {
    let pid = is_server_running()?
        .ok_or_else(|| anyhow::anyhow!("Server is not running"))?;

    // Send SIGTERM
    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;
        kill(Pid::from_raw(pid as i32), Signal::SIGTERM)?;
    }

    #[cfg(windows)]
    {
        // Windows: use taskkill or similar
        std::process::Command::new("taskkill")
            .args(&["/PID", &pid.to_string(), "/F"])
            .status()?;
    }

    // Delete PID file
    delete_pid()?;

    println!("✅ Server stopped (PID: {})", pid);
    Ok(())
}
```

**Status Command:**
```rust
pub fn status() -> anyhow::Result<()> {
    match is_server_running()? {
        Some(pid) => {
            let config = Config::load()?;
            println!("✅ Server is running");
            println!("   PID:  {}", pid);
            println!("   Port: {}", config.port);
            std::process::exit(0);
        }
        None => {
            println!("❌ Server is not running");
            std::process::exit(1);
        }
    }
}
```

**Restart Command:**
```rust
pub fn restart() -> anyhow::Result<()> {
    println!("🔄 Restarting server...");

    // Try to stop if running
    if is_server_running()?.is_some() {
        stop()?;
        // Wait for process to terminate
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    // Start server
    start()?;

    Ok(())
}
```

**Exit Codes:**
```rust
pub enum ExitCode {
    Success = 0,
    NotFound = 1,
    PortInUse = 2,
    OtherError = 3,
}

pub fn exit_with_code(code: ExitCode) -> ! {
    std::process::exit(code as i32);
}
```

### Dependencies Added

- `sysinfo = "0.31"` - Process management and system information

### Testing Standards

- **Unit Tests**: Test PID file management functions
- **Integration Tests**: Test start/stop/status/restart commands
- **Manual CLI Testing**: Run lifecycle commands and verify behavior
- **Exit Code Testing**: Verify correct exit codes for each scenario
- **Stale PID Testing**: Test cleanup of stale PID files
- **Port Conflict Testing**: Test "port already in use" scenario

### Learnings from Previous Stories

**From Story 2.1 (HTTP Server - Status: done)**

- **HTTP server exists**: Reuse server::run() from Story 2.1
- **Port configuration**: Config::port already available
- **Server startup**: Need to wrap in lifecycle management

**From Story 1.4 (Configuration Management - Status: done)**

- **Config struct available**: Reuse for port and settings
- **Config::load()**: Load configuration for server start

**From Story 1.5 (Error Handling - Status: done)**

- **Error types**: Use existing error infrastructure
- **Exit codes**: Follow error handling patterns
- **Context messages**: Consistent error reporting

### References

- [Source: docs/epics.md#Story-2.6] - Story requirements
- [sysinfo Documentation](https://docs.rs/sysinfo/0.31/)
- [Process Management in Rust](https://doc.rust-lang.org/std/process/)

## Dev Agent Record

### Context Reference

- Implementation builds on HTTP server from Story 2.1
- Leverages Config system from Story 1.4
- Uses error handling patterns from Story 1.5

### Agent Model Used

(To be filled during implementation)

### Debug Log References

(To be filled during implementation)

### Completion Notes List

(To be filled during implementation)

### File List

**New Files:**
- crates/x402-mock/src/lifecycle.rs
- crates/x402-mock/src/pid.rs

**Modified Files:**
- Cargo.toml (add sysinfo dependency)
- crates/x402-mock/Cargo.toml (add sysinfo)
- crates/x402-mock/src/lib.rs (export lifecycle module)
- crates/x402-cli/src/main.rs (add start/stop/status/restart commands)

## Change Log

(To be filled during implementation)

---

## Senior Developer Review (AI)

(To be filled after implementation)
