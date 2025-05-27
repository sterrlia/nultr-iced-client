use once_cell::sync::Lazy;
use serde::Deserialize;
use url::Url;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Variables {
    pub ws_url: Url
}

pub static VARIABLES: Lazy<Variables> = Lazy::new(|| {
    let config_str = fs::read_to_string("config.toml").expect("Failed to read config.toml");
    toml::from_str(&config_str).expect("Failed to parse config")
});

pub fn get_variables() -> &'static Variables {
    &VARIABLES
}

