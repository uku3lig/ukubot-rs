mod handler;

use serenity::client::{Context, EventHandler};
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::GatewayIntents;
use serenity::Client;
use std::env;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if let Err(e) = handler::message(&ctx, &message).await {
            println!("An error occurred in message handler: {:?}", e);
            let _ = message.reply(&ctx.http, "An error occurred").await;
        }
    }

    async fn ready(&self, _ctx: Context, data: Ready) {
        println!("{} is connected!", data.user.name);
    }
}

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

    let manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        manager.lock().await.shutdown_all().await;
    });

    if let Err(e) = client.start().await {
        println!("An error occurred while running the client: {:?}", e);
    }
}
