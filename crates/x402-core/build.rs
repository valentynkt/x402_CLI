use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Determine workspace root using CARGO_MANIFEST_DIR
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    let manifest_path = PathBuf::from(&manifest_dir);
    let workspace_root = manifest_path
        .parent()
        .and_then(|p| p.parent())
        .expect("Failed to determine workspace root");
    let ts_dir = workspace_root.join("ts");

    // Tell Cargo to rerun this build script if TypeScript source changes
    println!("cargo:rerun-if-changed={}", ts_dir.join("src").display());
    println!("cargo:rerun-if-changed={}", ts_dir.join("package.json").display());
    println!("cargo:rerun-if-changed={}", ts_dir.join("tsconfig.json").display());
    println!("cargo:rerun-if-changed={}", ts_dir.join("tsup.config.ts").display());

    // Verify npm and node are available
    if Command::new("node").arg("--version").output().is_err() {
        panic!("Node.js is not installed or not in PATH. Please install Node.js >= 18.0.0");
    }

    if Command::new("npm").arg("--version").output().is_err() {
        panic!("npm is not installed or not in PATH. Please install npm >= 9.0.0");
    }

    // Build TypeScript
    println!("cargo:warning=Building TypeScript runtime...");

    let status = Command::new("npm")
        .args(&["run", "build"])
        .current_dir(&ts_dir)
        .status()
        .expect("Failed to execute npm run build");

    if !status.success() {
        panic!("TypeScript build failed!");
    }

    println!("cargo:warning=TypeScript runtime built successfully");
}
