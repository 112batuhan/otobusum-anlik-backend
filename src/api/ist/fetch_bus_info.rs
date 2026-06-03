use cached::cached;

use crate::models::ist::bus_info::{BusInfo, BusInfoResponse};

use crate::api::ist::get_opendata_xml_body;

#[cached(
    ttl = 60,
    key = "String",
    convert = r#"{ "bus_infos".to_string() }"#,
    result = true
)]
pub async fn fetch_bus_infos(client: &reqwest::Client) -> anyhow::Result<Vec<BusInfo>> {
    let body = get_opendata_xml_body("GetFiloAracKonum_json", "KapiNo", "");

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

    let content = quick_xml::de::from_str::<BusInfoResponse>(&response.text().await?)?;

    Ok(serde_json::from_str(&content.content.content.content)?)
}
