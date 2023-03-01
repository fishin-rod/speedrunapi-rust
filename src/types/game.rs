use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GameData{
    pub(crate) data: Game,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    pub(crate) id: String,
    pub(crate) names: GameNames,
    pub(crate) boost_received: Option<u32>,
    pub(crate) boost_distinct_donors: Option<u32>,
    pub(crate) abbreviation: String,
    pub(crate) weblink: String,
    pub(crate) discord: String,
    pub(crate) released: u16,
    pub(crate) release_date: Option<String>,
    pub(crate) ruleset: GameRuleset,
    pub(crate) romhack: bool,
    pub(crate) gametypes: Option<Vec<String>>,
    pub(crate) platforms: Option<Vec<String>>,
    pub(crate) regions: Option<Vec<String>>,
    pub(crate) genres: Option<Vec<String>>,
    pub(crate) engines: Option<Vec<String>>,
    pub(crate) developers: Option<Vec<String>>,
    pub(crate) publishers: Option<Vec<String>>,
    pub(crate) moderators: serde_json::Map<String, serde_json::Value>,
    pub(crate) created: String,
    pub(crate) assets: GameAssets,
    pub(crate) links: Vec<GameLink>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameNames {
    pub(crate) international: String,
    pub(crate) japanese: Option<String>,
    pub(crate) twitch: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameRuleset {
    #[serde(rename = "show-milliseconds")]
    pub(crate) show_milliseconds: bool,
    #[serde(rename = "require-verification")]
    pub(crate) require_verification: bool,
    #[serde(rename = "require-video")]
    pub(crate) require_video: bool,
    #[serde(rename = "run-times")]
    pub(crate) run_times: Vec<String>,
    #[serde(rename = "default-time")]
    pub(crate) default_time: String,
    #[serde(rename = "emulators-allowed")]
    pub(crate) emulators_allowed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameAssets {
    pub(crate) logo: GameAssetUri,
    #[serde(rename = "cover-tiny")]
    pub(crate) cover_tiny: GameAssetUri,
    #[serde(rename = "cover-small")]
    pub(crate) cover_small: GameAssetUri,
    #[serde(rename = "cover-medium")]
    pub(crate) cover_medium: GameAssetUri,
    #[serde(rename = "cover-large")]
    pub(crate) cover_large: GameAssetUri,
    pub(crate) icon: GameAssetUri,
    #[serde(rename = "trophy-1st")]
    pub(crate) trophy_1st: GameAssetUri,
    #[serde(rename = "trophy-2nd")]
    pub(crate) trophy_2nd: GameAssetUri,
    #[serde(rename = "trophy-3rd")]
    pub(crate) trophy_3rd: GameAssetUri,
    #[serde(rename = "trophy-4th")]
    pub(crate) trophy_4th: Option<GameAssetUri>,
    pub(crate) background: GameAssetUri,
    pub(crate) foreground: Option<GameAssetUri>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameAssetUri {
    pub(crate) uri: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameLink {
    pub(crate) rel: String,
    pub(crate) uri: String,
}