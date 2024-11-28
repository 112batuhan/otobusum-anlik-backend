use axum::http::{HeaderMap, HeaderName};
use reqwest::header::CONTENT_TYPE;

use anyhow::Result;
use serde::de::DeserializeOwned;

pub trait UnwrapSoap<R: DeserializeOwned>: DeserializeOwned {
    fn get_relevant_data(self) -> R;
}

pub fn get_body(key: &str, soap_method: &str, content: Option<&str>) -> String {
    format!(
        r#"
        <soap:Envelope
            xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
            <soap:Body>
                <{soap_method}
                    xmlns="http://tempuri.org/">
                    <{key}></{key}>
                </{soap_method}>
            </soap:Body>
        </soap:Envelope>
        "#,
        key = key,
        soap_method = soap_method,
        // content = content
    )
}

pub async fn request_soap<Response: UnwrapSoap<T>, T: DeserializeOwned>(
    client: reqwest::Client,
    url: &str,
    soap_method: &str,
    hat_kodu: Option<&str>,
) -> Result<T> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "text/xml; charset=UTF-8".parse().unwrap());
    headers.insert(
        "SOAPAction".parse::<HeaderName>().unwrap(),
        format!("\"http://tempuri.org/{}\"", soap_method)
            .parse()
            .unwrap(),
    );

    let body = get_body("DurakKodu", soap_method, hat_kodu);

    let response = client
        .post(url)
        .body(body)
        .headers(headers)
        .send()
        .await
        .unwrap();

    let content = response.text().await.unwrap();

    let envelope: Response = serde_xml_rs::from_str(&content)?;
    let data = envelope.get_relevant_data();

    Ok(data)
}
