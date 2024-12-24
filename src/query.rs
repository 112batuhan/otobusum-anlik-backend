use serde::Deserialize;

use crate::database::city::City;

pub fn default_city() -> City {
    City::istanbul
}

#[derive(Deserialize, Debug)]
pub struct CityQuery {
    #[serde(default = "default_city")]
    pub city: City,
}
