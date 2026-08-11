mod config;
mod events;
mod handler;
mod utils;

use dotenvy::{dotenv, var};
use serenity::model::gateway::GatewayIntents;
use serenity::prelude::*;

use crate::config::Config;
use crate::handler::Handler;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env");

    let token = var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let config = Config::from_env();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler { config };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        eprintln!("❌ Client failed to start: {why:?}");
    } else {
        println!("✅ Client started successfully");
    }
}
