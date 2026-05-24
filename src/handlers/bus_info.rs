use anyhow::anyhow;
use axum::response::IntoResponse;
use cached::AsyncRedisCache;
use reqwest::StatusCode;
use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, State};
use cached::proc_macro::io_cached;

use crate::api::get_bus_info::get_bus_infos;
use crate::models::{app::{AppError, AppState}, locations::ist::BusInfoIst};

#[io_cached(
    map_error = r##"|e| anyhow!("{}", e) "##,
    ty = "AsyncRedisCache<String, Option<BusInfoIst>>",
    convert = r#"{ format!("{}", door_no) }"#,
    create = r##" {
        AsyncRedisCache::new("bus-info", 60)
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn bus_info_cached(
    door_no: String,
    state: Arc<AppState>,
) -> Result<Option<BusInfoIst>, AppError> {
    let bus_infos = get_bus_infos(&state.reqwest).await?;

    let info = bus_infos
        .into_iter()
        .find(|i| i.door_no == door_no);

    Ok(info)
}

#[axum::debug_handler]
pub async fn bus_info(
    Path(door_no): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let res = bus_info_cached(door_no, state)
        .await?;

    match res {
        Some(bus) => Ok((StatusCode::OK, Json(bus)).into_response()),
        None => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}
