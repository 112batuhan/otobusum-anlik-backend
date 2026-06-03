use std::collections::HashMap;

use anyhow::anyhow;
use cached::macros::cached;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{info, warn};

use crate::{
    api::ist::get_opendata_xml_body,
    models::ist::{
        bus_location::{
            BusLocation as BusLocationIst, BusLocationOpenData, BusLocationOpenDataResponse,
            BusLocationOtobusumNerede,
        },
        tokens::TokensResponse,
    },
};

use crate::models::bus_location::BusLocation;

#[derive(Serialize, Deserialize, Clone)]
struct IstOtobusumNeredeSearchResponse {
    #[serde(alias = "HAT_ID")]
    line_id: u32,
    #[serde(alias = "HAT_HAT_KODU")]
    line_code: String,
}

pub async fn fetch_bus_locations_opendata(
    client: &reqwest::Client,
    line_code: &str,
) -> anyhow::Result<Vec<BusLocationOpenData>> {
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

#[cached(
    ttl = 3500,
    key = "String",
    convert = r#"{ "credentials".to_string() }"#,
    result = true
)]
pub async fn fetch_otobusumnerede_credentials(
    // pub async fn get_ist_otobusum_nerede_credentials(
    client: &reqwest::Client,
    headers: &reqwest::header::HeaderMap,
) -> anyhow::Result<TokensResponse> {
    let mut auth_body = HashMap::new();

    auth_body.insert("client_id", std::env::var("IBB_CLIENT_ID").unwrap());
    auth_body.insert("client_secret", std::env::var("IBB_CLIENT_SECRET").unwrap());
    auth_body.insert("grant_type", "client_credentials".to_string());
    auth_body.insert("scope", std::env::var("IBB_CLIENT_SCOPE").unwrap());

    info!("getting credentials for internal api of otobusum nerede");

    Ok(client
        .post("https://ntcapi.iett.istanbul/oauth2/v2/auth")
        .headers(headers.clone())
        .json(&auth_body)
        .send()
        .await?
        .json()
        .await?)
}

pub async fn fetch_bus_locations_otobusumnerede(
    // pub async fn get_bus_locations_ist_otobusum_nerede(
    client: &reqwest::Client,
    line_code: &str,
) -> anyhow::Result<Vec<BusLocationOtobusumNerede>> {
    let mut headers = reqwest::header::HeaderMap::new();

    headers.append("Host", "ntcapi.iett.istanbul".parse().unwrap());
    headers.append(
        "Content-Type",
        "application/json; charset=UTF-8".parse().unwrap(),
    );
    headers.append("Accept-Encoding", "gzip".parse().unwrap());

    let credentials = fetch_otobusumnerede_credentials(client, &headers).await?;

    headers.insert(
        "Authorization",
        format!("Bearer {}", credentials.access_token)
            .parse()
            .unwrap(),
    );

    let search_body = HashMap::from([
        ("alias", json!("mainGetLine_basic_search")),
        ("data", json!({ "HATYONETIM.HAT.HAT_KODU": line_code })),
    ]);

    info!("getting search results using otobusum nerede internal api");

    let search_response = client
        .post("https://ntcapi.iett.istanbul/service")
        .headers(headers.clone())
        .json(&search_body)
        .send()
        .await?
        .json::<Vec<IstOtobusumNeredeSearchResponse>>()
        .await?;

    let id = search_response
        .iter()
        .find(|i| i.line_code.to_lowercase() == line_code.to_lowercase())
        .ok_or(anyhow!(
            "line id is not found in search results using otobusum nerede internal api"
        ))?
        .line_id;

    info!("line id {:?} found in search results", id);

    let location_body = HashMap::from([
        ("alias", json!("ybs")),
        (
            "data",
            json!({
                "data": {
                    "password": "n1!t8c7M1",
                    "username": "netuce"
                },
                "method": "POST",
                "path": [
                    "real-time-information",
                    "point-passing",
                    id.to_string()
                ]
            }),
        ),
    ]);

    let location_response = client
        .post("https://ntcapi.iett.istanbul/service")
        .headers(headers.clone())
        .json(&location_body)
        .send()
        .await?
        .json()
        .await?;

    Ok(location_response)
}

pub async fn fetch_bus_locations(
    client: &reqwest::Client,
    line_code: &str,
) -> anyhow::Result<Vec<BusLocation>> {
    let results = match fetch_bus_locations_opendata(client, line_code).await {
        Ok(response) => BusLocationIst::OpenDataResponse(response),
        Err(error) => {
            warn!("Trying getting locations from open data api has failed, falling back to internal api. {:?}", error);
            BusLocationIst::OtobusumNeredeResponse(
                fetch_bus_locations_otobusumnerede(client, line_code).await?,
            )
        }
    };

    Ok(match results {
        BusLocationIst::OpenDataResponse(op) => op
            .into_iter()
            .map(BusLocation::from)
            .collect::<Vec<BusLocation>>(),
        BusLocationIst::OtobusumNeredeResponse(on) => on
            .into_iter()
            .map(BusLocation::from)
            .collect::<Vec<BusLocation>>(),
    })
}
