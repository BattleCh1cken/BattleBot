use crate::{Context, Error};
use poise::serenity_prelude;
/*
 * TODO:
 * args:
 * lang, pronouns, color (red green Blue etc.)
 * buttons for each role
 * automatically create roles
 */


/// Query information about a Discord profile
#[poise::command(context_menu_command = "User information", slash_command)]
pub async fn roles(
    ctx: Context<'_>,
    #[description = "Discord profile to query information about"] user: serenity_prelude::User,
) -> Result<(), Error> {
    let response = format!(
        "**Name**: {}\n**Created**: {}",
        user.name,
        user.created_at()
    );

    ctx.say(response).await?;
    Ok(())
}

/// Echo content of a message
#[poise::command(context_menu_command = "Echo", slash_command)]
pub async fn echo(
    ctx: Context<'_>,
    #[description = "Message to echo (enter a link or ID)"] msg: serenity_prelude::Message,
) -> Result<(), Error> {
    ctx.say(&msg.content).await?;
    Ok(())
}
