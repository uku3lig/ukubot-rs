use std::env;

use serenity::builder::{CreateApplicationCommand, CreateButton};
use serenity::client::{Context, EventHandler};
use serenity::framework::StandardFramework;
use serenity::model::application::component::ComponentType;
use serenity::model::application::interaction::Interaction;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::GatewayIntents;
use serenity::Client;

use crate::core::{register_commands, PersistentButton, SlashCommand};

mod bot;
mod config;
mod core;
mod handler;
mod util;

struct Handler {
    commands: &'static Vec<&'static dyn SlashCommand>,
    buttons: &'static Vec<&'static dyn PersistentButton>,
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if let Err(e) = handler::message(&ctx, &message).await {
            tracing::error!("An error occurred in message handler: {:?}", e);
            let _ = message.reply(&ctx.http, "An error occurred").await;
        }
    }

    async fn ready(&self, ctx: Context, data: Ready) {
        register_commands(&ctx, self.commands).await;

        tracing::info!("{} is connected!", data.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => {
                // don't accept dm commands
                if command.guild_id.is_none() {
                    return;
                }

                tracing::info!("received command {}", command.data.name);
                for cmd in self.commands {
                    if command.data.name == get_cmd_name(cmd) {
                        if let Err(e) = cmd.on_command(&ctx, &command).await {
                            tracing::error!("An error occurred in command handler: {:?}", e);
                            let _ = command
                                .create_interaction_response(&ctx.http, |r| {
                                    r.interaction_response_data(|d| d.content("An error occurred"))
                                })
                                .await;
                        }
                    }
                }
            }
            Interaction::MessageComponent(component) => {
                if component.data.component_type == ComponentType::Button {
                    tracing::info!("received button {}", component.data.custom_id);

                    for btn in self.buttons {
                        if component.data.custom_id == get_btn_id(btn) {
                            if let Err(e) = btn.on_press(&ctx, &component).await {
                                tracing::error!("An error occurred in button handler: {:?}", e);
                                let _ = component
                                    .create_interaction_response(&ctx.http, |r| {
                                        r.interaction_response_data(|d| {
                                            d.content("An error occurred")
                                        })
                                    })
                                    .await;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("An error occurred while loading .env: {:?}", e);
    }

    tracing_subscriber::fmt::init();

    let token = env::var("UKUBOT_TOKEN").expect("Could not load token from UKUBOT_TOKEN");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let framework = StandardFramework::new();

    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(Handler {
            commands: &bot::COMMANDS,
            buttons: &bot::BUTTONS,
        })
        .await
        .expect("Could not create client");

    let manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        manager.lock().await.shutdown_all().await;
    });

    if let Err(e) = client.start().await {
        tracing::error!("An error occurred while running the client: {:?}", e);
    }
}

fn get_cmd_name(cmd: &&dyn SlashCommand) -> String {
    let mut com = CreateApplicationCommand::default();
    cmd.register(&mut com);

    com.0["name"].as_str().unwrap_or_default().into()
}

fn get_btn_id(cmd: &&dyn PersistentButton) -> String {
    let mut btn = CreateButton::default();
    cmd.create(&mut btn);

    btn.0["custom_id"].as_str().unwrap_or_default().into()
}
