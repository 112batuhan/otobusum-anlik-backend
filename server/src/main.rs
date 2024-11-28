use std::sync::Arc;

use axum::{routing::get, Router};
use server::{database::get_db_connection, models::app::AppState, routes};
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

    sqlx::migrate!("../migrations").run(&db_conn).await.ok();

    let state = Arc::new(AppState::new(db_conn));

    let app = Router::new()
        .route("/stop/:stop_id", get(routes::stop::get_stop))
        //.route("/search", get(routes::search::search))
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
