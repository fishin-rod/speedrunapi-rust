//! # User_types
//! 
//! Contains all of the JSON values for a user
//! 
//! Struct DATA is the main struct containing a vector of all the other data points.
//! Struct User is the struct that starts holding the values, and it then expands from there on the values 

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserNames{
    pub international: String,
    pub japanese: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NameColor{
    pub light: String,
    pub dark: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NameStyle{
    pub style: String,
    pub color: NameColor,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationNames{
    pub international: String,
    pub japanese: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Country{
    pub code: String,
    pub names: LocationNames,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegionNames{
    pub international: String,
    pub japanese: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Region{
    pub code: String,
    pub names: RegionNames
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location{
    pub country: Country,
    pub region: Region,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Twitch{
    pub uri: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hitbox{
    pub uri: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Youtube{
    pub uri: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Twitter{
    pub uri: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpeedRunsLive{
    pub uri: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link{
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    pub id: String,
    pub names: UserNames,
    pub weblink: String,
    pub pronouns: Option<String>,
    pub role: String,
    pub signup: Option<String>,
    pub location: Location,
    pub twitch: Option<Link>,
    pub hitbox: Option<Link>,
    pub youtube: Option<Link>,
    pub twitter: Option<Link>,
    pub speedrunslive: Option<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData{
    pub data: User, 
}