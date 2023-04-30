use serde::Deserialize;

#[derive(Deserialize)]
pub struct Discord {
    pub token: String,
    pub test_guild_id: u64,
}

#[derive(Deserialize)]
pub struct AniList {
    pub client_id: u16,
    pub client_secret: String,
    pub redirect_url: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub discord: Discord,
    pub anilist: AniList,
}