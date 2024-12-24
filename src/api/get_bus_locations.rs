use crate::models::location::{BusLocation, BusLocationResponse, BusLocationResponseIzmir};

fn get_body(key_outer: &str, key: &str, value: &str) -> String {
    format!(
        r#"
        <soap:Envelope
            xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
                <soap:Body>
                    <{key_outer}
                        xmlns="http://tempuri.org/">
                        <{key}>{value}</{key}>
                    </{key_outer}>
                </soap:Body>
            </soap:Envelope>
        "#
    )
}

pub async fn get_bus_locations(
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
    let response_parsed: BusLocationResponse = serde_xml_rs::from_str(&content)?;
    let inner_content = response_parsed.content.content.content;

    let bus_locations = serde_json::from_str(&inner_content)?;
    Ok(bus_locations)
}

pub async fn get_bus_locations_izmir(
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

    let location_response = response_izmir.json::<BusLocationResponseIzmir>().await?;

    let bus_locations: Vec<BusLocation> = location_response
        .bus_locations
        .into_iter()
        .map(BusLocation::from)
        .collect();

    Ok(bus_locations)
}
