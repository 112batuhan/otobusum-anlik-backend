use cached::proc_macro::cached;

use super::get_body;
use crate::models::locations::ist::{BusInfoIst, BusInfoIstResponse};

pub async fn get_bus_infos(client: &reqwest::Client) -> anyhow::Result<Vec<BusInfoIst>> {
    let body = get_body("GetFiloAracKonum_json", "KapiNo", "");

    let response = client
        .post("https://api.ibb.gov.tr/iett/FiloDurum/SeferGerceklesme.asmx")
        .header("Content-Type", "text/xml; charset=UTF-8")
        .header("SOAPAction", r#""http://tempuri.org/GetFiloAracKonum_json""#)
        .body(body)
        .send()
        .await?;

    let content = response.text().await?;
    let response_parsed = serde_xml_rs::from_str::<BusInfoIstResponse>(&content)?;
    let inner_content = response_parsed.content.content.content;

    let content = serde_json::from_str::<BusInfoIstResponse>(&inner_content)?
        .content
        .content
        .content;


    let infos = serde_json::from_str::<Vec<BusInfoIst>>(&content)?;
    Ok(infos)
}

#[cached(
    time = 60,
    key = "bool",
    convert = r#"{ true }"#
)]
pub async fn get_bus_infos_cached(client: &reqwest::Client) -> Vec<BusInfoIst> {
    get_bus_infos(client).await.unwrap_or(vec![])
}
