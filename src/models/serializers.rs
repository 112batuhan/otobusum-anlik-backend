use serde::{de, Deserialize, Deserializer};

pub fn deserialize_f64_from_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.chars()
        .filter(|&c| c != '.' && c != ' ')
        .map(|c| if c == ',' { '.' } else { c })
        .collect::<String>()
        .parse::<f64>()
        .map_err(de::Error::custom)
}
pub fn deserialize_u32_from_string<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<u32>().map_err(de::Error::custom)
}
