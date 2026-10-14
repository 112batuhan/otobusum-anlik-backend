use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    api::ist::fetch_stop_arrivals::fetch_stop_arrivals,
    models::{
        app::{AppError, AppState},
        ist::stop_arrival::StopArrival,
    },
};

pub async fn get_stop_arrivals(
    Path(stop_id): Path<u32>,
    State(state): State<Arc<AppState>>,
    // Query(query): Query<CityQuery>,
) -> Result<Json<Vec<StopArrival>>, AppError> {
    Ok(fetch_stop_arrivals(&state.reqwest, &stop_id.to_string())
        .await
        .map(Json)?)
}
