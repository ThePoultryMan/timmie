use dotenv::dotenv;
use serenity::framework::standard::CommandResult;
use std::env;

use serenity::async_trait;
use serenity::framework::standard::{StandardFramework, macros::{command, group}};
use serenity::model::channel::Message;
use serenity::prelude::*;

use commands::*;

mod commands;

struct Handler;

#[group]
#[commands(ping)]
#[sub_groups(Resin)]
struct General;

#[group]
#[commands(exp_wit)]
#[prefix("resin")]
struct Resin;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("?"))
        .group(&GENERAL_GROUP);

    let token = env::var("BOT_TOKEN").expect("Expected a bot token in the .env file.");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("error creating client");

    if let Err(err) = client.start().await {
        println!("An error occured while running the client {:?}", err);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}