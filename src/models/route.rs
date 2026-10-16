use std::fmt;

use geo_types::Geometry;
use geozero::wkb;
use serde::{Deserialize, Serialize};

use crate::models::serializers::{deserialize_wkb_geom, serialize_wkb_geom};

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    pub id: i32,
    pub agency_id: Option<i32>,
    pub code: Option<String>,
    pub title: Option<String>,
    pub r#type: Option<i32>,
    pub description: Option<String>,
    pub route_code: Option<String>,

    #[serde(
        serialize_with = "serialize_wkb_geom",
        deserialize_with = "deserialize_wkb_geom"
    )]
    pub path: wkb::Decode<Geometry>,
}

#[derive(Deserialize, Default, Debug, PartialEq)]
pub enum Direction {
    #[default]
    G = 0,
    D = 1,
}

impl TryFrom<i32> for Direction {
    type Error = ();

    fn try_from(v: i32) -> Result<Direction, ()> {
        match v {
            x if x == Direction::G as i32 => Ok(Direction::G),
            x if x == Direction::D as i32 => Ok(Direction::D),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Direction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::G => "G",
            Direction::D => "D",
        }
    }
}
