use crate::domain::{Email, Password, data_stores::UserStoreError, errors::PasswordError};
use axum::{extract::State, http::StatusCode, Json, response::IntoResponse};
use crate::{AppState, ErrorResponse};
use serde::{Deserialize, Serialize};

const BAD_EMAIL: &str = "malformed email";
const BAD_PASSWORD: &str = "password must be at least 10 characters";
const WEAK_PASSWORD: &str = "password must contain at lest one num and at least one special character";

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn login_handler(
    State(app_state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    let email = match get_email(&request.email){
        Ok(e) => e,
        Err(resp) => return resp,
    };

    let password = match get_password(&request.password){
        Ok(e) => e,
        Err(resp) => return resp,
    };

    let user_store = app_state.user_store.read().await;

    match user_store.validate_user(&email, &password).await {
        Ok(_) =>StatusCode::OK.into_response(),
        Err(err) => match err{
            UserStoreError::InvalidCredentials => StatusCode::UNAUTHORIZED.into_response(),
            UserStoreError::UserNotFound => StatusCode::NOT_FOUND.into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    }
}

fn get_email(email: &str) -> Result<Email, axum::response::Response>{
    match Email::parse(email) {
        Ok(e) => Ok(e),
        Err(_) => Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { error: BAD_EMAIL.to_string() }),
            ).into_response(),
        ),
    }
}

fn get_password(password: &str) -> Result<Password, axum::response::Response>{
    match Password::parse(password) {
        Ok(e) => Ok(e),
        Err(err) => match err {
            PasswordError::BadLength => Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse { error: BAD_PASSWORD.to_string() }),
                ).into_response()
            ),
            PasswordError::WeakPassword => Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse { error: WEAK_PASSWORD.to_string() }),
                ).into_response()
            ),
        },
    }
}
