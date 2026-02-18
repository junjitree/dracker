mod error;
mod http;
mod result;
mod util;

pub use self::error::*;
pub use self::result::*;

#[tokio::main]
async fn main() {
    util::init();

    let app = http::routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
