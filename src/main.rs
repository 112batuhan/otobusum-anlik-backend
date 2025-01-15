pub mod api;
pub mod database;
pub mod handlers;
pub mod models;
pub mod query;

use std::sync::Arc;

use crate::{database::get_db_connection, models::app::AppState};
use axum::{http::HeaderValue, routing::get, Router};
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

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

    sqlx::migrate!("./migrations").run(&db_conn).await.ok();

    let state = Arc::new(AppState::new(db_conn));

    let allow_origin = if cfg!(debug_assertions) {
        "*".parse::<HeaderValue>().unwrap()
    } else {
        "https://otobusumweb.metkm.win".parse::<HeaderValue>().unwrap()
    };

    let cors_layer = CorsLayer::new().allow_origin(allow_origin);

    let app = Router::new()
        .route("/stop/:stop_id", get(handlers::stop::get_stop))
        .route("/routes/:line_code", get(handlers::routes::routes))
        .route(
            "/bus-locations/:line_code",
            get(handlers::bus_locations::bus_locations),
        )
        .route(
            "/route-stops/:line_code",
            get(handlers::route_stops::route_stops),
        )
        .route("/timetable/:line_code", get(handlers::timetable::timetable))
        .route("/search", get(handlers::search::search))
        .layer(cors_layer)
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
