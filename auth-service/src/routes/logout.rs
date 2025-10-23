use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use crate::auth::auth::validate_token;
use crate::domain::constants::JWT_COOKIE_NAME;
use crate::{AppState, ErrorResponse};

const MISSING_COOKIE: &str = "missing cookie";
const BAD_COOKIE: &str = "bad cookie";

pub async fn logout_handler(jar: CookieJar, State(app_state): State<AppState>,) -> (CookieJar, impl IntoResponse) {
    let cookie = match jar.get(JWT_COOKIE_NAME){
        Some(c) => c,
        None => return return_error(jar, StatusCode::BAD_REQUEST, MISSING_COOKIE),
    };

    let token = cookie.value();
    let _ = match validate_token(token).await{
        Ok(c) => c,
        Err(_) => return return_error(jar, StatusCode::UNAUTHORIZED, BAD_COOKIE),
    };

    app_state.banned_token_store.ban(token).await;
    let updated_jar = jar.remove(JWT_COOKIE_NAME);

    (updated_jar, StatusCode::OK.into_response())
}

fn return_error(jar: CookieJar, code: StatusCode,error: &str) -> (CookieJar, axum_core::response::Response) {
    (
        jar,
        (
            code,
            Json(ErrorResponse { error: error.to_string() }),
        ).into_response()
    )
}