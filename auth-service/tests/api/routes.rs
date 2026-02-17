use crate::helpers::TestApp;
use serde_json::json;

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn signup_returns_http_200_status() {
    let app = TestApp::new().await;

    let response = app.post_signup(json!({
        "email": "user@example.com",
        "password": "string",
        "requires2FA": true
    })).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn login_returns_http_200_status() {
    let app = TestApp::new().await;

    let response = app.post_login(json!({
        "email": "user@example.com",
        "password": "string"
    })).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logout_returns_http_200_status() {
    let app = TestApp::new().await;

    let response = app.post_logout("string".to_string()).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_2fa_returns_http_200_status() {
    let app = TestApp::new().await;

    let response = app.post_verify_2fa(json!({
        "email": "user@example.com",
        "loginAttemptId": "string",
        "2FACode": "string"
    })).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_token_returns_http_200_status() {
    let app = TestApp::new().await;

    let response = app.post_verify_token(json!({
        "token": "string"
    })).await;

    assert_eq!(response.status().as_u16(), 200);
}

// TODO: Implement tests for all other routes (signup, login, logout, verify-2fa, and verify-token)
// For now, simply assert that each route returns a 200 HTTP status code.