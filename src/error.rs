use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use tracing::error;

#[derive(Debug)]
pub enum Error {
    BadRequest(String),
    Internal(String),

    Forbidden,
    InvalidCredentials,
    NotFound,
    Unauthorized,
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(err: sea_orm::DbErr) -> Self {
        Self::Internal(err.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::BadRequest(err) => err,
            Self::Internal(err) => err,

            Self::Forbidden => StatusCode::FORBIDDEN.canonical_reason().unwrap(),
            Self::InvalidCredentials => "Invalid Credentials",
            Self::NotFound => StatusCode::NOT_FOUND.canonical_reason().unwrap(),
            Self::Unauthorized => StatusCode::UNAUTHORIZED.canonical_reason().unwrap(),
        };

        write!(f, "{msg}")
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Internal(ref err) => {
                error!(err);
                StatusCode::INTERNAL_SERVER_ERROR
            }

            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
        };

        let mut msg = self.to_string();

        if !cfg!(debug_assertions) && status == StatusCode::INTERNAL_SERVER_ERROR {
            msg = StatusCode::INTERNAL_SERVER_ERROR
                .canonical_reason()
                .unwrap()
                .into()
        }

        (
            status,
            Json(json!({
                "msg": msg
            })),
        )
            .into_response()
    }
}
