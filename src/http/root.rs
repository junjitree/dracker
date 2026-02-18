use axum::{Router, http::StatusCode, routing::get};

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(index))
}

async fn index() -> StatusCode {
    StatusCode::IM_A_TEAPOT
}
