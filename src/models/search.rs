use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Serialize, Deserialize, Clone)]
pub struct Search {
    pub stops: Vec<models::stop::BusStop>,
    pub lines: Vec<models::line::Line>,
}
