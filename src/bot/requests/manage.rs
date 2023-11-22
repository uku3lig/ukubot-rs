use std::sync::Arc;

use crate::config::GuildConfig;
use crate::consts;
use crate::handler::PersistentButton;
use anyhow::anyhow;
use poise::{serenity_prelude as serenity, Modal};
use serenity::{CreateButton, CreateEmbed, MessageComponentInteraction, PermissionOverwrite};

use super::ticket;

pub struct AcceptRequestButton;

#[poise::async_trait]
impl PersistentButton for AcceptRequestButton {
    fn create<'a>(&self, button: &'a mut CreateButton) -> &'a mut CreateButton {
        button
            .custom_id("accept_request")
            .label("Accept request")
            .style(serenity::ButtonStyle::Success)
    }

    async fn on_press(
        &self,
        ctx: &serenity::Context,
        interaction: &MessageComponentInteraction,
    ) -> anyhow::Result<()> {
        // unwrapping here is safe because the button will always be in a guild
        let config = GuildConfig::get(interaction.guild_id.unwrap());

        let mut embed: CreateEmbed = interaction.message.embeds.first().unwrap().clone().into();
        embed
            .title("Request Accepted")
            .color(consts::ACCEPTED_COLOR);

        let user = super::get_user_from_embed(&embed)?.to_user(ctx).await?;

        let ticket_channel = interaction
            .guild_id
            .unwrap()
            .create_channel(ctx, |c| {
                c.category(config.ticket_category)
                    .name(user.name)
                    .kind(serenity::ChannelType::Text)
                    .permissions(vec![PermissionOverwrite {
                        allow: serenity::Permissions::VIEW_CHANNEL,
                        deny: serenity::Permissions::empty(),
                        kind: serenity::PermissionOverwriteType::Member(user.id),
                    }])
            })
            .await?;

        embed.description(format!("<#{}>", ticket_channel.id.0));

        ticket_channel
            .send_message(ctx, |m| {
                m.content(format!("<@{}>", user.id.0))
                    .set_embed(embed.clone())
            })
            .await?;

        interaction
            .edit_original_message(ctx, |r| {
                r.interaction_response_data(|m| {
                    m.set_embed(embed).components(|c| {
                        c.create_action_row(|a| {
                            a.create_button(|b| ticket::FinishRequestButton.create(b))
                                .create_button(|b| ticket::DiscontinueRequestButton.create(b))
                        })
                    })
                })
            })
            .await?;

        interaction
            .create_followup_message(ctx, |r| {
                r.content(format!("<#{}>", ticket_channel.id.0))
                    .ephemeral(true)
            })
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
    fn create<'a>(&self, button: &'a mut CreateButton) -> &'a mut CreateButton {
        button
            .custom_id("reject_request")
            .label("Reject request")
            .style(serenity::ButtonStyle::Danger)
    }

    async fn on_press(
        &self,
        ctx: &serenity::Context,
        interaction: &MessageComponentInteraction,
    ) -> anyhow::Result<()> {
        let info: RejectModal = poise::modal::execute_modal_on_component_interaction(
            Box::new(ctx.clone()),
            Arc::new(interaction.clone()),
            None,
            None,
        )
        .await?
        .ok_or(anyhow!("could not parse modal response"))?;

        let mut embed: CreateEmbed = interaction.message.embeds.first().unwrap().clone().into();
        embed
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
            .edit_original_interaction_response(ctx, |m| m.set_embed(embed).components(|c| c))
            .await?;

        interaction
            .create_followup_message(ctx, |r| r.content(response).ephemeral(true))
            .await?;

        Ok(())
    }
}
