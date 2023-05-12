use envy::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    app: Box<str>,
    lib: Box<str>,
}

impl LogConfig {
    pub(super) fn new() -> Result<Self, Error> {
        envy::prefixed("LOG_").from_env::<LogConfig>()
    }

    pub const fn app(&self) -> &str {
        &self.app
    }

    pub const fn lib(&self) -> &str {
        &self.lib
    }
}
