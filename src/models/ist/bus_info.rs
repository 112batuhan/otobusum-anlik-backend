use crate::models::serializers::{deserialize_f64_from_string, deserialize_u32_from_string};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BusInfoOpenData {
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
pub struct BusInfoOpenDataResponse {
    #[serde(alias = "Body")]
    pub content: BusInfoOpenDataResponseBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusInfoOpenDataResponseBody {
    #[serde(alias = "GetFiloAracKonum_jsonResponse")]
    pub content: BusInfoOpenDataResponseJson,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusInfoOpenDataResponseJson {
    #[serde(alias = "GetFiloAracKonum_jsonResult")]
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct BusInfoOtobusumNerede {
    #[serde(alias = "K_ARAC_KAPINUMARASI")]
    pub door_no: String,
    #[serde(alias = "K_ARAC_PLAKA")]
    pub plate: Option<String>,
    #[serde(alias = "H_OTOBUSKONUM_HIZ")]
    pub speed: u32,
    #[serde(alias = "H_OTOBUSKONUM_BOYLAM")]
    pub lng: f64,
    #[serde(alias = "H_OTOBUSKONUM_ENLEM")]
    pub lat: f64,
}
