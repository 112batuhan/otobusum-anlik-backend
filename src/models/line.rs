use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct BusLine {
    pub code: String,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct LineStop {
    pub id: i32,
    pub line_code: String,
    pub stop_code: i32,
}
