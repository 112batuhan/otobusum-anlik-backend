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
            line_code: value.code,
            name: value.title,
            r#type: value.r#type,
            desc: value.description,
            code: value.route_code,
            path: value.path,
        }
    }
}
