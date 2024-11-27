use serde::{Deserialize, Serialize};

use crate::models::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct BusStop {
    pub hatkodu: String,
    pub id: i32,
    pub durakkodu: i32,
    pub yon: String,
    pub durakadi: String,
    pub sirano: i32,
    pub xkoordinati: f64,
    pub ykoordinati: f64,
    pub duraktipi: String,
    pub isletmebolge: Option<String>,
    pub isletmealtbolge: String,
    pub ilceadi: String,
}

#[derive(Debug, Serialize)]
pub struct BusRouteWithCoordinates {
    pub route_code: String,
    pub direction: String,
    pub coordinates: Vec<Coordinates>,
}
