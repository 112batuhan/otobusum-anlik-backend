use geo_types::Geometry;
use geozero::wkb;

use crate::models::serializers::serialize_wkb_geom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Route {
    pub id: i32,
    pub agency_id: Option<i32>,
    pub line_code: String,
    pub name: String,
    pub r#type: Option<i32>,
    pub description: Option<String>,
    pub code: String,

    #[serde(serialize_with = "serialize_wkb_geom")]
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

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
