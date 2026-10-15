use crate::models::serializers::deserialize_bool_from_int;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StopArrival {
    #[serde(alias = "stopOrder")]
    pub stop_order: u32,
    #[serde(alias = "connections")]
    pub connections: Option<String>,
    #[serde(alias = "kod")]
    pub stop_code: String,
    #[serde(alias = "kapino")]
    pub door_no: String,
    #[serde(alias = "ototip")]
    pub garage: String,
    #[serde(alias = "engelli")]
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub accessible: bool,
    #[serde(alias = "hatkodu")]
    pub line_code: String,
    #[serde(alias = "hattip")]
    pub line_type: String,
    #[serde(alias = "usb")]
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub usb_available: bool,
    #[serde(alias = "wifi")]
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub wifi_available: bool,
    #[serde(alias = "klima")]
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub ac_active: bool,
    #[serde(alias = "bisiklet")]
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub bicycle_rack: bool,
    #[serde(alias = "son_konum")]
    pub last_location: String,
    #[serde(alias = "son_hiz")]
    pub last_speed: String,
    #[serde(alias = "saat")]
    pub arrival_time: String,
    #[serde(alias = "dakika")]
    pub minutes_left: u16,
    #[serde(alias = "tahmintipi")]
    pub prediction_type: u32,
    #[serde(alias = "hatadi")]
    pub line_name: String,
    #[serde(alias = "sistemsaati")]
    pub system_time: String,
    #[serde(alias = "son_konum_saati")]
    pub last_location_time: String,
    #[serde(alias = "guzergah")]
    pub route_code: String,
    #[serde(alias = "durak_adi")]
    pub stop_name: String,
    #[serde(alias = "kaynak")]
    pub data_source: String,
}
