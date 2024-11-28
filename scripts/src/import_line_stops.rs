// should implement later by making requests to ibb.
// used to get line stops from a table

#[tokio::main]
async fn main() {
    // dotenvy::dotenv().ok();

    // let db_conn = Arc::new(get_db_connection().await.unwrap());

    // let all_records = sqlx::query_as!(
    //     LineStop,
    //     r#"
    //         SELECT hatkodu, durakkodu FROM bus_route_stops
    //     "#
    // )
    // .fetch_all(&*db_conn)
    // .await
    // .unwrap();

    // const MAX_ROWS_PER_QUERY: usize = 20_000 / 10;

    // for values in all_records.chunks(MAX_ROWS_PER_QUERY) {
    //     let mut query_builder = QueryBuilder::new(
    //         r#"
    //         INSERT INTO line_stops (line_code, stop_code)
    //     "#,
    //     );

    //     query_builder.push_values(values, |mut b, stop| {
    //         b.push_bind(&stop.hatkodu).push_bind(&stop.durakkodu);
    //     });

    //     let query = query_builder.build();
    //     db_conn.execute(query).await.unwrap();
    // }
}
