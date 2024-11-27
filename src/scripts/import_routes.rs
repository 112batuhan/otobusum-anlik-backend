use std::{fs, sync::Arc};

use otobusum_anlik_backend::{csv_parse::{read_csv_from_string, Route}, db::get_db_connection};
use anyhow::Result;
use sqlx::{PgPool, QueryBuilder};

/// TODO: detect duplicate inserts, add unique index or change it
pub async fn insert_route_plan(
    pool: &PgPool,
    hatkodu: &str,
    yon: &str,
    coordinate_string: &str,
) -> Result<()> {
    let query = r#"
            INSERT INTO route_travel_plan (
                hatkodu, yon, points
            ) VALUES ($1, $2, $3)
        "#;

    sqlx::query(query)
        .bind(hatkodu)
        .bind(yon)
        .bind(coordinate_string)
        .execute(pool)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // let client = reqwest::Client::new();
    let db_conn = Arc::new(get_db_connection().await.unwrap());

    let file = fs::read_to_string("./routes.csv").unwrap();
    let routes: Vec<Route> = read_csv_from_string(&file).unwrap();

    // let routes_url = "https://data.ibb.gov.tr/dataset/8540e256-6df5-4719-85bc-e64e91508ede/resource/46dbe388-c8c2-45c4-ac72-c06953de56a2/download/routes.csv";
    // let routes: Vec<Route> = request_csv(client.clone(), routes_url).await.unwrap();

    let mut query_builder =
        QueryBuilder::new("INSERT INTO routes (id, agency_id, route_short_name, route_long_name, route_type, route_desc, route_code)");

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
