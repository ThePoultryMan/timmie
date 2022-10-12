use dotenv::dotenv;
use rust_embed::RustEmbed;

use std::env;

use poise::serenity_prelude as serenity;

use commands::*;

mod commands;
mod embed_helper;

pub struct Data;

#[derive(RustEmbed)]
#[folder = "src/resources/"]
pub struct Resources;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![register(), ping(), resin::resin(), meta::meta()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("?".to_owned()),
                ..Default::default()
            },
            ..Default::default()
        })
        .token(env::var("BOT_TOKEN").expect("Expected a bot token in the .env file."))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("ping!").await?;
    Ok(())
}
