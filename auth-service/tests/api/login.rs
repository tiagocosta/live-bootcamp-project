use crate::helpers::TestApp;
use serde_json::json;

#[tokio::test]
async fn login_returns_http_200_status() {
    let app = TestApp::new().await;

    let response = app.post_login(&json!({
        "email": "user@example.com",
        "password": "string"
    })).await;

    assert_eq!(response.status().as_u16(), 200);
}