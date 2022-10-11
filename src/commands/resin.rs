use poise::serenity_prelude::CreateEmbed;

use crate::{embed_helper::Colors, Context, Error};

#[poise::command(slash_command, prefix_command, subcommands("exp_wit"))]
pub async fn resin(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn exp_wit(
    ctx: Context<'_>,
    #[description = "The total number of Hero's Wit you want"] goal: i32,
    #[description = "Use the average calculation instead of the worst case scenario option"]
    mut average: Option<bool>,
) -> Result<(), Error> {
    average.unwrap_or(false);
    let (modified_goal, desc) = match average.get_or_insert(false) {
            true => {
                ((goal as f64 * 0.889).ceil() as i32, String::from("If we go by the (pseudo) average, in which you get ~4.5 Hero's Wit per blossom, "))
            },
            false => (goal, String::from("Assuming the worst case scenario, in which you only get 4 Hero's Wit per blossom, ")),
        };
    let completion_times = (modified_goal as f64 / 4.0).ceil() as i32;
    let resin = completion_times * 20;
    let mut embed = CreateEmbed::default();
    embed.title(format!("{resin} Resin"));
    embed.description(desc + &format!("it will cost you {resin} Resin to get {goal} Hero's Wit."));
    embed.color(Colors::Resin as i32);

    ctx.send(|r| {
        r.embed(|e| {
            *e = embed.clone();
            e
        })
    })
    .await?;
    Ok(())
}
