use serde::{Deserialize, Serialize};

pub mod app;
pub mod bus;

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
}
