use serde::{Deserialize, Serialize};

use crate::models::Coordinates;

#[derive(Serialize, Deserialize)]
pub struct BusRouteStopResponse {
    pub durakkodu: i32,
    pub durakadi: String,
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

#[derive(Debug)]
pub struct BusStopRow {
    pub route_code: String,
    pub direction: String,
    pub x: f64,
    pub y: f64,
    pub stop_order: i32,
}

#[derive(Debug, Deserialize)]
pub struct DurakDetayGYYResult {
    #[serde(rename = "NewDataSet")]
    pub dataset: DurakDetayNewDataSet,
}

#[derive(Debug, Deserialize)]
pub struct DurakDetayNewDataSet {
    #[serde(rename = "Table")]
    pub tables: Option<Vec<BusRouteStop>>,
}

#[derive(Debug, Deserialize)]
pub struct BusRouteStop {
    #[serde(rename = "HATKODU")]
    pub hatkodu: String,
    #[serde(rename = "YON")]
    pub yon: String,
    #[serde(rename = "SIRANO")]
    pub sirano: u32,
    #[serde(rename = "DURAKKODU")]
    pub durakkodu: u32,
    #[serde(rename = "DURAKADI")]
    pub durakadi: String,
    #[serde(rename = "XKOORDINATI")]
    pub xkoordinati: f64,
    #[serde(rename = "YKOORDINATI")]
    pub ykoordinati: f64,
    #[serde(rename = "DURAKTIPI")]
    pub duraktipi: String,
    #[serde(rename = "ISLETMEBOLGE")]
    pub isletmebolge: Option<String>,
    #[serde(rename = "ISLETMEALTBOLGE")]
    pub isletmealtbolge: String,
    #[serde(rename = "ILCEADI")]
    pub ilceadi: String,
}
