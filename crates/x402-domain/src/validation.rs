use crate::error::{DomainError, DomainResult};

/// Validates that a string is non-empty
pub fn validate_non_empty(s: &str, field_name: &str) -> DomainResult<()> {
    if s.is_empty() {
        Err(DomainError::InvalidAgentId(format!(
            "{} cannot be empty",
            field_name
        )))
    } else {
        Ok(())
    }
}

/// Validates UUID format (basic check: 36 characters with hyphens)
pub fn validate_uuid_format(s: &str) -> DomainResult<()> {
    if s.len() != 36 {
        return Err(DomainError::InvalidInvoiceMemo(
            "UUID must be 36 characters".into(),
        ));
    }

    // Check format: 8-4-4-4-12
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 5 {
        return Err(DomainError::InvalidInvoiceMemo(
            "UUID must have format XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX".into(),
        ));
    }

    if parts[0].len() != 8
        || parts[1].len() != 4
        || parts[2].len() != 4
        || parts[3].len() != 4
        || parts[4].len() != 12
    {
        return Err(DomainError::InvalidInvoiceMemo(
            "UUID has incorrect segment lengths".into(),
        ));
    }

    // Check all characters are hex digits or hyphens
    if !s.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
        return Err(DomainError::InvalidInvoiceMemo(
            "UUID must contain only hex digits and hyphens".into(),
        ));
    }

    Ok(())
}

/// Validates Base58 Solana address (32-44 characters)
pub fn validate_solana_address(s: &str) -> DomainResult<()> {
    if s.len() < 32 || s.len() > 44 {
        return Err(DomainError::InvalidSolanaAddress(
            "length must be between 32 and 44 characters".into(),
        ));
    }

    // Basic Base58 character validation (0-9, A-Z, a-z excluding 0, O, I, l)
    const BASE58_CHARS: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    if !s.chars().all(|c| BASE58_CHARS.contains(c)) {
        return Err(DomainError::InvalidSolanaAddress(
            "contains invalid Base58 characters (excludes 0, O, I, l)".into(),
        ));
    }

    Ok(())
}

/// Validates HTTP resource path format
pub fn validate_resource_path(s: &str) -> DomainResult<()> {
    if s.is_empty() {
        return Err(DomainError::InvalidResourcePath("cannot be empty".into()));
    }

    if !s.starts_with('/') {
        return Err(DomainError::InvalidResourcePath(
            "must start with '/'".into(),
        ));
    }

    // Check for valid URL path characters (including * for wildcards)
    if !s.chars().all(|c| {
        c.is_alphanumeric() || c == '/' || c == '-' || c == '_' || c == '.' || c == '~' || c == '*'
    }) {
        return Err(DomainError::InvalidResourcePath(
            "contains invalid path characters".into(),
        ));
    }

    Ok(())
}

/// Validates port range (1024-65535 for non-privileged ports)
pub fn validate_port(port: u16) -> DomainResult<()> {
    if port < 1024 {
        Err(DomainError::InvalidPort(
            "must be >= 1024 (non-privileged port)".into(),
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_non_empty() {
        assert!(validate_non_empty("test", "field").is_ok());
        assert!(validate_non_empty("", "field").is_err());
    }

    #[test]
    fn test_validate_uuid_format() {
        // Valid UUID
        assert!(validate_uuid_format("550e8400-e29b-41d4-a716-446655440000").is_ok());

        // Invalid UUIDs
        assert!(validate_uuid_format("").is_err());
        assert!(validate_uuid_format("not-a-uuid").is_err());
        assert!(validate_uuid_format("550e8400-e29b-41d4-a716").is_err()); // Too short
    }

    #[test]
    fn test_validate_solana_address() {
        // Valid addresses (length 32-44)
        assert!(validate_solana_address("7EqQdEULxWcraVx3mXKFjc84LhCkMGZCkRuDpvcMwJeK").is_ok());

        // Invalid addresses
        assert!(validate_solana_address("").is_err()); // Too short
        assert!(validate_solana_address("abc").is_err()); // Too short
        assert!(validate_solana_address("0OIl".repeat(11).as_str()).is_err()); // Invalid chars
    }

    #[test]
    fn test_validate_resource_path() {
        assert!(validate_resource_path("/api/v1/agents").is_ok());
        assert!(validate_resource_path("/").is_ok());

        assert!(validate_resource_path("").is_err()); // Empty
        assert!(validate_resource_path("no-slash").is_err()); // No leading slash
        assert!(validate_resource_path("/invalid space").is_err()); // Invalid chars
    }

    #[test]
    fn test_validate_port() {
        assert!(validate_port(8080).is_ok());
        assert!(validate_port(65535).is_ok());

        assert!(validate_port(80).is_err()); // Privileged port
        assert!(validate_port(1023).is_err()); // Below threshold
    }
}
