use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{AppState, domain::User, ErrorResponse, domain::data_stores::UserStoreError};
use crate::domain::errors::UserError::{EmailParsingError, PasswordParsingError};

pub async fn signup_handler(
    State(app_state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {

    let user = match User::new(request.email, request.password, request.requires_2fa) {
        Ok(u) => u,
        Err(e) => {
            return match e {
                EmailParsingError(e) => (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse { error: format!("{e:?}") }),
                ).into_response(),
                PasswordParsingError(e) => (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse { error: format!("{e:?}") }),
                ).into_response(),
            };
        }
    };

    let mut user_store = app_state.user_store.write().await;

    match user_store.add_user(user).await{
        Ok(_) => (),
        Err(UserStoreError::UserAlreadyExists) => {
            return (StatusCode::CONFLICT, Json(ErrorResponse { error: "User already exists".into() })).into_response();
        }
        Err(e) => {
            // handle other errors if needed
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("{e:?}") })).into_response();
        }
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    (StatusCode::CREATED, response).into_response()
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}