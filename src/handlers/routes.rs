use std::sync::Arc;

use axum::{extract::{Path, State}, Json};

use crate::{database::route::Route, models::app::{AppError, AppState}};

pub async fn routes(
    Path(line_code): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Route>>, AppError> {
    let routes = sqlx::query_as!(
        Route,
        r#"
            SELECT * FROM routes
            WHERE route_short_name = $1
        "#,
        line_code
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(
        routes
    ))
}
