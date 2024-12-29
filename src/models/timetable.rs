use serde::Serialize;
use sqlx::types::chrono::NaiveTime;

#[derive(Serialize)]
pub struct Timetable {
    pub route_short_name: Option<String>,
    pub sunday: Option<Vec<NaiveTime>>,
    pub monday: Option<Vec<NaiveTime>>,
    pub tuesday: Option<Vec<NaiveTime>>,
    pub wednesday: Option<Vec<NaiveTime>>,
    pub thursday: Option<Vec<NaiveTime>>,
    pub friday: Option<Vec<NaiveTime>>,
    pub saturday: Option<Vec<NaiveTime>>,
}
