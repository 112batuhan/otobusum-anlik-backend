use std::collections::HashSet;
use std::sync::Arc;

use otobusum_anlik_backend::csv_parse::Route;
use otobusum_anlik_backend::db::{
    delete_by_hatkodu, fetch_unique_hatkodus, get_db_connection, insert_bus_route_stop,
};
use otobusum_anlik_backend::models::bus::BusRouteStop;
use otobusum_anlik_backend::request::{request_csv, request_soap};
use otobusum_anlik_backend::xml_parse::{BusRouteMetadata, DurakDetay, HatServisi};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client = reqwest::Client::new();
    let db_conn = Arc::new(get_db_connection().await.unwrap());

    let routes_url = "https://data.ibb.gov.tr/datastore/dump/46dbe388-c8c2-45c4-ac72-c06953de56a2";
    let routes: Vec<Route> = request_csv(client.clone(), routes_url).await.unwrap();
    let bus_route_names: HashSet<String> = routes
        .into_iter()
        .map(|route| route.route_short_name)
        .collect();

    // I didn't go for concurrent requests because it might cause rate limit or blacklist issues
    // We only need to do it once so it's fine to wait for few seconds for every buss
    println!("loaded {} bus routes", bus_route_names.len());

    let skip_bus_route_names = fetch_unique_hatkodus(&db_conn).await.unwrap();
    for name in bus_route_names {
        if skip_bus_route_names.contains(&name) {
            println!("{} exists, skipping", name);
            continue;
        }
        let bus_route_stops =
            request_soap::<DurakDetay, Vec<BusRouteStop>>(client.clone(), "DurakDetay_GYY", &name)
                .await
                .unwrap();
        println!("requested {} from api", name);
        let mut join_set = JoinSet::new();
        for stop in bus_route_stops {
            let db_conn = db_conn.clone();
            join_set.spawn(async move {
                insert_bus_route_stop(&db_conn, stop).await.unwrap();
            });
        }

        join_set.join_all().await;
    }

    let inserted_bus_route_names = fetch_unique_hatkodus(&db_conn).await.unwrap();
    for name in inserted_bus_route_names {
        let bus_route_metadata = request_soap::<HatServisi, Option<BusRouteMetadata>>(
            client.clone(),
            "HatServisi_GYY",
            &name,
        )
        .await
        .unwrap();

        if bus_route_metadata.is_none() {
            println!("Deleting {} because inactive", name);
            delete_by_hatkodu(&db_conn, &name).await.unwrap()
        }
    }
}
