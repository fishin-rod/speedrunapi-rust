use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct User {
    pub(crate) id: String,
    pub(crate) names: Names,
    #[serde(rename = "supporterAnimation")]
    pub(crate) supporter_animation: bool,
    pub(crate) pronouns: Option<String>,
    pub(crate) weblink: String,
    //#[serde(rename = "name-style")]
    //pub(crate) name_style: NameStyle,
    pub(crate) role: String,
    pub(crate) signup: Option<String>,
    pub(crate) location: Location,
    pub(crate) twitch: Option<String>,
    pub(crate) hitbox: Option<String>,
    pub(crate) youtube: Option<String>,
    pub(crate) twitter: Option<String>,
    pub(crate) speedrunslive: Option<String>,
    pub(crate) assets: Assets,
    pub(crate) links: Vec<Link>,
}

#[derive(Deserialize, Debug)]
pub struct Names {
    pub(crate) international: String,
    pub(crate) japanese: Option<String>,
}
/* 
#[derive(Deserialize, Debug)]
pub struct NameStyle {
    pub(crate) style: String,
    pub(crate) color: Color,
}

#[derive(Deserialize, Debug)]
pub struct Color {
    pub(crate) light: String,
    pub(crate) dark: String,
}
*/
#[derive(Deserialize, Debug)]
pub struct Location {
    pub(crate) country: Country,
    pub(crate) region: Option<Region>,
}

#[derive(Deserialize, Debug)]
pub struct Country {
    pub(crate) code: String,
    pub(crate) names: Names,
}

#[derive(Deserialize, Debug)]
pub struct Region {
    pub(crate) code: String,
    pub(crate) names: Names,
}

#[derive(Deserialize, Debug)]
pub struct Assets {
    pub(crate) icon: Icon,
    #[serde(rename = "supporterIcon")]
    pub(crate) supporter_icon: Option<Icon>,
    pub(crate) image: Icon,
}

#[derive(Deserialize, Debug)]
pub struct Icon {
    pub(crate) uri: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Link {
    pub(crate) rel: String,
    pub(crate) uri: String,
}

#[derive(Deserialize, Debug)]
pub struct UserData {
    pub(crate) data: User
   // pub(crate) pagination: Pagination,
}

//#[derive(Deserialize, Debug)]
//pub struct Pagination {
  //  pub(crate) offset: u32,
  //  pub(crate) max: u32,
  //  pub(crate) size: u32,
  //  pub(crate) links: Vec<Link>,
//}