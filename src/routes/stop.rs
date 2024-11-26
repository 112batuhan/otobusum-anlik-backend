use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use tokio::try_join;

use crate::models::{
    app::{AppError, AppState},
    bus::BusRouteStopResponse,
};

use crate::database::stop::{fetch_hatkodu_by_durakkodu, fetch_stop_info_by_durakkodu};

#[derive(Serialize)]
pub struct BusesInStopResponse {
    #[serde(flatten)]
    stop_info: BusRouteStopResponse,
    buses: Vec<String>,
}

pub async fn get_stop_data(
    Path(stop_id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<BusesInStopResponse>, AppError> {
    let (buses, stop_info) = try_join!(
        fetch_hatkodu_by_durakkodu(&state.db, stop_id),
        fetch_stop_info_by_durakkodu(&state.db, stop_id),
    )?;

    Ok(Json(BusesInStopResponse { stop_info, buses }))
}
