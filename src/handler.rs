use anyhow::Result;
use chrono::Datelike;
use poise::serenity_prelude as serenity;
use serenity::{
    ComponentInteraction, Context, CreateButton, CreateInteractionResponseFollowup, EmojiId,
    FullEvent, Interaction, Message,
};

pub async fn handle(
    ctx: &serenity::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, (), anyhow::Error>,
    _: &(),
) -> Result<()> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            tracing::info!("{} is connected!", data_about_bot.user.name);
        }

        FullEvent::Message { new_message } => {
            message(ctx, new_message).await?;
        }

        FullEvent::InteractionCreate {
            interaction: Interaction::Component(interaction),
        } => {
            button_press(ctx, interaction).await?;
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

    if content.contains("kiyohime") {
        message
            .reply(&ctx.http, "<:kiyobean:739895868215263232>")
            .await?;
    }

    if content.contains("uku3lig") {
        message
            .react(&ctx.http, EmojiId::new(1007036728294527066))
            .await?;
    }

    if content.contains("gay") && chrono::Utc::now().month() == 6 {
        message
            .reply(&ctx.http, "<:gayge:1113614420715765881>")
            .await?;
    }

    Ok(())
}

async fn button_press(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    for (custom_id, button) in crate::bot::BUTTONS.iter() {
        if interaction.data.custom_id == *custom_id {
            if let Err(e) = button.on_press(ctx, interaction).await {
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
        interaction: &ComponentInteraction,
    ) -> anyhow::Result<()>;
}
