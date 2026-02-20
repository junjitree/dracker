use axum::{Json, Router, extract::State, routing::post};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use serde::Deserialize;

use crate::{Error, Result, entity::pings, state::AppState, util};

#[derive(Debug, Deserialize)]
struct PingParams {
    slug: String,
    lat: f64,
    lon: f64,
    note: String,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/ping", post(store))
}

async fn store(State(state): State<AppState>, Json(params): Json<PingParams>) -> Result<Json<u64>> {
    let sqids = util::sqids()?;
    let tracker_id = sqids.decode(&params.slug);
    if tracker_id.is_empty() {
        return Err(Error::BadRequest("Invalid tracker_id".into()));
    }

    let tracker_id = tracker_id[0];
    let ping = pings::ActiveModel {
        tracker_id: Set(tracker_id),
        lat: Set(params.lat),
        lon: Set(params.lon),
        note: Set(params.note),

        ..Default::default()
    }
    .insert(&state.db)
    .await?;

    Ok(Json(ping.id))
}
