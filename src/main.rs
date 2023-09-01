use serenity::client::bridge::gateway::ShardManager;
use serenity::client::{Context, EventHandler};
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::GatewayIntents;
use serenity::Client;

use std::env;
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.author.bot {
            return;
        }

        if message.content.contains("miguel") {
            message.reply(&ctx.http, "miguel :3").await.unwrap();
        }

        if message.content.contains("shutdown") {
            message.reply(&ctx.http, "shutting down...").await.unwrap();
            MANAGER.get().unwrap().lock().await.shutdown_all().await;
        }
    }

    async fn ready(&self, _ctx: Context, data: Ready) {
        println!("{} is connected!", data.user.name);
    }
}

static MANAGER: OnceLock<Arc<Mutex<ShardManager>>> = OnceLock::new();

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let token = env::var("UKUBOT_TOKEN").expect("Could not load token from UKUBOT_TOKEN");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let framework = StandardFramework::new();

    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Could not create client");

    MANAGER.set(client.shard_manager.clone()).unwrap();

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
