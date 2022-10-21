use std::vec;

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
    let mut limit_embed = Embed::from_file("character/level_limit.json");
    if goal > 90 {
        limit_embed.fill_placeholder("%L", &goal, EmbedTextType::Description);
        limit_embed.send(ctx).await?;
        return Ok(());
    }

    let exp_levels = match crate::read_resource("character/exp_level.json") {
        Ok(exp_levels) => exp_levels,
        Err(_) => ExpLevel { total_exp: vec![0] },
    };

    let mut level_embed = Embed::from_file("character/level.json");
    let exp_gap = match start {
        Some(level) => {
            if level > 90 {
                limit_embed.fill_placeholder("%L", &level, EmbedTextType::Description);
                limit_embed.send(ctx).await?;
                return Ok(());
            } else if level > goal {
                let mut embed = Embed::from_file("character/start_over_goal.json");
                embed.fill_placeholders(
                    vec!["%g", "%s"],
                    vec![&goal, &level],
                    EmbedTextType::Description,
                );
                embed.send(ctx).await?;
                return Ok(());
            }
            level_embed.fill_placeholder("%l", &level, EmbedTextType::Description);
            exp_levels.total_exp[(goal - 1) as usize] - exp_levels.total_exp[(level - 1) as usize]
        }
        None => {
            level_embed.fill_placeholder("%l", &0, EmbedTextType::Description);
            exp_levels.total_exp[(goal - 1) as usize]
        }
    };

    level_embed.fill_placeholders(
        vec!["%E", "%L"],
        vec![&exp_gap, &(goal as i32)],
        EmbedTextType::Title,
    );
    level_embed.fill_placeholders(
        vec!["%E", "%L"],
        vec![&exp_gap, &(goal as i32)],
        EmbedTextType::Description,
    );
    let wit_goal = (exp_gap as f32 / 20000.0).ceil() as i32;
    level_embed.fill_placeholder(
        "%R",
        &ushi::resin::get_exp_wit_resin(wit_goal, true),
        EmbedTextType::FieldBody(0),
    );
    level_embed.fill_placeholder("%W", &wit_goal, EmbedTextType::FieldBody(1));
    level_embed.send(ctx).await?;
    Ok(())
}
