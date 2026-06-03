use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    Json,
};
use reqwest::StatusCode;

use crate::{
    api::ist::fetch_bus_info::fetch_bus_infos,
    models::{
        app::{AppError, AppState},
        ist::bus_info::BusInfo,
    },
};

pub async fn bus_info(
    Path(door_no): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<BusInfo>, AppError> {
    let bus_list = fetch_bus_infos(&state.reqwest).await?;

    let Some(bus_info) = bus_list.into_iter().find(|b| b.door_no == door_no) else {
        return Err(AppError {
            status: StatusCode::NOT_FOUND,
            error: anyhow!("bus with {door_no} is not found"),
        });
    };

    Ok(Json(bus_info))
}
