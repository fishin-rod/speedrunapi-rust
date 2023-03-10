//! # Translations
//! 
//! This file is for the translations and conversions of certin items for working with the library

use tokio;
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
    let user_id = tokio::runtime::Runtime::new().unwrap().block_on(async_username_to_id(username)).unwrap();
    return user_id;
}

async fn async_username_to_id(username: &str) -> Result<String, Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();
    let url = format!("https://www.speedrun.com/api/v1/users?lookup={:1}", username);
    let response = client.get(url).send().await.unwrap().text().await.unwrap();
    let userid: UserData = serde_json::from_str(&response)?;
    let mut user_id = Vec::<String>::new();
    for user in userid.data{
        user_id.push(user.id);
    }
    Ok(user_id[0].to_string())
}
/* 
/// Translates a user id to a name
/// 
/// ## Arguments
/// 
/// id: &str, the id of the user you want a name for
/// 
/// ## Returns the name of the user from the id as a string
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::translate::user_id_to_name;
/// let result = user_id_to_name("jonryvl8");
/// assert_eq!(result, "fishin_rod");
/// ```

pub fn user_id_to_name(username: &str) -> String{
    let user_name = tokio::runtime::Runtime::new().unwrap().block_on(async_id_to_username(username)).unwrap();
    return user_name[0].to_string();
}

async fn async_id_to_username(id: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();
    let url = format!("https://www.speedrun.com/api/v1/users/{:1}", id);
    let response = client.get(url).send().await.unwrap().text().await.unwrap();
    let username: UserData = serde_json::from_str(&response)?;
    let mut user_name = Vec::<String>::new();
    for user in username.data{
        user_name.push(user.names.international);
    }
    Ok(user_name)
}
*/