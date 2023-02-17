use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Links{
    pub(crate) rel: String,
    pub(crate) uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Guest{
    pub(crate) name: String,
    pub(crate) links: Links,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GuestData{
    pub(crate) data: Guest,
}