use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{Path, Query, State},
    Json,
};

use cached::macros::concurrent_cached;
use cached::time::Duration;
use cached::AsyncRedisCache;

use crate::{
    database::city::City,
    models::{
        app::{AppError, AppState},
        routes::Route,
        v1::route::RouteV1,
    },
    query::CityQuery,
};

#[concurrent_cached(
    map_error = r##"|e| anyhow!("{}", e) "##,
    ty = "AsyncRedisCache<String, Vec<Route>>",
    convert = r#"{ format!("{}{:?}", line_code, city) }"#,
    create = r##" {
        AsyncRedisCache::new("routes", Duration::from_secs(600))
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn routes_cached(
    line_code: String,
    city: City,
    state: Arc<AppState>,
) -> Result<Vec<Route>, AppError> {
    let routes = sqlx::query_as!(
        Route,
        r#"
            SELECT 
                routes.id,
                agency_id,
                code,
                title,
                type,
                description,
                routes.route_code,
                route_paths.path
            FROM 
                routes
                LEFT JOIN route_paths on route_paths.route_code = routes.route_code
                    AND route_paths.city = $2
            WHERE
                code = $1
                AND routes.city = $2
        "#,
        line_code,
        city.as_str()
    )
    .fetch_all(&state.db)
    .await?;

    Ok(routes)
}

pub async fn routes(
    Path(line_code): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<CityQuery>,
) -> Result<Json<Vec<Route>>, AppError> {
    routes_cached(line_code, query.city, state).await.map(Json)
}

pub async fn routes_v1(
    Path(line_code): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<CityQuery>,
) -> Result<Json<Vec<RouteV1>>, AppError> {
    routes_cached(line_code, query.city, state)
        .await
        .map(|v| v.into_iter().map(RouteV1::from).collect())
        .map(Json)
}
