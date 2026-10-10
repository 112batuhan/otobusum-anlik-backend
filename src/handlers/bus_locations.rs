use anyhow::anyhow;
use cached::time::Duration;
use cached::AsyncRedisCache;
use chrono::Utc;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::Json;
use cached::macros::concurrent_cached;

use crate::database::city::City;
use crate::models::app::{AppError, AppState};
use crate::models::bus_location::{BusLocation, BusLocationResponseV1};
use crate::query::CityQuery;

use crate::api::ist::fetch_bus_locations::fetch_bus_locations as fetch_bus_locations_ist;
use crate::api::izm::fetch_bus_locations::fetch_bus_locations as fetch_bus_locations_izm;

#[concurrent_cached(
    map_error = r##"|e| anyhow!("{}", e)"##,
    ty = "AsyncRedisCache<String, BusLocationResponseV1>",
    convert = r#"{ format!("{}{:?}", line_code, city) }"#,
    create = r##" {
        AsyncRedisCache::new("bus-locations", Duration::from_secs(60))
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn bus_locations_cached(
    line_code: String,
    city: City,
    state: Arc<AppState>,
) -> Result<BusLocationResponseV1, AppError> {
    let bus_locations = match city {
        City::istanbul => fetch_bus_locations_ist(&state.reqwest, &line_code).await?,
        City::izmir => fetch_bus_locations_izm(&state.reqwest, &line_code).await?,
    };
    
    Ok(BusLocationResponseV1 {
        data: bus_locations,
        fetched_at: Utc::now()
    })
}

pub async fn bus_locations(
    Path(line_code): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<CityQuery>,
) -> Result<Json<Vec<BusLocation>>, AppError> {
    bus_locations_cached(line_code, query.city, state)
        .await
        .map(|x| x.data)
        .map(Json)
}

pub async fn bus_locations_v1(
    Path(line_code): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<CityQuery>,
) -> Result<Json<BusLocationResponseV1>, AppError> {
    bus_locations_cached(line_code, query.city, state)
        .await
        .map(Json)
}
