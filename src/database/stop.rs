use sqlx::PgPool;
use anyhow::{Context, Result};

use crate::models::bus::BusStop;

pub async fn fetch_line_code_with_stop_code(pool: &PgPool, stop_code: u32) -> Result<Vec<String>> {
    let hatkodus = sqlx::query!(
        r#"
            SELECT hatkodu
            FROM bus_route_stops
            WHERE durakkodu = $1
            "#,
        stop_code as i32
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|record| record.hatkodu)
    .collect();

    Ok(hatkodus)
}

pub async fn fetch_stop_with_stop_code(
    pool: &PgPool,
    stop_code: i32,
) -> Result<BusStop> {
    let stop_info = sqlx::query_as!(
        BusStop,
        r#"
            SELECT *
            FROM stops
            WHERE stop_code = $1
            LIMIT 1
        "#,
        stop_code as i32
    )
    .fetch_optional(pool)
    .await?;

    stop_info.context("Missing bus stop in Database")
}
