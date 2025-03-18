use anyhow::anyhow;
use poise::{Modal, serenity_prelude as serenity};
use serenity::{
    CreateActionRow, CreateEmbed, CreateInteractionResponseFollowup, CreateMessage, EditChannel,
    EditInteractionResponse,
};

use super::export;
use crate::{consts, handler::PersistentButton};

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
    fn create(&self) -> serenity::CreateButton {
        serenity::CreateButton::new("finish_request")
            .label("Finish request")
            .style(serenity::ButtonStyle::Primary)
    }

    async fn on_press(
        &self,
        ctx: &serenity::Context,
        data: &crate::config::Storage,
        interaction: &serenity::ComponentInteraction,
    ) -> anyhow::Result<()> {
        let embed = close_request(
            ctx,
            data,
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

        let config = data.get_config(interaction.guild_id.unwrap()).await?;
        config
            .finished_channel
            .send_message(ctx, CreateMessage::new().embed(embed))
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
    fn create(&self) -> serenity::CreateButton {
        serenity::CreateButton::new("discontinue_request")
            .label("Discontinue request")
            .style(serenity::ButtonStyle::Secondary)
    }

    async fn on_press(
        &self,
        ctx: &serenity::Context,
        data: &crate::config::Storage,
        interaction: &serenity::ComponentInteraction,
    ) -> anyhow::Result<()> {
        let _ = close_request(
            ctx,
            data,
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
    data: &crate::config::Storage,
    interaction: &serenity::ComponentInteraction,
    action: S,
    embed_builder: impl FnOnce(CreateEmbed, M) -> CreateEmbed,
) -> anyhow::Result<CreateEmbed> {
    let config = data.get_config(interaction.guild_id.unwrap()).await?;

    let info: M = poise::modal::execute_modal_on_component_interaction(
        Box::new(ctx.clone()),
        interaction.clone(),
        None,
        None,
    )
    .await?
    .ok_or(anyhow!("could not parse modal response"))?;

    let orig_embed = interaction.message.embeds.first().unwrap();
    let embed = embed_builder(orig_embed.clone().into(), info);

    super::get_channel_from_embed(orig_embed)?
        .edit(
            ctx,
            EditChannel::new()
                .category(config.closed_category)
                .permissions(vec![]),
        )
        .await?;

    let response = match super::dm_embed_to_user(ctx, &embed).await {
        Ok(_) => action.to_string(),
        Err(e) => {
            tracing::error!("could not send DM to user: {}", e);
            format!("{} Could not send DM to user: {}", action.to_string(), e)
        }
    };

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new()
                .embed(embed.clone())
                .components(vec![CreateActionRow::Buttons(vec![
                    export::ExportButton.create(),
                ])]),
        )
        .await?;

    interaction
        .create_followup(
            ctx,
            CreateInteractionResponseFollowup::new()
                .content(response)
                .ephemeral(true),
        )
        .await?;

    Ok(embed)
}
