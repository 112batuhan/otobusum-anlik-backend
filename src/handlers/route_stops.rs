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
        app::{AppError, AppState},
        stop::BusStop,
    },
    query::CityQuery,
};

#[io_cached(
    map_error = r##"|e| anyhow!("{}", e) "##,
    ty = "AsyncRedisCache<String, Vec<BusStop>>",
    convert = r#"{ format!("{}{:?}", route_code, city) }"#,
    create = r##" {
        AsyncRedisCache::new("route-stops", 600)
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn route_stops_cached(
    route_code: String,
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
                line_stops.route_code = $1
                AND line_stops.city = $2
            ORDER BY
                line_stops.stop_order
        "#,
        route_code,
        city.as_str()
    )
    .fetch_all(&state.db)
    .await?;

    Ok(stops)
}

pub async fn route_stops(
    Path(route_code): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<CityQuery>,
) -> Result<Json<Vec<BusStop>>, AppError> {
    route_stops_cached(route_code, query.city, state)
        .await
        .map(Json)
}
