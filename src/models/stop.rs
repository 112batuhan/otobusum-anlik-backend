use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BusStop {
    pub id: i32,
    pub stop_code: i32,
    pub name: String,
    pub lng: f64,
    pub lat: f64,
    pub province: Option<String>,
    pub city: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BusStopDetail {
    pub stop: BusStop,
    pub buses: Vec<String>,
}
