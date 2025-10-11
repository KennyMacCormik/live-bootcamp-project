use crate::api::helpers::{get_random_email, TestApp};

impl TestApp {
    pub async fn test_post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

#[tokio::test]
async fn signup_returns_200() {
    let app = TestApp::new().await;

    let ok_case = serde_json::json!({
            "email": get_random_email(),
            "password": "password123",
            "requires2FA": true
        }
    );

    let response = app.test_post_signup(
        &ok_case
    ).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn signup_returns_422() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),

        serde_json::json!({
            "email": get_random_email(),
            "requires2FA": true
        }),

        serde_json::json!({
            "email": get_random_email(),
            "password": "password123",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.test_post_signup(
            &test_case
        ).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}