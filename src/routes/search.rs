use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::models::app::{AppError, AppState};
use crate::models::bus::BusStop;
use crate::models::line::BusLine;

#[derive(Deserialize)]
pub struct Search {
    q: String
}

#[derive(Serialize)]
pub struct SearchResponse {
    stops: Vec<BusStop>,
    lines: Vec<BusLine>
}

pub async fn search(
    Query(Search { q }): Query<Search>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<SearchResponse>, AppError> {
    let stops = sqlx::query_as!(
        BusStop,
        r#"
            SELECT * FROM bus_route_stops WHERE hatkodu LIKE '%' || $1 || '%'
            LIMIT 10
        "#,
        q
    )
        .fetch_all(&state.db)
        .await?;

    let lines = sqlx::query_as!(
        BusLine,
        r#"
            SELECT * FROM lines WHERE to_tsvector(code) @@ to_tsquery('' || $1 || ':*')
            LIMIT 10
        "#,
        Some(q)
    )
        .fetch_all(&state.db)
        .await?;

    Ok(Json(SearchResponse { stops, lines }))
}
