use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
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
        .bind(bus_route_stop.durakkodu as i32)
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

pub async fn delete_by_hatkodu(pool: &PgPool, hatkodu: &str) -> Result<()> {
    sqlx::query!(
        r#"
            DELETE FROM bus_route_stops
            WHERE hatkodu = $1
            "#,
        hatkodu
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn fetch_unique_hatkodus(pool: &PgPool) -> Result<Vec<String>> {
    let hatkodus = sqlx::query!(
        r#"
            SELECT DISTINCT hatkodu
            FROM bus_route_stops
            "#
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|record| record.hatkodu)
    .collect();

    Ok(hatkodus)
}

pub async fn fetch_hatkodu_by_durakkodu(pool: &PgPool, durakkodu: u32) -> Result<Vec<String>> {
    let hatkodus = sqlx::query!(
        r#"
            SELECT hatkodu
            FROM bus_route_stops
            WHERE durakkodu = $1
            "#,
        durakkodu as i32
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|record| record.hatkodu)
    .collect();

    Ok(hatkodus)
}

#[derive(Serialize, Deserialize)]
pub struct BusRouteStopResponse {
    durakkodu: i32,
    durakadi: String,
    xkoordinati: f64,
    ykoordinati: f64,
    duraktipi: String,
    isletmebolge: Option<String>,
    isletmealtbolge: String,
    ilceadi: String,
}
pub async fn fetch_stop_info_by_durakkodu(
    pool: &PgPool,
    durakkodu: u32,
) -> Result<BusRouteStopResponse> {
    let stop_info = sqlx::query_as!(
        BusRouteStopResponse,
        r#"
        SELECT 
            durakkodu,
            durakadi,
            xkoordinati,
            ykoordinati,
            duraktipi,
            isletmebolge,
            isletmealtbolge,
            ilceadi
        FROM bus_route_stops
        WHERE durakkodu = $1
        LIMIT 1
        "#,
        durakkodu as i32
    )
    .fetch_optional(pool)
    .await?;
    stop_info.context("Missing bus stop in Database")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
}

pub async fn fetch_stop_coordinates(
    pool: &PgPool,
    hatkodu: &str,
    direction: &str,
) -> Result<Vec<Coordinates>> {
    let coordinates = sqlx::query_as!(
        Coordinates,
        r#"
        SELECT
            xkoordinati as x, ykoordinati as y
        FROM
            bus_route_stops
        WHERE
            hatkodu = $1 AND yon = $2
        ORDER BY sirano ASC
        "#,
        hatkodu,
        direction
    )
    .fetch_all(pool)
    .await?;
    Ok(coordinates)
}

#[derive(Debug)]
pub struct BusStopRow {
    pub route_code: String,
    pub direction: String,
    pub x: f64,
    pub y: f64,
    pub stop_order: i32,
}

pub async fn fetch_all_stop_coordinates(pool: &PgPool) -> Result<Vec<BusStopRow>> {
    let bus_stop_row = sqlx::query_as!(
        BusStopRow,
        r#"
        SELECT DISTINCT
            hatkodu AS route_code,
            yon AS direction,
            xkoordinati AS x,
            ykoordinati AS y,
            sirano AS stop_order
        FROM
            bus_route_stops
        ORDER BY
            hatkodu, yon, sirano  
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(bus_stop_row)
}

/// TODO: detect duplicate inserts, add unique index or change it
pub async fn insert_route_plan(
    pool: &PgPool,
    hatkodu: &str,
    yon: &str,
    coordinate_string: &str,
) -> Result<()> {
    let query = r#"
            INSERT INTO route_travel_plan (
                hatkodu, yon, points
            ) VALUES ($1, $2, $3)
        "#;

    sqlx::query(query)
        .bind(hatkodu)
        .bind(yon)
        .bind(coordinate_string)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn fetch_route_plan(pool: &PgPool, hatkodu: &str, yon: &str) -> Result<String> {
    let points_string = sqlx::query_scalar!(
        r#"
            SELECT points
            FROM
                route_travel_plan
            WHERE
                hatkodu = $1
                AND yon = $2
            "#,
        hatkodu,
        yon
    )
    .fetch_optional(pool)
    .await?;
    points_string.context("Missing bus stop in Database")
}
