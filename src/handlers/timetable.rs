use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    database::city::City,
    models::{
        app::{AppError, AppState},
        timetable::Timetable,
    },
    query::CityQuery,
};

pub async fn timetable_cached(
    route_code: String,
    city: City,
    state: Arc<AppState>,
) -> Result<Timetable, AppError> {
    let timetable = sqlx::query_as!(
        Timetable,
        r#"
            SELECT
                routes.route_long_name,
                timetable.route_code,
                timetable.city,
                sunday,
                monday,
                tuesday,
                wednesday,
                thursday,
                friday,
                saturday
            FROM
                timetable
                LEFT JOIN routes ON routes.route_code = timetable.route_code
            WHERE
                timetable.route_code = $1
                AND timetable.city = $2
        "#,
        route_code,
        city.as_str()
    )
    .fetch_one(&state.db)
    .await?;

    Ok(timetable)
}

pub async fn timetable(
    Path(route_code): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<CityQuery>,
) -> Result<Json<Timetable>, AppError> {
    timetable_cached(route_code, query.city, state)
        .await
        .map(Json)
}
