/// Epic 4 Test Framework - Reusable test utilities and fixtures
///
/// This module provides common test infrastructure for check and doctor commands.

use std::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Test fixture for creating temporary test directories
pub struct TestEnvironment {
    pub temp_dir: tempfile::TempDir,
    pub config_path: std::path::PathBuf,
}

impl TestEnvironment {
    pub fn new() -> Result<Self, std::io::Error> {
        let temp_dir = tempfile::tempdir()?;
        let config_path = temp_dir.path().join("x402.config.json");
        Ok(Self {
            temp_dir,
            config_path,
        })
    }

    pub fn write_config(&self, config: &str) -> Result<(), std::io::Error> {
        std::fs::write(&self.config_path, config)
    }

    pub fn write_package_json(&self, content: &str) -> Result<(), std::io::Error> {
        let package_path = self.temp_dir.path().join("package.json");
        std::fs::write(package_path, content)
    }
}

/// Mock HTTP server for testing check command
pub struct MockHttpServer {
    port: u16,
    handler: Arc<Mutex<Option<Box<dyn Fn() -> MockResponse + Send>>>>,
}

pub struct MockResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl MockHttpServer {
    pub fn new() -> Result<Self, std::io::Error> {
        // Find available port
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();
        drop(listener);

        Ok(Self {
            port,
            handler: Arc::new(Mutex::new(None)),
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    pub async fn with_response<F>(&mut self, handler: F)
    where
        F: Fn() -> MockResponse + Send + 'static,
    {
        let mut h = self.handler.lock().await;
        *h = Some(Box::new(handler));
    }

    pub fn response_402_with_invoice() -> MockResponse {
        MockResponse {
            status: 402,
            headers: vec![(
                "WWW-Authenticate".to_string(),
                r#"Lightning invoice="lnbc10n1pj9x7zspp5..." description="Test payment""#
                    .to_string(),
            )],
            body: "Payment Required".to_string(),
        }
    }

    pub fn response_200_ok() -> MockResponse {
        MockResponse {
            status: 200,
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: r#"{"message":"OK"}"#.to_string(),
        }
    }

    pub fn response_402_no_header() -> MockResponse {
        MockResponse {
            status: 402,
            headers: vec![],
            body: "Payment Required".to_string(),
        }
    }
}

/// Test fixture for creating valid config files
pub fn valid_config_json() -> String {
    serde_json::json!({
        "wallet": {
            "type": "lnd",
            "endpoint": "https://localhost:8080",
            "macaroon_path": "/path/to/admin.macaroon",
            "cert_path": "/path/to/tls.cert"
        },
        "server": {
            "port": 3402,
            "host": "127.0.0.1"
        },
        "policy": {
            "default_amount": 1000,
            "default_expiry": 3600
        }
    })
    .to_string()
}

/// Test fixture for creating invalid config files
pub fn invalid_config_json() -> String {
    r#"{"invalid": "json", "missing": "required_fields"}"#.to_string()
}

/// Test fixture for package.json
pub fn valid_package_json() -> String {
    serde_json::json!({
        "name": "test-api",
        "version": "1.0.0",
        "dependencies": {
            "x402-middleware": "^0.1.0"
        }
    })
    .to_string()
}

/// Helper to check if a port is available
pub fn is_port_available(port: u16) -> bool {
    TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok()
}

/// Helper to generate test BOLT11 invoices
pub fn generate_test_invoice(amount_msat: u64) -> String {
    // This is a mock BOLT11 invoice for testing
    // In real implementation, you would use lightning-invoice crate
    format!(
        "lnbc{}n1pj9x7zspp5mock_payment_hash_for_testing_purposes",
        amount_msat / 100_000_000
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_creation() {
        let env = TestEnvironment::new().expect("Failed to create test environment");
        assert!(env.temp_dir.path().exists());
        assert!(env.config_path.to_str().is_some());
    }

    #[test]
    fn test_valid_config_json() {
        let config = valid_config_json();
        let parsed: serde_json::Value =
            serde_json::from_str(&config).expect("Config should be valid JSON");
        assert!(parsed.get("wallet").is_some());
        assert!(parsed.get("server").is_some());
    }

    #[test]
    fn test_port_availability() {
        // Port 0 should always be available (OS assigns)
        assert!(is_port_available(0));
    }

    #[test]
    fn test_generate_test_invoice() {
        let invoice = generate_test_invoice(100_000);
        assert!(invoice.starts_with("lnbc"));
    }
}
