use config::{ConfigError, Environment};
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
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
            .add_source(Environment::default())
            .build()?
            .try_deserialize()
    }
}
