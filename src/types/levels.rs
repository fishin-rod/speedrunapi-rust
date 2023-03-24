use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LevelData{
    pub(crate) data: Level,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Level{
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) weblink: String,
    pub(crate) rules: String,
    pub(crate) links: Vec<Links>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Links{
    pub(crate) rel: String,
    pub(crate) uri: String,
}