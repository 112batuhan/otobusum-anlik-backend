use axum::response::{IntoResponse, Response};
use database::Coordinates;
use reqwest::StatusCode;
use serde::Serialize;

pub mod csv_parse;
pub mod database;
pub mod request;
pub mod xml_parse;

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

#[derive(Debug, Serialize)]
pub struct BusRouteWithCoordinates {
    pub route_code: String,
    pub direction: String,
    pub coordinates: Vec<Coordinates>,
}
