use anyhow::Result;
use chrono::Datelike;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::id::EmojiId;
use serenity::model::misc::EmojiIdentifier;

pub async fn message(ctx: &Context, message: &Message) -> Result<()> {
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
