use anyhow::anyhow;
use cached::AsyncRedisCache;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use cached::proc_macro::io_cached;
use serde::{de, Deserialize, Deserializer, Serialize};

use crate::api::get_bus_locations::get_bus_locations;
use crate::models::app::{AppError, AppState};

#[derive(Serialize, Deserialize, Clone)]
pub struct BusLocation {
    #[serde(alias = "kapino")]
    door_no: String,
    #[serde(alias = "boylam")]
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    lng: f64,
    #[serde(alias = "enlem")]
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    lat: f64,
    #[serde(alias = "hatkodu")]
    line_code: String,
    #[serde(alias = "guzergahkodu")]
    route_code: String,
    #[serde(alias = "hatad")]
    line_name: String,
    #[serde(alias = "yon")]
    direction: String,
    #[serde(alias = "son_konum_zamani")]
    last_location_update: String,
    #[serde(alias = "yakinDurakKodu")]
    #[serde(deserialize_with = "deserialize_u32_from_string")]
    closest_stop_code: u32,
}

pub fn deserialize_f64_from_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(de::Error::custom)
}
pub fn deserialize_u32_from_string<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<u32>().map_err(de::Error::custom)
}

#[derive(Serialize, Deserialize)]
pub struct BusLocationResponseJson {
    #[serde(alias = "GetHatOtoKonum_jsonResult")]
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct BusLocationResponseBody {
    #[serde(alias = "GetHatOtoKonum_jsonResponse")]
    content: BusLocationResponseJson,
}

#[derive(Serialize, Deserialize)]
pub struct BusLocationResponse {
    #[serde(alias = "Body")]
    content: BusLocationResponseBody,
}

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
