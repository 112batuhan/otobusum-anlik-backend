use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::models::app::{AppError, AppState};
use crate::models::bus::BusStop;
use crate::models::line::{BusLine, BusLineWithCoordinates};

#[derive(Deserialize)]
pub struct Search {
    q: String
}

#[derive(Serialize)]
pub struct SearchResponse {
    stops: Vec<BusStop>,
    lines: Vec<BusLineWithCoordinates>
}

pub async fn search(
    Query(Search { q }): Query<Search>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<SearchResponse>, AppError> {
    let stops = sqlx::query_as!(
        BusStop,
        r#"
            SELECT * FROM stops WHERE to_tsvector(stop_name) @@ websearch_to_tsquery('' || $1 || ':*')
            LIMIT 10
        "#,
        q
    )
        .fetch_all(&state.db)
        .await?;

    let lines = sqlx::query_as!(
        BusLineWithCoordinates,
        r#"
            SELECT code, title, rtp.points
            FROM
                lines
                LEFT JOIN route_travel_plan rtp ON lines.code = rtp.hatkodu
            WHERE
                TO_TSVECTOR( code ) @@ websearch_to_tsquery('' || $1 || ':*')
                OR TO_TSVECTOR( title ) @@ websearch_to_tsquery('' || $1 || ':*')
            LIMIT 10
        "#,
        q
    )
        .fetch_all(&state.db)
        .await?;

    Ok(Json(SearchResponse { stops, lines }))
}
