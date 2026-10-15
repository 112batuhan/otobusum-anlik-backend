use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{Path, Query, State},
    Json,
};

use cached::macros::concurrent_cached;
use cached::time::Duration;
use cached::AsyncRedisCache;

use tokio::try_join;

use crate::{
    models::{
        self,
        app::{AppError, AppState},
        city::City,
    },
    query::CityQuery,
};

#[concurrent_cached(
    map_error = r##"|e| anyhow!("{}", e) "##,
    ty = "AsyncRedisCache<String, models::stop::BusStopDetail>",
    convert = r#"{ format!("{}{:?}", stop_id, city) }"#,
    create = r##" {
        AsyncRedisCache::builder("get_stop")
            .ttl(Duration::from_secs(600))
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn cached_get_stop(
    stop_id: u32,
    city: City,
    state: Arc<AppState>,
) -> Result<models::stop::BusStopDetail, AppError> {
    let stop_future = sqlx::query_as!(
        models::stop::BusStop,
        r#"
            SELECT 
                id,
                stop_code,
                name,
                lng,
                lat,
                province,
                city
            FROM
                stops
            WHERE
                stop_code = $1
                AND city = $2
        "#,
        stop_id as i32,
        city.as_str(),
    )
    .fetch_one(&state.db);

    let line_stops_future = sqlx::query_as!(
        models::line::LineStop,
        r#"
            SELECT 
                DISTINCT ON (line_code) line_code,
                id,
                stop_code,
                route_code,
                city
            FROM
                line_stops
            WHERE
                stop_code = $1
                AND city = $2
        "#,
        stop_id as i32,
        city.as_str(),
    )
    .fetch_all(&state.db);

    let (stop, line_stops) = try_join!(stop_future, line_stops_future)?;
    let line_codes = line_stops.into_iter().map(|line| line.line_code).collect();

    Ok(models::stop::BusStopDetail {
        stop,
        buses: line_codes,
    })
}

pub async fn get_stop(
    Path(stop_id): Path<u32>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<CityQuery>,
) -> Result<Json<models::stop::BusStopDetail>, AppError> {
    cached_get_stop(stop_id, query.city, state).await.map(Json)
}
