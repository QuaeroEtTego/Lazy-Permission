use envy::Error;
use serde::Deserialize;
use std::fmt::{Debug, Formatter};

#[derive(Deserialize)]
pub struct DiscordConfig {
    token: Box<str>,
}

impl DiscordConfig {
    pub(super) fn new() -> Result<Self, Error> {
        envy::prefixed("DISCORD_").from_env::<DiscordConfig>()
    }

    pub fn token(&self) -> &str {
        self.token.as_ref()
    }
}

impl Debug for DiscordConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Token: HIDDEN")
    }
}
