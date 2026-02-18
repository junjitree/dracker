use axum::Router;

pub mod trackers;

pub fn routes() -> Router {
    Router::new().nest("/v1", Router::new().merge(trackers::routes()))
}
