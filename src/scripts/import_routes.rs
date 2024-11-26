use std::sync::Arc;

use otobusum_anlik_backend::{csv_parse::Route, db::get_db_connection, request::request_csv};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let db_conn = Arc::new(get_db_connection().await.unwrap());

    let routes_url = "https://data.ibb.gov.tr/dataset/8540e256-6df5-4719-85bc-e64e91508ede/resource/46dbe388-c8c2-45c4-ac72-c06953de56a2/download/routes.csv";
    let routes: Vec<Route> = request_csv(client.clone(), routes_url).await.unwrap();
}
