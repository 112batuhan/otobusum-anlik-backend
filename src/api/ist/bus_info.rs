use anyhow::Result;
use cached::concurrent_cached;
use serde_json::json;
use tracing::warn;
use std::time::Duration;

use crate::models::ist::bus_info::{
    BusInfoOpenData, BusInfoOpenDataResponse, BusInfoOtobusumNerede,
};

use crate::api::ist::{get_opendata_xml_body, otobusumnerede};
use crate::models::v1::bus::BusInfo;

pub async fn fetch_bus_infos_opendata(client: &reqwest::Client) -> Result<Vec<BusInfoOpenData>> {
    let body = get_opendata_xml_body("GetFiloAracKonum_json", "KapiNo", "");
    println!("test");

    let response = client
        .post("https://api.ibb.gov.tr/iett/FiloDurum/SeferGerceklesme.asmx")
        .header("Content-Type", "text/xml; charset=UTF-8")
        .header(
            "SOAPAction",
            r#""http://tempuri.org/GetFiloAracKonum_json""#,
        )
        .body(body)
        .send()
        .await?;

    println!("test1");

    let content = quick_xml::de::from_str::<BusInfoOpenDataResponse>(&response.text().await?)?;

    Ok(serde_json::from_str(&content.content.content.content)?)
}

pub async fn fetch_bus_info_otobusumnerede(
    client: &reqwest::Client,
    door_no: &str,
) -> Result<Vec<BusInfoOtobusumNerede>> {
    let body = json!({
        "alias": "mainGetBusLocation_basic",
        "data": {
            "AKYOLBILYENI.K_ARAC.KAPINUMARASI": door_no
        }
    });

    otobusumnerede::fetch_otobusumnerede(client, &body).await
}

#[concurrent_cached(
    ttl = "Duration::from_secs(60)",
    convert = r#"{ format!("{}_bus_infos", door_no) }"#,
    key = "String",
    redis = true
)]
pub async fn fetch_bus_info(client: &reqwest::Client, door_no: &str) -> Result<Option<BusInfo>> {
    let info = match fetch_bus_infos_opendata(client).await {
        Ok(infos) => infos
            .into_iter()
            .find(|b| b.door_no == door_no)
            .map(BusInfo::from),
        Err(error) => {
            warn!("Open Data request to fetch bus infos failed for bus {door_no}, falling back to Otobusum Nerede: {error:?}");

            fetch_bus_info_otobusumnerede(client, door_no)
            .await?
            .into_iter()
            .find(|b| b.door_no == door_no)
            .map(BusInfo::from)
        }
    };

    Ok(info)
}
