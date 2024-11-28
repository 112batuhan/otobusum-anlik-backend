use serde::{Deserialize, Serialize};

use super::bus::BusStopPoint;

#[derive(Serialize)]
pub struct BusLine {
    pub code: String,
    pub title: String
}

#[derive(Serialize, Deserialize, sqlx::Type)]
pub struct BusLineWithCoordinates {
    pub code: String,
    pub title: String,
    pub stop_coords: Option<Vec<BusStopPoint>>
}
