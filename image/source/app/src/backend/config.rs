use anyhow::anyhow;
use std::fs::read_to_string;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub music: frand_home_music::backend::config::Config,
}

impl Config {
    pub fn read(name: &str) -> anyhow::Result<Self> {
        let path = format!("./config/{name}.toml");
        let config = read_to_string(&path)
        .map_err(|err| anyhow!("Failed to read config file path: {path} err: {err}"))?;
            
        let config = toml::from_str::<Self>(&config)
        .map_err(|err| anyhow!("Failed to deserialize config file path: {path} err: {err}"))?;
                
        Ok(config)
    }
}