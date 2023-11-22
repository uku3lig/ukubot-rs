use std::{sync::Arc, vec};

use crate::{config::GuildConfig, consts, handler::PersistentButton};
use anyhow::anyhow;
use poise::{serenity_prelude as serenity, Modal};
use serenity::CreateEmbed;

use super::export;

pub struct FinishRequestButton;

#[derive(Modal)]
#[name = "Finish Request"]
struct FinishModal {
    #[name = "Artifact Link"]
    artifact_link: String,
    #[name = "Amount Received"]
    amount_received: String,
}

#[poise::async_trait]
impl PersistentButton for FinishRequestButton {
    fn create<'a>(&self, button: &'a mut serenity::CreateButton) -> &'a mut serenity::CreateButton {
        button
            .custom_id("finish_request")
            .label("Finish request")
            .style(serenity::ButtonStyle::Primary)
    }

    async fn on_press(
        &self,
        ctx: &serenity::Context,
        interaction: &serenity::MessageComponentInteraction,
    ) -> anyhow::Result<()> {
        let embed = close_request(
            ctx,
            interaction,
            "Request finished.",
            |e, m: FinishModal| {
                e.color(consts::FINISHED_COLOR)
                    .title("Request Finished")
                    .field("Artifact Link", m.artifact_link, false)
                    .field("Amount Received", m.amount_received, false)
            },
        )
        .await?;

        let config = GuildConfig::get(interaction.guild_id.unwrap());
        config
            .finished_channel
            .send_message(ctx, |m| m.set_embed(embed))
            .await?;

        Ok(())
    }
}

pub struct DiscontinueRequestButton;

#[derive(Modal)]
#[name = "Discontinue Request"]
struct DiscontinueModal {
    #[name = "Reason for discontinuation"]
    #[paragraph]
    reason: Option<String>,
}

#[poise::async_trait]
impl PersistentButton for DiscontinueRequestButton {
    fn create<'a>(&self, button: &'a mut serenity::CreateButton) -> &'a mut serenity::CreateButton {
        button
            .custom_id("discontinue_request")
            .label("Discontinue request")
            .style(serenity::ButtonStyle::Secondary)
    }

    async fn on_press(
        &self,
        ctx: &serenity::Context,
        interaction: &serenity::MessageComponentInteraction,
    ) -> anyhow::Result<()> {
        close_request(
            ctx,
            interaction,
            "Request discontinued.",
            |e, m: DiscontinueModal| {
                e.color(consts::DISCONTINUED_COLOR)
                    .title("Request Discontinued")
                    .field("Reason", m.reason.unwrap_or("None".into()), false)
            },
        )
        .await?;

        Ok(())
    }
}

async fn close_request<M: Modal, S: ToString>(
    ctx: &serenity::Context,
    interaction: &serenity::MessageComponentInteraction,
    action: S,
    embed_builder: impl FnOnce(&mut CreateEmbed, M) -> &mut CreateEmbed,
) -> anyhow::Result<CreateEmbed> {
    let config = GuildConfig::get(interaction.guild_id.unwrap());

    let info: M = poise::modal::execute_modal_on_component_interaction(
        Box::new(ctx.clone()),
        Arc::new(interaction.clone()),
        None,
        None,
    )
    .await?
    .ok_or(anyhow!("could not parse modal response"))?;

    let orig_embed = interaction.message.embeds.first().unwrap();
    let mut embed: CreateEmbed = orig_embed.clone().into();
    embed_builder(&mut embed, info);

    super::get_channel_from_embed(orig_embed)?
        .edit(ctx, |c| {
            c.category(config.closed_category).permissions(vec![])
        })
        .await?;

    let response = match super::dm_embed_to_user(ctx, &embed).await {
        Ok(_) => action.to_string(),
        Err(e) => {
            tracing::error!("could not send DM to user: {}", e);
            format!("{} Could not send DM to user: {}", action.to_string(), e)
        }
    };

    interaction
        .edit_original_interaction_response(ctx, |m| {
            m.set_embed(embed.clone()).components(|c| {
                c.create_action_row(|a| a.create_button(|b| export::ExportButton.create(b)))
            })
        })
        .await?;

    interaction
        .create_followup_message(ctx, |m| m.content(response).ephemeral(true))
        .await?;

    Ok(embed)
}
