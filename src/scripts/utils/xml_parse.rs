// use serde::{de::DeserializeOwned, Deserialize};

// use crate::models::stop::{BusRouteStop, DurakDetayGYYResult};

// pub trait UnwrapSoap<R: DeserializeOwned>: DeserializeOwned {
//     fn get_relevant_data(self) -> R;
// }

// #[derive(Debug, Deserialize)]
// pub struct DurakDetay {
//     #[serde(rename = "Body")]
//     pub body: DurakDetayBody,
// }

// impl UnwrapSoap<Vec<BusRouteStop>> for DurakDetay {
//     fn get_relevant_data(self) -> Vec<BusRouteStop> {
//         self.body.response.result.dataset.tables.unwrap_or_default()
//     }
// }

// #[derive(Debug, Deserialize)]
// pub struct DurakDetayBody {
//     #[serde(rename = "DurakDetay_GYYResponse")]
//     pub response: DurakDetayGYYResponse,
// }

// #[derive(Debug, Deserialize)]
// pub struct DurakDetayGYYResponse {
//     #[serde(rename = "DurakDetay_GYYResult")]
//     pub result: DurakDetayGYYResult,
// }



// #[derive(Debug, Deserialize)]
// #[serde(rename = "Envelope")]
// pub struct HatServisi {
//     #[serde(rename = "Body")]
//     pub body: HatServisiBody,
// }

// impl UnwrapSoap<Option<BusRouteMetadata>> for HatServisi {
//     fn get_relevant_data(self) -> Option<BusRouteMetadata> {
//         self.body.response.result.dataset.tables
//     }
// }

// #[derive(Debug, Deserialize)]
// pub struct HatServisiBody {
//     #[serde(rename = "HatServisi_GYYResponse")]
//     pub response: HatServisiResponse,
// }

// #[derive(Debug, Deserialize)]
// pub struct HatServisiResponse {
//     #[serde(rename = "HatServisi_GYYResult")]
//     pub result: HatServisiResult,
// }

// #[derive(Debug, Deserialize)]
// pub struct HatServisiResult {
//     #[serde(rename = "NewDataSet")]
//     pub dataset: HatServisiNewDataSet,
// }

// #[derive(Debug, Deserialize)]
// pub struct HatServisiNewDataSet {
//     #[serde(rename = "Table")]
//     pub tables: Option<BusRouteMetadata>,
// }

// #[derive(Debug, Deserialize)]
// pub struct BusRouteMetadata {
//     #[serde(rename = "HAT_KODU")]
//     pub hat_kodu: String,
//     #[serde(rename = "HAT_ADI")]
//     pub hat_adi: String,
//     #[serde(rename = "TAM_HAT_ADI")]
//     pub tam_hat_adi: String,
//     #[serde(rename = "HAT_DURUMU")]
//     pub hat_durumu: String,
//     #[serde(rename = "BOLGE")]
//     pub bolge: String,
//     #[serde(rename = "SEFER_SURESI")]
//     pub sefer_suresi: f64,
// }
