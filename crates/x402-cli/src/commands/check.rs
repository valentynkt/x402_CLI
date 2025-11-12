use crate::cli::CheckArgs;
use anyhow::{anyhow, Result};
use colored::Colorize;
use reqwest;
use std::collections::HashMap;

/// Parse WWW-Authenticate header into key-value pairs
///
/// Format: "x402-solana recipient=<addr> amount=<val> currency=USDC memo=<id> network=devnet"
fn parse_www_authenticate(header: &str) -> Result<HashMap<String, String>> {
    let mut fields = HashMap::new();

    // Split by whitespace
    let parts: Vec<&str> = header.split_whitespace().collect();

    // First part should be protocol identifier
    if parts.is_empty() || parts[0] != "x402-solana" {
        return Err(anyhow!("Invalid protocol identifier, expected 'x402-solana'"));
    }

    // Parse key=value pairs
    for part in &parts[1..] {
        if let Some((key, value)) = part.split_once('=') {
            fields.insert(key.to_string(), value.to_string());
        }
    }

    Ok(fields)
}

/// Validate invoice structure
fn validate_invoice(fields: &HashMap<String, String>) -> Vec<(String, bool, String)> {
    let mut results = Vec::new();

    // Check required fields
    let required_fields = vec!["recipient", "amount", "currency", "memo", "network"];
    for field in required_fields {
        let exists = fields.contains_key(field);
        let status = if exists { "present" } else { "missing" };
        results.push((
            format!("Field '{}'", field),
            exists,
            status.to_string(),
        ));
    }

    // Validate recipient (Base58, 32-44 chars)
    if let Some(recipient) = fields.get("recipient") {
        let valid_length = recipient.len() >= 32 && recipient.len() <= 44;
        let valid_base58 = recipient.chars().all(|c| {
            c.is_ascii_alphanumeric() && c != '0' && c != 'O' && c != 'I' && c != 'l'
        });
        let valid = valid_length && valid_base58;
        let status = if valid {
            format!("{} (valid Base58)", &recipient[..8])
        } else {
            "invalid format".to_string()
        };
        results.push((
            "Recipient address".to_string(),
            valid,
            status,
        ));
    }

    // Validate amount (parseable as f64, positive)
    if let Some(amount_str) = fields.get("amount") {
        let valid = amount_str.parse::<f64>()
            .map(|a| a > 0.0)
            .unwrap_or(false);
        let status = if valid {
            format!("{} USDC", amount_str)
        } else {
            "invalid amount".to_string()
        };
        results.push((
            "Amount".to_string(),
            valid,
            status,
        ));
    }

    // Validate currency (should be USDC)
    if let Some(currency) = fields.get("currency") {
        let valid = currency == "USDC";
        let status = if valid { "USDC" } else { "not USDC" };
        results.push((
            "Currency".to_string(),
            valid,
            status.to_string(),
        ));
    }

    // Validate memo (should start with req-)
    if let Some(memo) = fields.get("memo") {
        let valid = memo.starts_with("req-") && memo.len() > 4;
        let status = if valid { memo.clone() } else { "invalid format".to_string() };
        results.push((
            "Memo".to_string(),
            valid,
            status,
        ));
    }

    // Validate network (devnet, testnet, or mainnet-beta)
    if let Some(network) = fields.get("network") {
        let valid_networks = ["devnet", "testnet", "mainnet-beta", "mainnet"];
        let valid = valid_networks.contains(&network.as_str());
        let status = if valid {
            network.clone()
        } else {
            format!("invalid (expected devnet/testnet/mainnet-beta, got {})", network)
        };
        results.push((
            "Network".to_string(),
            valid,
            status,
        ));
    }

    results
}

/// Run the check command
pub async fn run(args: &CheckArgs) -> Result<()> {
    println!("{}", "x402 API Compliance Check".bold().cyan());
    println!("{}", "=========================".cyan());
    println!();
    println!("Checking: {}", args.url.yellow());
    println!();

    // Make HTTP request with 10 second timeout
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let response = client
        .get(&args.url)
        .send()
        .await
        .map_err(|e| anyhow!("Failed to connect to URL (timeout: 10s): {}", e))?;

    // Track validation results
    let mut checks_passed = 0;
    let mut checks_total = 0;

    // Protocol Validation Section
    println!("{}", "Protocol Validation:".bold());

    // Check 1: HTTP 402 status code
    checks_total += 1;
    let status = response.status();
    let status_check = status.as_u16() == 402;
    if status_check {
        checks_passed += 1;
        println!("  {} HTTP 402 status code: {}", "✅".green(), "PASS".green());
    } else {
        println!(
            "  {} HTTP 402 status code: {} (got {})",
            "❌".red(),
            "FAIL".red(),
            status.as_u16()
        );
    }

    // Check 2: WWW-Authenticate header
    checks_total += 1;
    let www_auth = response.headers().get("www-authenticate");
    let header_check = www_auth.is_some();
    if header_check {
        checks_passed += 1;
        println!("  {} WWW-Authenticate header: {}", "✅".green(), "PASS".green());
    } else {
        println!(
            "  {} WWW-Authenticate header: {}",
            "❌".red(),
            "FAIL (missing)".red()
        );
        println!();
        println!("{} {}", "Overall:".bold(), "❌ CHECKS FAILED".red().bold());
        std::process::exit(1);
    }

    // Parse and validate invoice structure
    let header_value = www_auth
        .unwrap()
        .to_str()
        .map_err(|e| anyhow!("Invalid header encoding: {}", e))?;

    println!();
    println!("{}", "Invoice Structure:".bold());

    let fields = match parse_www_authenticate(header_value) {
        Ok(f) => f,
        Err(e) => {
            println!("  {} Failed to parse header: {}", "❌".red(), e);
            println!();
            println!("{} {}", "Overall:".bold(), "❌ CHECKS FAILED".red().bold());
            std::process::exit(1);
        }
    };

    let validation_results = validate_invoice(&fields);

    for (name, passed, value) in validation_results {
        checks_total += 1;
        if passed {
            checks_passed += 1;
            println!("  {} {}: {}", "✅".green(), name, value);
        } else {
            println!("  {} {}: {}", "❌".red(), name, value.red());
        }
    }

    // Summary
    println!();
    if checks_passed == checks_total {
        println!(
            "{} {}",
            "Overall:".bold(),
            format!("✅ ALL CHECKS PASSED ({}/{})", checks_passed, checks_total)
                .green()
                .bold()
        );

        // JSON output if requested
        if args.format == "json" {
            let json_output = serde_json::json!({
                "status": "pass",
                "checks_passed": checks_passed,
                "checks_total": checks_total,
                "url": args.url,
            });
            println!();
            println!("{}", serde_json::to_string_pretty(&json_output)?);
        }

        Ok(())
    } else {
        println!(
            "{} {}",
            "Overall:".bold(),
            format!(
                "❌ CHECKS FAILED ({}/{} passed)",
                checks_passed, checks_total
            )
            .red()
            .bold()
        );

        // JSON output if requested
        if args.format == "json" {
            let json_output = serde_json::json!({
                "status": "fail",
                "checks_passed": checks_passed,
                "checks_total": checks_total,
                "url": args.url,
            });
            println!();
            println!("{}", serde_json::to_string_pretty(&json_output)?);
        }

        std::process::exit(1);
    }
}
