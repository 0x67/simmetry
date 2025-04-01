use config::{Config, ConfigError, Environment};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub redis_url: String,
    pub database_url: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_port() -> u16 {
    3002
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(Environment::default().separator("__")) // Reads from environment variables
            .build()?;

        let config: Self = settings.try_deserialize()?;

        // Explicit validation
        AppConfig::validate(&config)?;

        Ok(config)
    }

    fn validate(config: &AppConfig) -> Result<(), ConfigError> {
        if config.redis_url.is_empty() {
            return Err(ConfigError::Message(
                "Missing required env: REDIS_URL".to_string(),
            ));
        }
        if config.database_url.is_empty() {
            return Err(ConfigError::Message(
                "Missing required env: DATABASE_URL".to_string(),
            ));
        }
        Ok(())
    }
}

pub static APP_CONFIG: Lazy<Arc<AppConfig>> = Lazy::new(|| {
    let config = AppConfig::from_env().expect("Failed to load environment variables");
    Arc::new(config)
});
