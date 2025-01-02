use serde::Deserialize;

use crate::{database::city::City, models::routes::Direction};

pub fn default_city() -> City {
    City::istanbul
}

#[derive(Deserialize, Debug)]
pub struct CityQuery {
    #[serde(default = "default_city")]
    pub city: City,
}

#[derive(Deserialize, Debug)]
pub struct TimetableQuery {
    #[serde(default = "default_city")]
    pub city: City,
    pub direction: Direction,
}

#[derive(Deserialize, Debug)]
pub struct LineStopsQuery {
    #[serde(default = "default_city")]
    pub city: City,
    pub direction: Direction,
}
