use std::num::ParseIntError;

use anyhow::anyhow;
use serenity::client::Context;
use serenity::json::Value;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::message_component::MessageComponentInteraction;

pub trait ParseSnowflake {
    fn parse_snowflake(&self) -> anyhow::Result<u64>;
}

impl ParseSnowflake for Option<Value> {
    fn parse_snowflake(&self) -> anyhow::Result<u64> {
        self.as_ref()
            .ok_or(anyhow!("value is empty"))?
            .as_str()
            .ok_or(anyhow!("value is not a string"))?
            .parse()
            .map_err(|e: ParseIntError| e.into())
    }
}

#[serenity::async_trait]
pub trait QuickInteractionReply {
    async fn reply<S>(&self, ctx: &Context, content: S) -> anyhow::Result<()>
    where
        S: ToString + Send + Sync;

    async fn reply_ephemeral<S>(&self, ctx: &Context, content: S) -> anyhow::Result<()>
    where
        S: ToString + Send + Sync;
}

#[serenity::async_trait]
impl QuickInteractionReply for ApplicationCommandInteraction {
    async fn reply<S>(&self, ctx: &Context, content: S) -> anyhow::Result<()>
    where
        S: ToString + Send + Sync,
    {
        self.create_interaction_response(&ctx.http, |r| {
            r.interaction_response_data(|d| d.content(content))
        })
        .await?;

        Ok(())
    }

    async fn reply_ephemeral<S>(&self, ctx: &Context, content: S) -> anyhow::Result<()>
    where
        S: ToString + Send + Sync,
    {
        self.create_interaction_response(&ctx.http, |r| {
            r.interaction_response_data(|d| d.content(content).ephemeral(true))
        })
        .await?;

        Ok(())
    }
}

#[serenity::async_trait]
impl QuickInteractionReply for MessageComponentInteraction {
    async fn reply<S>(&self, ctx: &Context, content: S) -> anyhow::Result<()>
    where
        S: ToString + Send + Sync,
    {
        self.create_interaction_response(&ctx.http, |r| {
            r.interaction_response_data(|d| d.content(content))
        })
        .await?;

        Ok(())
    }

    async fn reply_ephemeral<S>(&self, ctx: &Context, content: S) -> anyhow::Result<()>
    where
        S: ToString + Send + Sync,
    {
        self.create_interaction_response(&ctx.http, |r| {
            r.interaction_response_data(|d| d.content(content).ephemeral(true))
        })
        .await?;

        Ok(())
    }
}
