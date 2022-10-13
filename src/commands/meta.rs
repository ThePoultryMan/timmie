use crate::{embed_helper::Embed, Context, Error};

#[poise::command(slash_command, prefix_command, subcommands("calculations", "sources"))]
pub async fn meta(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Get an explanation of all the different calculations used in various commands.
#[poise::command(slash_command, prefix_command)]
pub async fn calculations(ctx: Context<'_>) -> Result<(), Error> {
    Embed::from_file("meta/calculations.json").send(ctx).await?;
    Ok(())
}

/// Get all the sources used.
#[poise::command(slash_command, prefix_command)]
pub async fn sources(ctx: Context<'_>) -> Result<(), Error> {
    Embed::from_file("meta/sources.json").send(ctx).await?;
    Ok(())
}
