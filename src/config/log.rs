use envy::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub app: Box<str>,
    pub lib: Box<str>,
}

impl LogConfig {
    pub(super) fn new() -> Result<Self, Error> {
        envy::prefixed("LOG_").from_env::<LogConfig>()
    }
}
