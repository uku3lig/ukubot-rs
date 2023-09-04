mod command;
mod handler;

use crate::command::{register_commands, PingCommand, UkubotCommand};
use serenity::builder::CreateApplicationCommand;
use serenity::client::{Context, EventHandler};
use serenity::framework::StandardFramework;
use serenity::model::application::command::Command;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Interaction;
use serenity::prelude::GatewayIntents;
use serenity::Client;
use std::env;
use std::sync::OnceLock;

static REGISTERED_COMMANDS: OnceLock<Vec<Command>> = OnceLock::new();

struct Handler(Vec<&'static dyn UkubotCommand>);

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if let Err(e) = handler::message(&ctx, &message).await {
            tracing::error!("An error occurred in message handler: {:?}", e);
            let _ = message.reply(&ctx.http, "An error occurred").await;
        }
    }

    async fn ready(&self, ctx: Context, data: Ready) {
        match register_commands(&ctx, &self.0).await {
            Ok(c) => {
                tracing::info!("Successfully registered {} commands", c.len());
                REGISTERED_COMMANDS.set(c).expect("Could not set COMMANDS");
            }
            Err(e) => tracing::error!("An error occurred while registering commands: {:?}", e),
        }

        tracing::info!("{} is connected!", data.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            tracing::info!("received command {}", command.data.name);
            for cmd in &self.0 {
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
        } // turn this into a match later
    }
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    let token = env::var("UKUBOT_TOKEN").expect("Could not load token from UKUBOT_TOKEN");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let framework = StandardFramework::new();

    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(Handler(vec![&PingCommand]))
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

fn get_cmd_name(cmd: &&dyn UkubotCommand) -> String {
    let mut com = CreateApplicationCommand::default();
    cmd.register(&mut com);

    com.0["name"].as_str().unwrap_or("").into()
}
