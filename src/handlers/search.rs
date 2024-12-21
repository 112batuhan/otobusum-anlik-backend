use std::sync::Arc;

use anyhow::anyhow;
use axum::extract::{Query, State};
use axum::Json;
use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;
use serde::{Deserialize, Serialize};

use crate::models::app::{AppError, AppState};
use crate::models::bus::BusStop;
use crate::models::line::BusLine;

#[derive(Deserialize)]
pub struct Search {
    q: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchResponse {
    stops: Vec<BusStop>,
    lines: Vec<BusLine>,
}

#[io_cached(
    map_error = r##"|e| anyhow!("{}", e) "##,
    ty = "AsyncRedisCache<String, SearchResponse>",
    convert = "{q.clone()}",
    create = r##" {
        AsyncRedisCache::new("search", 600)
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn search_cached(q: String, state: Arc<AppState>) -> Result<SearchResponse, AppError> {
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
             FROM
                stops
             WHERE
                stop_name ILIKE '%' || $1 || '%'
                AND city = 'istanbul'
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
                AND city = 'istanbul'
            GROUP BY
                code, title
            LIMIT 20
        "#,
        q
    )
    .fetch_all(&state.db)
    .await?;

    Ok(SearchResponse { stops, lines })
}
pub async fn search(
    Query(Search { q }): Query<Search>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<SearchResponse>, AppError> {
    search_cached(q, state).await.map(Json)
}
