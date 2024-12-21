use serde::{Deserialize, Serialize};

pub mod app;
pub mod bus;
pub mod line;
pub mod location;
pub mod routes;
pub mod stop;

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
}
