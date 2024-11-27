use serde::Serialize;

#[derive(Serialize)]
pub struct BusLine {
    pub code: String,
    pub title: String
}
