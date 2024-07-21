use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::config_serializer::serialize_config;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub ip: String,
    pub packages: HashMap<String, Packages>,
}

#[derive(Deserialize, Serialize)]
pub struct Packages {
    pub url: String,
}

impl Config {
    pub fn toml(&self) -> String {
        serialize_config(self)
    }
}