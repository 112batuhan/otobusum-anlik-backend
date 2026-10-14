use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Serialize, Deserialize, Clone)]
pub struct BusLocationResponse {
    pub data: Vec<models::bus::BusLocation>,
    pub next_poll: i64,
}

impl cached::Expires for BusLocationResponse {
    fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        now > (self.next_poll - 5)
    }
}
