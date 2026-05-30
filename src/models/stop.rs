use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BusStop {
    pub id: i32,
    pub stop_code: i32,
    pub name: String,
    pub lng: f64,
    pub lat: f64,
    pub province: Option<String>,
    pub smart: Option<String>,
    pub r#type: Option<String>,
    pub disabled_can_use: Option<String>,
    pub physical: Option<String>,
    pub city: String,
}
