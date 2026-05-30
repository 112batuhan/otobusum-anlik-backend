use serde::{Deserialize, Serialize};

use crate::models::line::Line;

#[derive(Serialize, Deserialize, Clone)]
pub struct LineV1 {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub city: String,
}

impl From<Line> for LineV1 {
    fn from(value: Line) -> Self {
        Self {
            id: value.id,
            city: value.city,
            code: value.code,
            name: value.title,
        }
    }
}
