use std::sync::Arc;

use iett_stops_with_busses::database::{fetch_all_stop_coordinates, get_db_connection};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client = reqwest::Client::new();
    let db_conn = Arc::new(get_db_connection().await.unwrap());

    let all_routes = fetch_all_stop_coordinates(&db_conn).await.unwrap();
}
