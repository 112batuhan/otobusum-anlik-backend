mod models;
mod utils;

use otobusum_anlik_backend::db::get_db_connection;
use std::sync::Arc;

use models::stop::{BusStopSoap, BusStopsResponse};
use sqlx::QueryBuilder;
use utils::soap::request_soap;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_conn = Arc::new(get_db_connection().await.unwrap());
    let client = reqwest::Client::new();

    let stops = request_soap::<BusStopsResponse, String>(
        client,
        "https://api.ibb.gov.tr/iett/UlasimAnaVeri/HatDurakGuzergah.asmx?wsdl",
        "GetDurak_json",
        None,
    )
    .await
    .unwrap();

    let parsed = serde_json::from_str::<Vec<BusStopSoap>>(&stops).unwrap();

    const MAX_ROWS_PER_QUERY: usize = 50_000 / 10;

    for values in parsed.chunks(MAX_ROWS_PER_QUERY) {
        let mut query_builder = QueryBuilder::new(
            r#"
            INSERT INTO stops (
                stop_code,
                stop_name,
                x_coord,
                y_coord,
                province,
                direction,
                smart,
                physical,
                stop_type,
                disabled_can_use
            )"#,
        );

        query_builder.push_values(values, |mut b, stop| {
            b.push_bind(&stop.stop_code)
                .push_bind(&stop.stop_name)
                .push_bind(&stop.coord.x)
                .push_bind(&stop.coord.y)
                .push_bind(&stop.province)
                .push_bind(&stop.direction)
                .push_bind(&stop.smart)
                .push_bind(&stop.physical)
                .push_bind(&stop.stop_type)
                .push_bind(&stop.disabled_can_use);
        });

        let query = query_builder.build();
        query.execute(&*db_conn).await.unwrap();
    }
}
