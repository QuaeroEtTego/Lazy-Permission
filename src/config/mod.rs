mod discord;
mod log;

use std::{error::Error, fmt::{Display, Formatter, Result as FmtResult}};

use dotenv::{dotenv, Error as DotenvError};
use envy::Error as EnvyError;

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

#[derive(Debug)]
pub enum ConfigError {
    Discord(EnvyError),
    EnvFile(DotenvError),
    Log(EnvyError),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ConfigError::Discord(error) => {
                f.write_str("Invalid DiscordConfig ")?;
                Display::fmt(error, f)
            }
            ConfigError::EnvFile(error) => {
                f.write_str("Cannot load the .env file ")?;
                Display::fmt(error, f)
            }
            ConfigError::Log(error) => {
                f.write_str("Invalid LogConfig ")?;
                Display::fmt(error, f)
            }
        }
    }
}

impl Error for ConfigError {}
