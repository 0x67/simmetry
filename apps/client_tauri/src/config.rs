use std::{path::PathBuf, sync::LazyLock};

use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{error::ClientResult, global::APP_CONFIG_DIR};

use once_cell::sync::Lazy;
use std::env;

#[derive(Debug, Clone)]
pub struct Env {
    pub sentry_dsn: String,
}

pub static ENV: Lazy<Env> = Lazy::new(|| Env {
    sentry_dsn: env::var("SENTRY_DSN").unwrap_or_else(|_| "Missing env: SENTRY_DSN".to_string()),
});

pub fn get_env() -> &'static Env {
    &ENV
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
enum DarkMode {
    Dark,
    Light,
    #[default]
    System,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    dark_mode: DarkMode,
    #[serde(default = "default_transparent")]
    pub transparent: bool,
}

fn default_transparent() -> bool {
    true
}

impl Config {
    pub fn read_from_file() -> ClientResult<Config> {
        if !CONFIG_FILE_PATH.exists() {
            debug!("Config file not found, creating default config");
            let default_config = Config {
                transparent: default_transparent(),
                ..Default::default()
            };
            let config = serde_json::to_string(&default_config).map_err(|e| {
                error!("Failed to serialize default config: {:?}", e);
                e
            })?;
            std::fs::write(&*CONFIG_FILE_PATH, config).map_err(|e| {
                error!("Failed to write default config file: {:?}", e);
                e
            })?;
            info!("Created and wrote default config file");
            return Ok(default_config);
        }

        debug!("Reading config file from: {:?}", *CONFIG_FILE_PATH);
        let data = std::fs::read_to_string(&*CONFIG_FILE_PATH).map_err(|e| {
            error!("Failed to read config file: {:?}", e);
            e
        })?;

        let config: Config = serde_json::from_str(&data).map_err(|e| {
            error!("Failed to deserialize config file: {:?}", e);
            e
        })?;

        info!("Successfully read and parsed config file");

        Ok(config)
    }
}

static CONFIG_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| APP_CONFIG_DIR.join("simmetry.config.json"));

#[tauri::command]
pub async fn read_config_file() -> ClientResult<Config> {
    Config::read_from_file()
}

#[tauri::command]
pub async fn write_config_file(config: Config) -> ClientResult<()> {
    debug!("Writing new config: {:?}", config);
    let data = serde_json::to_string(&config).map_err(|e| {
        error!("Failed to serialize config: {:?}", e);
        e
    })?;

    fs::write(&*CONFIG_FILE_PATH, data).await.map_err(|e| {
        error!("Failed to write config file: {:?}", e);
        e
    })?;

    info!("Successfully wrote new config to file");

    Ok(())
}
