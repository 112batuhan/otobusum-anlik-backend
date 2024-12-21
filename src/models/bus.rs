use serde::{Deserialize, Serialize};

use crate::models::Coordinates;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub struct BusStopPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BusStop {
    pub id: i32,
    pub stop_code: i32,
    pub stop_name: String,
    pub x_coord: f64,
    pub y_coord: f64,
    pub province: Option<String>,
    pub smart: Option<String>,
    pub stop_type: Option<String>,
    pub disabled_can_use: Option<String>,
    pub physical: Option<String>,
    pub city: String,
}

#[derive(Debug, Serialize)]
pub struct BusRouteWithCoordinates {
    pub route_code: String,
    pub direction: String,
    pub coordinates: Vec<Coordinates>,
}
