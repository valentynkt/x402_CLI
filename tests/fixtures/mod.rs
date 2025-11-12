// Test fixtures module for x402-dev testing infrastructure
//
// This module provides reusable test fixtures for:
// - Policy YAML files (allowlists, denylists, rate limits, spending caps)
// - Configuration files (.x402dev.yaml)
// - x402 invoice structures
//
// All fixtures use realistic data and follow x402 protocol specifications.

pub mod policies;
pub mod configs;
pub mod invoices;

// Re-export commonly used fixture functions
pub use policies::{
    valid_policy_yaml, invalid_policy_yaml, minimal_policy_yaml,
    express_policy_yaml, fastify_policy_yaml, policy_with_pricing_tiers,
    empty_policy_yaml, malformed_yaml, missing_required_fields,
};

pub use configs::{
    valid_config_yaml, minimal_config_yaml, full_config_yaml,
    invalid_config_yaml, dev_environment_config, test_environment_config,
    prod_environment_config, config_with_invalid_port, config_with_bad_syntax,
};

pub use invoices::{
    valid_invoice, invalid_invoice, devnet_invoice, testnet_invoice,
    mainnet_invoice, invoice_with_invalid_recipient, invoice_with_negative_amount,
    invoice_with_invalid_currency, invoice_with_malformed_memo,
    invoice_missing_required_fields, random_valid_invoice,
};
