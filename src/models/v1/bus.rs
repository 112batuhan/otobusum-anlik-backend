use chrono::{DateTime, Utc};
use chrono_tz::Europe::Istanbul;
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Serialize, Deserialize, Clone)]
pub struct BusLocationResponse {
    pub data: Vec<models::v1::bus::BusLocation>,
    pub next_poll: i64,
}

impl cached::Expires for BusLocationResponse {
    fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        now > (self.next_poll - 5)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BusLocation {
    pub bus_id: String,
    pub lng: f64,
    pub lat: f64,
    pub route_code: String,
    pub closest_stop_code: Option<u32>,
    pub last_update_time: Option<DateTime<Utc>>,
}

impl From<models::ist::bus_location::BusLocationOpenData> for BusLocation {
    fn from(value: models::ist::bus_location::BusLocationOpenData) -> Self {
        let last_update_time =
            chrono::NaiveDateTime::parse_from_str(&value.last_location_update, "%Y-%m-%d %H:%M:%S")
                .ok()
                .and_then(|x| x.and_local_timezone(Istanbul).single())
                .map(|time| time.with_timezone(&Utc));

        Self {
            bus_id: value.door_no,
            lat: value.lat,
            lng: value.lng,
            route_code: value.route_code,
            closest_stop_code: Some(value.closest_stop_code),
            last_update_time,
        }
    }
}

impl From<models::ist::bus_location::BusLocationOtobusumNerede> for BusLocation {
    fn from(value: models::ist::bus_location::BusLocationOtobusumNerede) -> Self {
        let last_update_time =
            chrono::NaiveDateTime::parse_from_str(&value.last_location_update, "%Y-%m-%d %H:%M:%S")
                .ok()
                .and_then(|x| x.and_local_timezone(Istanbul).single())
                .map(|time| time.with_timezone(&Utc));

        Self {
            bus_id: value.bus_id,
            lat: value.lat,
            lng: value.lng,
            route_code: value.route_code,
            closest_stop_code: Some(value.closest_stop_code),
            last_update_time,
        }
    }
}

impl BusLocation {
    pub fn from_izm_bus_location(
        value: models::izm::bus_location::BusLocation,
        line_code: &str,
    ) -> Self {
        let mut dir =
            models::v1::route::Direction::try_from(value.direction as i32).unwrap_or_default();

        // For some reason directions are reversed in izmir. Wtf
        dir = if dir == models::v1::route::Direction::G {
            models::v1::route::Direction::D
        } else {
            models::v1::route::Direction::G
        };

        Self {
            bus_id: value.bus_id.to_string(),
            lat: value.x_coord, // izmir returns coordinates reversed for some reason
            lng: value.y_coord,
            route_code: format!("{line_code}_{}_D0", dir),
            closest_stop_code: None,
            last_update_time: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BusInfo {
    pub door_no: String,
    pub lng: f64,
    pub lat: f64,
    pub plate: Option<String>,
    pub speed: u32,
}

impl From<models::ist::bus_info::BusInfoOpenData> for BusInfo {
    fn from(value: models::ist::bus_info::BusInfoOpenData) -> Self {
        Self {
            door_no: value.door_no,
            lat: value.lat,
            lng: value.lng,
            plate: value.plate,
            speed: value.speed,
        }
    }
}

impl From<models::ist::bus_info::BusInfoOtobusumNerede> for BusInfo {
    fn from(value: models::ist::bus_info::BusInfoOtobusumNerede) -> Self {
        Self {
            door_no: value.door_no,
            lat: value.lat,
            lng: value.lng,
            plate: value.plate,
            speed: value.speed,
        }
    }
}
