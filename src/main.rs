use std::sync::Arc;

use anyhow::Result;
use axum::{
    debug_handler,
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use iett_stops_with_busses::{
    database::{fetch_hatkodu_by_durakkodu, get_db_connection},
    AppError,
};
use sqlx::PgPool;

pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_conn = get_db_connection()
        .await
        .expect("Failed to initialize DB connection");
    let state = Arc::new(AppState { db: db_conn });

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/busses-in-stop/:stop_id", get(get_busses_in_stop))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_busses_in_stop(
    Path(stop_id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<String>>, AppError> {
    let stops = fetch_hatkodu_by_durakkodu(&state.db, stop_id).await?;
    Ok(Json(stops))
}
