mod discord;
mod log;

use dotenv::{dotenv, Error as DotenvError};
use envy::Error as EnvyError;
use thiserror::Error as ThisError;

pub use discord::DiscordConfig;
pub use log::LogConfig;

#[derive(Debug)]
pub struct Config {
    pub discord: DiscordConfig,
    pub log: LogConfig,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        if let Err(error) = dotenv() {
            return Err(ConfigError::EnvFile(error));
        }

        Ok(Self {
            discord: DiscordConfig::new().map_err(ConfigError::Discord)?,
            log: LogConfig::new().map_err(ConfigError::Log)?,
        })
    }
}

#[derive(Debug, ThisError)]
pub enum ConfigError {
    #[error("Invalid DiscordConfig: {0}")]
    Discord(EnvyError),
    #[error("Cannot load the .env file: {0}")]
    EnvFile(DotenvError),
    #[error("Invalid LogConfig: {0}")]
    Log(EnvyError),
}
