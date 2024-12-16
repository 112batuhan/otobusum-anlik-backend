use anyhow::Result;
use sqlx::PgPool;

pub async fn get_db_connection() -> Result<PgPool> {
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let pool = PgPool::connect(&database_url).await?;

    Ok(pool)
}
