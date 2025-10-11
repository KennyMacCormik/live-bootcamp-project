use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Handler for POST /logout.
/// Placeholder for now.
/// Returns 200 OK.
pub async fn logout_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}