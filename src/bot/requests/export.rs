use crate::handler::PersistentButton;
use poise::serenity_prelude as serenity;
use serenity::futures::StreamExt;

pub struct ExportButton;

#[poise::async_trait]
impl PersistentButton for ExportButton {
    fn create<'a>(&self, button: &'a mut serenity::CreateButton) -> &'a mut serenity::CreateButton {
        button
            .custom_id("export_request")
            .label("Export request")
            .style(serenity::ButtonStyle::Primary)
    }

    async fn on_press(
        &self,
        ctx: &serenity::Context,
        interaction: &serenity::MessageComponentInteraction,
    ) -> anyhow::Result<()> {
        let channel = super::get_channel_from_embed(interaction.message.embeds.first().unwrap())?;
        let channel_name = channel
            .name(ctx)
            .await
            .ok_or(anyhow::anyhow!("no channel name"))?;

        interaction
            .create_interaction_response(ctx, |r| {
                r.interaction_response_data(|m| {
                    m.content(format!(
                        "exporting channel {}, please wait...",
                        channel_name
                    ))
                    .ephemeral(true)
                })
            })
            .await?;

        // messages here are in reverse order (newest first)
        let mut messages = channel.messages_iter(ctx).boxed();
        let mut export = String::new();

        while let Some(message) = messages.next().await {
            match message {
                Err(e) => tracing::warn!("could not export message: {}", e),
                Ok(message) => export = export_message(message) + &export,
            }
        }

        let guild_name = interaction.guild_id.unwrap().name(ctx).unwrap();
        let export = format!(
            "{}\nGuild: {}\nChannel: {}\n{0}\n\n{}",
            "=".repeat(20),
            guild_name,
            channel_name,
            export.trim_end()
        );

        let filename = format!("export-{}-{}.txt", channel_name, channel.0);

        interaction
            .user
            .create_dm_channel(ctx)
            .await?
            .send_message(ctx, |m| {
                m.content(format!("Log exports for {}", channel_name))
                    .add_file((export.as_bytes(), filename.as_str()))
            })
            .await?;

        let mut orig_msg = interaction.message.clone();
        orig_msg.edit(ctx, |m| m.components(|c| c)).await?;

        Ok(())
    }
}

fn export_message(message: serenity::Message) -> String {
    // header
    let mut export = format!("[{}] {}", message.timestamp, message.author.name);
    if message.pinned {
        export.push_str(" (pinned)");
    }
    export.push('\n');

    // content
    export.push_str(&message.content);
    export.push('\n');

    // attachments
    if !message.attachments.is_empty() {
        export.push_str("{Attachments}\n");
        for attachment in message.attachments {
            let s = format!("{} ({} bytes)\n", attachment.url, attachment.size);
            export.push_str(&s);
        }
        export.push('\n');
    }

    // embeds
    for embed in message.embeds {
        export.push_str("{Embed}\n");
        if let Some(author) = embed.author {
            let s = format!(
                "author: {} ({})\n",
                author.name,
                author.url.unwrap_or_default()
            );
            export.push_str(&s);
        }

        if let Some(url) = embed.url {
            let s = format!("url: {}\n", url);
            export.push_str(&s);
        }

        if let Some(title) = embed.title {
            let s = format!("title: {}\n", title);
            export.push_str(&s);
        }

        if let Some(description) = embed.description {
            let s = format!("{}\n", description);
            export.push_str(&s);
        }

        if !embed.fields.is_empty() {
            export.push('\n');
            for field in embed.fields {
                let s = format!("{}:\n{}\n\n", field.name, field.value);
                export.push_str(&s);
            }
        }

        if let Some(thumbnail) = embed.thumbnail {
            let s = format!("thumbnail: {}\n", thumbnail.url);
            export.push_str(&s);
        }

        if let Some(image) = embed.image {
            let s = format!("image: {}\n", image.url);
            export.push_str(&s);
        }

        if let Some(video) = embed.video {
            let s = format!("video: {}\n", video.url);
            export.push_str(&s);
        }

        if let Some(footer) = embed.footer {
            let s = format!("footer: {}\n", footer.text);
            export.push_str(&s);
        }

        export.push('\n');
    }

    if !message.sticker_items.is_empty() {
        export.push_str("{Stickers}\n");
        for sticker in message.sticker_items {
            let s = format!(
                "{} ({})\n",
                sticker.name,
                sticker.image_url().unwrap_or_default()
            );
            export.push_str(&s);
        }
        export.push('\n');
    }

    if !message.reactions.is_empty() {
        export.push_str("{Reactions}\n");
        for reaction in message.reactions {
            let name = match reaction.reaction_type {
                serenity::ReactionType::Custom {
                    name: Some(name), ..
                } => name,
                serenity::ReactionType::Unicode(s) => s,
                _ => "unknown".into(),
            };

            let s = format!("{} ({}) ", name, reaction.count);
            export.push_str(&s);
        }
        export.push('\n');
    }

    export.push('\n');

    export
}
