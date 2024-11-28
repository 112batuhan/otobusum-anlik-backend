// use otobusum_anlik_backend::models::Coordinates;
// use serde::{Deserialize, Serialize};
// use anyhow::Result;

// #[derive(Serialize)]
// struct GraphhopperPostBody {
//     points: Vec<Vec<f64>>,
//     profile: String,
//     instructions: bool,
//     points_encoded: bool,
//     curbside_strictness: String,
// }

// #[derive(Deserialize)]
// pub struct Points {
//     pub coordinates: Vec<Vec<f64>>,
// }

// #[derive(Deserialize)]
// pub struct Path {
//     pub points: Points,
// }

// #[derive(Deserialize)]
// pub struct GraphhopperResponseBody {
//     pub paths: Vec<Path>,
// }

// /// https://github.com/graphhopper/graphhopper/blob/master/docs/web/api-doc.md
// pub async fn request_graphhopper_routes(
//     client: reqwest::Client,
//     coordinates: Vec<Coordinates>,
// ) -> Result<Vec<Coordinates>> {
//     let points = coordinates
//         .into_iter()
//         .map(|coord| vec![coord.x, coord.y])
//         .collect();
//     let body = GraphhopperPostBody {
//         points,
//         profile: "car".to_string(),
//         instructions: false,
//         points_encoded: false,
//         curbside_strictness: "soft".to_string(),
//     };

//     let url = "http://localhost:8989/route";
//     let response_body: GraphhopperResponseBody =
//         client.post(url).json(&body).send().await?.json().await?;

//     let coordinates = response_body.paths[0]
//         .points
//         .coordinates
//         .iter()
//         .map(|coordinate_vec| Coordinates {
//             x: coordinate_vec[0],
//             y: coordinate_vec[1],
//         })
//         .collect();
//     Ok(coordinates)
// }
