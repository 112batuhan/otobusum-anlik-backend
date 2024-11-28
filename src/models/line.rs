use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct BusLine {
    pub code: String,
    pub title: String
}

#[derive(Serialize, Deserialize, sqlx::Type)]
pub struct BusLineWithCoordinates {
    pub code: String,
    pub title: String,
    pub stop_codes: Option<Vec<String>>
}
