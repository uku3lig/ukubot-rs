use anyhow::Result;
use serde::{Deserialize, Serialize};
use serenity::model::id::{ChannelId, GuildId};
use std::collections::HashMap;

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
    pub fn get<T: Into<GuildId>>(id: T) -> Result<Self> {
        let conf = std::fs::read_to_string(CONFIG_FILE)?;
        let conf: HashMap<GuildId, GuildConfig> = toml::from_str(&conf)?;

        Ok(conf.get(&id.into()).cloned().unwrap_or_default())
    }

    pub fn save<T: Into<GuildId>>(&self, id: T) -> Result<()> {
        let conf = std::fs::read_to_string(CONFIG_FILE)?;
        let mut conf: HashMap<GuildId, GuildConfig> = toml::from_str(&conf)?;
        conf.insert(id.into(), self.clone());

        let conf = toml::to_string(&conf)?;
        std::fs::write("config.toml", conf)?;
        Ok(())
    }
}
