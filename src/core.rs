use std::env;

use serenity::builder::{CreateApplicationCommand, CreateApplicationCommands, CreateButton};
use serenity::client::Context;
use serenity::model::application::command::Command;
use serenity::model::id::GuildId;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::message_component::MessageComponentInteraction;

#[serenity::async_trait]
pub trait SlashCommand: Send + Sync {
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

pub async fn register_commands(ctx: &Context, cmds: &Vec<&'static dyn SlashCommand>) {
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
    commands: &Vec<&'static dyn SlashCommand>,
) -> &'a mut CreateApplicationCommands {
    for cmd in commands {
        creator.create_application_command(|c| cmd.register(c));
    }

    creator
}

#[serenity::async_trait]
pub trait PersistentButton: Send + Sync {
    fn create<'a>(&self, button: &'a mut CreateButton) -> &'a mut CreateButton;

    async fn on_press(
        &self,
        ctx: &Context,
        interaction: &MessageComponentInteraction,
    ) -> anyhow::Result<()>;
}
