use dotenvy::{dotenv, Error as DotenvError};
use envy::Error as EnvyError;
use thiserror::Error as ThisError;

pub use self::discord::DiscordConfig;
pub use self::log::LogConfig;

mod discord;
mod log;

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
    #[error("Invalid DiscordConfig {0}")]
    Discord(EnvyError),
    #[error("Cannot load the .env file {0}")]
    EnvFile(DotenvError),
    #[error("Invalid LogConfig {0}")]
    Log(EnvyError),
}
