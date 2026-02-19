use crate::{
    AppState, Error,
    auth::AuthClaim,
    entity::{prelude::Trackers, trackers},
    http::params::QueryParams,
    skippy,
};
use axum::{
    Extension, Json, Router,
    extract::{Query, State},
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, EntityTrait, FromQueryResult,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Select,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::result::Result;

#[derive(Serialize, FromQueryResult)]
struct Dto {
    id: u64,
    name: String,
    desc: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
struct TrackerParams {
    #[validate(length(min = 1))]
    name: String,
    desc: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/trackers", get(index))
        .route("/trackers", post(store))
        .route("/trackers/count", get(count))
}

fn query(params: &QueryParams) -> Select<Trackers> {
    let q = params.q.clone().unwrap_or_default();
    let query = Trackers::find();

    if q.is_empty() {
        return query;
    }

    query.filter(
        Condition::any()
            .add(trackers::Column::Id.eq(&q))
            .add(trackers::Column::Name.contains(&q)),
    )
}

async fn index(
    State(state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> Result<Json<Vec<Dto>>> {
    let (skip, take) = skippy::skip(params.skip, params.take);
    let col = skippy::column(params.sort.clone(), trackers::Column::UpdatedAt);
    let ord = skippy::order(params.desc, true);

    let trackers = query(&params)
        .offset(skip)
        .limit(take)
        .order_by(col, ord)
        .into_model::<Dto>()
        .all(&state.db)
        .await?;

    Ok(Json(trackers))
}

async fn count(
    State(state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> Result<Json<u64>> {
    let count = query(&params).count(&state.db).await?;

    Ok(Json(count))
}

async fn store(
    Extension(auth): Extension<AuthClaim>,
    State(state): State<AppState>,
    Json(params): Json<TrackerParams>,
) -> Result<Json<u64>> {
    if let Err(err) = params.validate() {
        return Err(Error::BadRequest(err.to_string()));
    }

    let tracker = trackers::ActiveModel {
        user_id: Set(auth.user_id),
        name: Set(params.name),
        desc: Set(params.desc),

        ..Default::default()
    }
    .insert(&state.db)
    .await?;

    Ok(Json(tracker.id))
}
