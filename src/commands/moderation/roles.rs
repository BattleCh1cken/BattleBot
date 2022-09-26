//use poise::serenity_prelude;

use crate::{Context, Error};


///Select your roles!
#[poise::command(slash_command)]
pub async fn roles(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let response = format!("there will be a command here at some point");
    ctx.say(response).await?;
    Ok(())
}
