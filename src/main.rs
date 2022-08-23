// Imports

mod config;
mod commands;
mod utils;

use std::env;
use serenity::async_trait;
use serenity::client::{ Context, EventHandler, Client };
use serenity::model::channel::Message;
use serenity::model::gateway::{ GatewayIntents, Ready };

// Start Discord bot

#[tokio::main]
async fn main() {
    // Load bot token from env

    dotenvy::dotenv().expect("Error loading env");
    let token = env::var("BOT_TOKEN").expect("Error loading BOT_TOKEN from env");

    // Create bot client

    println!("Creating bot client...");
    let intents = GatewayIntents::GUILD_MESSAGES |
        GatewayIntents::DIRECT_MESSAGES |
        GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await.expect("Error creating bot client");

    if let Err(error) = client.start().await {
        eprintln!("Client terminated with error: {error}");
    }
}

// Event handler

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Initial login

    async fn ready(&self, _: Context, data: Ready) {
        println!("Bot client logged in as {}#{}", data.user.name, data.user.discriminator);
    }

    // Message event

    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(error) = commands::handle_message(&ctx, &msg).await {
            eprintln!("Internal error: {error}");
        };
    }
}