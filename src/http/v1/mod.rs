use axum::Router;

use crate::AppState;

pub mod auth;
pub mod trackers;

pub fn routes(state: &AppState) -> Router<AppState> {
    Router::new().nest(
        "/v1",
        Router::new()
            .merge(trackers::routes())
            .merge(auth::routes(state)),
    )
}
