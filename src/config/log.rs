use std::ops::Deref;
use std::str::FromStr;

use envy::Error;
use serde::{de, Deserialize};
use tracing::Level;

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    #[serde(deserialize_with = "from_str_level")]
    pub level: Level,
}

impl LogConfig {
    pub(super) fn new() -> Result<Self, Error> {
        envy::prefixed("LOG_").from_env::<LogConfig>()
    }
}

fn from_str_level<'de, D>(deserializer: D) -> Result<Level, D::Error>
where
    D: de::Deserializer<'de>,
{
    let level: Box<str> = Deserialize::deserialize(deserializer)?;
    Level::from_str(level.deref()).map_err(de::Error::custom)
}
