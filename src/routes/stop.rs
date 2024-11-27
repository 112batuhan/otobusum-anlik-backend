use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use serde::Serialize;
use tokio::try_join;

use crate::models::{
    app::{AppError, AppState},
    bus::BusStop
};

use crate::database::stop::{fetch_line_code_with_stop_code, fetch_stop_with_stop_code};

#[derive(Serialize)]
pub struct BussesInStopResponse {
    stop: BusStop,
    buses: Vec<String>,
}

pub async fn get_stop(
    Path(stop_id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<BussesInStopResponse>, AppError> {
    let (buses, stop) = try_join!(
        fetch_line_code_with_stop_code(&state.db, stop_id),
        fetch_stop_with_stop_code(&state.db, stop_id as i32),
    )?;

    Ok(Json(BussesInStopResponse { stop, buses }))
}
