use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    Json,
};
use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;

use crate::{
    models::app::{AppError, AppState},
    models::routes::Route,
};

#[io_cached(
    map_error = r##"|e| anyhow!("{}", e) "##,
    ty = "AsyncRedisCache<String, Vec<Route>>",
    convert = "{line_code.clone()}",
    create = r##" {
        AsyncRedisCache::new("routes", 600)
            .build()
            .await
            .expect("error building redis cache")
    } "##
)]
pub async fn routes_cached(
    line_code: String,
    state: Arc<AppState>,
) -> Result<Vec<Route>, AppError> {
    let routes = sqlx::query_as!(
        Route,
        r#"
            SELECT 
                routes.id,
                agency_id,
                route_short_name,
                route_long_name,
                route_type,
                route_desc,
                routes.route_code,
                route_paths.route_path
            FROM 
                routes
                LEFT JOIN route_paths on route_paths.route_code = routes.route_code
            WHERE
                route_short_name = $1
        "#,
        line_code
    )
    .fetch_all(&state.db)
    .await?;

    Ok(routes)
}

pub async fn routes(
    Path(line_code): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Route>>, AppError> {
    routes_cached(line_code, state).await.map(Json)
}
