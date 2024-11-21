use anyhow::{Ok, Result};
use reqwest::header::{HeaderMap, HeaderName, CONTENT_TYPE};
use serde::de::DeserializeOwned;

use crate::{
    csv_parse::read_csv_from_string,
    xml_parse::{BusRouteStop, Envelope},
};

pub fn get_body(key: &str, outer_key: &str, content: &str) -> String {
    format!(
        r#"
        <soap:Envelope
            xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
            <soap:Body>
                <{outer_key}
                    xmlns="http://tempuri.org/">
                    <{key}>{content}</{key}>
                </{outer_key}>
            </soap:Body>
        </soap:Envelope>
        "#,
        key = key,
        outer_key = outer_key,
        content = content
    )
}
pub async fn request_soap(client: reqwest::Client, hat_kodu: &str) -> Result<Vec<BusRouteStop>> {
    let url = "https://api.ibb.gov.tr/iett/ibb/ibb.asmx?wsdl";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "text/xml".parse().unwrap());
    headers.insert(
        "SOAPAction".parse::<HeaderName>().unwrap(),
        "\"http://tempuri.org/DurakDetay_GYY\"".parse().unwrap(),
    );

    let body = get_body("hat_kodu", "DurakDetay_GYY", hat_kodu);

    let res = client.post(url).headers(headers).body(body).send().await?;
    let response_string = &res.text().await?;
    let envelope: Envelope = serde_xml_rs::from_str(response_string)?;

    let bus_route_stops = envelope
        .body
        .response
        .result
        .dataset
        .tables
        .unwrap_or_default();
    Ok(bus_route_stops)
}

pub async fn request_csv<T: DeserializeOwned>(
    client: reqwest::Client,
    url: &str,
) -> Result<Vec<T>> {
    let res = client.get(url).send().await?;
    let response_string = &res.text().await?;
    Ok(read_csv_from_string(response_string)?)
}
