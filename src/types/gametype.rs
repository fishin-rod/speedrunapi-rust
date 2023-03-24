use serde::{Deserialize};
#[allow(dead_code, unused_variables)]

#[derive(Deserialize, Debug)]
pub struct Link {
    pub(crate) rel: String,
    pub(crate) uri: String,
}

#[derive(Deserialize, Debug)]
pub struct GameType{
    pub(crate) id: String,
    pub(crate) name: String,
   // pub(crate) links: Vec<Link>,
}

#[derive(Deserialize, Debug)]
pub struct GameTypeData{
    pub(crate) data: GameType,
}