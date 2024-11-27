// use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// pub struct DurakDetayGYYResult {
//     #[serde(rename = "NewDataSet")]
//     pub dataset: DurakDetayNewDataSet,
// }

// #[derive(Debug, Deserialize)]
// pub struct DurakDetayNewDataSet {
//     #[serde(rename = "Table")]
//     pub tables: Option<Vec<BusRouteStop>>,
// }

// #[derive(Debug, Deserialize)]
// pub struct BusRouteStop {
//     #[serde(rename = "HATKODU")]
//     pub hatkodu: String,
//     #[serde(rename = "YON")]
//     pub yon: String,
//     #[serde(rename = "SIRANO")]
//     pub sirano: u32,
//     #[serde(rename = "DURAKKODU")]
//     pub durakkodu: u32,
//     #[serde(rename = "DURAKADI")]
//     pub durakadi: String,
//     #[serde(rename = "XKOORDINATI")]
//     pub xkoordinati: f64,
//     #[serde(rename = "YKOORDINATI")]
//     pub ykoordinati: f64,
//     #[serde(rename = "DURAKTIPI")]
//     pub duraktipi: String,
//     #[serde(rename = "ISLETMEBOLGE")]
//     pub isletmebolge: Option<String>,
//     #[serde(rename = "ISLETMEALTBOLGE")]
//     pub isletmealtbolge: String,
//     #[serde(rename = "ILCEADI")]
//     pub ilceadi: String,
// }
