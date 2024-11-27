use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::database::route::Route;
use crate::models::app::{AppError, AppState};
use crate::models::bus::BusStop;

#[derive(Deserialize)]
pub struct Search {
    q: String
}

#[derive(Serialize)]
pub struct SearchResponse {
    stops: Vec<BusStop>,
    routes: Vec<Route>
}

pub async fn search(
    Query(Search { q }): Query<Search>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<SearchResponse>, AppError> {
    let stops = sqlx::query_as!(
        BusStop,
        r#"
            SELECT * FROM bus_route_stops WHERE hatkodu LIKE '%' || $1 || '%'
            LIMIT 10
        "#,
        q
    )
        .fetch_all(&state.db)
        .await?;

    let routes = sqlx::query_as!(
        Route,
        r#"
            SELECT * FROM routes WHERE to_tsvector(route_short_name) @@ to_tsquery('' || $1 || ':*')
            LIMIT 10
        "#,
        Some(q)
    )
        .fetch_all(&state.db)
        .await?;

    Ok(Json(SearchResponse { stops, routes }))
}
