use envy::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    #[serde(default = "default_level")]
    level: Box<str>,
}

impl LogConfig {
    pub(super) fn new() -> Result<Self, Error> {
        envy::prefixed("LOG_").from_env::<LogConfig>()
    }

    pub fn level(&self) -> &str {
        self.level.as_ref()
    }
}

fn default_level() -> Box<str> {
    Box::from("INFO")
}
