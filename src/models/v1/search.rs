use serde::{Deserialize, Serialize};

use crate::models::v1;

#[derive(Serialize, Deserialize)]
pub struct Search {
    pub stops: Vec<v1::stop::BusStop>,
    pub lines: Vec<v1::line::Line>,
}
