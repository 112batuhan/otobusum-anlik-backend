use std::sync::Arc;

use axum::extract::{Path, State};

use crate::models::app::{AppError, AppState};
use crate::api::get_bus_locations::get_bus_locations;

pub async fn bus_locations(
    Path(line_code): Path<String>,
    State(state): State<Arc<AppState>>
) -> Result<(), AppError> {
    let locations = get_bus_locations(&state.reqwest, &line_code)
        .await?;

    Ok(())
}
