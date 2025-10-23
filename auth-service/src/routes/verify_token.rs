use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use crate::auth::auth::validate_token;

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

pub async fn verify_token_handler(Json(body): Json<VerifyTokenRequest>) -> impl IntoResponse {
    match validate_token(&body.token).await{
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}