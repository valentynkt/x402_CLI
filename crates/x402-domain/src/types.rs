use crate::error::{DomainError, DomainResult};
use crate::validation::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::str::FromStr;

/// Type-safe agent identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(String);

impl AgentId {
    pub fn new(s: impl Into<String>) -> DomainResult<Self> {
        let s = s.into();
        validate_non_empty(&s, "AgentId")?;
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for AgentId {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

/// Type-safe policy identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PolicyId(String);

impl PolicyId {
    pub fn new(s: impl Into<String>) -> DomainResult<Self> {
        let s = s.into();
        if s.is_empty() {
            return Err(DomainError::InvalidPolicyId("cannot be empty".into()));
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for PolicyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PolicyId {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

/// Type-safe invoice memo with UUID validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InvoiceMemo(String);

impl InvoiceMemo {
    pub fn new(s: impl Into<String>) -> DomainResult<Self> {
        let s = s.into();
        validate_uuid_format(&s)?;
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for InvoiceMemo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InvoiceMemo {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

/// Type-safe Solana address with Base58 validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SolanaAddress(String);

impl SolanaAddress {
    pub fn new(s: impl Into<String>) -> DomainResult<Self> {
        let s = s.into();
        validate_solana_address(&s)?;
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for SolanaAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SolanaAddress {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

/// Type-safe HTTP resource path
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourcePath(String);

impl ResourcePath {
    pub fn new(s: impl Into<String>) -> DomainResult<Self> {
        let s = s.into();
        validate_resource_path(&s)?;
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ResourcePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ResourcePath {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

/// Type-safe port number with range validation (1024-65535)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Port(u16);

impl Port {
    pub fn new(port: u16) -> DomainResult<Self> {
        validate_port(port)?;
        Ok(Self(port))
    }

    pub fn get(&self) -> u16 {
        self.0
    }
}

impl Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Port {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let port = s
            .parse::<u16>()
            .map_err(|_| DomainError::InvalidPort("must be a valid u16".into()))?;
        Self::new(port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_id() {
        // Valid construction
        let id = AgentId::new("agent-123").unwrap();
        assert_eq!(id.as_str(), "agent-123");
        assert_eq!(id.to_string(), "agent-123");

        // Invalid: empty
        assert!(AgentId::new("").is_err());

        // FromStr
        let id: AgentId = "agent-456".parse().unwrap();
        assert_eq!(id.as_str(), "agent-456");
    }

    #[test]
    fn test_policy_id() {
        let id = PolicyId::new("policy-abc").unwrap();
        assert_eq!(id.as_str(), "policy-abc");

        assert!(PolicyId::new("").is_err());
    }

    #[test]
    fn test_invoice_memo() {
        // Valid UUID
        let memo = InvoiceMemo::new("550e8400-e29b-41d4-a716-446655440000").unwrap();
        assert_eq!(memo.as_str(), "550e8400-e29b-41d4-a716-446655440000");

        // Invalid UUID
        assert!(InvoiceMemo::new("not-a-uuid").is_err());
        assert!(InvoiceMemo::new("").is_err());
    }

    #[test]
    fn test_solana_address() {
        // Valid address
        let addr = SolanaAddress::new("7EqQdEULxWcraVx3mXKFjc84LhCkMGZCkRuDpvcMwJeK").unwrap();
        assert_eq!(
            addr.as_str(),
            "7EqQdEULxWcraVx3mXKFjc84LhCkMGZCkRuDpvcMwJeK"
        );

        // Invalid: too short
        assert!(SolanaAddress::new("abc").is_err());

        // Invalid: contains excluded characters
        assert!(SolanaAddress::new("0OIl".repeat(11)).is_err());
    }

    #[test]
    fn test_resource_path() {
        let path = ResourcePath::new("/api/v1/agents").unwrap();
        assert_eq!(path.as_str(), "/api/v1/agents");

        // Invalid: no leading slash
        assert!(ResourcePath::new("no-slash").is_err());

        // Invalid: empty
        assert!(ResourcePath::new("").is_err());
    }

    #[test]
    fn test_port() {
        let port = Port::new(8080).unwrap();
        assert_eq!(port.get(), 8080);

        // Invalid: privileged port
        assert!(Port::new(80).is_err());
        assert!(Port::new(1023).is_err());

        // Valid: edge cases
        assert!(Port::new(1024).is_ok());
        assert!(Port::new(65535).is_ok());
    }

    #[test]
    fn test_serialization() {
        let id = AgentId::new("test-agent").unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, r#""test-agent""#);

        let deserialized: AgentId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, id);
    }
}
