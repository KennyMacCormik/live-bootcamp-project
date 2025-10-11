use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Handler for POST /login.
/// Placeholder for now.
/// Returns 200 OK.
pub async fn login_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}