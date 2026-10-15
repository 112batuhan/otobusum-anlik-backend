use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BusStop {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct BusStopDetail {
    pub stop: BusStop,
    pub buses: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StopArrival {
    stop_order: u32,
    stop_code: u32,
    door_no: String,
    garage: String,
    line_code: String,
    lat: f64,
    lng: f64,
    last_speed: u8,
    arrival_time: String,
    minutes_left: u16,
    line_name: String,
    last_location_time: String,
    route_code: String,
    stop_name: String,
    features: BusFeatures,
}

#[derive(Serialize, Deserialize)]
struct BusFeatures {
    accessible: bool,
    usb_available: bool,
    wifi_avalable: bool,
    ac_active: bool,
    bicycle_rack: bool,
}

impl From<models::ist::stop_arrival::StopArrival> for StopArrival {
    fn from(value: models::ist::stop_arrival::StopArrival) -> Self {
        let (lat, lng) = match value.last_location.split_once(";") {
            Some((lng, lat)) => (
                lat.parse::<f64>().unwrap_or(0.),
                lng.parse::<f64>().unwrap_or(0.),
            ),
            None => (0., 0.),
        };

        Self {
            stop_order: value.stop_order,
            stop_code: value.stop_code.parse().unwrap_or(0),
            door_no: value.door_no,
            garage: value.garage,
            line_code: value.line_code,
            lat,
            lng,
            last_speed: value.last_speed.parse().unwrap_or(0),
            arrival_time: value.arrival_time,
            minutes_left: value.minutes_left,
            line_name: value.line_name,
            last_location_time: value.last_location_time,
            route_code: value.route_code,
            stop_name: value.stop_name,
            features: BusFeatures {
                accessible: value.accessible,
                usb_available: value.usb_available,
                wifi_avalable: value.wifi_available,
                ac_active: value.ac_active,
                bicycle_rack: value.bicycle_rack,
            },
        }
    }
}
