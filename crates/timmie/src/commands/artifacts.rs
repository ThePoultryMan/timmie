use crate::{embed_helper::Embed, Context, Error};

#[poise::command(slash_command, prefix_command, subcommands("distribution"))]
pub async fn artifacts(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Get the distribution of artifact main stats.
#[poise::command(slash_command, prefix_command)]
pub async fn distribution(ctx: Context<'_>) -> Result<(), Error> {
    Embed::from_file("artifact/distribution.json")
        .send(ctx)
        .await?;
    Ok(())
}
