use std::{fs, sync::Arc};

use otobusum_anlik_backend::{csv_parse::{read_csv_from_string, Route}, db::get_db_connection, request::request_csv};
use sqlx::QueryBuilder;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client = reqwest::Client::new();
    let db_conn = Arc::new(get_db_connection().await.unwrap());

    let file = fs::read_to_string("./routes.csv").unwrap();
    let routes: Vec<Route> = read_csv_from_string(&file).unwrap();

    // let routes_url = "https://data.ibb.gov.tr/dataset/8540e256-6df5-4719-85bc-e64e91508ede/resource/46dbe388-c8c2-45c4-ac72-c06953de56a2/download/routes.csv";
    // let routes: Vec<Route> = request_csv(client.clone(), routes_url).await.unwrap();

    let mut query_builder =
        QueryBuilder::new("INSERT INTO routes (id, agencyid, routeshortname, routelongname, routetype, routedesc, routecode)");

    query_builder.push_values(routes, |mut b, new_route| {
        b.push_bind(new_route.route_id)
            .push_bind(new_route.agency_id)
            .push_bind(new_route.route_short_name)
            .push_bind(new_route.route_long_name)
            .push_bind(new_route.route_type)
            .push_bind(new_route.route_desc)
            .push_bind(new_route.route_code);
    });

    let query = query_builder.build();
    query.execute(&*db_conn).await.unwrap();
}
