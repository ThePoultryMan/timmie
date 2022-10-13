use serde::{Deserialize, Serialize};

use crate::{
    embed_helper::{Embed, EmbedTextType},
    Context, Error,
};

#[derive(Deserialize, Serialize)]
struct ExpLevel {
    total_exp: Vec<i32>,
}

#[poise::command(slash_command, prefix_command, subcommands("level"))]
pub async fn character(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Tells you how many Hero's Wit, EXP, and Resin you need to get to your goal level, (from 0 or start).
#[poise::command(slash_command, prefix_command)]
pub async fn level(ctx: Context<'_>, goal: u32, start: Option<u32>) -> Result<(), Error> {
    let mut limit_embed = match crate::read_resource::<Embed>("character/level_limit.json") {
        Ok(embed) => embed,
        Err(err) => panic!("{:?}", err), // TODO: Add proper panicing
    };
    if goal > 90 {
        limit_embed.replace_text("%L", &goal, EmbedTextType::Description);
        limit_embed.send(ctx).await?;
        return Ok(())
    }

    let exp_levels = match crate::read_resource("character/exp_level.json") {
        Ok(exp_levels) => exp_levels,
        Err(_) => ExpLevel { total_exp: vec![0] },
    };
    let exp_gap = match start {
        Some(level) => {
            if level > 90 {
                limit_embed.replace_text("%L", &level, EmbedTextType::Description);
                limit_embed.send(ctx).await?;
                return Ok(())
            } else if level > goal {
                match crate::read_resource::<Embed>("character/start_over_goal.json") {
                    Ok(mut embed) => {
                        embed.replace_text("%g", &goal, EmbedTextType::Description);
                        embed.replace_text("%s", &level, EmbedTextType::Description);
                        embed.send(ctx).await?;
                    },
                    Err(err) => panic!("{:?}", err), // TODO: Add proper panicing
                };
                return Ok(())
            }
            exp_levels.total_exp[(goal - 1) as usize] - exp_levels.total_exp[(level - 1) as usize]
        }
        None => exp_levels.total_exp[(goal - 1) as usize],
    };
    ctx.say(format!("You are {exp_gap} EXP away from level {goal}!"))
        .await?;
    Ok(())
}
