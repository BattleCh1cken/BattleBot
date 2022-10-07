use crate::{Context, Error};

/*
 * TODO:
 * args:
 * lang, pronouns, color (red green blue etc.)
 * buttons for each role
 * automatically create roles
 */

/// Query information about a Discord profile
#[poise::command(slash_command)]
pub async fn roles(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
