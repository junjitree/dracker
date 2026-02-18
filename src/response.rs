use axum::Json;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use serde::Serialize;

#[derive(Serialize)]
struct BatchResult {
    created_ids: Vec<u64>,
    skipped_records: Vec<String>,
}

#[derive(Debug)]
pub enum Response {
    Accepted,
    AuthCookie(CookieJar),
    AuthToken(String),
    Created(u64),
    CreatedBatch(Vec<u64>, Vec<String>),
    Csrf(CookieJar, HeaderMap),
    NoContent,
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        match self {
            Response::Accepted => StatusCode::ACCEPTED.into_response(),
            Response::AuthCookie(jar) => (StatusCode::CREATED, jar).into_response(),
            Response::AuthToken(token) => (StatusCode::CREATED, Json(token)).into_response(),
            Response::Created(id) => (StatusCode::CREATED, Json(id)).into_response(),
            Response::Csrf(jar, header) => (StatusCode::CREATED, jar, header).into_response(),
            Response::NoContent => StatusCode::NO_CONTENT.into_response(),
            Response::CreatedBatch(created_ids, skipped_records) => {
                let response_body = BatchResult {
                    created_ids,
                    skipped_records,
                };
                // Return 201 Created status with the structured JSON body
                (StatusCode::CREATED, Json(response_body)).into_response()
            }
        }
    }
}
