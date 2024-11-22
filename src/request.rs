use anyhow::{Ok, Result};
use reqwest::header::{HeaderMap, HeaderName, CONTENT_TYPE};
use serde::de::DeserializeOwned;

use crate::{
    csv_parse::read_csv_from_string,
    xml_parse::{BusRouteStop, DurakDetayEnvelope, UnwrapSoap},
};

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
pub async fn request_soap<T: UnwrapSoap<T, R>, R: DeserializeOwned>(
    client: reqwest::Client,
    soap_method: &str,
    hat_kodu: &str,
) -> Result<R>
where
{
    let url = "https://api.ibb.gov.tr/iett/ibb/ibb.asmx?wsdl";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "text/xml; charset=UTF-8".parse().unwrap());
    headers.insert(
        "SOAPAction".parse::<HeaderName>().unwrap(),
        format!("\"http://tempuri.org/{}\"", soap_method)
            .parse()
            .unwrap(),
    );

    let body = get_body("hat_kodu", soap_method, hat_kodu);

    let res = client.post(url).headers(headers).body(body).send().await?;
    let response_string = &res.text().await?;
    let envelope: T = serde_xml_rs::from_str(response_string)?;

    Ok(envelope.get_relevant_data())
}

pub async fn request_csv<T: DeserializeOwned>(
    client: reqwest::Client,
    url: &str,
) -> Result<Vec<T>> {
    let res = client.get(url).send().await?;
    let response_string = &res.text().await?;
    Ok(read_csv_from_string(response_string)?)
}
