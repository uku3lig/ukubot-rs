use anyhow::Result;
use chrono::Datelike;
use poise::{serenity_prelude as serenity, Event};
use serenity::{
    Context, CreateButton, EmojiId, EmojiIdentifier, Interaction, Message,
    MessageComponentInteraction,
};

pub async fn handle(
    ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, (), anyhow::Error>,
    _: &(),
) -> Result<()> {
    match event {
        Event::Ready { data_about_bot } => {
            tracing::info!("{} is connected!", data_about_bot.user.name);
        }

        Event::Message { new_message } => {
            message(ctx, new_message).await?;
        }

        Event::InteractionCreate {
            interaction: Interaction::MessageComponent(interaction),
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
        let emoji = EmojiIdentifier {
            id: EmojiId(1007036728294527066),
            name: "uku".into(),
            animated: false,
        };

        message.react(&ctx.http, emoji).await?;
    }

    if content.contains("gay") && chrono::Utc::now().month() == 6 {
        message
            .reply(&ctx.http, "<:gayge:1113614420715765881>")
            .await?;
    }

    Ok(())
}

async fn button_press(ctx: &Context, interaction: &MessageComponentInteraction) -> Result<()> {
    for (custom_id, button) in crate::bot::BUTTONS.iter() {
        if interaction.data.custom_id == *custom_id {
            button.on_press(ctx, interaction).await?;
        }
    }

    Ok(())
}

#[serenity::async_trait]
pub trait PersistentButton: Send + Sync {
    fn create<'a>(&self, button: &'a mut CreateButton) -> &'a mut CreateButton;

    async fn on_press(
        &self,
        ctx: &Context,
        interaction: &MessageComponentInteraction,
    ) -> anyhow::Result<()>;
}
