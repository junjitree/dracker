use axum::{
    Extension, Json, Router,
    extract::State,
    http::{HeaderMap, HeaderName, HeaderValue, header},
    middleware,
    routing::{delete, get, post},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use base64::{Engine, engine::general_purpose};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, Header, encode};
use rand::RngCore;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

pub const SESSION_DAYS: i64 = 365;
pub const X_CSRF_TOKEN: &str = "x-csrf-token";

use crate::{
    AppState, Error, Response, Result,
    auth::{self, AuthClaim},
    entity::{prelude::*, user_tokens, users},
    http::middleware::auth,
};

#[derive(Debug, Deserialize, Validate)]
struct UserParams {
    #[validate(email)]
    email: String,
    #[validate(length(min = 1))]
    password: String,
}

pub fn routes(state: &AppState) -> Router<AppState> {
    // INFO: PUBLIC ROUTES
    let publ_router = Router::new()
        .route("/csrf", get(csrf))
        .route("/login", post(token))
        .route("/login/cookie", post(cookie));

    // WARN: AUTHENTICATED ROUTES
    let auth_router = Router::new()
        .route("/logout", delete(logout))
        .layer(middleware::from_fn_with_state(state.clone(), auth));

    Router::new().merge(publ_router).merge(auth_router)
}

async fn csrf(mut headers: HeaderMap, jar: CookieJar) -> Result<Response> {
    let token = match jar.get(X_CSRF_TOKEN) {
        Some(cookie) => cookie.value().to_string(),
        None => {
            let mut token_bytes = [0u8; 32];
            rand::rng().fill_bytes(&mut token_bytes);
            general_purpose::URL_SAFE_NO_PAD.encode(token_bytes)
        }
    };

    let jar = jar.add(
        Cookie::build((X_CSRF_TOKEN, token.clone()))
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(SameSite::None)
            .max_age(cookie::time::Duration::days(SESSION_DAYS)),
    );

    headers.insert(
        HeaderName::from_static(X_CSRF_TOKEN),
        HeaderValue::from_str(&token).unwrap(),
    );

    Ok(Response::Csrf(jar, headers))
}

async fn token(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(params): Json<UserParams>,
) -> Result<Response> {
    let token = login(params, headers, &state).await?;

    Ok(Response::AuthToken(token))
}

async fn cookie(
    headers: HeaderMap,
    State(state): State<AppState>,
    jar: CookieJar,
    Json(params): Json<UserParams>,
) -> Result<Response> {
    let token = login(params, headers, &state).await?;

    let jar = jar.add(
        Cookie::build((header::AUTHORIZATION.as_str(), token))
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(SameSite::None)
            .max_age(cookie::time::Duration::days(SESSION_DAYS)),
    );

    Ok(Response::AuthCookie(jar))
}

async fn logout(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthClaim>,
) -> Result<Response> {
    UserTokens::find()
        .filter(user_tokens::Column::Token.eq(auth.uuid))
        .one(&state.db_admin)
        .await?
        .ok_or(Error::Unauthorized)?
        .delete(&state.db_admin)
        .await?;

    Ok(Response::NoContent)
}

async fn login(params: UserParams, headers: HeaderMap, state: &AppState) -> Result<String> {
    if let Err(err) = params.validate() {
        return Err(Error::BadRequest(err.to_string()));
    }

    let user = Users::find()
        .filter(users::Column::Email.eq(params.email))
        .one(&state.db_admin)
        .await?
        .ok_or(Error::InvalidCredentials)?;

    if !auth::verify_password(&params.password, &user.password) {
        return Err(Error::InvalidCredentials);
    }

    let agent = match headers.get(header::USER_AGENT) {
        Some(header) => match header.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => "Unknown".to_string(),
        },
        None => "Unknown".to_string(),
    };

    let now = Utc::now();
    let exp = (now + Duration::days(SESSION_DAYS)).timestamp() as usize;

    let auth = AuthClaim {
        user_id: user.id,
        uuid: Uuid::new_v4(),
        exp,
    };

    user_tokens::ActiveModel {
        user_id: Set(auth.user_id),
        token: Set(auth.uuid.into()),
        agent: Set(agent),
        ..Default::default()
    }
    .insert(&state.db_admin)
    .await?;

    let token = match encode(&Header::new(Algorithm::EdDSA), &auth, &state.prv_key) {
        Ok(token) => token,
        Err(_) => return Err(Error::InvalidCredentials),
    };

    Ok(token)
}
