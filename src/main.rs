use axum::Router;
use axum::http::HeaderName;
use axum::http::Method;
use axum::http::header;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::cors::CorsLayer;
use tracing::info;

mod entity;
mod error;
mod http;
mod result;
mod util;

pub use self::error::*;
pub use self::result::*;

#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

const X_CSRF_TOKEN: &str = "x-csrf-token";
const DEFAULT_PORT: u16 = 3000;

#[tokio::main]
async fn main() -> Result<()> {
    util::init();

    let app_port: u16 = env::var("APP_PORT")
        .map(|s| s.parse::<u16>())
        .unwrap_or(Ok(DEFAULT_PORT))
        .unwrap_or(DEFAULT_PORT);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(db_url).await?;

    let state = AppState { db };

    let cors = CorsLayer::new()
        .allow_origin([
            // INFO: This is for local development
            "http://localhost:9001".parse().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
            HeaderName::from_static(X_CSRF_TOKEN),
        ])
        .expose_headers([HeaderName::from_static(X_CSRF_TOKEN)])
        .max_age(Duration::from_secs(3600))
        .allow_credentials(true);

    let app = Router::new()
        .merge(http::root::routes())
        .merge(http::v1::routes())
        .with_state(state)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], app_port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}
