use std::str::FromStr;
use tracing::Level;

use super::LogConfig;

pub struct Logger;

impl Logger {
    pub fn init(config: &LogConfig) {
        let level = Level::from_str(config.level()).unwrap_or(Level::INFO);

        tracing_subscriber::fmt().with_max_level(level).init()
    }
}
