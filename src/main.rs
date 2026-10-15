pub mod api;
pub mod app;
pub mod handlers;
pub mod models;
pub mod query;

use std::sync::Arc;

use anyhow::Result;
use axum::{http::HeaderValue, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use crate::models::app::AppState;

fn v1_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/stop/:stop_id", get(handlers::v1::stop::get_stop))
        .route(
            "/stop/:stop_id/arrivals",
            get(handlers::v1::stop::get_stop_arrivals),
        )
        .route("/routes/:line_code", get(handlers::v1::route::get_routes))
        .route(
            "/bus-locations/:line_code",
            get(handlers::v1::bus::get_bus_locations),
        )
        .route(
            "/route-stops/:line_code",
            get(handlers::v1::route::get_route_stops),
        )
        .route(
            "/timetable/:line_code",
            get(handlers::timetable::get_timetable),
        )
        .route("/search", get(handlers::v1::search::get_search))
        .route("/bus-info/:door_no", get(handlers::v1::bus::get_bus_info))
}

fn legacy_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/stop/:stop_id", get(handlers::stop::get_stop))
        .route("/routes/:line_code", get(handlers::routes::routes))
        .route(
            "/bus-locations/:line_code",
            get(handlers::bus::get_bus_locations),
        )
        .route(
            "/route-stops/:line_code",
            get(handlers::route_stops::get_route_stops),
        )
        .route(
            "/timetable/:line_code",
            get(handlers::timetable::get_timetable),
        )
        .route("/search", get(handlers::search::search))
        .route("/bus-info/:door_no", get(handlers::v1::bus::get_bus_info))
}

#[tokio::main]
async fn main() -> Result<()> {
    let app_state = app::create_app().await?;

    let allow_origin = if cfg!(debug_assertions) {
        HeaderValue::from_static("*")
    } else {
        HeaderValue::from_static("https://otobusumweb.metkm.win")
    };

    let cors_layer = CorsLayer::new().allow_origin(allow_origin);

    let compression_layer = CompressionLayer::new()
        .gzip(true)
        .deflate(true)
        .zstd(true)
        .br(true);

    let router = Router::new()
        .nest("/v1", v1_routes())
        .merge(legacy_routes())
        .layer(compression_layer)
        .layer(cors_layer)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let port = std::env::var("PORT").unwrap_or("8000".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, router).await?;

    Ok(())
}
