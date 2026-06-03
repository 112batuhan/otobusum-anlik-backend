use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Route {
    pub id: i32,
    pub agency_id: Option<i32>,
    pub line_code: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<i32>,
    pub desc: Option<String>,
    pub code: Option<String>,
    pub path: Option<sqlx::types::JsonValue>,
}

impl From<crate::models::route::Route> for Route {
    fn from(value: crate::models::route::Route) -> Self {
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
