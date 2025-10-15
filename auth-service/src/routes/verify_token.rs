use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Handler for POST /verify-token.
/// Placeholder for now.
/// Returns 200 OK.
pub async fn verify_token_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}