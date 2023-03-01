//! # Users
//!
//! Handles user requests and returing the user data.
//! 
//! What are users anyways?
//! Users are the people on speedrun.com that have an account.
//! This account then can submit runs and chat in fourms on the website.
//! 
//! Users have different data points assosiated with them:
//! - id: The unique id given to a user at signup - *This value will always be the same*.
//! - name: The name they have chooses to display for speedrun.com - *This value can change*.
//! - weblink: The link to the users profile, you can input it on your browser and it will pull up the profile.
//! - pronouns: The pronouns the person has listed on their profile - *This value can be None*.
//! - role: The role of the user on the website.
//! - signup: The date the user signed up for the website and created an account.
//! - region: The region where the person lives (state, country, city, etc.).
//! - country: The country where the person lives (continent, nation, etc.).
//! - links: The links for the other api endpoints for some more specific data.
//! 
//! You can use the functions in this module to get differnt parts and peices of the data or the whole set!
//!
//! ## Arguments
//!
//! All of the functions will take in a name parameter
//! 
//! name: &str, the name of a person on speedrun.com

//TODO! name-style!
use crate::translate::tl_time;
use crate::types::user::UserData;
use std::string::String;

/// makes a request for user data 
///
/// # Parameters
/// name: &str, the name or id of a person on speedrun.com
///
/// I recommend using the id of the user since a person can change their name
#[tokio::main]
async fn user_reqwest(name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    //Initialize a client and send the request
    let client = reqwest::Client::new();
    let url = format!("https://www.speedrun.com/api/v1/users?lookup={:1}", name);
    let response = client.get(url).send().await.unwrap().text().await.unwrap();
    let userdata: UserData = serde_json::from_str(&response)?;
    //TODO allow user ids
    //TODO allow name-style

    //Create a user
    let mut user_data = Vec::<String>::new();
    for user in userdata.data{
        //Get each of the users data points from the structs
        let time = tl_time(&user.signup.unwrap_or(String::from("None")));
        //let links = [user.twitch, &user.hitbox, &user.youtube, &user.twitch, &user.speedrunslive];
    user_data.extend([user.id, user.names.international, user.names.japanese.unwrap_or(String::from("None")),
        user.supporter_animation.to_string(), user.weblink, user.pronouns.unwrap_or(String::from("None")), user.role, time,
        user.location.region.as_ref().map(|code| code.code.clone()).unwrap_or(String::from("None")), 
        user.location.region.as_ref().map(|name| name.names.international.clone()).unwrap_or(String::from("None")), user.location.country.code, 
        user.location.country.names.international, user.twitch.unwrap_or(String::from("None")),
        user.hitbox.unwrap_or(String::from("None")), user.youtube.unwrap_or(String::from("None")),
        user.twitter.unwrap_or(String::from("None")), user.speedrunslive.unwrap_or(String::from("None")), 
        user.assets.icon.uri.unwrap_or(String::from("None")), user.assets.supporter_icon.map(|icon| icon.uri).flatten().unwrap_or(String::from("None")),
        user.assets.image.uri.unwrap_or(String::from("None"))]);
        let rels: Vec<String> = user.links.iter().map(|rel| rel.rel.clone()).collect();
        let urls: Vec<String> = user.links.iter().map(|link| link.uri.clone()).collect();
        user_data.extend(rels);
        user_data.extend(urls); 
    }
    Ok(user_data)
}

/// Returns all of the data for a user
///
/// ## Arguments
///
/// name: &str, the username of a user on speedrun.com
///
/// ## Returns
///
/// A Vector containing strings of all of the data points
///
/// ex(id, name, weblink, pronouns, role, signup, region_code, region_name, country_code, country_name )
pub fn user_data(name: &str) -> Vec<String> {
    main(name.to_string())
}

/// Returns the ID of the user
///
/// ## Returns
///
/// The id of the user as a string
///
/// ## Examples
///
/// ```rust
/// use speedrunapi::user_id;
/// let result = user_id("fishin_rod");
/// assert_eq!(result, "jonryvl8");
/// ```

pub fn user_id(name: &str) -> std::string::String {
    let user_id = &main(name.to_string())[0];
    return user_id.to_string();
}

/// Returns if the user has a supporter animation on their profile
/// 
/// ## Returns
/// 
/// A boolen value. 
/// The value is true if the user has a suppoter animation, otehrwise its false.
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::user_animation;
/// let result = user_animation("fishin_rod");
/// assert_eq!(result, false);
/// ```
pub fn user_animation(name: &str) -> bool{
    let user_animation = &main(name.to_string())[3];
    return user_animation.parse::<bool>().unwrap();
}

/// Returns the speedrun.com link to a users profile
///
/// ## Returns
///
/// The link to a users profile as a string
///
/// ## Examples
///
/// ```rust
/// use speedrunapi::user_link;
/// let result = user_link("fishin_rod");
/// assert_eq!(result, "https://www.speedrun.com/user/fishin_rod");
/// ```

pub fn user_link(name: &str) -> std::string::String {
    let user_link = &main(name.to_string())[4];
    return user_link.to_string();
}

/// Returns the pronouns of a user
///
///
/// ## Returns
///
/// The pronouns listed on a users profile
/// If no pronouns are provided the value is set to None
///
/// ## Examples
///
/// ```rust
/// use speedrunapi::user_pronouns;
/// let result = user_pronouns("fishin_rod");
/// assert_eq!(result, "He/Him");
/// ```

pub fn user_pronouns(name: &str) -> std::string::String {
    let user_pronoun = &main(name.to_string())[5];
    return user_pronoun.to_string();
}

/// Returns the roles of a user on speedrun.com
///
/// ## Returns
///
/// The the role of a user
/// What it can be: (banned, user, trusted, moderator, admin, programer, and guest)
///
/// ## Examples
///
/// ```rust
/// use speedrunapi::user_role;
/// let result = user_role("fishin_rod");
/// assert_eq!(result, "user");
/// ```

pub fn user_role(name: &str) -> std::string::String {
    let user_role = &main(name.to_string())[6];
    return user_role.to_string();
}

/// Returns a users sign up date
///
/// ## Returns
///
/// The date a user signed up for speedrun.com in a Y:M:D H-M-S format
/// The date is translated from an rfc3339 format
///
/// ## Examples
///
/// ```rust
/// use speedrunapi::user_signup;
/// let result = user_signup("fishin_rod");
/// assert_eq!(result, "2022-12-20 01:16:42")
/// ```

pub fn user_signup(name: &str) -> std::string::String {
    let user_time = &main(name.to_string())[7];
    return user_time.to_string();
}

/// Returns the region a user has listed on their speedrun.com profile
/// 
/// ## Returns
/// 
/// The region (State, City, Country) that the user has listed on their profile
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::user_region;
/// let result = user_region("fishin_rod");
/// assert_eq!(result, "Colorado, USA");
/// ```

pub fn user_region(name: &str) -> std::string::String{
    let user_region_name = &main(name.to_string())[9];
    return user_region_name.to_string();
}

/// Returns the country a user has listed on their speedrun.com profile
/// 
/// ## Returns
/// 
/// The country (Country, Continent) that the user has listed on their profile
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::user_country;
/// let result = user_country("fishin_rod");
/// assert_eq!(result, "United States");
/// ```

pub fn user_country(name: &str) -> std::string::String{
    let user_country_name = &main(name.to_string())[11];
    return user_country_name.to_string();
}

/// Returns the assests the users has
/// 
/// ## Returns
/// 
/// A vetor contaning 3 strings repersenting differnt assests a user can have.
/// The first is the uri to the icon they have set for their profile, 
/// the next is the supporter icon the uri to the supporter icon they have (if they dont have one returns None),
/// the last is the image they have set.
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::user_assets;
/// let result = user_assets("fishin_rod");
/// assert_eq!(result, ["None", "None", "None"])
/// ```
pub fn user_assets(name: &str) -> Vec<String>{
    let data = &main(name.to_string());
    let assets = vec![&data[12], &data[13], &data[14]];
    let strings: Vec<String> = assets.iter().map(|s| s.to_string()).collect();
    return strings.to_vec();
}

/// Returns the links listed on a users profile
/// 
/// ## Returns
/// 
/// A vector containing 5 strings that are the links
/// The order of the links: (Twitch, Hitbox, Youtube, Twitter, Speedrunslive)
/// Any of the links can be equal to None
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::user_links;
/// let result = user_links("fishin_rod");
/// assert_eq!(result, ["None", "None", "None", "None", "None"]);
/// ```

pub fn user_links(name: &str) -> Vec<String>{
    let data = &main(name.to_string());
    let profile_links = vec![&data[15], &data[16], &data[17], &data[18], &data[19]];
    let strings: Vec<String> = profile_links.iter().map(|s| s.to_string()).collect();
    return strings.to_vec();
}

fn main(name: String) -> Vec<String> {
    user_reqwest(&name.to_string()).expect("User Not Found")
}
/*
if response.data.is_empty(){
        let url = format!("https://www.speedrun.com/api/v1/users/{:1}", name);
        println!("{}, {}", name, url);
        let response: User = client.get(url).send().await?.json::<User>().await?;
    } 
    jonryvl8
    */

/*
 user.name_style.style, user.name_style.color.light, user.name_style.color.dark, 
*/