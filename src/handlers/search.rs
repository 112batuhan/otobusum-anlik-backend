use std::sync::Arc;

use anyhow::anyhow;
use axum::extract::{Query, State};
use axum::Json;
use cached::macros::concurrent_cached;
use cached::time::Duration;
use cached::AsyncRedisCache;

use crate::models;
use crate::models::app::{AppError, AppState};
use crate::query::SearchQuery;

#[concurrent_cached(
    map_error = r##"|e| anyhow!("{}", e) "##,
    ty = "AsyncRedisCache<String, models::search::Search>",
    convert = r#"{ format!("{}{}", query.q, query.city) }"#,
    create = r##" {
        AsyncRedisCache::builder("search")
            .ttl(Duration::from_secs(600))
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn search_cached(
    query: SearchQuery,
    state: Arc<AppState>,
) -> Result<models::search::Search, AppError> {
    let stops = sqlx::query_as!(
        models::stop::BusStop,
        r#"
            SELECT
                id,
                stop_code,
                name,
                lng,
                lat,
                physical,
                province,
                smart,
                type,
                disabled_can_use,
                city
             FROM
                stops
             WHERE
                name ILIKE '%' || $1 || '%'
                AND city = $2
            LIMIT 10
        "#,
        query.q,
        query.city.as_str()
    )
    .fetch_all(&state.db)
    .await?;

    let lines = sqlx::query_as!(
        models::line::Line,
        r#"
            SELECT
                id,
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
                code, title, city, id
            ORDER BY
                code
            LIMIT 40
        "#,
        query.q,
        query.city.as_str()
    )
    .fetch_all(&state.db)
    .await?;

    Ok(models::search::Search { stops, lines })
}

pub async fn search(
    Query(query): Query<SearchQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<models::search::Search>, AppError> {
    search_cached(query, state).await.map(Json)
}
