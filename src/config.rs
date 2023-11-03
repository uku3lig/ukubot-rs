use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serenity::model::id::{ChannelId, GuildId};

pub const CONFIG_FILE: &str = "ukubot_config.toml";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GuildConfig {
    pub requests_open: bool,
    pub form_channel: ChannelId,
    pub ticket_category: ChannelId,
    pub closed_category: ChannelId,
    pub finished_channel: ChannelId,
}

impl GuildConfig {
    fn read() -> Result<HashMap<GuildId, GuildConfig>> {
        let conf = match std::fs::read_to_string(CONFIG_FILE) {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("Could not read config file: {:?}", e);
                anyhow::bail!(e);
            }
        };

        Ok(toml::from_str(&conf)?)
    }

    pub fn get<T: Into<GuildId>>(id: T) -> Self {
        let conf = GuildConfig::read().unwrap_or_default();

        conf.get(&id.into()).cloned().unwrap_or_default()
    }

    pub fn save<T: Into<GuildId>>(&self, id: T) -> Result<()> {
        let mut conf = GuildConfig::read().unwrap_or_default();
        conf.insert(id.into(), self.clone());

        let conf = toml::to_string(&conf)?;
        std::fs::write(CONFIG_FILE, conf)?;
        Ok(())
    }
}
