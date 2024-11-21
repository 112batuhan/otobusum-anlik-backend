use anyhow::Result;
use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Deserializer,
};
use std::io::Cursor;

#[derive(Debug, Deserialize)]
pub struct Stop {
    #[serde(deserialize_with = "remove_non_numeric")]
    pub stop_id: u32,
    pub stop_code: u32,
    pub stop_name: String,
    pub stop_desc: String,
    pub stop_lat: f64,
    pub stop_lon: f64,
    pub location_type: u32,
}

#[derive(Debug, Deserialize)]
pub struct Route {
    pub _id: u32,
    pub route_id: u32,
    pub agency_id: u32,
    pub route_short_name: String,
    pub route_long_name: String,
    pub route_type: u32,
    pub route_desc: String,
    pub route_code: String,
}

fn remove_non_numeric<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let numeric_only: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    let number = numeric_only.parse::<u32>().map_err(de::Error::custom)?;
    Ok(number)
}

pub fn read_csv_from_string<T: DeserializeOwned>(data: &str) -> Result<Vec<T>> {
    let mut reader = csv::Reader::from_reader(Cursor::new(data));
    let mut data = Vec::new();

    for result in reader.deserialize() {
        match result {
            Ok(line) => data.push(line),
            Err(err) => {
                println!("{}", err);
                continue;
            }
        }
    }
    Ok(data)
}
