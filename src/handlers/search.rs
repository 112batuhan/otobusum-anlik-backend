use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::models::app::{AppError, AppState};
use crate::models::bus::BusStop;
use crate::models::line::BusLine;

#[derive(Deserialize)]
pub struct Search {
    q: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    stops: Vec<BusStop>,
    lines: Vec<BusLine>,
}

pub async fn search(
    Query(Search { q }): Query<Search>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<SearchResponse>, AppError> {
    let stops = sqlx::query_as!(
        BusStop,
        r#"
            SELECT
                id,
                stop_code,
                stop_name,
                x_coord,
                y_coord,
                physical as "physical!",
                province as "province!",
                smart as "smart!",
                stop_type as "stop_type!",
                disabled_can_use "disabled_can_use!",
                city
             FROM stops WHERE stop_name ILIKE '%' || $1 || '%'
            LIMIT 10
        "#,
        q
    )
    .fetch_all(&state.db)
    .await?;

    // COALESCE(NULLIF(ARRAY_AGG((bus_stops)), '{NULL}'), '{}') as "stop_codes: Vec<i32>"
    let lines = sqlx::query_as!(
        BusLine,
        r#"
            SELECT
                code,
                title
            FROM
                lines
            WHERE
                code ILIKE '%' || $1 || '%'
                OR TO_TSVECTOR( title ) @@ websearch_to_tsquery('' || $1 || ':*')
            GROUP BY
                code, title
            LIMIT 20
        "#,
        q
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(SearchResponse { stops, lines }))
}
