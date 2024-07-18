use anyhow::anyhow;
use poise::{serenity_prelude as serenity, Modal};
use serenity::{
    ComponentInteraction, CreateActionRow, CreateChannel, CreateEmbed,
    CreateInteractionResponseFollowup, CreateMessage, EditInteractionResponse, Mentionable,
    PermissionOverwrite,
};

use super::ticket;
use crate::consts;
use crate::handler::PersistentButton;

pub struct AcceptRequestButton;

#[poise::async_trait]
impl PersistentButton for AcceptRequestButton {
    fn create(&self) -> serenity::CreateButton {
        serenity::CreateButton::new("accept_request")
            .label("Accept request")
            .style(serenity::ButtonStyle::Success)
    }

    async fn on_press(
        &self,
        ctx: &serenity::Context,
        data: &crate::config::Storage,
        interaction: &ComponentInteraction,
    ) -> anyhow::Result<()> {
        interaction.defer(ctx).await?;

        // unwrapping here is safe because the button will always be in a guild
        let config = data.get_config(interaction.guild_id.unwrap()).await?;

        let embed: CreateEmbed = interaction.message.embeds.first().unwrap().clone().into();
        let embed = embed
            .title("Request Accepted")
            .color(consts::ACCEPTED_COLOR);

        let user = super::get_user_from_embed(&embed)
            .ok_or_else(|| anyhow!("user not found in embed"))?
            .to_user(ctx)
            .await?;

        let mention = user.mention().to_string();

        let ticket_channel = interaction
            .guild_id
            .unwrap()
            .create_channel(
                ctx,
                CreateChannel::new(user.name)
                    .category(config.ticket_category)
                    .kind(serenity::ChannelType::Text)
                    .permissions(vec![PermissionOverwrite {
                        allow: serenity::Permissions::VIEW_CHANNEL,
                        deny: serenity::Permissions::empty(),
                        kind: serenity::PermissionOverwriteType::Member(user.id),
                    }]),
            )
            .await?;

        let embed = embed.description(ticket_channel.mention().to_string());

        ticket_channel
            .send_message(
                ctx,
                CreateMessage::new().content(mention).embed(embed.clone()),
            )
            .await?;

        let components = vec![CreateActionRow::Buttons(vec![
            ticket::FinishRequestButton.create(),
            ticket::DiscontinueRequestButton.create(),
        ])];

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .embed(embed)
                    .components(components),
            )
            .await?;

        interaction
            .create_followup(
                ctx,
                CreateInteractionResponseFollowup::new()
                    .content(ticket_channel.mention().to_string())
                    .ephemeral(true),
            )
            .await?;

        Ok(())
    }
}

pub struct RejectRequestButton;

#[derive(Modal)]
#[name = "Reject Request"]
struct RejectModal {
    #[name = "Reason for rejection"]
    #[paragraph]
    reason: Option<String>,
}

#[poise::async_trait]
impl PersistentButton for RejectRequestButton {
    fn create(&self) -> serenity::CreateButton {
        serenity::CreateButton::new("reject_request")
            .label("Reject request")
            .style(serenity::ButtonStyle::Danger)
    }

    async fn on_press(
        &self,
        ctx: &serenity::Context,
        _: &crate::config::Storage,
        interaction: &ComponentInteraction,
    ) -> anyhow::Result<()> {
        let info: RejectModal = poise::modal::execute_modal_on_component_interaction(
            Box::new(ctx.clone()),
            interaction.clone(),
            None,
            None,
        )
        .await?
        .ok_or(anyhow!("could not parse modal response"))?;

        let embed: CreateEmbed = interaction.message.embeds.first().unwrap().clone().into();
        let embed = embed
            .color(consts::REJECTED_COLOR)
            .field("Reason", info.reason.unwrap_or("None".into()), false)
            .title("Request Rejected");

        let response = match super::dm_embed_to_user(ctx, &embed).await {
            Ok(_) => "Request rejected.".into(),
            Err(e) => {
                tracing::error!("could not send DM to user: {}", e);
                format!("Request rejected. Could not send DM to user: {}", e)
            }
        };

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .embed(embed)
                    .components(vec![]),
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

        Ok(())
    }
}
