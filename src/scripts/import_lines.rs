mod utils;
mod models;

use std::{fs, sync::Arc};

use models::route::Route;
use otobusum_anlik_backend::db::get_db_connection;
use sqlx::{Postgres, QueryBuilder};
use utils::csv_parse::read_csv_from_string;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_conn = Arc::new(get_db_connection().await.unwrap());
    
    let file = fs::read_to_string("./routes.csv").unwrap();
    let routes: Vec<Route> = read_csv_from_string(&file).unwrap();

    let mut query_builder: QueryBuilder<'_, Postgres> =
    QueryBuilder::new("INSERT INTO lines (code, title)");

    let routes_filtered = routes
        .into_iter()
        .filter(|x| {
            let Some(ref code) = x.route_code else {
                return false;
            };

            if !code.ends_with("G_D0") {
                return false;
            }

            return true;
        });

    query_builder.push_values(routes_filtered, |mut b, new_route| {
        b.push_bind(new_route.route_short_name.unwrap())
            .push_bind(new_route.route_long_name.unwrap());
    });

    let query = query_builder.build();
    query.execute(&*db_conn).await.unwrap();
}
