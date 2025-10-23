use crate::api::helpers::TestApp;
use reqwest::Url;

const JWT_COOKIE_NAME: &str = "jwt";

impl TestApp {
    pub async fn test_post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

#[tokio::test]
async fn post_logout_returns_200() {
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

    // add cookie from login request to successfully logout
    app.cookie_jar.add_cookie_str(
        auth_cookie.value(),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.test_post_logout().await;

    assert_eq!(
        response.status(),
        reqwest::StatusCode::OK,
        "correct cookie shall return 200",
    );
}

#[tokio::test]
async fn double_logout_400() {
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

    // add cookie from login request to successfully logout
    app.cookie_jar.add_cookie_str(
        auth_cookie.value(),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.test_post_logout().await;

    assert_eq!(
        response.status(),
        reqwest::StatusCode::OK,
        "correct cookie shall return 200",
    );

    // second logout should fail
    let response = app.test_post_logout().await;

    assert_eq!(
        response.status(),
        reqwest::StatusCode::BAD_REQUEST,
        "second logout should fail",
    );
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.test_post_logout().await;

    assert_eq!(
        response.status(),
        reqwest::StatusCode::UNAUTHORIZED,
        "bad cookie shall get 401",
    );
}

#[tokio::test]
async fn should_return_400_if_no_cookie() {
    let app = TestApp::new().await;

    let response = app.test_post_logout().await;

    assert_eq!(
        response.status(),
        reqwest::StatusCode::BAD_REQUEST,
        "no cookie shall get 400",
    );
}