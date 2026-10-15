use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchResponseOtobusumNerede {
    #[serde(alias = "HAT_ID")]
    pub line_id: u32,
    #[serde(alias = "HAT_HAT_KODU")]
    pub line_code: String,
}
