use super::LogConfig;

pub fn init(config: &LogConfig) {
    tracing_subscriber::fmt()
        .with_max_level(config.level)
        .init()
}
