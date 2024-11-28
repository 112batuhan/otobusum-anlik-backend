use anyhow::Result;
use serde::de::DeserializeOwned;
use std::io::Cursor;

// pub fn remove_non_numeric<'de, D>(deserializer: D) -> Result<u32, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s: String = Deserialize::deserialize(deserializer)?;
//     let numeric_only: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
//     let number = numeric_only.parse::<u32>().map_err(de::Error::custom)?;
//     Ok(number)
// }

pub fn read_csv_from_string<T: DeserializeOwned>(data: &str) -> Result<Vec<T>> {
    let mut reader = csv::Reader::from_reader(Cursor::new(data));
    let mut data = Vec::new();

    for result in reader.deserialize() {
        match result {
            Ok(line) => data.push(line),
            Err(err) => {
                println!("{}", err);
                continue;
            }
        }
    }
    Ok(data)
}
