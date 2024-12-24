#![allow(non_camel_case_types)]

use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum City {
    izmir,
    istanbul,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl City {
    pub fn as_str(&self) -> &'static str {
        match self {
            City::istanbul => "istanbul",
            City::izmir => "izmir",
        }
    }
}
