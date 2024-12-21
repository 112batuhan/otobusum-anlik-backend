use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use sqlx::{PgPool, Pool, Postgres};

pub struct AppState {
    pub db: PgPool,
    pub reqwest: reqwest::Client,
}

impl AppState {
    pub fn new(conn: Pool<Postgres>) -> Self {
        Self {
            db: conn,
            reqwest: reqwest::Client::new()
        }
    }
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
