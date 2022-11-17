use crate::{Data, Error};
use poise::serenity_prelude;
//Event nonsense
pub async fn event_listener(
    _ctx: &serenity_prelude::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            ready(data_about_bot, _ctx, _framework).await?;
        }
        _ => {
            println!("{}", event.name());
        }
    }

    Ok(())
}
//ready
async fn ready(
    data: &poise::serenity_prelude::Ready,
    ctx: &serenity_prelude::Context,
    framework: poise::FrameworkContext<'_, Data, Error>,
) -> Result<(), Error> {
    println!("{} is connected!", data.user.name);
    Ok(())
}
