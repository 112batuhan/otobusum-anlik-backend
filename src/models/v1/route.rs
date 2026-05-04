use serde::{Deserialize, Serialize};

use crate::models::routes::Route;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RouteV1 {
    pub id: i32,
    pub agency_id: Option<i32>,
    pub line_code: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<i32>,
    pub desc: Option<String>,
    pub code: Option<String>,
    pub path: Option<sqlx::types::JsonValue>,
}

impl From<Route> for RouteV1 {
    fn from(value: Route) -> Self {
        Self {
            id: value.id,
            agency_id: value.agency_id,
            line_code: value.route_short_name,
            name: value.route_long_name,
            r#type: value.route_type,
            desc: value.route_desc,
            code: value.route_code,
            path: value.route_path,
        }
    }
}

// impl From<Vec<Route>> for Vec<RouteV1> {
//     fn from(value: Vec<Route>) -> Self {
//         value.into_iter().map(RouteV1::from).collect()
//     }
// }
