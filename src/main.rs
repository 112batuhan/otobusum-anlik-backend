use std::sync::Arc;

use anyhow::Result;
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use iett_stops_with_busses::{
    database::{fetch_hatkodu_by_durakkodu, get_db_connection},
    AppError,
};
use sqlx::PgPool;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::fmt::format::FmtSpan;

pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let db_conn = get_db_connection()
        .await
        .expect("Failed to initialize DB connection");
    let state = Arc::new(AppState { db: db_conn });

    let app = Router::new()
        .route("/busses-in-stop/:stop_id", get(get_busses_in_stop))
        .layer(CorsLayer::very_permissive())
        .layer(
            CompressionLayer::new()
                .gzip(true)
                .deflate(true)
                .zstd(true)
                .br(true),
        )
        .layer(TraceLayer::new_for_http())
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
