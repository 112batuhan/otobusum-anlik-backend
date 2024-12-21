use std::sync::Arc;
use anyhow::anyhow;

use axum::{extract::{Path, State}, Json};
use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;

use crate::models::{app::{AppError, AppState}, bus::BusStop};

#[io_cached(
    map_error = r##"|e| anyhow!("{}", e) "##,
    ty = "AsyncRedisCache<String, Vec<BusStop>>",
    convert = "{route_code.clone()}",
    create = r##" {
        AsyncRedisCache::new("routes", 600)
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn route_stops_cached(
    route_code: String,
    state: Arc<AppState>
) -> Result<Vec<BusStop>, AppError> {
    let stops = sqlx::query_as!(
        BusStop,
        r#"
            SELECT
                stops.id,
                stops.stop_code,
                stops.stop_name,
                stops.x_coord,
                stops.y_coord,
                stops.province,
                stops.smart,
                stops.stop_type,
                stops.disabled_can_use,
                stops.physical,
                stops.city
            FROM
                line_stops
                RIGHT JOIN stops ON stops.stop_code = line_stops.stop_code
            WHERE
                line_stops.route_code = $1
        "#,
        route_code
    )
        .fetch_all(&state.db)
        .await?;

    Ok(stops)
}

pub async fn route_stops(
    Path(route_code): Path<String>,
    State(state): State<Arc<AppState>>
) -> Result<Json<Vec<BusStop>>, AppError> {
    route_stops_cached(route_code, state).await.map(Json)
}
