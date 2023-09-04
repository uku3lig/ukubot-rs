use serenity::builder::{CreateApplicationCommand, CreateApplicationCommands};
use serenity::client::Context;
use serenity::model::application::command::Command;
use serenity::model::id::GuildId;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use std::env;

#[serenity::async_trait]
pub trait UkubotCommand: Send + Sync {
    #[allow(clippy::mut_from_ref)]
    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand;

    async fn on_command(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) -> anyhow::Result<()>;
}

pub async fn register_commands(
    ctx: &Context,
    cmds: &Vec<&'static dyn UkubotCommand>,
) -> serenity::Result<Vec<Command>> {
    if let Ok(g) = env::var("GUILD_ID") {
        let guild_id = GuildId(g.parse().expect("Could not parse GUILD_ID"));
        GuildId::set_application_commands(&guild_id, &ctx.http, |c| {
            register_commands_internal(c, cmds)
        })
        .await
    } else {
        Command::set_global_application_commands(&ctx.http, |c| register_commands_internal(c, cmds))
            .await
    }
}

fn register_commands_internal<'a>(
    creator: &'a mut CreateApplicationCommands,
    commands: &Vec<&'static dyn UkubotCommand>,
) -> &'a mut CreateApplicationCommands {
    for cmd in commands {
        creator.create_application_command(|c| cmd.register(c));
    }

    creator
}

pub struct PingCommand;

#[serenity::async_trait]
impl UkubotCommand for PingCommand {
    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command.name("ping").description("Ping the bot")
    }

    async fn on_command(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) -> anyhow::Result<()> {
        interaction
            .create_interaction_response(&ctx.http, |r| {
                r.interaction_response_data(|d| d.content("Pong!"))
            })
            .await?;

        Ok(())
    }
}
