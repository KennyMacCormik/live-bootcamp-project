use auth_service::{ErrorResponse, routes::signup::SignupResponse};
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
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let ok_case = serde_json::json!({
            "email": get_random_email(),
            "password": "password!23",
            "requires2FA": true
        }
    );

    let response = app.test_post_signup(
        &ok_case
    ).await;

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    // Assert that we are getting the correct response body!
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let invalid_inputs = [
        // empty email
        (serde_json::json!({
            "email": "",
            "password": "password123",
            "requires2FA": true
        }), "InvalidEmail"),
        // no @ in email
        (serde_json::json!({
            "email": "invalid.example.com",
            "password": "password123",
            "requires2FA": true
        }), "InvalidEmail"),
        // short password
        (serde_json::json!({
            "email": get_random_email(),
            "password": "short",
            "requires2FA": true
        }), "BadLength"),
        // no number in password
        (serde_json::json!({
            "email": get_random_email(),
            "password": "password!",
            "requires2FA": true
        }), "BadLength"),
        // no symbol in password
        (serde_json::json!({
            "email": get_random_email(),
            "password": "password",
            "requires2FA": true
        }), "BadLength"),
    ];

    for i in invalid_inputs.iter() {
        let response = app.test_post_signup(&i.0).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", i.0);

        let body = response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse");

        assert_eq!(body.error, i.1.to_owned());
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let first = serde_json::json!({
        "email": email,
        "password": "password!23",
        "requires2FA": true
    });

    // First signup should succeed
    let resp1 = app.test_post_signup(&first).await;
    let status = resp1.status();
    assert_eq!(status, reqwest::StatusCode::CREATED, "first call shall succeed");

    // Second signup with same email should conflict
    let resp2 = app.test_post_signup(&first).await;
    assert_eq!(resp2.status(), reqwest::StatusCode::CONFLICT);

    let body = resp2
        .json::<ErrorResponse>()
        .await
        .expect("Could not deserialize response body to ErrorResponse");

    assert_eq!(body.error, "User already exists".to_owned());
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
