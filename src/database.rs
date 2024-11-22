use anyhow::Result;
use sqlx::PgPool;

use crate::xml_parse::BusRouteStop;

pub async fn get_db_connection() -> Result<PgPool> {
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}

pub async fn insert_bus_route_stop(pool: &PgPool, bus_route_stop: BusRouteStop) -> Result<()> {
    let query = r#"
            INSERT INTO bus_route_stops (
                hatkodu, yon, sirano, durakkodu, durakadi,
                xkoordinati, ykoordinati, duraktipi, isletmebolge,
                isletmealtbolge, ilceadi
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#;

    sqlx::query(query)
        .bind(&bus_route_stop.hatkodu)
        .bind(&bus_route_stop.yon)
        .bind(bus_route_stop.sirano as i32)
        .bind(&bus_route_stop.durakkodu)
        .bind(&bus_route_stop.durakadi)
        .bind(bus_route_stop.xkoordinati)
        .bind(bus_route_stop.ykoordinati)
        .bind(&bus_route_stop.duraktipi)
        .bind(&bus_route_stop.isletmebolge)
        .bind(&bus_route_stop.isletmealtbolge)
        .bind(&bus_route_stop.ilceadi)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn hatkodu_exist(pool: &PgPool, hatkodu: &str) -> Result<bool> {
    let result = sqlx::query_scalar!(
        r#"
            SELECT COUNT(*)
            FROM bus_route_stops
            WHERE hatkodu = $1
            "#,
        hatkodu
    )
    .fetch_one(pool)
    .await?;

    Ok(result.is_some_and(|count| count > 0))
}

pub async fn delete_by_hatkodu(pool: &PgPool, hatkodu: &str) -> Result<u64> {
    let rows_affected = sqlx::query!(
        r#"
            DELETE FROM bus_route_stops
            WHERE hatkodu = $1
            "#,
        hatkodu
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected)
}
