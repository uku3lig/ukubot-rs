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

pub async fn register_commands(ctx: &Context, cmds: &Vec<&'static dyn UkubotCommand>) {
    let commands = if let Ok(g) = env::var("GUILD_ID") {
        let guild_id = GuildId(g.parse().expect("Could not parse GUILD_ID"));
        guild_id
            .set_application_commands(&ctx.http, |c| register_commands_internal(c, cmds))
            .await
    } else {
        Command::set_global_application_commands(&ctx.http, |c| register_commands_internal(c, cmds))
            .await
    };

    match commands {
        Ok(c) => tracing::info!("Successfully registered {} commands", c.len()),
        Err(e) => tracing::error!("An error occurred while registering commands: {:?}", e),
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