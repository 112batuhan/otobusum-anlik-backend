use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{Path, Query, State},
    Json,
};

use cached::macros::concurrent_cached;
use cached::time::Duration;
use cached::AsyncRedisCache;

use serde::{Deserialize, Serialize};
use tokio::try_join;

use crate::{
    database::city::City,
    models::v1::stop::BusStop as BusStopV1,
    models::{
        app::{AppError, AppState},
        line::LineStop,
        stop::BusStop,
    },
    query::CityQuery,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct BussesInStopResponse {
    stop: BusStop,
    buses: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BussesInStopResponseV1 {
    stop: BusStopV1,
    buses: Vec<String>,
}

impl From<BussesInStopResponse> for BussesInStopResponseV1 {
    fn from(value: BussesInStopResponse) -> Self {
        Self {
            buses: value.buses,
            stop: BusStopV1::from(value.stop),
        }
    }
}

#[concurrent_cached(
    map_error = r##"|e| anyhow!("{}", e) "##,
    ty = "AsyncRedisCache<String, BussesInStopResponse>",
    convert = r#"{ format!("{}{:?}", stop_id, city) }"#,
    create = r##" {
        AsyncRedisCache::new("get_stop", Duration::from_secs(600))
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn cached_get_stop(
    stop_id: u32,
    city: City,
    state: Arc<AppState>,
) -> Result<BussesInStopResponse, AppError> {
    let stop_future = sqlx::query_as!(
        BusStop,
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
                stop_code = $1
                AND city = $2
        "#,
        stop_id as i32,
        city.as_str(),
    )
    .fetch_one(&state.db);

    let line_stops_future = sqlx::query_as!(
        LineStop,
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

    Ok(BussesInStopResponse {
        stop,
        buses: line_codes,
    })
}

pub async fn get_stop(
    Path(stop_id): Path<u32>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<CityQuery>,
) -> Result<Json<BussesInStopResponse>, AppError> {
    cached_get_stop(stop_id, query.city, state).await.map(Json)
}

pub async fn get_stop_v1(
    Path(stop_id): Path<u32>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<CityQuery>,
) -> Result<Json<BussesInStopResponseV1>, AppError> {
    cached_get_stop(stop_id, query.city, state)
        .await
        .map(BussesInStopResponseV1::from)
        .map(Json)
}
