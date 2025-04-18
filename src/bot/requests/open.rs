use anyhow::anyhow;
use poise::{CreateReply, Modal, serenity_prelude as serenity};
use serenity::{
    ComponentInteraction, CreateActionRow, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, Timestamp,
};

use super::manage;
use crate::{Context, handler::PersistentButton};

/// opens the server for requests in the current channel
#[poise::command(slash_command, guild_only, required_permissions = "ADMINISTRATOR")]
pub async fn open_requests(ctx: Context<'_>) -> anyhow::Result<()> {
    let guild = ctx
        .guild_id()
        .ok_or(anyhow!("command must be run in a guild"))?;

    let config = ctx.data().get_config(guild).await?;
    let channels = guild.channels(&ctx).await?;

    let mut missing = vec![];

    if !channels.contains_key(&config.requests_channel) {
        missing.push("requests_channel");
    }

    if !channels.contains_key(&config.ticket_category) {
        missing.push("ticket_category");
    }

    if !channels.contains_key(&config.closed_category) {
        missing.push("closed_category");
    }

    if !channels.contains_key(&config.finished_channel) {
        missing.push("finished_channel");
    }

    if !missing.is_empty() {
        let missing = missing.join(", ");
        ctx.send(
            CreateReply::default()
                .content(format!("missing channels: {missing}"))
                .ephemeral(true),
        )
        .await?;

        return Ok(());
    }

    let avatar = ctx
        .framework()
        .bot_id
        .to_user(&ctx)
        .await?
        .avatar_url()
        .unwrap_or_default();

    let embed = CreateEmbed::new() .title("Request a mod/plugin")
    .description("Click the button below to request a mod or plugin.")
    .field(
        "‼️ Do not request an already made mod/plugin!",
        "Please make sure to double check my [Modrinth page](https://modrinth.com/user/HiuxcjYJ) to see what is already available.",
        false,
    )
    .field(
        "📚 Make sure to read the terms",
        "They are subject to be updated at any time, so please check them everytime you request something.",
        false,
    )
    .field(
        "🈲 Do not troll",
        "This one should be common sense, but we never know.",
        false,
    )
    .field(
        "🛑 Failure to respect those rules exposes you to being permanently blacklisted from requesting.",
        " - uku",
        false,
    )
    .color(0x9b59b6)
    .footer(CreateEmbedFooter::new("ukubot v7.2.7").icon_url(avatar));

    let components = vec![CreateActionRow::Buttons(vec![CreateRequestButton.create()])];

    ctx.channel_id()
        .send_message(
            &ctx,
            CreateMessage::new().embed(embed).components(components),
        )
        .await?;

    ctx.send(CreateReply::default().content("done!").ephemeral(true))
        .await?;

    Ok(())
}

pub struct CreateRequestButton;

#[derive(Modal)]
#[name = "Create Request"]
struct RequestModal {
    #[name = "Extended description of your idea"]
    #[min_length = 20]
    #[paragraph]
    mod_desc: String,
    #[name = "How much are you willing to pay?"]
    #[max_length = 50]
    amount: String,
    #[name = "Desired Minecraft version (if applicable)"]
    version: String,
    #[name = "(Optional) When do you need this by?"]
    #[max_length = 50]
    deadline: Option<String>,
}

#[poise::async_trait]
impl PersistentButton for CreateRequestButton {
    fn create(&self) -> serenity::CreateButton {
        serenity::CreateButton::new("open_mod_request")
            .label("Create a request")
            .emoji('📑')
    }

    async fn on_press(
        &self,
        ctx: &serenity::Context,
        data: &crate::config::Storage,
        interaction: &ComponentInteraction,
    ) -> anyhow::Result<()> {
        // unwrapping here is safe because the button will always be in a guild
        let config = data.get_config(interaction.guild_id.unwrap()).await?;

        if !config.requests_open {
            interaction
                .create_response(
                    ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("requests are currently closed. please try again later.")
                            .ephemeral(true),
                    ),
                )
                .await?;

            return Ok(());
        }

        let info: RequestModal = poise::modal::execute_modal_on_component_interaction(
            Box::new(ctx.clone()),
            interaction.clone(),
            None,
            None,
        )
        .await?
        .ok_or(anyhow!("could not parse modal response"))?;

        let user = &interaction.user;

        let embed = CreateEmbed::new()
            .author(
                CreateEmbedAuthor::new(&user.name).icon_url(user.avatar_url().unwrap_or_default()),
            )
            .field("Description", info.mod_desc, false)
            .field("Amount", info.amount, false)
            .field("Version", info.version, false)
            .field("Deadline", info.deadline.unwrap_or("None".into()), false)
            .timestamp(Timestamp::now())
            .footer(CreateEmbedFooter::new(user.id.get().to_string()));

        let components = vec![CreateActionRow::Buttons(vec![
            manage::AcceptRequestButton.create(),
            manage::RejectRequestButton.create(),
        ])];

        config
            .requests_channel
            .send_message(
                ctx,
                CreateMessage::new().embed(embed).components(components),
            )
            .await?;

        Ok(())
    }
}
