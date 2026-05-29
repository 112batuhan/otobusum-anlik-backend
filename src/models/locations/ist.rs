use crate::models::serializers::{deserialize_f64_from_string, deserialize_u32_from_string};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct IstTokensResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub expire_date: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BusLocationIstOtobusumNerede {
    #[serde(alias = "K_ARAC_KAPINUMARASI")]
    pub bus_id: String,
    #[serde(alias = "BOYLAM")]
    pub lng: f64,
    #[serde(alias = "ENLEM")]
    pub lat: f64,
    #[serde(alias = "K_GUZERGAH_GUZERGAHKODU")]
    pub route_code: String,
    #[serde(alias = "H_GOREV_DURAK_GECIS_DURAKID")]
    pub closest_stop_code: u32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BusLocationIstOpenData {
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
pub struct BusLocationIstOpenDataResponse {
    #[serde(alias = "Body")]
    pub content: BusLocationIstResponseBody,
}

pub enum BusLocationIst {
    OpenDataResponse(Vec<BusLocationIstOpenData>),
    OtobusumNeredeResponse(Vec<BusLocationIstOtobusumNerede>)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BusInfoIst {
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
    pub plate: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct BusInfoIstResponseJson {
    #[serde(alias = "GetFiloAracKonum_jsonResult")]
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct BusInfoIstResponseBody {
    #[serde(alias = "GetFiloAracKonum_jsonResponse")]
    pub content: BusInfoIstResponseJson,
}

#[derive(Serialize, Deserialize)]
pub struct BusInfoIstResponse {
    #[serde(alias = "Body")]
    pub content: BusInfoIstResponseBody,
}
