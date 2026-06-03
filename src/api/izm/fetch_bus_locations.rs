use crate::models::bus_location::BusLocation;
use crate::models::izm::bus_location::BusLocationResponse;

pub async fn fetch_bus_locations(
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

    let location_response = response_izmir.json::<BusLocationResponse>().await?;

    let bus_locations: Vec<BusLocation> = location_response
        .bus_locations
        .into_iter()
        .map(|loc| BusLocation::from_izm_bus_location(loc, line_code))
        .collect();

    Ok(bus_locations)
}
