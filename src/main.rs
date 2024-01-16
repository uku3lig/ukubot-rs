use std::env;

use anyhow::Result;
use poise::{
    serenity_prelude as serenity, CreateReply, Framework, FrameworkError, FrameworkOptions,
};
use serenity::{ClientBuilder, GatewayIntents};

mod bot;
mod config;
mod consts;
mod handler;

type Context<'a> = poise::Context<'a, (), anyhow::Error>;

#[tokio::main]
async fn main() {
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("an error occurred while loading .env: {:?}", e);
    }

    tracing_subscriber::fmt::init();

    let framework = poise::Framework::builder()
        .options(FrameworkOptions {
            commands: bot::commands(),
            event_handler: |ctx, event, framework, data| {
                Box::pin(handler::handle(ctx, event, framework, data))
            },
            on_error: |e| Box::pin(on_error(e)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| Box::pin(setup(ctx, framework)))
        .build();

    let mut client = ClientBuilder::new(
        env::var("UKUBOT_TOKEN").expect("missing UKUBOT_TOKEN"),
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,
    )
    .framework(framework)
    .await
    .unwrap();

    let manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");

        manager.shutdown_all().await;
    });

    if let Err(e) = client.start().await {
        tracing::error!("an error occurred while running the client: {:?}", e);
    }
}

async fn setup(ctx: &serenity::Context, framework: &Framework<(), anyhow::Error>) -> Result<()> {
    let commands = &framework.options().commands;

    if let Ok(g) = env_guild_id() {
        poise::builtins::register_in_guild(ctx, commands, g).await?;
        tracing::info!("registered {} commands in guild {}", commands.len(), g);
    } else {
        poise::builtins::register_globally(ctx, commands).await?;
        tracing::info!("registered {} commands globally", commands.len());
    }

    Ok(())
}

async fn on_error(error: FrameworkError<'_, (), anyhow::Error>) {
    match error {
        FrameworkError::Setup { error, .. } => {
            panic!("failed to start bot: {}", error);
        }
        FrameworkError::Command { error, ctx, .. } => {
            tracing::error!("command error: {}", error);

            if let Err(e) = ctx
                .send(
                    CreateReply::default()
                        .content("an unknown error occurred")
                        .ephemeral(true),
                )
                .await
            {
                tracing::error!("failed to send error message: {}", e);
            }
        }
        FrameworkError::UnknownInteraction { interaction, .. } => {
            tracing::warn!(
                "unknown interaction: {} (name: {})",
                interaction.id,
                interaction.data.name
            );
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                tracing::error!("error while handling an error: {}", e);
            }
        }
    }
}

fn env_guild_id() -> Result<serenity::GuildId> {
    let env = env::var("GUILD_ID")?;
    let id = env.parse::<u64>()?;

    Ok(serenity::GuildId::new(id))
}
