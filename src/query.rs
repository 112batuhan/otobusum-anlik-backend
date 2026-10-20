use serde::Deserialize;

use crate::{models::city::City, models::route::Direction};

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

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    #[serde(default = "default_city")]
    pub city: City,
    pub q: String,
}

#[derive(Deserialize, Debug)]
pub struct QueryAreaQuery {
    pub lat: f64,
    pub lng: f64,
    #[serde(default = "default_city")]
    pub city: City,
}
