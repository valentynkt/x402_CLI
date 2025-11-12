//! HTTP mock server utilities using wiremock
//!
//! Provides helper functions for creating mock HTTP servers that simulate
//! x402 payment protocol responses for testing purposes.
//!
//! # Examples
//!
//! ```no_run
//! use helpers::mock_server::mock_402_server;
//!
//! #[tokio::test]
//! async fn test_402_response() {
//!     let server = mock_402_server().await;
//!     let response = reqwest::get(&server.uri()).await.unwrap();
//!     assert_eq!(response.status(), 402);
//! }
//! ```

use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

/// Extension trait for MockServer with x402-specific helpers
pub trait MockServerExt {
    /// Get the base URI of the mock server
    fn base_uri(&self) -> String;

    /// Get a specific endpoint URI
    fn endpoint_uri(&self, path: &str) -> String;
}

impl MockServerExt for MockServer {
    fn base_uri(&self) -> String {
        self.uri()
    }

    fn endpoint_uri(&self, endpoint: &str) -> String {
        format!("{}{}", self.uri(), endpoint)
    }
}

/// Creates a mock server that responds with 402 Payment Required
///
/// The server will respond with:
/// - Status: 402 Payment Required
/// - Header: WWW-Authenticate with x402 invoice
/// - Header: X-402-Invoice with JSON invoice data
/// - Body: JSON error message with payment details
///
/// # Example
///
/// ```no_run
/// #[tokio::test]
/// async fn test_payment_required() {
///     let server = mock_402_server().await;
///     let client = reqwest::Client::new();
///
///     let response = client.get(&server.uri()).await.unwrap();
///
///     assert_eq!(response.status(), 402);
///     assert!(response.headers().contains_key("www-authenticate"));
/// }
/// ```
pub async fn mock_402_server() -> MockServer {
    mock_402_server_with_amount(1000).await
}

/// Creates a mock 402 server with a custom payment amount
///
/// # Arguments
///
/// * `amount_lamports` - Payment amount in lamports (1 SOL = 1,000,000,000 lamports)
///
/// # Example
///
/// ```no_run
/// let server = mock_402_server_with_amount(5000).await;
/// ```
pub async fn mock_402_server_with_amount(amount_lamports: u64) -> MockServer {
    let server = MockServer::start().await;

    let invoice = json!({
        "amount": amount_lamports,
        "currency": "SOL",
        "address": "x402TestPaymentAddress",
        "expires_at": chrono::Utc::now()
            .checked_add_signed(chrono::Duration::minutes(15))
            .unwrap()
            .to_rfc3339()
    });

    let response = ResponseTemplate::new(402)
        .insert_header(
            "WWW-Authenticate",
            format!("X402 invoice=\"{}\"", invoice.to_string())
        )
        .insert_header("X-402-Invoice", invoice.to_string())
        .insert_header("Access-Control-Allow-Origin", "*")
        .set_body_json(json!({
            "error": "Payment required",
            "amount_lamports": amount_lamports,
            "message": format!("Pay {} lamports to access this endpoint", amount_lamports)
        }));

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(response)
        .mount(&server)
        .await;

    server
}

/// Creates a mock server that responds with 200 OK (successful payment)
///
/// Simulates a successful response after payment verification.
///
/// # Example
///
/// ```no_run
/// #[tokio::test]
/// async fn test_successful_payment() {
///     let server = mock_200_server().await;
///     let response = reqwest::get(&server.uri()).await.unwrap();
///
///     assert_eq!(response.status(), 200);
/// }
/// ```
pub async fn mock_200_server() -> MockServer {
    mock_200_server_with_data("Protected content accessed successfully!").await
}

/// Creates a mock 200 server with custom response data
///
/// # Arguments
///
/// * `data` - The data to return in the response body
///
/// # Example
///
/// ```no_run
/// let server = mock_200_server_with_data("Custom data").await;
/// ```
pub async fn mock_200_server_with_data(data: &str) -> MockServer {
    let server = MockServer::start().await;

    let response = ResponseTemplate::new(200)
        .insert_header("Content-Type", "application/json")
        .set_body_json(json!({
            "data": data,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }));

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(response)
        .mount(&server)
        .await;

    server
}

/// Creates a mock server with custom invoice data
///
/// # Arguments
///
/// * `invoice` - Custom invoice JSON object
///
/// # Example
///
/// ```no_run
/// let custom_invoice = json!({
///     "amount": 2000,
///     "currency": "SOL",
///     "address": "custom-address",
///     "expires_at": "2024-12-31T23:59:59Z"
/// });
/// let server = mock_server_with_invoice(custom_invoice).await;
/// ```
pub async fn mock_server_with_invoice(invoice: serde_json::Value) -> MockServer {
    let server = MockServer::start().await;

    let response = ResponseTemplate::new(402)
        .insert_header(
            "WWW-Authenticate",
            format!("X402 invoice=\"{}\"", invoice.to_string())
        )
        .insert_header("X-402-Invoice", invoice.to_string())
        .set_body_json(json!({
            "error": "Payment required",
            "amount_lamports": invoice["amount"],
            "message": "Custom invoice test"
        }));

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(response)
        .mount(&server)
        .await;

    server
}

/// Creates a mock server that simulates network timeout
///
/// Useful for testing error handling and timeout scenarios.
///
/// # Example
///
/// ```no_run
/// #[tokio::test]
/// async fn test_timeout_handling() {
///     let server = mock_timeout_server().await;
///
///     let client = reqwest::Client::builder()
///         .timeout(std::time::Duration::from_millis(100))
///         .build()
///         .unwrap();
///
///     let result = client.get(&server.uri()).await;
///     assert!(result.is_err()); // Should timeout
/// }
/// ```
pub async fn mock_timeout_server() -> MockServer {
    let server = MockServer::start().await;

    let response = ResponseTemplate::new(200)
        .set_delay(std::time::Duration::from_secs(30));

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(response)
        .mount(&server)
        .await;

    server
}

/// Creates a mock server that returns network errors
///
/// Simulates server errors for testing error handling.
///
/// # Example
///
/// ```no_run
/// let server = mock_error_server(500).await;
/// ```
pub async fn mock_error_server(status_code: u16) -> MockServer {
    let server = MockServer::start().await;

    let response = ResponseTemplate::new(status_code)
        .set_body_json(json!({
            "error": "Internal server error",
            "message": "Something went wrong"
        }));

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(response)
        .mount(&server)
        .await;

    server
}

/// Creates a mock server for testing the /health endpoint
///
/// # Example
///
/// ```no_run
/// let server = mock_health_endpoint().await;
/// ```
pub async fn mock_health_endpoint() -> MockServer {
    let server = MockServer::start().await;

    let response = ResponseTemplate::new(200)
        .set_body_json(json!({
            "status": "healthy",
            "service": "MCP Server with x402 Payments",
            "version": "1.0.0"
        }));

    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(response)
        .mount(&server)
        .await;

    server
}

/// Creates a mock server that requires payment proof header
///
/// Returns 402 if X-402-Payment-Proof header is missing,
/// returns 200 if header is present.
///
/// # Example
///
/// ```no_run
/// let server = mock_payment_verification_server().await;
/// ```
pub async fn mock_payment_verification_server() -> MockServer {
    let server = MockServer::start().await;

    // Without payment proof - 402
    let response_402 = ResponseTemplate::new(402)
        .set_body_json(json!({
            "error": "Payment required",
            "amount_lamports": 1000
        }));

    Mock::given(method("GET"))
        .and(path("/data"))
        .respond_with(response_402)
        .mount(&server)
        .await;

    server
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_402_server_returns_correct_status() {
        let server = mock_402_server().await;
        let response = reqwest::get(&server.uri()).await.unwrap();
        assert_eq!(response.status(), 402);
    }

    #[tokio::test]
    async fn test_mock_200_server_returns_success() {
        let server = mock_200_server().await;
        let response = reqwest::get(&server.uri()).await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_mock_server_ext_endpoint_uri() {
        let server = MockServer::start().await;
        let uri = server.endpoint_uri("/test");
        assert!(uri.ends_with("/test"));
    }
}
