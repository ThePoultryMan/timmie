use crate::{embed_helper::Embed, Context, Error};

#[poise::command(slash_command, prefix_command, subcommands("calculations"))]
pub async fn meta(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn calculations(ctx: Context<'_>) -> Result<(), Error> {
    let embed = Embed::from_file("calculations.json");
    ctx.send(|r| {
        r.embed(|e| {
            *e = embed.build();
            e
        })
    })
    .await?;
    Ok(())
}
