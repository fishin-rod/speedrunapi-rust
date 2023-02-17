//! # Translations
//! 
//! This file is for the translations and conversions of certin items for working with the library

use tokio::runtime::Runtime;
use chrono::prelude::*;
use crate::types::user::UserData;

/// This function helps with translating time
/// 
/// ## Arguments
/// 
/// signup: String, a time in the format rfc3339 
/// 
/// ## Returns
/// 
/// A time that is more readable by a human as a string
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::translate::tl_time;
/// let result = tl_time("2014-10-02T12:34:23Z");
/// assert_eq!(result, "2014-10-02 12:34:23");
/// ```

pub fn tl_time(signup: &str) -> String{
    let dt = DateTime::parse_from_rfc3339(&signup).unwrap();
    let readable_time = dt.format("%Y-%m-%d %H:%M:%S").to_string();
    return readable_time
}

/// Translates a username to an id
/// 
/// ## Arguments
/// 
/// username: String, the username of a user on speedrun.com
/// 
/// ## Returns
/// 
/// The id of the user as a string
/// 
/// ## Examples
/// 
/// ```rust
/// use speedrunapi::translate::username_to_id;
/// let result = username_to_id("fishin_rod");
/// assert_eq!(result, "jonryvl8");
/// ```

pub fn username_to_id(username: &str) -> String {
    let rt = Runtime::new().unwrap();
    rt.block_on(async_username_to_id(username)).to_string()
}

async fn async_username_to_id(username: &str) -> String{
    let client = reqwest::Client::new();
    let url = format!("https://www.speedrun.com/api/v1/users?lookup={:1}", username);
    let response = client.get(url).send().await.unwrap().json::<UserData>().await.unwrap();

    return response.data.id;
}
