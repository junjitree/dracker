use axum::{Router, middleware};

use crate::{AppState, http::middleware::auth};

pub mod auth;
pub mod trackers;

pub fn routes(state: &AppState) -> Router<AppState> {
    // INFO: PUBLIC ROUTES
    let publ_router = auth::routes(state);

    // WARN: AUTHENTICATED ROUTES
    let auth_router = trackers::routes().layer(middleware::from_fn_with_state(state.clone(), auth));

    Router::new().nest("/v1", Router::new().merge(publ_router).merge(auth_router))
}
