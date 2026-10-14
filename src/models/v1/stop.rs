use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BusStop {
    pub id: i32,
    pub code: i32,
    pub name: String,
    pub lng: f64,
    pub lat: f64,
    pub province: Option<String>,
    pub smart: Option<String>,
    pub stop_type: Option<String>,
    pub disabled_can_use: Option<String>,
    pub physical: Option<String>,
    pub city: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusStopDetail {
    pub stop: BusStop,
    pub buses: Vec<String>,
}
