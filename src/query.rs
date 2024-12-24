use serde::Deserialize;

use crate::database::city::City;

fn default_city() -> City {
    return City::istanbul;
}

#[derive(Deserialize, Debug)]
pub struct CityQuery {
    #[serde(default = "default_city")]
    pub city: City,
}
