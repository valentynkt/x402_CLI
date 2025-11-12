// Test to verify that all fixture modules compile and function correctly

#[path = "../fixtures/mod.rs"]
mod fixtures;

use fixtures::*;
use fixtures::invoices::{
    invoice_with_zero_amount, invoice_with_excessive_amount,
    invoice_with_invalid_network, expired_invoice, almost_expired_invoice,
};

#[test]
fn test_valid_policy_yaml_fixture() {
    let yaml = valid_policy_yaml();
    assert!(yaml.contains("policies:"));
    assert!(yaml.contains("type: allowlist"));
    assert!(yaml.contains("type: rate_limit"));
    assert!(yaml.contains("type: spending_cap"));
}

#[test]
fn test_minimal_policy_yaml_fixture() {
    let yaml = minimal_policy_yaml();
    assert!(yaml.contains("policies:"));
    assert_eq!(yaml.matches("type:").count(), 1);
}

#[test]
fn test_framework_specific_policy_fixtures() {
    let express = express_policy_yaml();
    assert!(express.contains("/api/v1/users"));

    let fastify = fastify_policy_yaml();
    assert!(fastify.contains("/v1/health"));
}

#[test]
fn test_policy_with_pricing_tiers_fixture() {
    let yaml = policy_with_pricing_tiers();
    assert!(yaml.contains("/api/free/*"));
    assert!(yaml.contains("/api/premium/*"));
    assert!(yaml.contains("spending_cap"));
}

#[test]
fn test_empty_policy_yaml_fixture() {
    let yaml = empty_policy_yaml();
    assert!(yaml.contains("policies: []"));
}

#[test]
fn test_malformed_yaml_fixture() {
    let yaml = malformed_yaml();
    assert!(yaml.contains("invalid"));
}

#[test]
fn test_missing_required_fields_fixtures() {
    let variants = vec![
        "allowlist_no_field",
        "allowlist_no_values",
        "rate_limit_no_max_requests",
        "rate_limit_no_window",
        "spending_cap_no_amount",
        "spending_cap_no_currency",
    ];

    for variant in variants {
        let yaml = missing_required_fields(variant);
        assert!(!yaml.is_empty(), "Variant {} should not be empty", variant);
        assert!(yaml.contains("policies:"));
    }
}

#[test]
fn test_invalid_policy_yaml_fixtures() {
    let variants = vec![
        "empty_field_name",
        "empty_values_list",
        "zero_max_requests",
        "negative_max_requests",
        "zero_window_seconds",
        "negative_amount",
        "zero_amount",
        "empty_currency",
    ];

    for variant in variants {
        let yaml = invalid_policy_yaml(variant);
        assert!(!yaml.is_empty(), "Variant {} should not be empty", variant);
    }
}

#[test]
fn test_valid_config_yaml_fixture() {
    let yaml = valid_config_yaml();
    assert!(yaml.contains("port:"));
    assert!(yaml.contains("solana_rpc:"));
    assert!(yaml.contains("pricing:"));
}

#[test]
fn test_minimal_config_yaml_fixture() {
    let yaml = minimal_config_yaml();
    assert!(yaml.contains("port: 8402"));
    assert!(yaml.contains("solana_rpc:"));
    assert!(!yaml.contains("log_level:"));
}

#[test]
fn test_full_config_yaml_fixture() {
    let yaml = full_config_yaml();
    assert!(yaml.contains("log_level:"));
    assert!(yaml.contains("simulation_mode:"));
    assert!(yaml.contains("timeout_delay_ms:"));
    assert!(yaml.contains("per_resource:"));
}

#[test]
fn test_environment_config_fixtures() {
    let dev = dev_environment_config();
    assert!(dev.contains("localhost:8899"));
    assert!(dev.contains("log_level: debug"));

    let test = test_environment_config();
    assert!(test.contains("devnet"));
    assert!(test.contains("log_level: warn"));

    let prod = prod_environment_config();
    assert!(prod.contains("mainnet-beta"));
    assert!(prod.contains("log_level: info"));
}

#[test]
fn test_invalid_config_fixtures() {
    let invalid_port = config_with_invalid_port();
    assert!(invalid_port.contains("port: 80"));

    let bad_syntax = config_with_bad_syntax();
    assert!(bad_syntax.contains("not_a_number"));
}

#[test]
fn test_invalid_config_yaml_fixtures() {
    let variants = vec![
        "port_too_low",
        "port_too_high",
        "invalid_rpc_url",
        "invalid_log_level",
        "invalid_simulation_mode",
        "negative_pricing",
        "excessive_pricing",
        "negative_per_resource_pricing",
        "timeout_too_low",
        "timeout_too_high",
    ];

    for variant in variants {
        let yaml = invalid_config_yaml(variant);
        assert!(!yaml.is_empty(), "Variant {} should not be empty", variant);
    }
}

#[test]
fn test_valid_invoice_fixture() {
    let invoice = valid_invoice();
    assert_eq!(invoice.currency, "USDC");
    assert_eq!(invoice.network, "devnet");
    assert!(invoice.amount > 0.0);
    assert!(invoice.expires_at > invoice.timestamp);
    assert!(!invoice.recipient.is_empty());
    assert!(!invoice.memo.is_empty());
}

#[test]
fn test_network_specific_invoice_fixtures() {
    let devnet = devnet_invoice(0.01);
    assert_eq!(devnet.network, "devnet");
    assert_eq!(devnet.amount, 0.01);

    let testnet = testnet_invoice(0.02);
    assert_eq!(testnet.network, "testnet");
    assert_eq!(testnet.amount, 0.02);

    let mainnet = mainnet_invoice(0.05);
    assert_eq!(mainnet.network, "mainnet-beta");
    assert_eq!(mainnet.amount, 0.05);
}

#[test]
fn test_invalid_recipient_fixtures() {
    let variants = vec!["empty", "too_short", "invalid_chars", "special_chars"];

    for variant in variants {
        let invoice = invoice_with_invalid_recipient(variant);
        // Should have invalid recipient
        match variant {
            "empty" => assert_eq!(invoice.recipient, ""),
            "too_short" => assert_eq!(invoice.recipient.len(), 3),
            _ => assert!(!invoice.recipient.is_empty()),
        }
    }
}

#[test]
fn test_amount_edge_case_fixtures() {
    let negative = invoice_with_negative_amount();
    assert!(negative.amount < 0.0);

    let zero = invoice_with_zero_amount();
    assert_eq!(zero.amount, 0.0);

    let excessive = invoice_with_excessive_amount();
    assert!(excessive.amount > 1000000.0);
}

#[test]
fn test_invalid_currency_fixtures() {
    let variants = vec!["empty", "wrong_currency", "lowercase", "invalid"];

    for variant in variants {
        let invoice = invoice_with_invalid_currency(variant);
        assert_ne!(invoice.currency, "USDC", "Variant {} should have invalid currency", variant);
    }
}

#[test]
fn test_malformed_memo_fixtures() {
    let variants = vec!["empty", "not_uuid", "wrong_format", "no_hyphens", "invalid_chars"];

    for variant in variants {
        let invoice = invoice_with_malformed_memo(variant);
        // Valid UUID format: 36 characters with 4 hyphens
        let is_valid_uuid = invoice.memo.len() == 36 && invoice.memo.matches('-').count() == 4;
        assert!(!is_valid_uuid || variant == "invalid_chars",
                "Variant {} should have invalid memo", variant);
    }
}

#[test]
fn test_invalid_network_fixtures() {
    let variants = vec!["empty", "wrong_network", "typo"];

    for variant in variants {
        let invoice = invoice_with_invalid_network(variant);
        assert!(
            invoice.network != "devnet"
            && invoice.network != "testnet"
            && invoice.network != "mainnet-beta",
            "Variant {} should have invalid network", variant
        );
    }
}

#[test]
fn test_expired_invoice_fixtures() {
    let expired = expired_invoice();
    assert!(chrono::Utc::now() > expired.expires_at);

    let almost = almost_expired_invoice();
    let time_left = almost.expires_at - chrono::Utc::now();
    assert!(time_left.num_seconds() <= 30);
}

#[test]
fn test_missing_invoice_fields_fixtures() {
    let variants = vec!["no_recipient", "no_amount", "no_currency", "no_memo", "no_network"];

    for variant in variants {
        let json = invoice_missing_required_fields(variant);
        assert!(!json.is_empty(), "Variant {} should not be empty", variant);
        assert!(json.contains("{"));
        assert!(json.contains("}"));
        // Verify the field is actually missing
        match variant {
            "no_recipient" => assert!(!json.contains("recipient")),
            "no_amount" => assert!(!json.contains("amount")),
            "no_currency" => assert!(!json.contains("currency")),
            "no_memo" => assert!(!json.contains("memo")),
            "no_network" => assert!(!json.contains("network")),
            _ => {}
        }
    }
}

#[test]
fn test_random_invoice_fixture_uniqueness() {
    let invoice1 = random_valid_invoice();
    let invoice2 = random_valid_invoice();

    // Should generate unique memos
    assert_ne!(invoice1.memo, invoice2.memo);

    // Both should be valid
    assert!(invoice1.amount > 0.0);
    assert!(invoice2.amount > 0.0);
    assert_eq!(invoice1.currency, "USDC");
    assert_eq!(invoice2.currency, "USDC");
}

#[test]
fn test_invalid_invoice_convenience_function() {
    let error_types = vec![
        "invalid_recipient",
        "negative_amount",
        "zero_amount",
        "invalid_currency",
        "malformed_memo",
        "invalid_network",
        "expired",
    ];

    for error_type in error_types {
        let invoice = invalid_invoice(error_type);
        // Verify each invoice has the expected error
        match error_type {
            "negative_amount" => assert!(invoice.amount < 0.0),
            "zero_amount" => assert_eq!(invoice.amount, 0.0),
            "invalid_currency" => assert_ne!(invoice.currency, "USDC"),
            "expired" => assert!(chrono::Utc::now() > invoice.expires_at),
            _ => {} // Other error types validated in their specific tests
        }
    }
}

#[test]
fn test_valid_test_addresses_constant() {
    use fixtures::invoices::VALID_TEST_ADDRESSES;

    assert!(VALID_TEST_ADDRESSES.len() >= 6);

    for addr in VALID_TEST_ADDRESSES {
        assert!(addr.len() >= 32 && addr.len() <= 44,
                "Address {} has invalid length", addr);

        // Check Base58 characters only (no 0, O, I, l)
        for c in addr.chars() {
            assert!(c.is_ascii_alphanumeric(),
                    "Address {} contains non-alphanumeric char: {}", addr, c);
            assert!(c != '0' && c != 'O' && c != 'I' && c != 'l',
                    "Address {} contains invalid Base58 char: {}", addr, c);
        }
    }
}

#[test]
fn test_invalid_addresses_constant() {
    use fixtures::invoices::INVALID_ADDRESSES;

    assert!(INVALID_ADDRESSES.len() >= 4);

    // First address should be empty
    assert_eq!(INVALID_ADDRESSES[0], "");

    // Second should be too short
    assert!(INVALID_ADDRESSES[1].len() < 32);

    // Third should contain invalid Base58 characters
    assert!(INVALID_ADDRESSES[2].contains('0') || INVALID_ADDRESSES[2].contains('I'));
}
