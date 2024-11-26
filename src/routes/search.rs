use std::sync::Arc;

use axum::extract::{Query, State};
use serde::Deserialize;

use crate::models::app::AppState;
use crate::models::bus::{ BusStop };

#[derive(Deserialize)]
pub struct Search {
    q: String
}

pub struct SearchResponse {
    busses: Vec<BusStop>
}

pub async fn search(
    Query(Search { q }): Query<Search>,
    State(state): State<Arc<AppState>>,
) {
    let stops = sqlx::query_as!(
        BusStop,
        r#" SELECT * FROM bus_route_stops WHERE hatkodu LIKE '%' || $1 || '%' "#,
        q
    )
        .fetch_one(&state.db);

    // let results = sqlx::query!(
    //     r#"
    //         SELECT *
    //         FROM bus_route_stops RIGHT JOIN route_travel_plan
    //         ON route_travel_plan.hatkodu LIKE '%' || $1 || '%'
    //             OR bus_route_stops.hatkodu LIKE '%' || $1 || '%'
    //     "#,
    //     q
    // );
}
