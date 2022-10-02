use std::collections::HashSet;
use commands::*;
use poise::serenity_prelude::{self, UserId};
mod commands;
mod events;

type Context<'a> = poise::Context<'a, Data, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;

// User data, which is stored and accessible in all command invocations
pub struct Data {}






#[tokio::main]
async fn main() {
    //Fetch the bot's token from an environment variable
    let token = std::env::var("DISCORD_TOKEN").expect("missing token, you nerd");
    let owner: HashSet<UserId> = HashSet::from([UserId(524768045273448449)]);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            listener: |_ctx, event, _framework, _data| {
                Box::pin(events::event_listener(_ctx, event, _framework, _data))
            },
            commands: vec![
                //misc
                misc::age::age(),
                misc::ping::ping(),
                //moderation
                moderation::roles::roles(),
                //owner
                owner::register::register(),
            ],
            owners: owner,
            ..Default::default()
        })
        .token(&token)
        .intents(serenity_prelude::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));
    framework.run().await.unwrap();
}
