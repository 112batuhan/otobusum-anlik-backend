use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{info, warn};

use crate::{
    api::ist::{get_opendata_xml_body, otobusumnerede},
    models::ist::bus_location::{
        BusLocationOpenData, BusLocationOpenDataResponse, BusLocationOtobusumNerede,
    },
};

use crate::models::bus::BusLocation;

#[derive(Serialize, Deserialize, Clone)]
struct IstOtobusumNeredeSearchResponse {
    #[serde(alias = "HAT_ID")]
    line_id: u32,
    #[serde(alias = "HAT_HAT_KODU")]
    line_code: String,
}

async fn fetch_bus_locations_opendata(
    client: &reqwest::Client,
    line_code: &str,
) -> Result<Vec<BusLocationOpenData>> {
    let body = get_opendata_xml_body("GetHatOtoKonum_json", "HatKodu", line_code);

    let response = client
        .post("https://api.ibb.gov.tr/iett/FiloDurum/SeferGerceklesme.asmx")
        .header("Content-Type", "text/xml; charset=UTF-8")
        .header("SOAPAction", r#""http://tempuri.org/GetHatOtoKonum_json""#)
        .body(body)
        .send()
        .await?;

    let content = quick_xml::de::from_str::<BusLocationOpenDataResponse>(&response.text().await?)?;

    Ok(serde_json::from_str(&content.content.content.content)?)
}

async fn fetch_bus_locations_otobusumnerede(
    client: &reqwest::Client,
    line_code: &str,
) -> Result<Vec<BusLocationOtobusumNerede>> {
    let search_body = json!({
        "alias": "ybs",
        "data": {
            "HATYONETIM.HAT.HAT_KODU": line_code
        }
    });

    info!("getting search results using otobusum nerede internal api");

    let search_response: Vec<IstOtobusumNeredeSearchResponse> =
        otobusumnerede::fetch_otobusumnerede(client, &search_body).await?;

    let id = search_response
        .iter()
        .find(|i| i.line_code.to_lowercase() == line_code.to_lowercase())
        .ok_or(anyhow!(
            "line id is not found in search results using otobusum nerede internal api"
        ))?
        .line_id;

    info!("line id {:?} found in search results", id);

    let location_body = json!({
        "alias": "ybs",
        "data": {
            "data": {
                "password": "n1!t8c7M1",
                "username": "netuce"
            },
            "method": "POST",
            "path": [
                "real-time-information",
                "point-passing",
                id
            ]
        }
    });

    otobusumnerede::fetch_otobusumnerede(client, &location_body).await
}

pub async fn fetch_bus_locations(
    client: &reqwest::Client,
    line_code: &str,
) -> Result<Vec<BusLocation>> {
    info!("getting {line_code} bus locations using open data");

    let locations = match fetch_bus_locations_opendata(client, line_code).await {
        Ok(response) => response.into_iter().map(BusLocation::from).collect(),
        Err(error) => {
            warn!("Open Data request failed for {line_code}, falling back to Otobusum Nerede: {error:?}");

            fetch_bus_locations_otobusumnerede(client, line_code)
                .await?
                .into_iter()
                .map(BusLocation::from)
                .collect()
        }
    };

    Ok(locations)
}
