use anyhow::Result;
use cached::cached;
use serde_json::json;
use std::time::Duration;

use crate::{api::ist::otobusumnerede, models::ist::stop_arrival::StopArrival};

#[cached(
    ttl = "Duration::from_secs(60)",
    key = "String",
    convert = r#"{ format!("{}_stop_arrivals", stop_code) }"#
)]
pub async fn fetch_stop_arrivals(
    client: &reqwest::Client,
    stop_code: &str,
) -> Result<Vec<StopArrival>> {
    let body = json!({
        "alias": "ybs",
        "data": {
            "data": {
                "password": "n1!t8c7M1",
                "username": "netuce"
            },
            "method": "POST",
            "path": [
                "real-time-information",
                "stop-arrivals",
                stop_code
            ]
        }
    });

    otobusumnerede::fetch_otobusumnerede(client, &body).await
}
