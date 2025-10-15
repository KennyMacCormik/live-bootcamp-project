use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Handler for POST /verify-2fa.
/// Placeholder for now.
/// Returns 200 OK.
pub async fn verity_2fa_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}