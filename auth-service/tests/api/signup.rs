use crate::helpers::TestApp;
use serde_json::json;

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