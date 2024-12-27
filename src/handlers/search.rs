use std::sync::Arc;

use anyhow::anyhow;
use axum::extract::{Query, State};
use axum::Json;
use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;
use serde::{Deserialize, Serialize};

use crate::database::city::City;
use crate::models::app::{AppError, AppState};
use crate::models::line::BusLine;
use crate::models::stop::BusStop;
use crate::query::default_city;

#[derive(Deserialize, Debug)]
pub struct Search {
    #[serde(default = "default_city")]
    city: City,
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
    convert = r#"{ format!("{}{}", query.q, query.city) }"#,
    create = r##" {
        AsyncRedisCache::new("search", 600)
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn search_cached(
    query: Search,
    state: Arc<AppState>,
) -> Result<SearchResponse, AppError> {
    let stops = sqlx::query_as!(
        BusStop,
        r#"
            SELECT
                id,
                stop_code,
                stop_name,
                x_coord,
                y_coord,
                physical,
                province,
                smart,
                stop_type,
                disabled_can_use,
                city
             FROM
                stops
             WHERE
                stop_name ILIKE '%' || $1 || '%'
                AND city = $2
            LIMIT 10
        "#,
        query.q,
        query.city.as_str()
    )
    .fetch_all(&state.db)
    .await?;

    // COALESCE(NULLIF(ARRAY_AGG((bus_stops)), '{NULL}'), '{}') as "stop_codes: Vec<i32>"
    let lines = sqlx::query_as!(
        BusLine,
        r#"
            SELECT
                code,
                title,
                city
            FROM
                lines
            WHERE
                (
                    code ILIKE '%' || $1 || '%'
                    OR TO_TSVECTOR( title ) @@ websearch_to_tsquery('' || $1 || ':*')
                )
                AND city = $2
            GROUP BY
                code, title, city
            ORDER BY
                code
            LIMIT 40
        "#,
        query.q,
        query.city.as_str()
    )
    .fetch_all(&state.db)
    .await?;

    Ok(SearchResponse { stops, lines })
}
pub async fn search(
    Query(query): Query<Search>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<SearchResponse>, AppError> {
    search_cached(query, state).await.map(Json)
}
