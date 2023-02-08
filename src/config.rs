use std::fs;

use serde_derive::Deserialize;
use toml;

use crate::common::{self, Error};

#[derive(Deserialize, Debug)]
pub struct BlocklistConfig {
    pub name: String,
    pub url: String,
    pub update: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub blocklist: Vec<BlocklistConfig>,
}

pub fn parse_config_file(config_file: String) -> Result<Config, common::Error> {
    let contents = match fs::read_to_string(config_file) {
        Ok(c) => c,
        Err(err) => {
            return Err(Error::FailedToReadConfig {
                err: err.to_string(),
            })
        }
    };

    let config: Config = match toml::from_str(&contents) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to parse, returning blank config: {}", e);
            Config {
                blocklist: Vec::new(),
            }
        }
    };

    println!("Config Contaings {} Sources", config.blocklist.len());

    return Ok(config);
}
