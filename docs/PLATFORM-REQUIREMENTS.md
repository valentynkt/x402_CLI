# Platform Requirements

**Date:** 2025-11-11
**Epic:** 2 - Mock Facilitator Server
**Status:** ✅ DOCUMENTED

---

## Supported Platforms

### ✅ **Unix/Linux (Fully Supported)**
- **Operating Systems:** Linux, macOS, BSD variants
- **Tested On:**
  - macOS Darwin 25.0.0 (Apple Silicon)
  - Linux (Ubuntu, Debian, Fedora recommended)

### ❌ **Windows (Not Supported)**
- **Status:** NOT SUPPORTED
- **Reason:** Server lifecycle management (Story 2.6) uses Unix-specific signal handling
- **Workaround:** Use WSL2 (Windows Subsystem for Linux) for Windows development

---

## Technical Details

### Unix-Specific Dependencies

**1. Signal Handling (Story 2.6)**
- **Crate:** `nix` v0.29
- **Feature:** `signal` module for SIGTERM graceful shutdown
- **Location:** `crates/x402-cli/src/commands/mock.rs:73-78`
- **Code:**
  ```rust
  use nix::sys::signal::{kill, Signal};
  use nix::unistd::Pid as NixPid;

  kill(NixPid::from_raw(pid as i32), Signal::SIGTERM)
  ```

**2. Process Management**
- **Crate:** `sysinfo` v0.31 (cross-platform, but signal handling is Unix-only)
- **Feature:** Process status checking, PID validation
- **Location:** `crates/x402-cli/src/commands/mock.rs:58-69`

**3. File Locking**
- **Crate:** `fs2` v0.4
- **Feature:** Exclusive file locking for PID file (TOCTOU mitigation)
- **Location:** `crates/x402-cli/src/commands/mock.rs:54-58`
- **Platform Support:** Unix, Linux, macOS (cross-platform with different implementations)

---

## Platform-Specific Behavior

### PID File Location
**Path:** `~/.x402dev/mock-server.pid`

**Unix/Linux:**
- Resolved via `BaseDirs::home_dir()` from `directories` crate
- Typical paths:
  - macOS: `/Users/<username>/.x402dev/mock-server.pid`
  - Linux: `/home/<username>/.x402dev/mock-server.pid`

**Windows (WSL2):**
- Resolved to WSL2 home directory: `/home/<username>/.x402dev/mock-server.pid`

### Process Signal Handling

**Unix/Linux:**
- **SIGTERM (15):** Graceful shutdown request
- **Behavior:** Server receives signal, cleans up PID file, exits gracefully
- **Timeout:** 5 seconds (configurable via `SHUTDOWN_TIMEOUT_SECS`)

**Windows:**
- **NOT SUPPORTED:** Windows does not have POSIX signals
- **Alternative:** Process termination via Windows APIs (not implemented)

---

## Dependencies Requiring Unix

| Crate | Version | Unix-Only Feature | Alternative for Windows |
|-------|---------|-------------------|------------------------|
| `nix` | 0.29 | `signal::kill()` | Windows Process API (not implemented) |
| `sysinfo` | 0.31 | Process management | Cross-platform (works on Windows) |
| `fs2` | 0.4 | File locking | Cross-platform (works on Windows) |

---

## Running on Windows

### Option 1: WSL2 (Recommended)

**Install WSL2:**
```bash
wsl --install
wsl --set-default-version 2
```

**Install Rust in WSL2:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Build and Run:**
```bash
cd /path/to/x402-hackathon
cargo build --release
./target/release/x402-dev mock
```

### Option 2: Native Windows Support (Future Work)

**Changes Required:**
1. Replace `nix` signal handling with Windows Process API
2. Implement graceful shutdown using `SetConsoleCtrlHandler`
3. Test PID file locking on Windows NTFS
4. Conditional compilation with `#[cfg(unix)]` and `#[cfg(windows)]`

**Estimated Effort:** 4-6 hours (post-hackathon)

---

## Build Requirements

### Minimum Rust Version
- **Version:** 1.75+ (edition 2021)
- **Check:** `rustc --version`

### System Dependencies

**macOS:**
```bash
# No additional system dependencies required
# Xcode Command Line Tools recommended
xcode-select --install
```

**Linux (Debian/Ubuntu):**
```bash
sudo apt-get update
sudo apt-get install build-essential pkg-config libssl-dev
```

**Linux (Fedora/RHEL):**
```bash
sudo dnf install gcc pkg-config openssl-devel
```

---

## Testing Platform Support

### Verify Unix Features
```bash
# Test signal handling
x402-dev mock &
PID=$!
kill -TERM $PID  # Should gracefully shutdown

# Test PID file locking
x402-dev mock &
x402-dev mock  # Should fail with "already running" error
```

### Check Platform Detection
```bash
# Verify home directory resolution
rust -e 'use directories::BaseDirs; println!("{:?}", BaseDirs::new().unwrap().home_dir());'
```

---

## Known Limitations

### 1. **Windows Native Support**
- **Status:** NOT AVAILABLE
- **Workaround:** Use WSL2
- **Future:** Planned for post-hackathon (Epic 5+)

### 2. **macOS Notarization**
- **Status:** NOT IMPLEMENTED
- **Impact:** macOS Gatekeeper may block unsigned binaries
- **Workaround:** Build from source or use `xattr -d com.apple.quarantine`

### 3. **Signal Handling Edge Cases**
- **SIGKILL (9):** Forceful termination leaves stale PID file
- **Mitigation:** Stale PID file cleanup on next startup
- **Location:** `crates/x402-cli/src/commands/mock.rs:165-167`

---

## Compatibility Matrix

| Platform | Server Startup | Lifecycle Management | PID Tracking | File Locking | Status |
|----------|----------------|---------------------|--------------|--------------|--------|
| **macOS** | ✅ | ✅ | ✅ | ✅ | **SUPPORTED** |
| **Linux** | ✅ | ✅ | ✅ | ✅ | **SUPPORTED** |
| **WSL2** | ✅ | ✅ | ✅ | ✅ | **SUPPORTED** |
| **Windows Native** | ✅ | ❌ | ⚠️ | ✅ | **NOT SUPPORTED** |
| **BSD** | ✅ | ✅ | ✅ | ✅ | **UNTESTED** |

**Legend:**
- ✅ Fully supported and tested
- ⚠️ Partially supported (stale PID detection works, graceful shutdown doesn't)
- ❌ Not supported

---

## Recommendations

### For Development
1. **Use Unix/Linux/macOS** for full feature support
2. **WSL2 on Windows** works perfectly for development and testing
3. **Docker** can be used for isolated testing environments

### For Production
1. **Deploy to Linux servers** (Ubuntu 22.04 LTS, Debian 12, or similar)
2. **Avoid Windows Server** for production deployments
3. **macOS** is suitable for local testing and demos

### For CI/CD
```yaml
# GitHub Actions example
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest]
    # Exclude windows-latest (not supported)
```

---

## Future Work

### Windows Native Support (Epic 5+)
- Replace Unix signal handling with Windows APIs
- Conditional compilation for platform-specific code
- Test suite for Windows-specific functionality

### Enhanced Cross-Platform Support
- Abstract signal handling behind platform-agnostic trait
- Unified lifecycle management API
- Cross-platform integration tests

---

## References

- **ADR-001:** Pure Rust KISS Architecture (`docs/decisions/001-pure-rust-kiss-architecture.md`)
- **Story 2.6:** Server Lifecycle Management (`docs/stories/2-6-lifecycle-management.md`)
- **Nix Crate:** https://docs.rs/nix/0.29/nix/sys/signal/
- **Sysinfo Crate:** https://docs.rs/sysinfo/0.31/sysinfo/

---

**Document Owner:** System Architect
**Last Updated:** 2025-11-11
**Status:** ✅ COMPLETE
