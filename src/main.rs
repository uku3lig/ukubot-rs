use std::env;

use anyhow::Result;
use config::Storage;
use poise::{
    serenity_prelude as serenity, CreateReply, Framework, FrameworkError, FrameworkOptions,
};
use serenity::{ClientBuilder, GatewayIntents};
use tokio::signal::{
    ctrl_c,
    unix::{signal, SignalKind},
};

mod bot;
mod config;
mod consts;
mod handler;

type Context<'a> = poise::Context<'a, Storage, anyhow::Error>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("an error occurred while loading .env: {e:?}");
    }

    tracing_subscriber::fmt::init();

    let options = FrameworkOptions {
        commands: bot::commands(),
        event_handler: |ctx, event, framework, data| {
            Box::pin(handler::handle(ctx, event, framework, data))
        },
        on_error: |e| Box::pin(on_error(e)),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .setup(|ctx, _ready, framework| Box::pin(setup(ctx, framework)))
        .build();

    let mut client = ClientBuilder::new(
        env::var("UKUBOT_TOKEN").expect("missing UKUBOT_TOKEN"),
        GatewayIntents::non_privileged()
            | GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_MEMBERS,
    )
    .framework(framework)
    .await?;

    let manager = client.shard_manager.clone();

    let mut sigterm = signal(SignalKind::terminate())?;

    // waits for one of the futures to complete
    tokio::select! {
        result = client.start() => result.map_err(anyhow::Error::from),
        _ = sigterm.recv() => {
            manager.shutdown_all().await;
            std::process::exit(0);
        }
        _ = ctrl_c() => {
            tracing::warn!("received SIGINT, shutting down...");
            manager.shutdown_all().await;
            std::process::exit(130);
        }
    }
}

async fn setup(
    ctx: &serenity::Context,
    framework: &Framework<Storage, anyhow::Error>,
) -> Result<Storage> {
    let storage = config::Storage::from_env()?;

    let commands = &framework.options().commands;

    if let Ok(g) = env_guild_id() {
        poise::builtins::register_in_guild(ctx, commands, g).await?;
        tracing::info!("registered {} commands in guild {}", commands.len(), g);
    } else {
        poise::builtins::register_globally(ctx, commands).await?;
        tracing::info!("registered {} commands globally", commands.len());
    }

    Ok(storage)
}

async fn on_error(error: FrameworkError<'_, Storage, anyhow::Error>) {
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
