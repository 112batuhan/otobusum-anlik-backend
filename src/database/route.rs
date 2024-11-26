use anyhow::{Context, Result};
use sqlx::PgPool;
// use std::sync::Arc;

// use axum::{
//     extract::{Path, State},
//     Json,
// };

// use crate::models::{
//     self,
//     app::{AppError, AppState},
//     bus::BusRouteWithCoordinates,
// };

// pub async fn fetch_route_plan(pool: &PgPool, hatkodu: &str, yon: &str) -> Result<String> {
//     let points_string = sqlx::query_scalar!(
//         r#"
//             SELECT points
//             FROM
//                 route_travel_plan
//             WHERE
//                 hatkodu = $1
//                 AND yon = $2
//             "#,
//         hatkodu,
//         yon
//     )
//     .fetch_optional(pool)
//     .await?;

//     points_string.context("Missing bus stop in Database")
// }

// async fn get_route_data_dynamic(
//     Path((route_code, direction)): Path<(String, String)>,
//     State(state): State<Arc<AppState>>,
// ) -> Result<Json<BusRouteWithCoordinates>, AppError> {
//     let stop_coordinates = fetch_stop_coordinates(&state.db, &route_code, &direction).await?;
//     let coordinates = request_graphhopper_routes(state.reqwest.clone(), stop_coordinates).await?;
//     Ok(Json(BusRouteWithCoordinates {
//         route_code,
//         direction,
//         coordinates,
//     }))
// }

// pub async fn get_route_data(
//     Path((route_code, direction)): Path<(String, String)>,
//     State(state): State<Arc<AppState>>,
// ) -> Result<Json<BusRouteWithCoordinates>, AppError> {
//     let coordinates = fetch_route_plan(&state.db, &route_code, &direction).await?;

//     let coordinates: Vec<models::Coordinates> = serde_json::from_str(&coordinates)?;
//     Ok(Json(BusRouteWithCoordinates {
//         route_code,
//         direction,
//         coordinates,
//     }))
// }

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
