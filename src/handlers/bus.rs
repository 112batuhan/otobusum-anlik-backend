use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::Json;

use crate::models::app::{AppError, AppState};
use crate::models::bus::BusLocation;
use crate::query::CityQuery;

pub async fn get_bus_locations(
    Path(line_code): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<CityQuery>,
) -> Result<Json<Vec<BusLocation>>, AppError> {
    super::v1::bus::_get_bus_locations(line_code, state, query.city)
        .await
        .map(|x| x.data)
        .map(Json)
}
