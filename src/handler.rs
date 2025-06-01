use anyhow::Result;
use chrono::Datelike;
use poise::serenity_prelude as serenity;
use serenity::{
    ComponentInteraction, Context, CreateButton, CreateInteractionResponseFollowup, EmojiId,
    FullEvent, GuildMemberUpdateEvent, Interaction, Message, ReactionType,
};

use crate::config::Storage;

pub async fn handle(
    ctx: &serenity::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Storage, anyhow::Error>,
    storage: &Storage,
) -> Result<()> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            tracing::info!("{} is connected!", data_about_bot.user.name);
        }

        FullEvent::Message { new_message } => {
            message(ctx, new_message).await?;
        }

        FullEvent::GuildMemberUpdate { event, .. } => {
            member_update(ctx, storage, event).await?;
        }

        FullEvent::InteractionCreate {
            interaction: Interaction::Component(interaction),
        } => {
            button_press(ctx, storage, interaction).await?;
        }

        _ => {}
    }

    Ok(())
}

async fn message(ctx: &Context, message: &Message) -> Result<()> {
    if message.author.bot {
        return Ok(());
    }

    let content = message.content.to_lowercase();

    if content.contains("uku3lig") {
        let reaction = ReactionType::Custom {
            animated: false,
            id: EmojiId::new(1007036728294527066),
            name: Some("uku".to_string()),
        };

        message.react(&ctx.http, reaction).await?;
    }

    if content.contains("gay") && chrono::Utc::now().month() == 6 {
        message
            .reply(&ctx.http, "<:gayge:1113614420715765881>")
            .await?;
    }

    Ok(())
}

async fn member_update(
    ctx: &Context,
    storage: &Storage,
    event: &GuildMemberUpdateEvent,
) -> Result<()> {
    let config = storage.get_config(event.guild_id).await?;

    if event.roles.contains(&config.autoban_role) {
        event
            .guild_id
            .ban_with_reason(ctx, &event.user, 1, "autobanned for having role")
            .await?;

        tracing::info!(
            "automatically banned user {} ({}) for having autoban role in guild {}",
            event.user.name,
            event.user.id,
            event.guild_id
        );
    }

    Ok(())
}

async fn button_press(
    ctx: &Context,
    storage: &Storage,
    interaction: &ComponentInteraction,
) -> Result<()> {
    for (custom_id, button) in crate::bot::BUTTONS.iter() {
        if interaction.data.custom_id == *custom_id {
            if let Err(e) = button.on_press(ctx, storage, interaction).await {
                tracing::error!("error handling '{}' button press: {}", custom_id, e);

                interaction
                    .create_followup(
                        ctx,
                        CreateInteractionResponseFollowup::new()
                            .content("An unknown error occurred.")
                            .ephemeral(true),
                    )
                    .await?;
            }
        }
    }

    Ok(())
}

#[poise::async_trait]
pub trait PersistentButton: Send + Sync {
    fn create(&self) -> CreateButton;

    async fn on_press(
        &self,
        ctx: &Context,
        data: &Storage,
        interaction: &ComponentInteraction,
    ) -> anyhow::Result<()>;
}
