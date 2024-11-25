use std::sync::Arc;

use anyhow::Result;
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use otobusum_anlik_backend::{
    database::{
        fetch_hatkodu_by_durakkodu, fetch_stop_info_by_durakkodu, get_db_connection,
        BusRouteStopResponse,
    },
    AppError,
};
use serde::Serialize;
use sqlx::PgPool;
use tokio::{join, try_join};
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
use tracing::info;
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
        .route("/stop/:stop_id", get(get_stop_data))
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

    let port = std::env::var("PORT").unwrap_or("8000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &port))
        .await
        .unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
pub struct BusesInStopResponse {
    #[serde(flatten)]
    stop_info: BusRouteStopResponse,
    buses: Vec<String>,
}

async fn get_stop_data(
    Path(stop_id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<BusesInStopResponse>, AppError> {
    let (buses, stop_info) = try_join!(
        fetch_hatkodu_by_durakkodu(&state.db, stop_id),
        fetch_stop_info_by_durakkodu(&state.db, stop_id),
    )?;

    Ok(Json(BusesInStopResponse { stop_info, buses }))
}
