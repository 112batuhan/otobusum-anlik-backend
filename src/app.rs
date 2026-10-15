use std::sync::Arc;

use sqlx::PgPool;

use anyhow::Result;
use tracing_subscriber::fmt::format::FmtSpan;

use crate::models::app::AppState;

pub async fn get_db_connection() -> Result<PgPool> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env should be set");
    Ok(PgPool::connect(&database_url).await?)
}

pub async fn init() -> Result<()> {
    dotenvy::dotenv()?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    Ok(())
}

pub async fn create_app() -> Result<Arc<AppState>> {
    init().await?;

    let db = get_db_connection().await?;
    sqlx::migrate!("./migrations").run(&db).await?;

    Ok(Arc::new(AppState::new(db)))
}
