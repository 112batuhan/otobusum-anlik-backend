use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BusRouteStop {
    #[serde(rename = "HATKODU")]
    pub hatkodu: String,
    #[serde(rename = "YON")]
    pub yon: String,
    #[serde(rename = "SIRANO")]
    pub sirano: u32,
    #[serde(rename = "DURAKKODU")]
    pub durakkodu: String,
    #[serde(rename = "DURAKADI")]
    pub durakadi: String,
    #[serde(rename = "XKOORDINATI")]
    pub xkoordinati: f64,
    #[serde(rename = "YKOORDINATI")]
    pub ykoordinati: f64,
    #[serde(rename = "DURAKTIPI")]
    pub duraktipi: String,
    #[serde(rename = "ISLETMEBOLGE")]
    pub isletmebolge: Option<String>,
    #[serde(rename = "ISLETMEALTBOLGE")]
    pub isletmealtbolge: String,
    #[serde(rename = "ILCEADI")]
    pub ilceadi: String,
}

#[derive(Debug, Deserialize)]
pub struct NewDataSet {
    #[serde(rename = "Table")]
    pub tables: Option<Vec<BusRouteStop>>,
}

#[derive(Debug, Deserialize)]
pub struct DurakDetayGYYResult {
    #[serde(rename = "NewDataSet")]
    pub dataset: NewDataSet,
}

#[derive(Debug, Deserialize)]
pub struct DurakDetayGYYResponse {
    #[serde(rename = "DurakDetay_GYYResult")]
    pub result: DurakDetayGYYResult,
}

#[derive(Debug, Deserialize)]
pub struct SoapBody {
    #[serde(rename = "DurakDetay_GYYResponse")]
    pub response: DurakDetayGYYResponse,
}

#[derive(Debug, Deserialize)]
pub struct Envelope {
    #[serde(rename = "Body")]
    pub body: SoapBody,
}
