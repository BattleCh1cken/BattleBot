use commands::*;
use poise::serenity_prelude;
mod commands;
mod config;
mod events;
use config::Config;
mod utils;
type Error = Box<dyn std::error::Error + Send + Sync>;
use crate::utils::context::Context;
// User data, which is stored and accessible in all command invocations
pub struct Data {}

#[tokio::main]
async fn main() {
    let config: Config = Config::from_environment().expect("error in config");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            listener: |_ctx, event, _framework, _data| {
                Box::pin(events::event_listener(_ctx, event, _framework, _data))
            },
            commands: vec![
                //misc
                misc::age::age(),
                misc::ping::ping(),
                misc::boop(),
                //moderation
                moderation::roles::roles(),
                //owner
                owner::register::register(),
                //sushi
            ],
            owners: config.owner_id.clone(),
            ..Default::default()
        })
        .token(config.discord_token)
        .intents(serenity_prelude::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));
    framework.run().await.unwrap();
}
