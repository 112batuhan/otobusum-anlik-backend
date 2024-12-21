use serde::{de, Deserialize, Deserializer, Serialize};

pub fn deserialize_f64_from_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(de::Error::custom)
}
pub fn deserialize_u32_from_string<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<u32>().map_err(de::Error::custom)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BusLocation {
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
pub struct BusLocationResponse {
    #[serde(alias = "Body")]
    pub content: BusLocationResponseBody,
}
