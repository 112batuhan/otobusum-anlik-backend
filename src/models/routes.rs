use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinate {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Route {
    pub id: i32,
    pub agency_id: Option<i32>,
    pub route_short_name: Option<String>,
    pub route_long_name: Option<String>,
    pub route_type: Option<i32>,
    pub route_desc: Option<String>,
    pub route_code: Option<String>,
    pub route_path: Option<sqlx::types::JsonValue>,
}
