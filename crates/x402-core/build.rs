// DISABLED: Pure Rust implementation (ADR-001)
// TypeScript runtime integration removed - see docs/epics.md Story 1.8
//
// This build.rs previously bundled TypeScript code for deno_core integration.
// Kept for reference but disabled per KISS refactoring (2025-11-10).
// Can be re-enabled in ~2 hours if requirements change post-hackathon.

fn main() {
    // No build steps needed for pure Rust implementation
    println!("cargo:warning=Pure Rust build (no TypeScript bundling)");
}
