use std::{error::Error, str::FromStr};

use tracing_subscriber::{
    filter::{Directive, LevelFilter},
    EnvFilter,
};

use super::LogConfig;

pub fn init(config: &LogConfig) -> Result<(), Box<dyn Error + Send + Sync>> {
    let default_level = LevelFilter::from_str(config.lib())?;

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(Directive::from(default_level))
                .add_directive(directive("lazy_permission", config.app())?),
        )
        .init();

    Ok(())
}

fn directive(app_name: &str, level: &str) -> Result<Directive, Box<dyn Error + Send + Sync>> {
    let capacity = app_name.len() + 1 + level.len();

    let mut directive = String::with_capacity(capacity);
    directive.push_str(app_name);
    directive.push('=');
    directive.push_str(level);

    Ok(Directive::from_str(&directive)?)
}
