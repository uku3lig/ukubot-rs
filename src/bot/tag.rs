use anyhow::Result;
use indoc::indoc;
use poise::{serenity_prelude as serenity, CreateReply};
use serenity::CreateAllowedMentions;

use crate::Context;

#[poise::command(slash_command, user_cooldown = 5)]
pub async fn tag(ctx: Context<'_>, tag: Tags) -> Result<()> {
    let mentions = CreateAllowedMentions::default().replied_user(false);
    let builder = CreateReply::default().allowed_mentions(mentions);

    ctx.send(tag.make_reply(builder)).await?;

    Ok(())
}

#[derive(poise::ChoiceParameter)]
enum Tags {
    UkuButton,
    Payment,
    Standards,
}

impl Tags {
    fn make_reply(&self, builder: CreateReply) -> CreateReply {
        match self {
            Self::UkuButton => builder.content("https://media.discordapp.net/attachments/1049703332043837460/1049705678194876446/image.png"),
            Self::Payment => builder.content(indoc! {"
                * [PayPal](<https://paypal.me/uku3lig>) *(preferred)*
                * [GitHub Sponsors](<https://github.com/sponsors/uku3lig>)
                * [Ko-Fi](<https://ko-fi.com/uku3lig>)
            "}),
            Self::Standards => builder.content("https://xkcd.com/927")
        }
    }
}
