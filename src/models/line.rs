use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Line {
    pub id: i32,
    pub code: String,
    pub title: String,
    pub city: String,
}

#[derive(Serialize, Deserialize)]
pub struct LineStop {
    pub id: i32,
    pub line_code: String,
    pub stop_code: i32,
    pub route_code: String,
    pub city: String,
}
