use serde::Serialize;

use super::Coordinates;

#[derive(Serialize)]
pub struct BusLine {
    pub code: String,
    pub title: String
}

#[derive(Serialize)]
pub struct BusLineWithCoordinates {
    pub code: String,
    pub title: String,
    pub points: Vec<Coordinates>
}
