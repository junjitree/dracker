use crate::{AppState, entity::prelude::Tracker};
use axum::{Json, Router, extract::State, routing::get};
use sea_orm::{EntityTrait, FromQueryResult};
use serde::Serialize;

use crate::result::Result;

#[derive(Serialize, FromQueryResult)]
struct Dto {
    id: u64,
    name: String,
    created_at: String,
    updated_at: String,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/trackers", get(index))
}

async fn index(State(state): State<AppState>) -> Result<Json<Vec<Dto>>> {
    let trackers = Tracker::find().into_model::<Dto>().all(&state.db).await?;
    Ok(Json(trackers))
}
