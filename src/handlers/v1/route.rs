use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    models::{
        app::{AppError, AppState},
        v1,
    },
    query::{CityQuery, LineStopsQuery},
};

pub async fn get_route_stops(
    Path(line_code): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<LineStopsQuery>,
) -> Result<Json<Vec<v1::stop::BusStop>>, AppError> {
    let stops = sqlx::query_as!(
        v1::stop::BusStop,
        r#"
            SELECT
                stops.id,
                stops.stop_code AS code,
                stops.name,
                stops.lng,
                stops.lat,
                stops.province,
                stops.city
            FROM
                line_stops
                RIGHT JOIN stops ON stops.stop_code = line_stops.stop_code
            WHERE
                line_stops.line_code = $1
                AND line_stops.city = $3
                AND line_stops.route_code = $1 || '_' || $2 || '_D0'
            ORDER BY
                line_stops.stop_order
        "#,
        line_code,
        query.direction.as_str(),
        query.city.as_str()
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(stops))
}

pub async fn get_routes(
    Path(line_code): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<CityQuery>,
) -> Result<Json<Vec<v1::route::Route>>, AppError> {
    let routes = sqlx::query_as!(
        v1::route::Route,
        r#"
            SELECT 
                routes.id,
                agency_id,
                code AS line_code,
                name,
                type,
                description,
                routes.route_code AS code,
                route_paths.path as "path!: _" 
            FROM 
                routes
                LEFT JOIN route_paths ON route_paths.route_code = routes.route_code
                    AND route_paths.city = $2
            WHERE
                code = $1
                AND routes.city = $2
        "#,
        line_code,
        query.city.as_str()
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(routes))
}
