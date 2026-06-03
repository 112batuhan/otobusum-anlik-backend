use crate::models::serializers::deserialize_f64_from_string;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BusLocation {
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
pub struct BusLocationResponse {
    #[serde(alias = "HataMesaj")]
    pub error_message: String,
    #[serde(alias = "HatOtobusKonumlari")]
    pub bus_locations: Vec<BusLocation>,
    #[serde(alias = "HataVarMi")]
    pub is_error: bool,
}
