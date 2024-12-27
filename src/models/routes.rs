use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinate {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Route {
    pub id: i32,
    pub agency_id: Option<i32>,
    pub route_short_name: Option<String>,
    pub route_long_name: Option<String>,
    pub route_type: Option<i32>,
    pub route_desc: Option<String>,
    pub route_code: Option<String>,
    pub route_path: Option<sqlx::types::JsonValue>,
}

#[derive(Default, Debug)]
pub enum Direction {
    #[default] G = 0,
    D = 1
}

impl TryFrom<i32> for Direction {
    type Error = ();

    fn try_from(v: i32) -> Result<Direction, ()> {
        match v {
            x if x == Direction::G as i32 => Ok(Direction::G),
            x if x == Direction::D as i32 => Ok(Direction::D),
            _ => Err(())
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
