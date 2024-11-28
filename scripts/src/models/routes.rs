use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Route {
    pub route_id: i32,
    pub agency_id: Option<i32>,
    pub route_short_name: Option<String>,
    pub route_long_name: Option<String>,
    pub route_type: Option<i32>,
    pub route_desc: Option<String>,
    pub route_code: Option<String>,
}
