use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

use crate::embed_helper::Colors;
use crate::helper::parse_arg;

struct ExpWitGoal {
    original: f64,
    modified: f64,
}

#[command("exp-wit")]
pub async fn exp_wit(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if !args.is_empty() {
        let first_arg: String = parse_arg(&args);
        let goal = match first_arg.parse::<f64>() {
            Ok(arg) => ExpWitGoal {
                original: arg,
                modified: arg,
            },
            Err(_) => {
                if first_arg == "avg" && args.len() == 2 {
                    ExpWitGoal {
                        original: parse_arg(args.advance()),
                        modified: (parse_arg::<f64>(&args) * 0.889).ceil(),
                    }
                } else {
                    return send_exp_wit_error(ctx, msg).await;
                }
            }
        };
        let completion_amount = (goal.modified / 4.0).ceil() as i32;
        msg.channel_id
                    .send_message(ctx, |m| {
                        m.embed(|e| {
                            let resin = completion_amount * 20;
                            let desc = match goal.original == goal.modified {
                                true => "Assuming the worst case scenario, in which you only get 4 Hero's Wit per blossom, ",
                                false => "If we go by the (pseudo) average, in which you get ~4.5 Hero's Wit per blossom, ",
                            }.to_owned();
                            e.title(format!("{resin} Resin"));
                            e.description(desc + &format!("it will cost you {resin} Resin to get {} Hero's Wit.", goal.original));
                            e.color(Colors::Resin as i32)
                        })
                    }).await?;
        Ok(())
    } else {
        send_exp_wit_error(ctx, msg).await
    }
}

async fn send_exp_wit_error(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Invalid Syntax");
                e.color(Colors::InvalidSyntax as i32)
            })
        })
        .await?;
    Ok(())
}
