use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StopArrival {
    #[serde(alias = "stopOrder")]
    pub stop_order: u32,

    // Defined as Option<String> (or Option<Vec<String>>) since it is null in the JSON
    #[serde(alias = "connections")]
    pub connections: Option<String>,

    #[serde(alias = "kod")]
    pub stop_code: String,

    #[serde(alias = "kapino")]
    pub door_number: String,

    #[serde(alias = "ototip")]
    pub bus_type: String,

    // u8 used for binary flags (0 or 1)
    #[serde(alias = "engelli")]
    pub accessible: u8,

    #[serde(alias = "hatkodu")]
    pub line_code: String,

    #[serde(alias = "hattip")]
    pub line_type: String,

    #[serde(alias = "usb")]
    pub usb_available: u8,

    #[serde(alias = "wifi")]
    pub wifi_available: u8,

    #[serde(alias = "klima")]
    pub ac_active: u8,

    #[serde(alias = "bisiklet")]
    pub bicycle_rack: u8,

    #[serde(alias = "son_konum")]
    pub last_location: String,

    #[serde(alias = "son_hiz")]
    pub last_speed: String,

    #[serde(alias = "saat")]
    pub arrival_time: String,

    #[serde(alias = "dakika")]
    pub minutes_left: u32,

    #[serde(alias = "tahmintipi")]
    pub prediction_type: u32,

    #[serde(alias = "hatadi")]
    pub line_name: String,

    #[serde(alias = "sistemsaati")]
    pub system_time: String,

    #[serde(alias = "son_konum_saati")]
    pub last_location_time: String,

    #[serde(alias = "guzergah")]
    pub route_id: String,

    #[serde(alias = "durak_adi")]
    pub stop_name: String,

    #[serde(alias = "kaynak")]
    pub data_source: String,
}
