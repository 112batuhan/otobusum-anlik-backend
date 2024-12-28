use serde::{de, Deserialize, Deserializer};

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

#[derive(Deserialize, Debug)]
pub struct BusLocationIzm {
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

#[derive(Deserialize, Debug)]
pub struct BusLocationIzmResponse {
    #[serde(alias = "HataMesaj")]
    pub error_message: String,
    #[serde(alias = "HatOtobusKonumlari")]
    pub bus_locations: Vec<BusLocationIzm>,
    #[serde(alias = "HataVarMi")]
    pub is_error: bool,
}
