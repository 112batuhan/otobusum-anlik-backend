use geo_types::{Coord, Geometry, LineString};
use geozero::wkb;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

pub fn deserialize_f64_from_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.replace(",", ".")
        .parse::<f64>()
        .map_err(de::Error::custom)
}

pub fn deserialize_u32_from_string<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse::<u32>()
        .map_err(de::Error::custom)
}

pub fn deserialize_bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1.. => Ok(true),
    }
}

#[derive(Deserialize, Serialize)]
struct LatLng {
    lat: f64,
    lng: f64,
}

pub fn serialize_wkb_geom<S>(
    decode: &wkb::Decode<Geometry>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(Geometry::LineString(line_string)) = &decode.geometry {
        let coords: Vec<LatLng> = line_string
            .0 // Extract internal vector of Points
            .iter()
            .map(|coord| LatLng {
                lat: coord.y,
                lng: coord.x,
            })
            .collect();

        coords.serialize(serializer)
    } else {
        let empty: Vec<[f64; 2]> = vec![];
        empty.serialize(serializer)
    }
}

pub fn deserialize_wkb_geom<'de, D>(deserializer: D) -> Result<wkb::Decode<Geometry>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_coords: Vec<LatLng> = Vec::deserialize(deserializer)?;

    let coords: Vec<Coord<f64>> = raw_coords
        .into_iter()
        .map(|latlng| Coord {
            x: latlng.lng,
            y: latlng.lat,
        })
        .collect();

    let line_string = LineString::new(coords);

    Ok(wkb::Decode {
        geometry: Some(Geometry::LineString(line_string)),
    })
}
