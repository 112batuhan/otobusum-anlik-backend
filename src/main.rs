use std::sync::Arc;

use anyhow::Result;
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use otobusum_anlik_backend::{
    database::{
        fetch_hatkodu_by_durakkodu, fetch_route_plan, fetch_stop_coordinates,
        fetch_stop_info_by_durakkodu, get_db_connection, BusRouteStopResponse, Coordinates,
    },
    request::request_graphhopper_routes,
    AppError, BusRouteWithCoordinates,
};
use serde::Serialize;
use sqlx::PgPool;
use tokio::try_join;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

pub struct AppState {
    db: PgPool,
    reqwest: reqwest::Client,
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
    let state = Arc::new(AppState {
        db: db_conn,
        reqwest: reqwest::Client::new(),
    });

    let app = Router::new()
        .route(
            "/routetest/:route_code/:direction",
            get(get_route_data_dynamic),
        )
        .route("/route/:route_code/:direction", get(get_route_data))
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

async fn get_route_data(
    Path((route_code, direction)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<BusRouteWithCoordinates>, AppError> {
    let coordinates = fetch_route_plan(&state.db, &route_code, &direction).await?;
    let coordinates: Vec<Coordinates> = serde_json::from_str(&coordinates)?;
    Ok(Json(BusRouteWithCoordinates {
        route_code,
        direction,
        coordinates,
    }))
}

async fn get_route_data_dynamic(
    Path((route_code, direction)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<BusRouteWithCoordinates>, AppError> {
    let stop_coordinates = fetch_stop_coordinates(&state.db, &route_code, &direction).await?;
    let coordinates = request_graphhopper_routes(state.reqwest.clone(), stop_coordinates).await?;
    Ok(Json(BusRouteWithCoordinates {
        route_code,
        direction,
        coordinates,
    }))
}
