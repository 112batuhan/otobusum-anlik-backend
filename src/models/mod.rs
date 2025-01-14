use serde::{Deserialize, Serialize};

pub mod app;
pub mod bus;
pub mod line;
pub mod locations;
pub mod routes;
pub mod serializers;
pub mod stop;
pub mod timetable;

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
}
