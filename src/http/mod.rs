use axum::Router;

pub mod root;
pub mod v1;

pub fn routes() -> Router {
    Router::new().merge(root::routes()).merge(v1::routes())
}
