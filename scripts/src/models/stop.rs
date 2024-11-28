use serde::Deserialize;

use crate::utils::soap::UnwrapSoap;

#[derive(Deserialize)]
#[serde(rename = "Envelope")]
pub struct BusStopsResponse {
    #[serde(rename = "Body")]
    pub body: BusStopsResponseBody,
}

#[derive(Deserialize)]
pub struct BusStopsResponseBody {
    #[serde(rename = "GetDurak_jsonResponse")]
    pub response: GetDurakJsonResponse,
}

#[derive(Debug, Deserialize)]
pub struct GetDurakJsonResponse {
    #[serde(rename = "GetDurak_jsonResult")]
    pub result: Vec<BusStopSoap>,
}

impl UnwrapSoap<Vec<BusStopSoap>> for BusStopsResponse {
    fn get_relevant_data(self) -> Vec<BusStopSoap> {
        self.body.response.result
    }
}

#[derive(Debug)]
pub struct BusStopPoint {
    pub x: f64,
    pub y: f64,
}

impl<'de> Deserialize<'de> for BusStopPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        let parsed = s
            .strip_prefix("POINT (")
            .and_then(|s| s.strip_suffix(")"))
            .and_then(|coords| {
                let mut parts = coords.split(' ');
                let x = parts.next()?.parse::<f64>().ok()?;
                let y = parts.next()?.parse::<f64>().ok()?;
                Some((x, y))
            });

        if let Some((x, y)) = parsed {
            Ok(Self { x, y })
        } else {
            Err(serde::de::Error::custom("Error splitting point values"))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BusStopSoap {
    #[serde(rename = "SDURAKKODU")]
    pub stop_code: i32,
    #[serde(rename = "SDURAKADI")]
    pub stop_name: String,
    #[serde(rename = "KOORDINAT")]
    pub coord: BusStopPoint,
    #[serde(rename = "ILCEADI")]
    pub province: String,
    #[serde(rename = "SYON")]
    pub direction: String,
    #[serde(rename = "AKILLI")]
    pub smart: String,
    #[serde(rename = "FIZIKI")]
    pub physical: Option<String>,
    #[serde(rename = "DURAK_TIPI")]
    pub stop_type: String,
    #[serde(rename = "ENGELLIKULLANIM")]
    pub disabled_can_use: String,
}
