use crate::api::helpers::TestApp;
const JWT_COOKIE_NAME: &str = "jwt";

impl TestApp {
    pub async fn test_post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

#[tokio::test]
async fn bad_body_422() {
    let app = TestApp::new().await;

    let bad_body = serde_json::json!({
        "bad_json": "bad_json",
    });

    let response = app.test_post_verify_token(&bad_body).await;

    assert_eq!(
        response.status(),
        reqwest::StatusCode::UNPROCESSABLE_ENTITY,
        "bad json shall fail",
    );
}

#[tokio::test]
async fn bad_token_401() {
    let app = TestApp::new().await;

    let bad_body = serde_json::json!({
        "token": "bad_token",
    });

    let response = app.test_post_verify_token(&bad_body).await;

    assert_eq!(
        response.status(),
        reqwest::StatusCode::UNAUTHORIZED,
        "bad json shall fail",
    );
}

#[tokio::test]
async fn good_token_200() {
    // creating user
    let app = TestApp::new().await;

    let ok_case = serde_json::json!({
            "email": "a@b.com",
            "password": "password!23",
            "requires2FA": true
        }
    );

    let response = app.test_post_signup(&ok_case).await;

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    // logging in
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

    let test = auth_cookie.value();

    //query token
    let bad_body = serde_json::json!({
        "token": auth_cookie.value().to_string(),
    });

    let response = app.test_post_verify_token(&bad_body).await;

    assert_eq!(
        response.status(),
        reqwest::StatusCode::OK,
        "bad json shall fail",
    );
}