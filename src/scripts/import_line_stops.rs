use otobusum_anlik_backend::db::get_db_connection;
use sqlx::{Executor, QueryBuilder};
use std::sync::Arc;

struct LineStops {
    hatkodu: String,
    durakkodu: i32,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_conn = Arc::new(get_db_connection().await.unwrap());

    let all_records = sqlx::query_as!(
        LineStops,
        r#"
            SELECT hatkodu, durakkodu FROM bus_route_stops
        "#
    )
    .fetch_all(&*db_conn)
    .await
    .unwrap();

    const MAX_ROWS_PER_QUERY: usize = 20_000 / 10;

    for values in all_records.chunks(MAX_ROWS_PER_QUERY) {
        let mut query_builder = QueryBuilder::new(
            r#"
            INSERT INTO bus_stops (line_code, stop_code)
        "#,
        );

        query_builder.push_values(values, |mut b, stop| {
            b.push_bind(&stop.hatkodu).push_bind(&stop.durakkodu);
        });

        let query = query_builder.build();
        db_conn.execute(query).await.unwrap();
    }
}
