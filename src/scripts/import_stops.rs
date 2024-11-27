mod utils;

use std::sync::Arc;

use axum::http::{HeaderMap, HeaderName};
use reqwest::header::CONTENT_TYPE;
use serde::de::DeserializeOwned;
use anyhow::Result;

use otobusum_anlik_backend::db::get_db_connection;
use utils::soap::{get_body, request_soap, UnwrapSoap};



#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_conn = Arc::new(get_db_connection().await.unwrap());
    let client = reqwest::Client::new();

    // let content = request_soap(
    //     client,
    //     "https://api.ibb.gov.tr/iett/UlasimAnaVeri/HatDurakGuzergah.asmx?wsdl",
    //     "GetDurak_json",
    //     None
    // ).await;
}
