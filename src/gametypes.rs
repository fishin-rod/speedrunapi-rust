//! Game Types
//! 
//! Game types are classifications for unofficial games, such as ROMS.
//! 
//! ## Arguments
//! 
//! Each of the functions takes the argument game.
//! game: &str, the name or id of the gametype you are trying to find.
//! 
//! ## Tips
//! 
//! If you know the name or id of a game type but not the other you can use the opposite function to find it.
//! 
//! Ex: (You know the gametype Fangame but you dont know the id, you can just use gametype_id("Fangame")) 

use crate::types::GameTypeData;

#[tokio::main]
async fn gametype_reqwest(game: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();
    let url = format!("https://www.speedrun.com/api/v1/gametypes/{:1}", game);
    let response = client.get(url).send().await.unwrap().text().await.unwrap();
    let gamedata: GameTypeData = serde_json::from_str(&response)?;
    let mut game_data = Vec::<String>::new();
    
    let game = gamedata.data;
    game_data.extend([game.name, game.id]);
    let rels: Vec<String> = game.links.iter().map(|rel| rel.rel.clone()).collect();
    let urls: Vec<String> = game.links.iter().map(|link| link.uri.clone()).collect();
    game_data.extend(rels);
    game_data.extend(urls);
    Ok(game_data)
}

/// Returns all of the data about a game type
/// 
/// ## Returns
/// 
/// A vector contating strings with all of the data points:
/// - Name
/// - Id
/// - Rel(s)
/// - Url(s)
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::gametype_data;
/// let result = gametype_data("Fangame");
/// assert_eq!(result, ["Fangame", "d91jd1ex", "self", "games", "https://www.speedrun.com/api/v1/gametypes/d91jd1ex", "https://www.speedrun.com/api/v1/games?gametype=d91jd1ex"]);
/// ```

pub fn gametype_data(game: &str) -> Vec<std::string::String>{
    return main(game.to_string());
}

/// Returns the name of a gametype
/// 
/// ## Returns 
/// 
/// A string of the name of the gametype 
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::gametype_name;
/// let result = gametype_name("v4m291qw");
/// assert_eq!(result, "ROM Hack")
/// ```

pub fn gametype_name(game: &str) -> String{
    let name = &main(game.to_string())[0];
    return name.to_string();
}

/// Returns the id of a gametype
/// 
/// ## Returns
/// 
/// The id of the gametype as a string
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::gametype_id;
/// let result = gametype_id("Fangame");
/// assert_eq!(result, "d91jd1ex")
/// ```
pub fn gametype_id(game: &str) -> String{
    let id = &main(game.to_string())[1];
    return id.to_string();
}

fn main(game: String) -> Vec<String> {
    gametype_reqwest(&game.to_string()).expect("Game Type Not Found")
}