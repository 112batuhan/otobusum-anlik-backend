use crate::models::serializers::{deserialize_f64_from_string, deserialize_u32_from_string};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BusLocationOtobusumNerede {
    #[serde(alias = "K_ARAC_KAPINUMARASI")]
    pub bus_id: String,
    #[serde(alias = "BOYLAM")]
    pub lng: f64,
    #[serde(alias = "ENLEM")]
    pub lat: f64,
    #[serde(alias = "K_GUZERGAH_GUZERGAHKODU")]
    pub route_code: String,
    #[serde(alias = "H_GOREV_DURAK_GECIS_DURAKID")]
    pub closest_stop_code: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BusLocationOpenData {
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
pub struct BusLocationResponseJson {
    #[serde(alias = "GetHatOtoKonum_jsonResult")]
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct BusLocationResponseBody {
    #[serde(alias = "GetHatOtoKonum_jsonResponse")]
    pub content: BusLocationResponseJson,
}

#[derive(Serialize, Deserialize)]
pub struct BusLocationOpenDataResponse {
    #[serde(alias = "Body")]
    pub content: BusLocationResponseBody,
}

pub enum BusLocation {
    OpenDataResponse(Vec<BusLocationOpenData>),
    OtobusumNeredeResponse(Vec<BusLocationOtobusumNerede>),
}
