use anyhow::anyhow;
use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;

use crate::{
    database::city::City,
    models::{
        app::{AppError, AppState}, routes::Direction, stop::BusStop
    },
    query::LineStopsQuery,
};

#[io_cached(
    map_error = r##"|e| anyhow!("{}", e) "##,
    ty = "AsyncRedisCache<String, Vec<BusStop>>",
    convert = r#"{ format!("{}{}{:?}", line_code, direction, city) }"#,
    create = r##" {
        AsyncRedisCache::new("route-stops", 600)
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn route_stops_cached(
    line_code: String,
    direction: Direction,
    city: City,
    state: Arc<AppState>,
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
                line_stops.line_code = $1
                AND line_stops.city = $3
                AND line_stops.route_code LIKE $1 || '_' || $2 || '_D0'
            ORDER BY
                line_stops.stop_order
        "#,
        line_code,
        direction.as_str(),
        city.as_str()
    )
    .fetch_all(&state.db)
    .await?;

    Ok(stops)
}

pub async fn route_stops(
    Path(line_code): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<LineStopsQuery>,
) -> Result<Json<Vec<BusStop>>, AppError> {
    route_stops_cached(
            line_code,
            query.direction,
            query.city, 
            state
    )
        .await
        .map(Json)
}
