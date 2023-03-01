//! Games
//! 
//! Handles requests for a games data
//! 
//! A game is where users can submit runs for different categories and leaderboards connected to the game.
//! 
//! ## Arguments
//! 
//! All of the functons take the argument game
//! 
//! game: &str, the name of the game you are trying to serach for data about

use crate::types::{GameData, game::GameAssetUri};
use crate::translate::{tl_time};
//use crate::translate::user_id_to_name;
// Get working ^

#[tokio::main]
async fn game_reqwest(game: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();
    // TODO: Game ids
    let url = format!("https://www.speedrun.com/api/v1/games/{:1}", game);
    let response = client.get(url).send().await.unwrap().text().await?;
    let gamedata: GameData = serde_json::from_str(&response).unwrap();

    let mut game_data = Vec::<String>::new();
    let mut mod_list = Vec::<String>::new();
    let game = gamedata.data;
    //translations for things like this 
    for moderator in game.moderators{
        mod_list.extend([moderator.0.to_string(), 
        moderator.1.to_string().trim_matches(|c| c == '\\' || c == '"').to_string()]);
    }
    game_data.extend([game.id, game.names.international, game.names.japanese.unwrap_or(String::from("None")), 
    game.names.twitch, game.boost_received.unwrap_or(0).to_string(), game.boost_distinct_donors.unwrap_or(0).to_string(), 
    game.abbreviation, game.weblink, game.discord, game.released.to_string(), game.release_date.unwrap_or(String::from("None")),
    game.ruleset.show_milliseconds.to_string(), game.ruleset.require_verification.to_string(),
    game.ruleset.require_video.to_string()]);
    game_data.extend([game.ruleset.default_time, game.ruleset.emulators_allowed.to_string(), game.romhack.to_string()]);
    //better way and translations
    // Add items of ? size
    /* 
    game_data.append(&mut game.gametypes.unwrap_or(vec![String::from("None")]));
    game_data.append(&mut game.platforms.unwrap_or(vec![String::from("None")]));
    game_data.append(&mut game.regions.unwrap_or(vec![String::from("None")]));
    game_data.append(&mut game.genres.unwrap_or(vec![String::from("None")]));
    game_data.append(&mut game.engines.unwrap_or(vec![String::from("None")]));
    game_data.append(&mut game.developers.unwrap_or(vec![String::from("None")]));
    game_data.append(&mut game.publishers.unwrap_or(vec![String::from("None")]));
    game_data.append(&mut mod_list);
    */
    game_data.extend([game.created, game.assets.logo.uri.unwrap_or(String::from("None")), 
    game.assets.cover_tiny.uri.unwrap_or(String::from("None")), game.assets.cover_small.uri.unwrap_or(String::from("None")), 
    game.assets.cover_medium.uri.unwrap_or(String::from("None")), game.assets.cover_large.uri.unwrap_or(String::from("None")),
    game.assets.trophy_1st.uri.unwrap_or(String::from("None")), game.assets.trophy_2nd.uri.unwrap_or(String::from("None")),
    game.assets.trophy_3rd.uri.unwrap_or(String::from("None")), 
    game.assets.trophy_4th.iter().map(|uri: &GameAssetUri| uri.uri.clone()).flatten().collect::<String>(),
    game.assets.background.uri.unwrap_or(String::from("None")), 
    game.assets.foreground.iter().map(|uri: &GameAssetUri| uri.uri.clone()).flatten().collect::<String>()]);
    game_data.append(&mut game.links.iter().map(|rel| rel.rel.clone()).collect());
    game_data.append(&mut game.links.iter().map(|uri| uri.uri.clone()).collect());
    
    Ok(game_data)
}

pub fn game_data(game: &str) -> Vec<String>{
    main(game.to_string())
}

/// Returns the game id of a game
/// 
/// ## Returns
/// 
/// game_id: `String`, the id of the game
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::game_id;
/// let result = game_id("Mc");
/// assert_eq!(result, "j1npme6p");
/// ```

pub fn game_id(game: &str) -> String{
    let id = &main(game.to_string())[0];
    return id.to_string();
}

#[doc(hidden)]
/// ### NOTE!
/// This is a value for the future to be used when support for game ids being queryed gets added!
pub fn game_name(game: &str) -> String{
    let name = &main(game.to_string())[1];
    return name.to_string();
}

/// Returns the number of boosts a game has on speedrun.com
/// 
/// ## Returns
/// 
/// boosts: `i32`, the number of boosts a game has
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::game_boosts;
/// let result: i32 = game_boosts("Mc");
/// assert_eq!(result, 0);
/// ```

pub fn game_boosts(game: &str) -> i32{
    let boosts = &main(game.to_string())[4];
    return boosts.parse::<i32>().unwrap();
}

/// Returns the abbreviation of a game
/// 
/// ## Returns
/// 
/// abbreviation: `String`, the abbreviation of the game
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::game_abbreviation;
/// let result = game_abbreviation("Mc");
/// assert_eq!(result, "mc");
/// ```

pub fn game_abbreviation(game: &str) -> String{
    let abbrev = &main(game.to_string())[6];
    return abbrev.to_string();
}

/// Returns a games weblink
/// 
/// ## Returns
/// 
/// weblink: `String`, the weblink of the game 
/// The weblink is the link you would put in a search bar to see the games home page
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::game_weblink;
/// let result = game_weblink("Mc");
/// assert_eq!(result, "https://www.speedrun.com/mc");
/// ```

pub fn game_weblink(game: &str) -> String{
    let weblink = &main(game.to_string())[7];
    return weblink.to_string();
}

/// Returns the discord of a game
/// 
/// ## Returns
/// 
/// discord: `String`, the discord of the game
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::game_discord;
/// let result = game_discord("Mc");
/// assert_eq!(result, "https://discord.gg/jmdFn3C");
/// ```

pub fn game_discord(game: &str) -> String{
    let discord = &main(game.to_string())[8];
    return discord.to_string();
}

/// Returns the release date of a game
/// 
/// ## Returns
/// 
/// release_date: `i32`, the release date of the game
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::game_release_date;
/// let result = game_release_date("Mc");
/// assert_eq!(result, 2011);
/// ```

pub fn game_release_date(game: &str) -> i32{
    let release_date = &main(game.to_string())[9];
    return release_date.parse::<i32>().unwrap();
}

/// Returns the ruleset of a game
/// 
/// ## Returns
/// 
/// ruleset: `Vec<String>`, the ruleset of the game
/// Each of the points in the ruleset can be indexed to get the value of the corresponding point
/// The poins are in order:
/// 
/// [Show Milliseconds [True/Flase], Require Verification [True/False], 
/// Require Video [True/False], Defult Time [String], Emulators Allowed [True/False], 
/// Romhacks Allowed [True/False]]
/// 
/// **Notes:**
/// 
/// -The values that are true or flase can be parsed into a booleen
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::game_ruleset;
/// let result = game_ruleset("Mc");
/// assert_eq!(result, ["true", "true", "true", "ingame", "false", "false"])
/// ```
/// This shows returning the whole array with all of the values
/// 
/// ```rust
/// use speedrunapi::game_ruleset;
/// let result = game_ruleset("Mc");
/// assert_eq!(result[0].parse::<bool>().unwrap(), true)
/// ```
/// This shows indexing a value and parsing it to its booleen value (True)

pub fn game_ruleset(game: &str) -> Vec<String>{
    let ruleset = &main(game.to_string());
    let rulesetpoints = vec![&ruleset[11], &ruleset[12], &ruleset[13], &ruleset[14], &ruleset[15], 
    &ruleset[16]];
    // You must collect the data points to change ownership and ensure order is correct
    let strings: Vec<String> = rulesetpoints.iter().map(|s| s.to_string()).collect();
    return strings.to_vec()
}

/// Returns the date where the game was added to speedrun.com
/// 
/// ## Returns 
/// 
/// creation_date: `String`, the date when the game was added to speedrun.com
/// 
/// **Notes**
/// 
/// - The date of the game has to be translated into a human readable date. 
/// This translation (relying on an external crate) may be unoptimised and slow.
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::game_creation;
/// let result = game_creation("Mc");
///  assert_eq!(result, "2015-01-29 23:41:21")
/// ```

pub fn game_creation(game: &str) -> String{
    let creation = &main(game.to_string())[17];
    let translated = tl_time(creation);
    return translated
}

/// Returns the assets of a game
/// 
/// ## Returns
/// 
/// assets: `Vec<String>`, the assets of the game
/// 
/// Each of the assets can be indexed to get the value of the corresponding asset
/// 
/// The assets are in order:
/// 
/// [Cover Tiny [String], Cover Medium [String], Cover Large [String], Trophy 1st [String], 
/// Trophy 2nd [String], Trophy 3rd [String], Trophy 4th [String], Background [String], Foreground [String]]
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::game_assets;
/// let result = game_assets("Mc");
/// assert_eq!(result[0], "https://www.speedrun.com/themeasset/2wo6q4we/logo?v=413b0b3")
/// ```

pub fn game_assets(game: &str) -> Vec<String>{
    let asset = &main(game.to_string());
    let assets = vec![&asset[18], &asset[19], &asset[20], &asset[21], &asset[22], &asset[23], &asset[24], 
    &asset[25], &asset[26], &asset[27], &asset[28]];
    // You must collect the data points to change ownership and ensure order is correct
    let strings: Vec<String> = assets.iter().map(|s| s.to_string()).collect();
    return strings.to_vec()
}

// Add in links (may take a bit of work)

fn main(game: String) -> Vec<String>{
    game_reqwest(&game.to_string()).expect("An Unexpected error occured when fetching the game!")
}