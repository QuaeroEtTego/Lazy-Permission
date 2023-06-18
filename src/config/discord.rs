use std::fmt::{Debug, Formatter};

use envy::Error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DiscordConfig {
    token: Box<str>,
}

impl DiscordConfig {
    pub(super) fn new() -> Result<Self, Error> {
        envy::prefixed("DISCORD_").from_env()
    }

    pub const fn token(&self) -> &str {
        &self.token
    }
}

impl Debug for DiscordConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DiscordConfig")
            .field("token", &"hidden")
            .finish()
    }
}
