use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use crate::AppState;
use crate::auth::auth::validate_token;

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

pub async fn verify_token_handler(State(app_state): State<AppState>, Json(body): Json<VerifyTokenRequest>) -> impl IntoResponse {
    if app_state.banned_token_store.is_banned(&body.token).await{
        return StatusCode::UNAUTHORIZED.into_response();
    }

    match validate_token(&body.token).await{
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}