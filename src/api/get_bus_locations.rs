use crate::models::locations::{
    ist::{BusLocationIst, BusLocationIstResponse},
    izm::BusLocationIzmResponse,
    BusLocation,
};
use super::get_body;

pub async fn get_bus_locations_ist(
    client: &reqwest::Client,
    line_code: &str,
) -> anyhow::Result<Vec<BusLocation>> {
    let body = get_body("GetHatOtoKonum_json", "HatKodu", line_code);

    let response = client
        .post("https://api.ibb.gov.tr/iett/FiloDurum/SeferGerceklesme.asmx")
        .header("Content-Type", "text/xml; charset=UTF-8")
        .header("SOAPAction", r#""http://tempuri.org/GetHatOtoKonum_json""#)
        .body(body)
        .send()
        .await?;

    let content = response.text().await?;
    let response_parsed = serde_xml_rs::from_str::<BusLocationIstResponse>(&content)?;
    let inner_content = response_parsed.content.content.content;

    let bus_locations = serde_json::from_str::<Vec<BusLocationIst>>(&inner_content)?
        .into_iter()
        .map(BusLocation::from)
        .collect();

    Ok(bus_locations)
}

pub async fn get_bus_locations_izm(
    client: &reqwest::Client,
    line_code: &str,
) -> anyhow::Result<Vec<BusLocation>> {
    let response_izmir = client
        .get(format!(
            "https://openapi.izmir.bel.tr/api/iztek/hatotobuskonumlari/{line_code}"
        ))
        .header("Content-Type", "application/json; charset=utf-8")
        .send()
        .await?;

    let location_response = response_izmir.json::<BusLocationIzmResponse>().await?;

    let bus_locations: Vec<BusLocation> = location_response
        .bus_locations
        .into_iter()
        .map(|loc| BusLocation::from_bus_location_izm(loc, line_code))
        .collect();

    Ok(bus_locations)
}
