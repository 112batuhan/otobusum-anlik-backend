use ist::BusLocationIst;
use izm::BusLocationIzm;
use serde::{Deserialize, Serialize};

use super::routes::Direction;

pub mod ist;
pub mod izm;

#[derive(Serialize, Deserialize, Clone)]
pub struct BusLocation {
    pub bus_id: String,
    pub lng: f64,
    pub lat: f64,
    pub route_code: String,
}

impl From<BusLocationIst> for BusLocation {
    fn from(value: BusLocationIst) -> Self {
        Self {
            bus_id: value.door_no,
            lat: value.lat,
            lng: value.lng,
            route_code: value.route_code,
        }
    }
}

impl BusLocation {
    pub fn from_bus_location_izm(value: BusLocationIzm, line_code: &str) -> Self {
        let dir = Direction::try_from(value.direction as i32).unwrap_or_default();

        Self {
            bus_id: value.bus_id.to_string(),
            lat: value.y_coord,
            lng: value.x_coord,
            route_code: format!("{line_code}_{}_D0", dir),
        }
    }
}
