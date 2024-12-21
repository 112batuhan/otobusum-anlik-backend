use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    Json,
};
use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;
use serde::{Deserialize, Serialize};
use tokio::try_join;

use crate::models::{
    app::{AppError, AppState},
    stop::BusStop,
    line::LineStop,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct BussesInStopResponse {
    stop: BusStop,
    buses: Vec<String>,
}

#[io_cached(
    map_error = r##"|e| anyhow!("{}", e) "##,
    ty = "AsyncRedisCache<u32, BussesInStopResponse>",
    convert = "{stop_id}",
    create = r##" {
        AsyncRedisCache::new("get_stop", 600)
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn cached_get_stop(
    stop_id: u32,
    state: Arc<AppState>,
) -> Result<BussesInStopResponse, AppError> {
    let stop_future = sqlx::query_as!(
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
            FROM stops WHERE stop_code = $1
        "#,
        stop_id as i32
    )
    .fetch_one(&state.db);

    let line_stops_future = sqlx::query_as!(
        LineStop,
        r#"
            SELECT 
                id,
                line_code,
                stop_code,
                route_code as "route_code!",
                city
            FROM
                line_stops
            WHERE
                stop_code = $1
        "#,
        stop_id as i32
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
) -> Result<Json<BussesInStopResponse>, AppError> {
    cached_get_stop(stop_id, state).await.map(Json)
}
