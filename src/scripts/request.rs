use anyhow::{Ok, Result};
use reqwest::header::{HeaderMap, HeaderName, CONTENT_TYPE};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{csv_parse::read_csv_from_string, models::Coordinates, xml_parse::UnwrapSoap};

pub fn get_body(key: &str, soap_method: &str, content: &str) -> String {
    format!(
        r#"
        <soap:Envelope
            xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
            <soap:Body>
                <{soap_method}
                    xmlns="http://tempuri.org/">
                    <{key}>{content}</{key}>
                </{soap_method}>
            </soap:Body>
        </soap:Envelope>
        "#,
        key = key,
        soap_method = soap_method,
        content = content
    )
}

pub async fn request_csv<T: DeserializeOwned>(
    client: reqwest::Client,
    url: &str,
) -> Result<Vec<T>> {
    let res = client.get(url).send().await?;
    let response_string = &res.text().await?;
    Ok(read_csv_from_string(response_string)?)
}

#[derive(Serialize)]
struct GraphhopperPostBody {
    points: Vec<Vec<f64>>,
    profile: String,
    instructions: bool,
    points_encoded: bool,
    curbside_strictness: String,
}

#[derive(Deserialize)]
pub struct Points {
    pub coordinates: Vec<Vec<f64>>,
}

#[derive(Deserialize)]
pub struct Path {
    pub points: Points,
}

#[derive(Deserialize)]
pub struct GraphhopperResponseBody {
    pub paths: Vec<Path>,
}

/// https://github.com/graphhopper/graphhopper/blob/master/docs/web/api-doc.md
pub async fn request_graphhopper_routes(
    client: reqwest::Client,
    coordinates: Vec<Coordinates>,
) -> Result<Vec<Coordinates>> {
    let points = coordinates
        .into_iter()
        .map(|coord| vec![coord.x, coord.y])
        .collect();
    let body = GraphhopperPostBody {
        points,
        profile: "car".to_string(),
        instructions: false,
        points_encoded: false,
        curbside_strictness: "soft".to_string(),
    };

    let url = "http://localhost:8989/route";
    let response_body: GraphhopperResponseBody =
        client.post(url).json(&body).send().await?.json().await?;

    let coordinates = response_body.paths[0]
        .points
        .coordinates
        .iter()
        .map(|coordinate_vec| Coordinates {
            x: coordinate_vec[0],
            y: coordinate_vec[1],
        })
        .collect();
    Ok(coordinates)
}
