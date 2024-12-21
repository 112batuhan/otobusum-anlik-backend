use anyhow::anyhow;
use cached::AsyncRedisCache;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use cached::proc_macro::io_cached;

use crate::api::get_bus_locations::get_bus_locations;
use crate::models::app::{AppError, AppState};
use crate::models::location::{BusLocation, BusLocationResponse};

#[io_cached(
    map_error = r##"|e| anyhow!("{}", e) "##,
    ty = "AsyncRedisCache<String, Vec<BusLocation>>",
    convert = "{line_code.clone()}",
    create = r##" {
        AsyncRedisCache::new("bus-locations", 60)
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn bus_locations_cached(
    line_code: String,
    state: Arc<AppState>,
) -> Result<Vec<BusLocation>, AppError> {
    let content = get_bus_locations(&state.reqwest, &line_code).await?;

    let response: BusLocationResponse = serde_xml_rs::from_str(&content)?;
    let inner_content = response.content.content.content;

    let bus_locations = serde_json::from_str(&inner_content)?;
    Ok(bus_locations)
}

pub async fn bus_locations(
    Path(line_code): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<BusLocation>>, AppError> {
    bus_locations_cached(line_code, state).await.map(Json)
}
