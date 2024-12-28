use crate::models::serializers::{deserialize_f64_from_string, deserialize_u32_from_string};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BusLocationIst {
    #[serde(alias = "kapino")]
    pub door_no: String,
    #[serde(alias = "boylam")]
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub lng: f64,
    #[serde(alias = "enlem")]
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub lat: f64,
    #[serde(alias = "hatkodu")]
    pub line_code: String,
    #[serde(alias = "guzergahkodu")]
    pub route_code: String,
    #[serde(alias = "hatad")]
    pub line_name: String,
    #[serde(alias = "yon")]
    pub direction: String,
    #[serde(alias = "son_konum_zamani")]
    pub last_location_update: String,
    #[serde(alias = "yakinDurakKodu")]
    #[serde(deserialize_with = "deserialize_u32_from_string")]
    pub closest_stop_code: u32,
}

#[derive(Serialize, Deserialize)]
pub struct BusLocationIstResponseJson {
    #[serde(alias = "GetHatOtoKonum_jsonResult")]
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct BusLocationIstResponseBody {
    #[serde(alias = "GetHatOtoKonum_jsonResponse")]
    pub content: BusLocationIstResponseJson,
}

#[derive(Serialize, Deserialize)]
pub struct BusLocationIstResponse {
    #[serde(alias = "Body")]
    pub content: BusLocationIstResponseBody,
}
