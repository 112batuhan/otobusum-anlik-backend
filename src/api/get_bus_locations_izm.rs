use crate::models::locations::{BusLocation, izm::BusLocationIzmResponse};


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
