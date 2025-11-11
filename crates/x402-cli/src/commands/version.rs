use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use semver::Version;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

const CRATES_IO_API: &str = "https://crates.io/api/v1/crates/x402-dev";
const CHECK_INTERVAL_SECS: u64 = 604800; // 7 days in seconds

#[derive(Debug, Serialize, Deserialize)]
struct UpdateCache {
    last_check: u64,
    latest_version: String,
}

#[derive(Debug, Deserialize)]
struct CratesIoResponse {
    #[serde(rename = "crate")]
    crate_info: CrateInfo,
}

#[derive(Debug, Deserialize)]
struct CrateInfo {
    max_version: String,
}

pub async fn run(args: &crate::cli::VersionArgs) -> Result<()> {
    // Display version information
    println!("x402-dev v{}", env!("CARGO_PKG_VERSION"));
    println!(
        "Platform: {}-{}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );

    // Display Rust version
    // Note: CARGO_PKG_RUST_VERSION is only available in Rust 1.56+
    // For now, we'll skip the Rust version display since we can't reliably get it
    // without external crates. We can add rustc_version crate later if needed.

    // Check for updates (if not disabled)
    if !args.no_update_check {
        // Silently ignore any errors from update check
        // Update check is optional and should never fail the version command
        let _ = check_for_updates().await;
    }

    Ok(())
}

async fn check_for_updates() -> Result<()> {
    let cache_path = get_cache_path()?;
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("Failed to get current time")?
        .as_secs();

    // Check if we should perform update check
    if let Ok(cache_data) = fs::read_to_string(&cache_path) {
        if let Ok(cache) = serde_json::from_str::<UpdateCache>(&cache_data) {
            // If last check was within the interval, use cached result
            if current_time - cache.last_check < CHECK_INTERVAL_SECS {
                let current_version = env!("CARGO_PKG_VERSION");
                if is_newer_version(&cache.latest_version, current_version) {
                    println!(
                        "\n✨ Update available: {} → {}",
                        current_version, cache.latest_version
                    );
                    println!("Run: cargo install x402-dev");
                }
                return Ok(());
            }
        }
    }

    // Perform update check
    let latest_version = fetch_latest_version().await?;
    let current_version = env!("CARGO_PKG_VERSION");

    // Save cache
    let cache = UpdateCache {
        last_check: current_time,
        latest_version: latest_version.clone(),
    };

    // Ensure cache directory exists
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent).context("Failed to create cache directory")?;
    }

    fs::write(
        &cache_path,
        serde_json::to_string_pretty(&cache).context("Failed to serialize cache")?,
    )
    .context("Failed to write cache file")?;

    // Display update notification if newer version available
    if is_newer_version(&latest_version, current_version) {
        println!(
            "\n✨ Update available: {} → {}",
            current_version, latest_version
        );
        println!("Run: cargo install x402-dev");
    }

    Ok(())
}

async fn fetch_latest_version() -> Result<String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .context("Failed to create HTTP client")?;

    let response = client
        .get(CRATES_IO_API)
        .send()
        .await
        .context("Failed to connect to crates.io")?;

    let crate_info: CratesIoResponse = response
        .json()
        .await
        .context("Failed to parse crates.io response")?;

    Ok(crate_info.crate_info.max_version)
}

fn get_cache_path() -> Result<PathBuf> {
    // Use home directory + .x402dev for cross-platform compatibility
    let home_dir = dirs::home_dir()
        .context("Failed to determine home directory")?;

    Ok(home_dir.join(".x402dev").join("update-check.json"))
}

mod dirs {
    use std::path::PathBuf;

    pub fn home_dir() -> Option<PathBuf> {
        directories::UserDirs::new().map(|ud| ud.home_dir().to_path_buf())
    }
}

fn is_newer_version(latest: &str, current: &str) -> bool {
    // Proper semantic versioning comparison using semver crate
    // Returns true if latest > current, false otherwise or if parsing fails
    match (Version::parse(latest), Version::parse(current)) {
        (Ok(latest_ver), Ok(current_ver)) => latest_ver > current_ver,
        _ => {
            // If version parsing fails, fall back to string comparison
            // This handles edge cases like pre-release versions
            latest != current && latest > current
        }
    }
}
