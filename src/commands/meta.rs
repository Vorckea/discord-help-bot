use std::time::Instant;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let before = Instant::now();
    let mut m = msg.channel_id.say(&ctx.http, "pong!").await?;
    let after = Instant::now();

    let content = m.content.clone();
    m.edit(ctx, |m| m.content(format!("{} - {}ms", content, (after - before).as_millis()))).await?;

    Ok(())
}
