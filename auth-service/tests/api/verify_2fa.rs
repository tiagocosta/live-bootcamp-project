use crate::helpers::TestApp;
use serde_json::json;

#[tokio::test]
async fn verify_2fa_returns_http_200_status() {
    let app = TestApp::new().await;

    let response = app.post_verify_2fa(&json!({
        "email": "user@example.com",
        "loginAttemptId": "string",
        "2FACode": "string"
    })).await;

    assert_eq!(response.status().as_u16(), 200);
}