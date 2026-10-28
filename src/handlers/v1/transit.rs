use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use tokio::try_join;

use crate::models;
use crate::models::app::AppError;
use crate::query::QueryAreaQuery;
use crate::AppState;

pub async fn get_transit_nearby(
    State(state): State<Arc<AppState>>,
    Query(query): Query<QueryAreaQuery>,
) -> Result<Json<models::v1::nearby::RoutesAndStopsNearbyResponse>, AppError> {
    const STOPS_RADIUS: f64 = 500.0;
    const ROUTES_RADIUS: f64 = 250.0;

    let stops_query = sqlx::query_as!(
        models::v1::stop::BusStop,
        r#"
            SELECT
                id, stop_code AS code, name, ST_X(location) AS "lng!",
                ST_Y(location) AS "lat!", province, city
            FROM
                stops
            WHERE
                ST_DWithin(location, ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography, $3)
        "#,
        &query.lng,
        &query.lat,
        &STOPS_RADIUS
    )
    .fetch_all(&state.db);

    let routes_query = sqlx::query_as!(
        models::v1::route::Route,
        r#"
            SELECT DISTINCT ON (line_code)
                routes.id,
                agency_id,
                code AS line_code,
                name,
                type,
                description,
                routes.route_code AS code,
                route_paths.path as "path!: _" 
            FROM routes
                LEFT JOIN route_paths
                    ON route_paths.route_code = routes.route_code
                    AND route_paths.city = $3
                
            WHERE 
                ST_DWithin(
                    route_paths.path::geography,
                    ST_SetSRID(
                        ST_MakePoint($1, $2),
                        4326
                    )::geography,
                    $4
                )
                AND routes.city = $3
        "#,
        query.lng,
        query.lat,
        query.city.as_str(),
        ROUTES_RADIUS,
    )
    .fetch_all(&state.db);

    let (stops, routes) = try_join!(stops_query, routes_query)?;

    Ok(Json(models::v1::nearby::RoutesAndStopsNearbyResponse {
        stops,
        routes,
        routes_radius: ROUTES_RADIUS,
        stops_radius: STOPS_RADIUS,
    }))
}
