use crate::{Data, Error};
use poise::serenity_prelude;
//Event nonsense
pub async fn event_listener(
    _ctx: &serenity_prelude::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    //poise::builtins::register_application_commands(_ctx.clone(), false);

    match event {
        poise::Event::Ready { data_about_bot } => {
            ready(data_about_bot);
        }
        _ => {
            println!("{}", event.name());
        }
    }

    Ok(())
}
//ready
fn ready(
    data: &poise::serenity_prelude::Ready,
    //ctx: &serenity_prelude::Context,
    //framework: poise::FrameworkContext<'_, Data, Error>,
    ) {
    println!("{} is connected!", data.user.name);
    /*
    println!("Registering slash commands!");
    let commands = &framework.options().commands;
    let create_commands = poise::builtins::create_application_commands(&commands);
    serenity_prelude::Command::set_global_application_commands(ctx, |b| {
        *b = create_commands; // replace the given builder with the one prepared by poise
        b
    })
    .await?;
    */
    //Ok(())
}
