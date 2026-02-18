use crate::{
    AppState,
    entity::{prelude::Trackers, trackers},
    http::params::QueryParams,
    skippy,
};
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
};
use chrono::{DateTime, Utc};
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, FromQueryResult, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, Select,
};
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
    Router::new()
        .route("/trackers", get(index))
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
