use crate::api::helpers::{TestApp};
const JWT_COOKIE_NAME: &str = "jwt";

impl TestApp {
    pub async fn test_post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let first = serde_json::json!({
        "bad_json": "bad_json",
    });

    let resp1 = app.test_post_login(&first).await;
    let status = resp1.status();
    assert_eq!(status, reqwest::StatusCode::UNPROCESSABLE_ENTITY, "bad json shall fail");

    let resp = app.test_post_login(&serde_json::json!({})).await;
    let status = resp.status();
    assert_eq!(status, reqwest::StatusCode::UNPROCESSABLE_ENTITY, "empty json shall fail");
}

#[tokio::test]
async fn should_return_404_with_missing_user() {
    let app = TestApp::new().await;

    let missing_user = serde_json::json!({
        "email": "a@b.com",
        "password": "password123!",
    });

    let resp1 = app.test_post_login(&missing_user).await;
    let status = resp1.status();
    assert_eq!(status, reqwest::StatusCode::NOT_FOUND, "bad email shall fail");
}

#[tokio::test]
async fn should_return_401_with_existing_user_and_bad_password() {
    let app = TestApp::new().await;

    let ok_case = serde_json::json!({
            "email": "a@b.com",
            "password": "password!23",
            "requires2FA": true
        }
    );

    let response = app.test_post_signup(&ok_case).await;

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    let missing_user = serde_json::json!({
        "email": "a@b.com",
        "password": "password!23!",
    });

    let resp1 = app.test_post_login(&missing_user).await;
    let status = resp1.status();
    assert_eq!(status, reqwest::StatusCode::UNAUTHORIZED, "bad password shall fail");
}

#[tokio::test]
async fn should_return_200_with_valid_request() {
    let app = TestApp::new().await;

    let ok_case = serde_json::json!({
            "email": "a@b.com",
            "password": "password!23",
            "requires2FA": true
        }
    );

    let response = app.test_post_signup(&ok_case).await;

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    let login = serde_json::json!({
        "email": "a@b.com",
        "password": "password!23",
    });

    let resp1 = app.test_post_login(&login).await;
    let status = resp1.status();
    assert_eq!(status, reqwest::StatusCode::OK, "valid request shall succeed");

    let auth_cookie = resp1
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}