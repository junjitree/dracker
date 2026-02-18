use axum::{Json, Router, routing::get};
use serde::Serialize;

use crate::result::Result;

#[derive(Serialize)]
struct Dto {
    id: u64,
    name: String,
    created_at: String,
    updated_at: String,
}

pub fn routes() -> Router {
    Router::new().route("/trackers", get(index))
}

async fn index() -> Result<Json<Vec<Dto>>> {
    Ok(Json(vec![Dto {
        id: 1,
        name: "Test".into(),
        created_at: "1970-01-01-00:00:00".into(),
        updated_at: "1970-01-01-00:00:00".into(),
    }]))
}
