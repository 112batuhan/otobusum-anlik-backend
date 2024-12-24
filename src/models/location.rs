use serde::{de, Deserialize, Deserializer, Serialize};

pub fn deserialize_f64_from_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.chars()
        .filter(|&c| c != '.' && c != ' ')
        .map(|c| if c == ',' { '.' } else { c })
        .collect::<String>()
        .parse::<f64>()
        .map_err(de::Error::custom)
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
    pub door_no: Option<String>,
    #[serde(alias = "boylam")]
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub lng: f64,
    #[serde(alias = "enlem")]
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub lat: f64,
    #[serde(alias = "hatkodu")]
    pub line_code: Option<String>,
    #[serde(alias = "guzergahkodu")]
    pub route_code: Option<String>,
    #[serde(alias = "hatad")]
    pub line_name: Option<String>,
    #[serde(alias = "yon")]
    pub direction: Option<String>,
    #[serde(alias = "son_konum_zamani")]
    pub last_location_update: Option<String>,
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

// For izmir

#[derive(Deserialize, Debug)]
pub struct BusLocationResponseIzmir {
    #[serde(alias = "HataMesaj")]
    pub error_message: String,
    #[serde(alias = "HatOtobusKonumlari")]
    pub bus_locations: Vec<BusLocationIzmir>,
    #[serde(alias = "HataVarMi")]
    pub is_error: bool,
}

#[derive(Deserialize, Debug)]
pub struct BusLocationIzmir {
    #[serde(alias = "OtobusId")]
    pub bus_id: u32,
    #[serde(alias = "Yon")]
    pub direction: u32,
    #[serde(alias = "KoorX")]
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub x_coord: f64,
    #[serde(alias = "KoorY")]
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    pub y_coord: f64,
}

impl From<BusLocationIzmir> for BusLocation {
    fn from(value: BusLocationIzmir) -> Self {
        Self {
            lng: value.x_coord,
            lat: value.y_coord,
            closest_stop_code: 0,
            door_no: None,
            line_code: None,
            direction: None,
            line_name: None,
            route_code: None,
            last_location_update: None,
        }
    }
}
