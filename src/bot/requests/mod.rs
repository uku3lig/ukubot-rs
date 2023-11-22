use poise::serenity_prelude as serenity;

pub mod export;
pub mod manage;
pub mod open;
pub mod ticket;

fn get_user_from_embed(embed: &serenity::CreateEmbed) -> anyhow::Result<serenity::UserId> {
    // this is fine :cdisaster:
    let footer = embed.0["footer"].as_object().unwrap();
    Ok(serenity::UserId(
        footer["text"].as_str().unwrap().parse::<u64>()?,
    ))
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

    Ok(serenity::ChannelId(id.parse::<u64>()?))
}

async fn dm_embed_to_user(
    ctx: &serenity::Context,
    embed: &serenity::CreateEmbed,
) -> anyhow::Result<()> {
    let user = get_user_from_embed(embed)?;
    let channel = user.create_dm_channel(ctx).await?;
    channel
        .send_message(ctx, |m| m.set_embed(embed.clone()))
        .await?;

    Ok(())
}
