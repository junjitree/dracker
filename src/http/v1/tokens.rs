use axum::{
    Extension, Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get},
};
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, Select, prelude::DateTimeUtc,
};
use serde::Serialize;

use crate::{
    AppState, Error, Response, Result, auth::AuthClaim, entity::prelude::UserTokens,
    entity::user_tokens, http::params::QueryParams, skippy,
};

#[derive(Debug, Serialize)]
pub struct Dto {
    pub id: u64,
    pub user_id: u64,
    pub agent: String,
    pub current: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

fn dto(model: user_tokens::Model, auth: &AuthClaim) -> Dto {
    Dto {
        id: model.id,
        user_id: model.user_id,
        agent: model.agent,
        current: model.token == auth.uuid.as_bytes(),
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}

fn query(id: u64, params: &QueryParams) -> Select<UserTokens> {
    let q = params.q.clone().unwrap_or_default();
    let col = skippy::column(params.sort.clone(), user_tokens::Column::Id);
    let ord = skippy::order(params.desc, true);

    let query = UserTokens::find()
        .filter(user_tokens::Column::UserId.eq(id))
        .order_by(col, ord);

    if q.is_empty() {
        return query;
    }

    query.filter(
        Condition::any()
            .add(user_tokens::Column::Id.eq(&q))
            .add(user_tokens::Column::Agent.contains(&q)),
    )
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/tokens", get(index))
        .route("/tokens", delete(destroy_all))
        .route("/tokens/count", get(count))
        .route("/tokens/{id}", delete(destroy))
}

async fn index(
    Extension(auth): Extension<AuthClaim>,
    Query(params): Query<QueryParams>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Dto>>> {
    let query = query(auth.user_id, &params);
    let (skip, take) = skippy::skip(params.skip, params.take);

    let tokens = query
        .offset(skip)
        .limit(take)
        .all(&state.db)
        .await?
        .into_iter()
        .map(|model| dto(model, &auth))
        .collect();

    Ok(Json(tokens))
}

async fn count(
    Extension(auth): Extension<AuthClaim>,
    Query(params): Query<QueryParams>,
    State(state): State<AppState>,
) -> Result<Json<u64>> {
    let count = query(auth.user_id, &params).count(&state.db).await?;

    Ok(Json(count))
}

async fn destroy(
    Extension(auth): Extension<AuthClaim>,
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> Result<Response> {
    UserTokens::find_by_id(id)
        .filter(user_tokens::Column::UserId.eq(auth.user_id))
        .one(&state.db)
        .await?
        .ok_or(Error::NotFound)?
        .delete(&state.db)
        .await?;

    Ok(Response::NoContent)
}

async fn destroy_all(
    Extension(auth): Extension<AuthClaim>,
    State(state): State<AppState>,
) -> Result<Response> {
    UserTokens::delete_many()
        .filter(user_tokens::Column::UserId.eq(auth.user_id))
        .filter(user_tokens::Column::Token.ne(auth.uuid))
        .exec(&state.db)
        .await?;

    Ok(Response::NoContent)
}
