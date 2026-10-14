use std::sync::Arc;
use std::time::Duration;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use cached::{concurrent_cached, AsyncRedisCache};

use anyhow::anyhow;
use tokio::try_join;

use crate::{
    models::{
        app::{AppError, AppState},
        city::City,
        v1,
    },
    query::CityQuery,
};

#[concurrent_cached(
    map_error = r##"|e| anyhow!("{}", e) "##,
    ty = "AsyncRedisCache<String, v1::stop::BusStopDetail>",
    convert = r#"{ format!("{}{:?}", stop_id, city) }"#,
    create = r##" {
        AsyncRedisCache::builder("get_stop")
            .ttl(Duration::from_secs(600))
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn _get_stop(
    stop_id: i32,
    state: Arc<AppState>,
    city: City,
) -> Result<v1::stop::BusStopDetail, AppError> {
    let stop_query = sqlx::query_as!(
        v1::stop::BusStop,
        r#"SELECT
            id, stop_code AS code, name, lng, lat, physical, province, smart, type AS stop_type, disabled_can_use, city
        FROM
            stops 
        WHERE
            stop_code = $1 AND city = $2"#,
        stop_id,
        city.as_str()
    )
        .fetch_one(&state.db);

    let line_stops_query = sqlx::query_as!(
        v1::line::LineStop,
        r#"SELECT
            DISTINCT ON (line_code) line_code,
            id,
            stop_code,
            route_code,
            city
        FROM
            line_stops
        WHERE stop_code = $1 AND city = $2"#,
        stop_id,
        city.as_str()
    )
    .fetch_all(&state.db);

    let (stop, line_stops) = try_join!(stop_query, line_stops_query)?;
    let line_codes = line_stops
        .into_iter()
        .map(|line_stop| line_stop.line_code)
        .collect();

    Ok(v1::stop::BusStopDetail {
        stop,
        buses: line_codes,
    })
}

pub async fn get_stop(
    Path(stop_id): Path<i32>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<CityQuery>,
) -> Result<Json<v1::stop::BusStopDetail>, AppError> {
    _get_stop(stop_id, state, query.city).await.map(Json)
}
