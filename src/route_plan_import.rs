use std::{collections::HashMap, sync::Arc};

use otobusum_anlik_backend::{
    database::{
        fetch_all_stop_coordinates, get_db_connection, insert_route_plan, BusStopRow, Coordinates,
    },
    request::request_graphhopper_routes,
    BusRouteWithCoordinates,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client = reqwest::Client::new();
    let db_conn = Arc::new(get_db_connection().await.unwrap());

    let all_route_rows = fetch_all_stop_coordinates(&db_conn).await.unwrap();
    let grouped_routes = group_routes(all_route_rows);
    let mut failed_routes = Vec::new();
    for route in grouped_routes {
        println!(
            "Requesting and inserting route plan for {}",
            route.route_code
        );
        let Ok(paths) = request_graphhopper_routes(client.clone(), route.coordinates).await else {
            println!("Failed to request route plan for {}", route.route_code);
            failed_routes.push(route.route_code.clone());
            continue;
        };
        let path_string = serde_json::to_string(&paths).unwrap();
        insert_route_plan(&db_conn, &route.route_code, &route.direction, &path_string)
            .await
            .unwrap();
    }

    dbg!(failed_routes);
}

/// Grouping routes by key. We aren't doing any sorting for the coordinates because the data comes
/// sorted from database
fn group_routes(route_rows: Vec<BusStopRow>) -> Vec<BusRouteWithCoordinates> {
    let mut bus_routes: HashMap<(String, String), Vec<Coordinates>> = HashMap::new();
    for row in route_rows {
        let route = bus_routes
            .entry((row.route_code, row.direction))
            .or_default();
        route.push(Coordinates { x: row.x, y: row.y });
    }
    bus_routes
        .into_iter()
        .map(
            |((route_code, direction), coordinates)| BusRouteWithCoordinates {
                route_code,
                direction,
                coordinates,
            },
        )
        .collect()
}
