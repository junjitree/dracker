use crate::{AppState, entity::prelude::Trackers};
use axum::{Json, Router, extract::State, routing::get};
use chrono::{DateTime, Utc};
use sea_orm::{EntityTrait, FromQueryResult};
use serde::Serialize;

use crate::result::Result;

#[derive(Serialize, FromQueryResult)]
struct Dto {
    id: u64,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/trackers", get(index))
}

async fn index(State(state): State<AppState>) -> Result<Json<Vec<Dto>>> {
    let trackers = Trackers::find().into_model::<Dto>().all(&state.db).await?;
    Ok(Json(trackers))
}
