use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use serde::Serialize;

use crate::models::{
    app::{AppError, AppState},
    bus::BusStop, line::LineStop
};

#[derive(Serialize)]
pub struct BussesInStopResponse {
    stop: BusStop,
    buses: Vec<String>,
}

pub async fn get_stop(
    Path(stop_id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<BussesInStopResponse>, AppError> {
    let stop = sqlx::query_as!(
        BusStop,
        r#"
            SELECT * FROM stops WHERE stop_code = $1
        "#,
        stop_id as i32
    )
        .fetch_one(&state.db)
        .await?;

    let line_stops = sqlx::query_as!(
        LineStop,
        r#"
            SELECT * FROM line_stops WHERE stop_code = $1
        "#,
        stop_id as i32
    )   
        .fetch_all(&state.db)
        .await?;

    let line_codes = line_stops
        .into_iter()
        .map(|line| line.line_code)
        .collect();

    Ok(Json(BussesInStopResponse { stop, buses: line_codes }))
}
