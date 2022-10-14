use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, subcommands("exp_wit"))]
pub async fn resin(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Calculate how much resin you need to get an inputted number of Hero's Wit.
#[poise::command(slash_command, prefix_command)]
pub async fn exp_wit(
    ctx: Context<'_>,
    #[description = "The total number of Hero's Wit you want"] goal: i32,
    #[description = "Use the average calculation instead of the worst case scenario option"]
    average: Option<bool>,
) -> Result<(), Error> {
    let (avg, _desc) = match average {
            Some(avg) => {
                (avg, String::from("If we go by the (pseudo) average, in which you get ~4.5 Hero's Wit per blossom, "))
            },
            None => (false, String::from("Assuming the worst case scenario, in which you only get 4 Hero's Wit per blossom, ")),
        };
    let _resin = ushi::resin::get_exp_wit_resin(goal, avg);
    Ok(())
}
