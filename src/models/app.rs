use std::time::Duration;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use cached::RedisCacheError;
use reqwest::StatusCode;
use serde_json::json;
use sqlx::{PgPool, Pool, Postgres};

pub struct AppState {
    pub db: PgPool,
    pub http_client: reqwest::Client,
}

impl AppState {
    pub fn new(conn: Pool<Postgres>) -> Self {
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(3))
            .build()
            .unwrap();

        Self {
            db: conn,
            http_client: client,
        }
    }
}

pub struct AppError {
    pub status: StatusCode,
    pub error: anyhow::Error,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(json!({
                "status": self.status.as_u16(),
                "message": self.error.to_string()
            })),
        )
            .into_response()
    }
}

impl AppError {
    pub fn internal<E>(error: E) -> Self
    where
        E: Into<anyhow::Error>,
    {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: error.into(),
        }
    }
}

impl From<RedisCacheError> for AppError {
    fn from(error: RedisCacheError) -> Self {
        Self::internal(error)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        Self::internal(error)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self {
                status: StatusCode::NOT_FOUND,
                error: value.into(),
            },
            _ => Self {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                error: value.into(),
            },
        }
    }
}
