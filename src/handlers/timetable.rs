use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;

use crate::{
    database::city::City,
    models::{
        app::{AppError, AppState}, routes::Direction, timetable::Timetable
    },
    query::default_city,
};

pub async fn timetable_cached(
    line_code: String,
    city: City,
    direction: Direction,
    state: Arc<AppState>,
) -> Result<Timetable, AppError> {
    let timetable = sqlx::query_as!(
        Timetable,
        r#"
           SELECT
                route_short_name,
                array_remove(array_agg(sunday), null) as sunday,
                array_remove(array_agg(monday), null) as monday,
                array_remove(array_agg(tuesday), null) as tuesday,
                array_remove(array_agg(wednesday), null) as wednesday,
                array_remove(array_agg(thursday), null) as thursday,
                array_remove(array_agg(friday), null) as friday,
                array_remove(array_agg(saturday), null) as saturday
            FROM
                (
                    SELECT
                        timetable.route_code,
                        timetable.city,
                        routes.route_short_name,
                        unnest(sunday) as sunday,
                        unnest(monday) as monday,
                        unnest(tuesday) as tuesday,
                        unnest(wednesday) as wednesday,
                        unnest(thursday) as thursday,
                        unnest(friday) as friday,
                        unnest(saturday) as saturday
                    FROM
                        routes
                        RIGHT JOIN timetable ON routes.route_code = timetable.route_code
                    WHERE
                        route_short_name = $1
                        AND routes.city = $2
                        AND routes.route_code LIKE '%_' || $3 || '_%'
                )
            GROUP BY
                route_short_name
        "#,
        line_code,
        city.as_str(),
        direction.as_str()
    )
    .fetch_one(&state.db)
    .await?;

    Ok(timetable)
}

#[derive(Deserialize, Debug)]
pub struct TimetableQuery {
    #[serde(default = "default_city")]
    pub city: City,
    pub direction: Direction
}

pub async fn timetable(
    Path(line_code): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<TimetableQuery>,
) -> Result<Json<Timetable>, AppError> {
    timetable_cached(line_code, query.city, query.direction, state)
        .await
        .map(Json)
}
