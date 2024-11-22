use serde::{de::DeserializeOwned, Deserialize};

pub trait UnwrapSoap<T: DeserializeOwned, R: DeserializeOwned>: DeserializeOwned {
    fn get_relevant_data(self) -> R;
}

#[derive(Debug, Deserialize)]
pub struct DurakDetayEnvelope {
    #[serde(rename = "Body")]
    pub body: DurakDetayBody,
}

impl UnwrapSoap<DurakDetayEnvelope, Vec<BusRouteStop>> for DurakDetayEnvelope {
    fn get_relevant_data(self) -> Vec<BusRouteStop> {
        self.body.response.result.dataset.tables.unwrap_or_default()
    }
}

#[derive(Debug, Deserialize)]
pub struct DurakDetayBody {
    #[serde(rename = "DurakDetay_GYYResponse")]
    pub response: DurakDetayGYYResponse,
}

#[derive(Debug, Deserialize)]
pub struct DurakDetayGYYResponse {
    #[serde(rename = "DurakDetay_GYYResult")]
    pub result: DurakDetayGYYResult,
}

#[derive(Debug, Deserialize)]
pub struct DurakDetayGYYResult {
    #[serde(rename = "NewDataSet")]
    pub dataset: DurakDetayNewDataSet,
}

#[derive(Debug, Deserialize)]
pub struct DurakDetayNewDataSet {
    #[serde(rename = "Table")]
    pub tables: Option<Vec<BusRouteStop>>,
}

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
#[serde(rename = "Envelope")]
pub struct HatServisiEnvelope {
    #[serde(rename = "Body")]
    pub body: HatServisiBody,
}

impl UnwrapSoap<HatServisiEnvelope, Option<RouteMetadata>> for HatServisiEnvelope {
    fn get_relevant_data(self) -> Option<RouteMetadata> {
        self.body.response.result.dataset.tables
    }
}

#[derive(Debug, Deserialize)]
pub struct HatServisiBody {
    #[serde(rename = "HatServisi_GYYResponse")]
    pub response: HatServisiResponse,
}

#[derive(Debug, Deserialize)]
pub struct HatServisiResponse {
    #[serde(rename = "HatServisi_GYYResult")]
    pub result: HatServisiResult,
}

#[derive(Debug, Deserialize)]
pub struct HatServisiResult {
    #[serde(rename = "NewDataSet")]
    pub dataset: HatServisiNewDataSet,
}

#[derive(Debug, Deserialize)]
pub struct HatServisiNewDataSet {
    #[serde(rename = "Table")]
    pub tables: Option<RouteMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct RouteMetadata {
    #[serde(rename = "HAT_KODU")]
    pub hat_kodu: String,
    #[serde(rename = "HAT_ADI")]
    pub hat_adi: String,
    #[serde(rename = "TAM_HAT_ADI")]
    pub tam_hat_adi: String,
    #[serde(rename = "HAT_DURUMU")]
    pub hat_durumu: String,
    #[serde(rename = "BOLGE")]
    pub bolge: String,
    #[serde(rename = "SEFER_SURESI")]
    pub sefer_suresi: f64,
}
