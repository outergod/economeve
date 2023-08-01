use config::{ConfigError, Environment, File, FileFormat};
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub characters: Vec<String>,
    pub database_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub scopes: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();

        config::Config::builder()
            .add_source(File::new("config.toml", FileFormat::Toml))
            .add_source(Environment::default())
            .build()?
            .try_deserialize()
    }
}
