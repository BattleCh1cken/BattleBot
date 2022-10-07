use commands::*;
use poise::serenity_prelude;
mod commands;
mod events;
mod config;
use config::Config;
type Context<'a> = poise::Context<'a, Data, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;

// User data, which is stored and accessible in all command invocations
pub struct Data {}

#[tokio::main]
async fn main() {
    let config = Config::from_environment().expect("error in config");
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
                //sushi
            ],
            owners: config.owner_id,
            ..Default::default()
        })
        .token(config.discord_token)
        .intents(serenity_prelude::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));
    framework.run().await.unwrap();
}
