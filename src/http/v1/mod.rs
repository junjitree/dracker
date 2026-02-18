use axum::{Router, middleware};

use crate::{AppState, http::middleware::auth};

pub mod auth;
pub mod password;
pub mod trackers;

pub fn routes(state: &AppState) -> Router<AppState> {
    // INFO: PUBLIC ROUTES
    let publ_router = Router::new()
        .merge(auth::routes(state))
        .merge(password::routes());

    // WARN: AUTHENTICATED ROUTES
    let auth_router = Router::new()
        .nest("/v1", Router::new().merge(trackers::routes()))
        .layer(middleware::from_fn_with_state(state.clone(), auth));

    Router::new().merge(publ_router).merge(auth_router)
}
