use crate::models::serializers::{deserialize_f64_from_string, deserialize_u32_from_string};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BusInfo {
    #[serde(alias = "Operator")]
    pub operator: String,
    #[serde(alias = "Garaj")]
    pub garage: Option<String>,
    #[serde(alias = "KapiNo")]
    pub door_no: String,
    #[serde(alias = "Saat")]
    pub hour: String,
    #[serde(alias = "Boylam")]
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub lng: f64,
    #[serde(alias = "Enlem")]
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub lat: f64,
    #[serde(alias = "Hiz")]
    #[serde(deserialize_with = "deserialize_u32_from_string")]
    pub speed: u32,
    #[serde(alias = "Plaka")]
    pub plate: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusInfoResponse {
    #[serde(alias = "Body")]
    pub content: BusInfoResponseBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusInfoResponseBody {
    #[serde(alias = "GetFiloAracKonum_jsonResponse")]
    pub content: BusInfoResponseJson,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusInfoResponseJson {
    #[serde(alias = "GetFiloAracKonum_jsonResult")]
    pub content: String,
}
