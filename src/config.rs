use poise::serenity_prelude::{GuildId, RoleId, UserId};
use std::collections::HashSet;
pub struct Config {
    pub discord_token: String,
    pub guild_id: GuildId,
    pub owner_id: HashSet<UserId>,

    pub roles_color: Vec<RoleId>,
    pub roles_lang: Vec<RoleId>,
}

impl Config {
    pub fn from_environment() -> Option<Config> {
        Some(Config {
            discord_token: std::env::var("TOKEN").expect("no token"),
            guild_id: GuildId::from(
                std::env::var("GUILD")
                    .expect("no guild id")
                    .parse::<u64>()
                    .unwrap(),
            ),
            owner_id: HashSet::from([UserId::from(
                std::env::var("OWNER")
                    .expect("no owner id")
                    .parse::<u64>()
                    .unwrap(),
            )]),

            roles_color: vec![],
            roles_lang: vec![],
        })
    }
}
