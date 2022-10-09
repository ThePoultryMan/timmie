use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command("exp-wit")]
pub async fn exp_wit(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let goal: f64 = args.parse().expect("crash");
    let completion_amount = (goal / 4.0).ceil() as i32;
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                let resin = completion_amount * 20;
                e.title(format!("{resin} Resin"));
                e.description("Assuming the worst case scenario, in which you only get 4 Hero's Wit per blossom, ".to_owned() +
                &format!("it will cost you {resin} Resin to get {goal} Hero's Wit."))
            })
        }).await?;
    Ok(())
}
