use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Line {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub city: String,
}

impl From<crate::models::line::Line> for Line {
    fn from(value: crate::models::line::Line) -> Self {
        Self {
            id: value.id,
            city: value.city,
            code: value.code,
            name: value.title,
        }
    }
}
