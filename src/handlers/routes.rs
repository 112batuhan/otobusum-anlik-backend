use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    database::route::Route,
    models::app::{AppError, AppState},
};

pub async fn routes(
    Path(line_code): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Route>>, AppError> {
    let routes = sqlx::query_as!(
        Route,
        r#"
            SELECT 
                id,
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

    Ok(Json(routes))
}
