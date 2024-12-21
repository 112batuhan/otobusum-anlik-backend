use serde::Serialize;

use crate::models::Coordinates;

#[derive(Debug, Serialize)]
pub struct RouteWithCoordinates {
    pub route_code: String,
    pub direction: String,
    pub coordinates: Vec<Coordinates>,
}
