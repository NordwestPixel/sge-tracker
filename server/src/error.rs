use crate::config::ConfigError;
use crate::sync::error::SyncError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Sqlx(e) => {
                error!("SQLx Error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum InitError {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Pool(#[from] sqlx::Error),
    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error("bind: {address} with error: {source}")]
    Bind {
        address: String,
        source: std::io::Error,
    },
    #[error("Server stopped accepting connections")]
    Serve(#[from] std::io::Error),
    #[error(transparent)]
    Sync(#[from] SyncError),
}
