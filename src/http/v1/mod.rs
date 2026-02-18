use axum::Router;

use crate::AppState;

pub mod trackers;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/v1", Router::new().merge(trackers::routes()))
}
