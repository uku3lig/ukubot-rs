use std::str::FromStr;

use anyhow::anyhow;
use poise::serenity_prelude as serenity;
use serenity::CreateMessage;

pub mod export;
pub mod manage;
pub mod open;
pub mod ticket;

// lets go guys we are so back i want to kill myself
fn get_user_from_embed(embed: &serenity::CreateEmbed) -> Option<serenity::UserId> {
    let value = serde_json::to_value(embed).ok()?;

    let id = value
        .as_object()?
        .get("footer")?
        .as_object()?
        .get("text")?
        .as_str()?;

    serenity::UserId::from_str(id).ok()
}

fn get_channel_from_embed(embed: &serenity::Embed) -> anyhow::Result<serenity::ChannelId> {
    let desc = embed
        .description
        .as_ref()
        .ok_or(anyhow::anyhow!("no description"))?;

    let id = desc
        .chars()
        .skip(2) // <#
        .take_while(|c| *c != '>')
        .collect::<String>();

    Ok(serenity::ChannelId::from_str(&id)?)
}

async fn dm_embed_to_user(
    ctx: &serenity::Context,
    embed: &serenity::CreateEmbed,
) -> anyhow::Result<()> {
    let user =
        get_user_from_embed(embed).ok_or_else(|| anyhow!("could not get user from embed"))?;

    let channel = user.create_dm_channel(ctx).await?;
    channel
        .send_message(ctx, CreateMessage::new().embed(embed.clone()))
        .await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use poise::serenity_prelude as serenity;

    #[test]
    fn test_user_from_embed() {
        let user = serenity::UserId::from(319463560356823050);
        let id_str = format!("{}", u64::from(user));

        let embed = serenity::CreateEmbed::new().footer(serenity::CreateEmbedFooter::new(id_str));
        let computed_user = super::get_user_from_embed(&embed);

        assert_eq!(computed_user, Some(user));
    }
}
