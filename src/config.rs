use anyhow::Result;
use poise::serenity_prelude as serenity;
use redis::{AsyncCommands, Client, ConnectionLike};
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use serenity::model::id::{ChannelId, GuildId, RoleId};

#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
#[serde(default)]
pub struct GuildConfig {
    pub requests_open: bool,
    pub requests_channel: ChannelId,
    pub ticket_category: ChannelId,
    pub closed_category: ChannelId,
    pub finished_channel: ChannelId,
    pub autoban_role: RoleId,
}

pub struct Storage {
    redis: Client,
}

impl Storage {
    pub fn from_env() -> Result<Self> {
        let url = std::env::var("REDIS_URL")?;
        let mut redis = Client::open(url.clone())?;

        if !redis.check_connection() {
            anyhow::bail!("failed to connect to redis at {url}");
        } else {
            tracing::info!("successfully connected to redis at {url}");
        }

        Ok(Self { redis })
    }

    pub async fn get_config(&self, id: GuildId) -> Result<GuildConfig> {
        let mut con = self.redis.get_multiplexed_async_connection().await?;
        let config: GuildConfig = con.get(u64::from(id)).await.unwrap_or_default();

        tracing::debug!("successfully fetched config for guild {id}");

        Ok(config)
    }

    pub async fn save_config(&self, id: GuildId, config: GuildConfig) -> Result<()> {
        let mut con = self.redis.get_multiplexed_async_connection().await?;
        con.set(u64::from(id), config).await?;

        tracing::debug!("successfully saved config for guild {id}");

        Ok(())
    }
}
