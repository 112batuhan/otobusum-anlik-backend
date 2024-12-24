use anyhow::anyhow;
use cached::AsyncRedisCache;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::Json;
use cached::proc_macro::io_cached;

use crate::api::get_bus_locations::{get_bus_locations, get_bus_locations_izmir};
use crate::database::city::City;
use crate::models::app::{AppError, AppState};
use crate::models::location::BusLocation;
use crate::query::CityQuery;

#[io_cached(
    map_error = r##"|e| anyhow!("{}", e) "##,
    ty = "AsyncRedisCache<String, Vec<BusLocation>>",
    convert = r#"{ format!("{}{:?}", line_code, city) }"#,
    create = r##" {
        AsyncRedisCache::new("bus-locations", 60)
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn bus_locations_cached(
    line_code: String,
    city: City,
    state: Arc<AppState>,
) -> Result<Vec<BusLocation>, AppError> {
    let bus_locations = match city {
        City::istanbul => get_bus_locations(&state.reqwest, &line_code).await?,
        City::izmir => get_bus_locations_izmir(&state.reqwest, &line_code).await?,
    };

    Ok(bus_locations)
}

pub async fn bus_locations(
    Path(line_code): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<CityQuery>,
) -> Result<Json<Vec<BusLocation>>, AppError> {
    bus_locations_cached(line_code, query.city, state)
        .await
        .map(Json)
}
