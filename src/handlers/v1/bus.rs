use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use cached::cached;
use chrono::Utc;
use reqwest::StatusCode;

use crate::{
    api::{
        ist::{self, bus_info::fetch_bus_infos},
        izm,
    },
    models::{
        app::{AppError, AppState},
        city::City,
        ist::bus_info::BusInfo,
        v1,
    },
    query::CityQuery,
};

#[cached(
    key = "String",
    convert = r#"{ format!("{}{:?}", line_code, city) }"#,
    expires = true,
    sync_writes = true
)]
pub async fn _get_bus_locations(
    line_code: String,
    state: Arc<AppState>,
    city: City,
) -> Result<v1::bus::BusLocationResponse, AppError> {
    let bus_locations = match city {
        City::istanbul => {
            ist::bus_locations::fetch_bus_locations(&state.http_client, &line_code).await?
        }
        City::izmir => {
            izm::bus_locations::fetch_bus_locations(&state.http_client, &line_code).await?
        }
    };

    let next_poll = bus_locations
        .iter()
        .filter_map(|x| x.last_update_time)
        .max()
        .unwrap_or_else(Utc::now)
        .timestamp();

    Ok(v1::bus::BusLocationResponse {
        data: bus_locations,
        next_poll: next_poll + 70,
    })
}

pub async fn get_bus_locations(
    Path(line_code): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<CityQuery>,
) -> Result<Json<v1::bus::BusLocationResponse>, AppError> {
    _get_bus_locations(line_code, state, query.city)
        .await
        .map(Json)
}

pub async fn get_bus_info(
    Path(door_no): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<BusInfo>, AppError> {
    let bus_list = fetch_bus_infos(&state.http_client).await?;

    let Some(bus_info) = bus_list.into_iter().find(|b| b.door_no == door_no) else {
        return Err(AppError {
            status: StatusCode::NOT_FOUND,
            error: anyhow!("bus with {door_no} is not found"),
        });
    };

    Ok(Json(bus_info))
}
