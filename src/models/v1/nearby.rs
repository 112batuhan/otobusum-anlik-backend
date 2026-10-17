use crate::models;
use serde::Serialize;

#[derive(Serialize)]
pub struct RoutesAndStopsNearbyResponse {
    pub routes_radius: f64,
    pub stops_radius: f64,
    pub stops: Vec<models::v1::stop::BusStop>,
    pub routes: Vec<models::v1::route::Route>,
}
