// Test fixtures module for x402-dev testing infrastructure
//
// This module provides reusable test fixtures for:
// - Policy YAML files (allowlists, denylists, rate limits, spending caps)
// - Configuration files (.x402dev.yaml)
// - x402 invoice structures
//
// All fixtures use realistic data and follow x402 protocol specifications.

pub mod configs;
pub mod invoices;
pub mod policies;

// Re-export commonly used fixture functions
pub use policies::{
    empty_policy_yaml, express_policy_yaml, fastify_policy_yaml, invalid_policy_yaml,
    malformed_yaml, minimal_policy_yaml, missing_required_fields, policy_with_pricing_tiers,
    valid_policy_yaml,
};

pub use configs::{
    config_with_bad_syntax, config_with_invalid_port, dev_environment_config, full_config_yaml,
    invalid_config_yaml, minimal_config_yaml, prod_environment_config, test_environment_config,
    valid_config_yaml,
};

pub use invoices::{
    devnet_invoice, invalid_invoice, invoice_missing_required_fields,
    invoice_with_invalid_currency, invoice_with_invalid_recipient, invoice_with_malformed_memo,
    invoice_with_negative_amount, mainnet_invoice, random_valid_invoice, testnet_invoice,
    valid_invoice,
};
