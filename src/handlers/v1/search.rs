use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use tokio::try_join;

use crate::{
    models::{
        app::{AppError, AppState},
        v1,
    },
    query::SearchQuery,
};

pub async fn get_search(
    Query(query): Query<SearchQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<v1::search::Search>, AppError> {
    let stops_query = sqlx::query_as!(
        v1::stop::BusStop,
        r#"
            SELECT
                id, stop_code AS code, name, lng, lat, physical, province, smart, type AS stop_type, disabled_can_use, city
             FROM
                stops
             WHERE
                name ILIKE '%' || $1 || '%'
                AND city = $2
            LIMIT 10
        "#,
        query.q,
        query.city.as_str()
    )
    .fetch_all(&state.db);

    let lines_query = sqlx::query_as!(
        v1::line::Line,
        r#"
            SELECT id, code, title AS name, city FROM lines
            WHERE
                (
                    code ILIKE '%' || $1 || '%'
                    OR TO_TSVECTOR( title ) @@ websearch_to_tsquery('' || $1 || ':*')
                )
                AND city = $2
            GROUP BY
                code, title, city, id
            ORDER BY
                code
            LIMIT 40
        "#,
        query.q,
        query.city.as_str()
    )
    .fetch_all(&state.db);

    let (stops, lines) = try_join!(stops_query, lines_query)?;

    Ok(Json(v1::search::Search { stops, lines }))
}
