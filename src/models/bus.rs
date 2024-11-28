use serde::{Deserialize, Serialize};

use crate::models::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct BusStopPoint {
    pub x: f64,
    pub y: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusStop {
    pub stop_code: i32,
    pub stop_name: String,
    pub coordinate: BusStopPoint,
    pub province: String,
    pub direction: String,
    pub smart: String,
    pub physical: Option<String>,
    pub stop_type: String,
    pub disabled_can_use: String,
}

#[derive(Debug, Serialize)]
pub struct BusRouteWithCoordinates {
    pub route_code: String,
    pub direction: String,
    pub coordinates: Vec<Coordinates>,
}
