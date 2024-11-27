use axum::http::{HeaderMap, HeaderName};
use reqwest::header::CONTENT_TYPE;
use serde::de::DeserializeOwned;

use anyhow::Result;

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
                    <{key}>{content:?}</{key}>
                </{soap_method}>
            </soap:Body>
        </soap:Envelope>
        "#,
        key = key,
        soap_method = soap_method,
        content = content
    )
}

pub async fn request_soap<T: UnwrapSoap<T>>(
// pub async fn request_soap<T: UnwrapSoap<R>, R: DeserializeOwned>(
    client: reqwest::Client,
    url: &str,
    soap_method: &str,
    hat_kodu: Option<&str>,
) -> Result<T>
where
{
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "text/xml; charset=UTF-8".parse().unwrap());
    headers.insert(
        "SOAPAction".parse::<HeaderName>().unwrap(),
        format!("\"http://tempuri.org/{}\"", soap_method)
            .parse()
            .unwrap(),
    );

    let body = get_body("hat_kodu", soap_method, hat_kodu);
    let response = client
        .post(url)
        .headers(headers)
        .send()
        .await?;

    // let res = client.post(url).headers(headers).body(body).send().await?;
    // let response_string = &res.text().await?;
    // let envelope: T = serde_xml_rs::from_str(response_string)?;

    // Ok(envelope.get_relevant_data())

    let content = response.text().await?;
    let envelope: T = serde_xml_rs::from_str(&content)?;

    Ok(envelope.get_relevant_data())
}

// #[derive(Debug, Deserialize)]
// pub struct DurakDetay {
//     #[serde(rename = "Body")]
//     pub body: DurakDetayBody,
// }

// impl UnwrapSoap<Vec<BusRouteStop>> for DurakDetay {
//     fn get_relevant_data(self) -> Vec<BusRouteStop> {
//         self.body.response.result.dataset.tables.unwrap_or_default()
//     }
// }

// #[derive(Debug, Deserialize)]
// pub struct DurakDetayBody {
//     #[serde(rename = "DurakDetay_GYYResponse")]
//     pub response: DurakDetayGYYResponse,
// }

// #[derive(Debug, Deserialize)]
// pub struct DurakDetayGYYResponse {
//     #[serde(rename = "DurakDetay_GYYResult")]
//     pub result: DurakDetayGYYResult,
// }



// #[derive(Debug, Deserialize)]
// #[serde(rename = "Envelope")]
// pub struct HatServisi {
//     #[serde(rename = "Body")]
//     pub body: HatServisiBody,
// }

// impl UnwrapSoap<Option<BusRouteMetadata>> for HatServisi {
//     fn get_relevant_data(self) -> Option<BusRouteMetadata> {
//         self.body.response.result.dataset.tables
//     }
// }

// #[derive(Debug, Deserialize)]
// pub struct HatServisiBody {
//     #[serde(rename = "HatServisi_GYYResponse")]
//     pub response: HatServisiResponse,
// }

// #[derive(Debug, Deserialize)]
// pub struct HatServisiResponse {
//     #[serde(rename = "HatServisi_GYYResult")]
//     pub result: HatServisiResult,
// }

// #[derive(Debug, Deserialize)]
// pub struct HatServisiResult {
//     #[serde(rename = "NewDataSet")]
//     pub dataset: HatServisiNewDataSet,
// }

// #[derive(Debug, Deserialize)]
// pub struct HatServisiNewDataSet {
//     #[serde(rename = "Table")]
//     pub tables: Option<BusRouteMetadata>,
// }

// #[derive(Debug, Deserialize)]
// pub struct BusRouteMetadata {
//     #[serde(rename = "HAT_KODU")]
//     pub hat_kodu: String,
//     #[serde(rename = "HAT_ADI")]
//     pub hat_adi: String,
//     #[serde(rename = "TAM_HAT_ADI")]
//     pub tam_hat_adi: String,
//     #[serde(rename = "HAT_DURUMU")]
//     pub hat_durumu: String,
//     #[serde(rename = "BOLGE")]
//     pub bolge: String,
//     #[serde(rename = "SEFER_SURESI")]
//     pub sefer_suresi: f64,
// }
