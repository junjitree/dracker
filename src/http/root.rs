use axum::{Router, http::StatusCode, routing::get};

pub fn routes() -> Router {
    Router::new().route("/", get(index))
}

async fn index() -> StatusCode {
    StatusCode::IM_A_TEAPOT
}
