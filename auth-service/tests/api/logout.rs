use crate::helpers::TestApp;

#[tokio::test]
async fn logout_returns_http_200_status() {
    let app = TestApp::new().await;

    let response = app.post_logout("string".to_string()).await;

    assert_eq!(response.status().as_u16(), 200);
}