use axum::Router;

pub mod pets;

pub fn routes() -> Router {
    Router::new().nest("/v1", Router::new().merge(pets::routes()))
}
