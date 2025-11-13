// Unit tests for mock server tools
//
// Epic 8, Phase 3: Mock server tool testing
// Tests: x402__server_mock_start, x402__server_mock_status, x402__server_mock_stop

use serde_json::json;
use x402_mcp_server::{MockStartParams, MockStartResponse, MockStatusResponse};

#[test]
fn test_mock_start_params_deserialization() {
    let json = json!({
        "port": 3000,
        "pricing": 0.001,
        "simulation_mode": "success"
    });

    let params: MockStartParams = serde_json::from_value(json).unwrap();
    assert_eq!(params.port, 3000);
    assert_eq!(params.pricing, 0.001);
    assert_eq!(params.simulation_mode, "success");
}

#[test]
fn test_mock_start_params_with_failure_mode() {
    let json = json!({
        "port": 8080,
        "pricing": 0.005,
        "simulation_mode": "failure"
    });

    let params: MockStartParams = serde_json::from_value(json).unwrap();
    assert_eq!(params.port, 8080);
    assert_eq!(params.pricing, 0.005);
    assert_eq!(params.simulation_mode, "failure");
}

#[test]
fn test_mock_start_response_serialization() {
    let response = MockStartResponse {
        status: "started".to_string(),
        port: 3000,
        pid: Some(12345),
        message: "Mock server started successfully".to_string(),
    };

    let json = serde_json::to_value(&response).unwrap();
    assert_eq!(json["status"], "started");
    assert_eq!(json["pid"], 12345);
    assert_eq!(json["port"], 3000);
}

#[test]
fn test_mock_status_response_running() {
    let response = MockStatusResponse {
        status: "running".to_string(),
        pid: Some(12345),
        port: Some(3000),
    };

    let json = serde_json::to_value(&response).unwrap();
    assert_eq!(json["status"], "running");
    assert_eq!(json["pid"], 12345);
    assert_eq!(json["port"], 3000);
}

#[test]
fn test_mock_status_response_not_running() {
    let response = MockStatusResponse {
        status: "stopped".to_string(),
        pid: None,
        port: None,
    };

    let json = serde_json::to_value(&response).unwrap();
    assert_eq!(json["status"], "stopped");
    assert!(json["pid"].is_null());
    assert!(json["port"].is_null());
}

#[test]
fn test_mock_start_params_defaults() {
    let json = json!({});

    let params: MockStartParams = serde_json::from_value(json).unwrap();
    assert_eq!(params.port, 3402); // Default port
    assert_eq!(params.pricing, 0.01); // Default pricing
    assert_eq!(params.simulation_mode, "success"); // Default mode
}

#[test]
fn test_mock_start_params_port_range() {
    // Valid port
    let json = json!({
        "port": 8080,
        "pricing": 0.001,
        "simulation_mode": "success"
    });
    let params: MockStartParams = serde_json::from_value(json).unwrap();
    assert_eq!(params.port, 8080);

    // Maximum port
    let json = json!({
        "port": 65535,
        "pricing": 0.001,
        "simulation_mode": "success"
    });
    let params: MockStartParams = serde_json::from_value(json).unwrap();
    assert_eq!(params.port, 65535);
}

#[test]
fn test_mock_start_response_fields() {
    let response = MockStartResponse {
        status: "started".to_string(),
        port: 4000,
        pid: Some(9999),
        message: "Test message".to_string(),
    };

    assert_eq!(response.status, "started");
    assert_eq!(response.pid, Some(9999));
    assert_eq!(response.port, 4000);
    assert_eq!(response.message, "Test message");
}

#[test]
fn test_mock_start_timeout_simulation() {
    let json = json!({
        "port": 3000,
        "pricing": 0.001,
        "simulation_mode": "timeout"
    });

    let params: MockStartParams = serde_json::from_value(json).unwrap();
    assert_eq!(params.simulation_mode, "timeout");
}

#[test]
fn test_mock_status_response_partial_fields() {
    // Test response when some optional fields are missing
    let response = MockStatusResponse {
        status: "running".to_string(),
        pid: None,
        port: Some(8080),
    };

    let json = serde_json::to_value(&response).unwrap();
    assert_eq!(json["status"], "running");
    assert!(json["pid"].is_null());
    assert_eq!(json["port"], 8080);
}
