use serde::{Deserialize, Serialize};

use crate::models::stop::BusStop;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BusStopV1 {
    pub id: i32,
    pub code: i32,
    pub name: String,
    pub lng: f64,
    pub lat: f64,
    pub province: Option<String>,
    pub smart: Option<String>,
    pub stop_type: Option<String>,
    pub disabled_can_use: Option<String>,
    pub physical: Option<String>,
    pub city: String,
}

impl From<BusStop> for BusStopV1 {
    fn from(value: BusStop) -> Self {
        Self {
            id: value.id,
            code: value.stop_code,
            name: value.name,
            lng: value.lng,
            lat: value.lat,
            province: value.province,
            smart: value.smart,
            stop_type: value.r#type,
            disabled_can_use: value.disabled_can_use,
            physical: value.physical,
            city: value.city,
        }
    }
}
