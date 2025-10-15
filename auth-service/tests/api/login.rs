use crate::api::helpers::TestApp;

impl TestApp {
    pub async fn test_post_login(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

#[tokio::test]
async fn post_login_returns_200() {
    let app = TestApp::new().await;

    let response = app.test_post_login().await;

    assert_eq!(response.status().as_u16(), 200);
}