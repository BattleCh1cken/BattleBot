use crate::{utils::context::Context, Error};
use poise::serenity_prelude as serenity;
/*
 * TODO:
 * args:
 * lang, pronouns, color (red green blue etc.)
 * buttons for each role
 * get existing roles from config
 */

/// Query information about a Discord profile
#[poise::command(slash_command)]
pub async fn roles(ctx: Context<'_>) -> Result<(), Error> {
    let uuid = ctx.id();
    ctx.send(|m| {
        m.content("Choose Your Role").components(|c| {
            c.create_action_row(|ar| {
                ar.create_select_menu(|s| {
                    s.custom_id("role").min_values(1).max_values(1);
                    s.options(|o| o.create_option(|o| o.label("red").value("bob")))
                })
            })
        })
    })
    .await?;

    while let Some(mci) = serenity::CollectComponentInteraction::new(ctx.discord())
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == uuid.to_string())
        .await
    {

        let mut msg = mci.message.clone();
        msg.edit(ctx.discord(), |m| {
            m.content(format!("selected something. Idk wut tho"))
        })
        .await?;

        mci.create_interaction_response(ctx.discord(), |ir| {
            ir.kind(serenity::InteractionResponseType::DeferredUpdateMessage)
        })
        .await?;
    }


    Ok(())
}
