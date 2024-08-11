use anyhow::{bail, Context, Error};
use serde_derive::Deserialize;
use std::str::FromStr;

pub enum RunMode {
    Production,
    Development,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
    pub connections: u32,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: String,
}

#[derive(Debug, Deserialize)]
pub struct FirebaseConfig {
    pub key: String,
    pub url: String,
}


#[derive(Debug, Deserialize)]
pub struct GeminiConfig {
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub database: Database,
    pub server: Server,
    pub firebase_config: FirebaseConfig,
    pub gemini_config: GeminiConfig,
}

impl FromStr for RunMode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "production" => Ok(RunMode::Production),
            "development" => Ok(RunMode::Development),
            _ => bail!("parsing run mode error, valid input development|production"),
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, Error> {
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        RunMode::from_str(&run_mode)?;

        let s = config::Config::builder()
            .add_source(config::File::with_name("configs/default"))
            .add_source(config::File::with_name(&format!("config/{}", run_mode)).required(false))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        let settings = s
            .try_deserialize()
            .map_err(anyhow::Error::new)
            .context("failed to deserialize")?;
        Ok(settings)
    }
}
