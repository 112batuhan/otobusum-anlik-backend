use serde::Deserialize;

use crate::utils::xml_parse::UnwrapSoap;

#[derive(Deserialize)]
#[serde(rename = "Envelope")]
pub struct BusStopsResponse {
    #[serde(rename = "Body")]
    pub body: BusStopsResponseBody
}

#[derive(Deserialize)]
pub struct BusStopsResponseBody {
    #[serde(rename = "GetDurak_jsonResponse")]
    pub response: GetDurakJsonResponse,
}

#[derive(Debug, Deserialize)]
pub struct GetDurakJsonResponse {
    #[serde(rename = "GetDurak_jsonResult")]
    pub result: String,
}

impl UnwrapSoap<String> for BusStopsResponse {
    fn get_relevant_data(self) -> String {
        self.body.response.result
    }
}

#[derive(Debug, Deserialize)]
pub struct BusStopSoap {
    #[serde(rename = "SDURAKKODU")]
    pub stop_code: i32,
    #[serde(rename = "SDURAKADI")]
    pub stop_name: String,
    #[serde(rename = "KOORDINAT")]
    pub coordinate: String,
    #[serde(rename = "ILCEADI")]
    pub province: String,
    #[serde(rename = "SYON")]
    pub direction: String,
    #[serde(rename = "AKILLI")]
    pub smart: String,
    #[serde(rename = "FIZIKI")]
    pub physical: Option<String>,
    #[serde(rename = "DURAK_TIPI")]
    pub stop_type: String,
    #[serde(rename = "ENGELLIKULLANIM")]
    pub disabled_can_use: String,
}
