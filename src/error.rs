use std::io;

use axum::response::IntoResponse;
use http::StatusCode;
use thiserror::Error;

use crate::db;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("{0:?}")]
    Io(#[from] io::Error),
    #[error("{0:?}")]
    Db(#[from] db::Error),
    #[error("{0:?}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{0:?}")]
    Rss(#[from] rss::Error),
    #[error("Not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
}

impl IntoResponse for ApiError {
    /// Convert any errors in handlers into responses and log them
    fn into_response(self) -> axum::response::Response {
        tracing::error!("API Error: {self}");
        match self {
            Self::Io(_) | Self::Db(_) | Self::Reqwest(_) | Self::Rss(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error").into_response()
            }
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized").into_response(),
            Self::NotFound => (StatusCode::NOT_FOUND, "not found").into_response(),
        }
    }
}
