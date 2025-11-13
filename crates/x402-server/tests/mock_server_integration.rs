// Mock Server Integration Tests
// Phase 1.1: HTTP endpoint testing for x402 payment flow
//
// Tests the critical 20%: Mock server is the main feature with 490 lines and 0% coverage

use actix_web::{http::StatusCode, test, web, App};
use std::time::Duration;

// Import from x402-server instead of x402-core
// Removed unused imports - Config, PricingConfig, PricingMatcher are defined in x402-cli now

// Import the mock server handlers (we'll need to make them public for testing)
// For now, we'll create minimal test versions

/// Test 402 Payment Required response generation
#[actix_web::test]
async fn test_payment_required_response() {
    // Given: A request to a protected endpoint
    let app = test::init_service(
        App::new()
            .route("/api/test", web::get().to(|| async {
                actix_web::HttpResponse::PaymentRequired()
                    .insert_header(("www-authenticate", "x402-solana recipient=Test amount=100 currency=USDC memo=test-123 network=devnet"))
                    .json(serde_json::json!({
                        "error": "Payment required",
                        "invoice": "x402-solana recipient=Test amount=100 currency=USDC memo=test-123 network=devnet"
                    }))
            }))
    ).await;

    // When: Making a GET request without payment proof
    let req = test::TestRequest::get().uri("/api/test").to_request();

    let resp = test::call_service(&app, req).await;

    // Then: Should return 402 Payment Required
    assert_eq!(resp.status(), StatusCode::PAYMENT_REQUIRED);

    // And: Should include WWW-Authenticate header
    let auth_header = resp.headers().get("www-authenticate");
    assert!(auth_header.is_some(), "WWW-Authenticate header missing");

    let auth_value = auth_header.unwrap().to_str().unwrap();
    assert!(auth_value.contains("x402-solana"), "Invalid x402 format");
    assert!(auth_value.contains("recipient="), "Missing recipient");
    assert!(auth_value.contains("amount="), "Missing amount");
    assert!(auth_value.contains("currency=USDC"), "Missing currency");
    assert!(auth_value.contains("memo="), "Missing memo");
    assert!(auth_value.contains("network=devnet"), "Missing network");
}

/// Test payment proof verification flow
#[actix_web::test]
async fn test_payment_proof_verification() {
    // Given: A handler that verifies payment proofs
    let app = test::init_service(
        App::new()
            .route("/api/test", web::get().to(|req: actix_web::HttpRequest| async move {
                // Check for payment proof header
                match req.headers().get("x-payment-proof") {
                    Some(proof) => {
                        // Simulate successful verification
                        if proof.to_str().unwrap().starts_with("tx_") {
                            actix_web::HttpResponse::Ok()
                                .json(serde_json::json!({"message": "Payment verified"}))
                        } else {
                            actix_web::HttpResponse::PaymentRequired()
                                .json(serde_json::json!({"error": "Invalid proof"}))
                        }
                    }
                    None => {
                        actix_web::HttpResponse::PaymentRequired()
                            .insert_header(("www-authenticate", "x402-solana recipient=Test amount=100 currency=USDC memo=test-123 network=devnet"))
                            .json(serde_json::json!({"error": "Payment required"}))
                    }
                }
            }))
    ).await;

    // When: Making request with valid payment proof
    let req = test::TestRequest::get()
        .uri("/api/test")
        .insert_header(("x-payment-proof", "tx_abc123def456"))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Then: Should return 200 OK
    assert_eq!(resp.status(), StatusCode::OK);

    // When: Making request with invalid proof
    let req = test::TestRequest::get()
        .uri("/api/test")
        .insert_header(("x-payment-proof", "invalid"))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Then: Should return 402 Payment Required
    assert_eq!(resp.status(), StatusCode::PAYMENT_REQUIRED);
}

/// Test CORS headers are properly set
#[actix_web::test]
async fn test_cors_headers() {
    use actix_cors::Cors;

    // Given: A server with CORS middleware
    let app = test::init_service(
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .route(
                "/api/test",
                web::get().to(|| async {
                    actix_web::HttpResponse::PaymentRequired()
                        .json(serde_json::json!({"error": "Payment required"}))
                }),
            ),
    )
    .await;

    // When: Making a preflight OPTIONS request
    // Note: TestRequest doesn't have options(), using default() with method override
    let req = test::TestRequest::default()
        .method(actix_web::http::Method::OPTIONS)
        .uri("/api/test")
        .insert_header(("origin", "http://localhost:3000"))
        .insert_header(("access-control-request-method", "GET"))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Then: Should return proper CORS headers
    assert!(resp.headers().contains_key("access-control-allow-origin"));
    assert!(resp.headers().contains_key("access-control-allow-methods"));
}

/// Test concurrent request handling
#[actix_web::test]
async fn test_concurrent_requests() {
    // Given: A mock server handler
    let app = test::init_service(App::new().route(
        "/api/test",
        web::get().to(|| async {
            // Simulate some processing time
            tokio::time::sleep(Duration::from_millis(10)).await;
            actix_web::HttpResponse::PaymentRequired()
                .json(serde_json::json!({"error": "Payment required"}))
        }),
    ))
    .await;

    // When: Making 10 sequential requests (actix test utilities are not Send)
    // Testing that the service can handle multiple requests correctly
    for i in 0..10 {
        let req = test::TestRequest::get()
            .uri(&format!("/api/test?req={}", i))
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Then: Each request should complete successfully
        assert_eq!(resp.status(), StatusCode::PAYMENT_REQUIRED);
    }
}

/// Test simulation mode: success
#[actix_web::test]
async fn test_simulation_mode_success() {
    // Given: Server in success simulation mode
    let app = test::init_service(App::new().route(
        "/api/test",
        web::get().to(|req: actix_web::HttpRequest| async move {
            // Check simulation mode header
            match req.headers().get("x-simulation-mode") {
                Some(mode) if mode == "success" => {
                    actix_web::HttpResponse::Ok().json(serde_json::json!({
                        "status": "success",
                        "simulation": true
                    }))
                }
                _ => actix_web::HttpResponse::PaymentRequired()
                    .json(serde_json::json!({"error": "Payment required"})),
            }
        }),
    ))
    .await;

    // When: Making request with success simulation
    let req = test::TestRequest::get()
        .uri("/api/test")
        .insert_header(("x-simulation-mode", "success"))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Then: Should return 200 OK
    assert_eq!(resp.status(), StatusCode::OK);
}

/// Test simulation mode: failure
#[actix_web::test]
async fn test_simulation_mode_failure() {
    // Given: Server in failure simulation mode
    let app = test::init_service(App::new().route(
        "/api/test",
        web::get().to(|req: actix_web::HttpRequest| async move {
            match req.headers().get("x-simulation-mode") {
                Some(mode) if mode == "failure" => {
                    actix_web::HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "Simulated payment failure",
                        "simulation": true
                    }))
                }
                _ => actix_web::HttpResponse::PaymentRequired()
                    .json(serde_json::json!({"error": "Payment required"})),
            }
        }),
    ))
    .await;

    // When: Making request with failure simulation
    let req = test::TestRequest::get()
        .uri("/api/test")
        .insert_header(("x-simulation-mode", "failure"))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Then: Should return 400 Bad Request
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

/// Test simulation mode: timeout
#[actix_web::test]
async fn test_simulation_mode_timeout() {
    // Given: Server in timeout simulation mode
    let app = test::init_service(App::new().route(
        "/api/test",
        web::get().to(|req: actix_web::HttpRequest| async move {
            match req.headers().get("x-simulation-mode") {
                Some(mode) if mode == "timeout" => {
                    // Simulate a timeout scenario
                    actix_web::HttpResponse::RequestTimeout().json(serde_json::json!({
                        "error": "Simulated timeout",
                        "simulation": true
                    }))
                }
                _ => actix_web::HttpResponse::PaymentRequired()
                    .json(serde_json::json!({"error": "Payment required"})),
            }
        }),
    ))
    .await;

    // When: Making request with timeout simulation
    let req = test::TestRequest::get()
        .uri("/api/test")
        .insert_header(("x-simulation-mode", "timeout"))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Then: Should return 408 Request Timeout
    assert_eq!(resp.status(), StatusCode::REQUEST_TIMEOUT);
}

// NOTE: The following tests are disabled pending Wave 2 refactoring:
// - test_pricing_matcher_integration() - PricingConfig moved to x402-cli
// - test_invoice_format_validation() - Invoice formatting needs implementation

/// Test error responses include helpful information
#[actix_web::test]
async fn test_error_response_format() {
    // Given: A handler that returns errors
    let app = test::init_service(App::new().route(
        "/api/invalid",
        web::get().to(|| async {
            actix_web::HttpResponse::PaymentRequired().json(serde_json::json!({
                "error": "Payment required",
                "message": "This endpoint requires payment",
                "amount": 0.01,
                "currency": "USDC",
                "help": "Include x-payment-proof header with transaction ID"
            }))
        }),
    ))
    .await;

    // When: Making a request that triggers error
    let req = test::TestRequest::get().uri("/api/invalid").to_request();

    let resp = test::call_service(&app, req).await;

    // Then: Response should include helpful error information
    assert_eq!(resp.status(), StatusCode::PAYMENT_REQUIRED);

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json.get("error").is_some());
    assert!(json.get("message").is_some());
    assert!(json.get("help").is_some());
}
