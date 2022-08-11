use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult, Delimiter};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::info;

#[command]
#[owners_only]
async fn ban(ctx: &Context, msg: &Message) -> CommandResult {
    let mut _args = Args::new(&msg.content, &[Delimiter::Single(' ')]);

    let mut _user_to_ban = msg.mentions.first().unwrap().id;

    if let Err(err) = msg.guild_id.unwrap().ban_with_reason(&ctx.http, _user_to_ban, 0, msg.content.as_str()).await {
        info!("could not ban user: {:?}", err);
      }

    Ok(())
}

#[command]
#[owners_only]
async fn kick(ctx: &Context, msg: &Message) -> CommandResult {
    let mut _args = Args::new(&msg.content, &[Delimiter::Single(' ')]);

    let mut _user_to_kick = msg.mentions.first().unwrap().id;

    if let Err(err) = msg.guild_id.unwrap().kick(&ctx.http, _user_to_kick).await {
        info!("could not ban user: {:?}", err);
      }

    Ok(())
} 