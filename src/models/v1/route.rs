use geo_types::Geometry;
use geozero::wkb;

use crate::models::serializers::serialize_wkb_geom;
use serde::Serialize;

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
