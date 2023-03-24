//! Games
//! 
//! Handles requests for a games data
//! 
//! What is a game?
//! A game is where users can submit runs for different categories and leaderboards connected to the game.
//! 

use crate::types::GameData as Data;
use crate::tl_time;

#[derive(Debug)]
pub struct GameData{
    pub name: String,
    categories: bool,
    levels: bool,
    variables: bool,
    derived_games: bool,
    records: bool,
}

#[derive(Debug)]
pub enum GameResult {
    Game(Data),
    None,
    Error(GameError),
}

#[derive(Debug)]
pub enum GameError {
    GameNotFound,
    InvalidArguments,
    ReqwestError(reqwest::Error),
}

/// Generates a function that gets a datype from the result.
macro_rules! Generate_Function {
    ($doc:expr, $name:ident, $($field:tt).+, $datatype:ty $(, $args:ident($($extra_args:tt)*))?) => {
        #[doc = $doc]
        pub fn $name(&self) -> $datatype {
            if let GameResult::Game(game_data) = self {
                let value = game_data.data.$($field).+.clone();
                $(let value = value.clone().$args($($extra_args)*);)?
                value
            } else {
                panic!("Cannot get data from: {:?}", self);
            }
        }
    };
}

impl GameResult{

   // Generate_Function!("Test", abbreviations, names.international, String);
   // Generate_Function!("## test2 ", data, platforms, Vec<String>, unwrap_or(vec!["None".to_string()]));

    Generate_Function!("Returns the name of the game \n ## Returns:  \n The name of the game in english as a string \n ## Example: \n
    use speedrunapi::GameData;
    let result = GameData::new(\"Mc\").run();
    assert_eq!(result.name(), \"Minecraft: Java Edition\")", name, names.international, String);

    Generate_Function!("Returns the japanese name of the game \n ## Returns: \n The name of the game in japanese as a string \n #### Notes:
    \n The japanese name may be \"None\" \n ## Example: \n
    use speedrunapi::GameData;
    let result = GameData::new(\"Mc\").run();
    assert_eq!(result.japanese_name(), \"None\")", japanese_name, names.japanese, String, unwrap_or(String::from("None")));

    Generate_Function!("Returns the twitch name of the game \n ## Returns: \n The twitch name of the game as a String \n ## Example: \n
    use speedrunapi::GameData;
    let result = GameData::new(\"Mc\").run();
    assert_eq!(result.twitch_name(), \"Minecraft\")", twitch_name, names.twitch, String);

    Generate_Function!("Returns the ID of the game \n ## Returns: \n The ID of the game as a String \n ## Example: \n
    use speedrunapi::GameData;
    let result = GameData::new(\"Mc\").run();
    assert_eq!(result.id(), \"j1npme6p\")", id, id, String);

    Generate_Function!("Returns the number of boosts a game has received \n ## Returns: \n The number of boosts a game has recived as an i32 \n ## Example:
    use speedrunapi::GameData;
    let result = GameData::new(\"Mc\").run();
    assert_eq!(result.boosts(), 0);", boosts, boosts_received, i32, unwrap_or(i32::from(0)));

    Generate_Function!("Returns the number of unique players who have boosted a game \n ## Returns: \n The number of uniqe players who have boosted a game as an i32 \n ## Example:
    use speedrunapi::GameData;
    let result = GameData::new(\"Mc\").run();
    assert_eq!(result.boosters(), 0);", boosters, boost_distinct_donors, i32, unwrap_or(i32::from(0)));

    Generate_Function!("Returns the abbreviation of a game \n ## Returns: \n The abbreviation of a game as a string \n ## Example: \n
    use speedrunapi::GameData;
    let result = GameData::new(\"Mc\").run();
    assert_eq!(result.abbreviation(), \"mc\")", abbreviation, abbreviation, String);

    Generate_Function!("Returns the weblink of a game \n ## Returns: \n The weblink of a game as a string \n ## Example: \n
    use speedrunapi::GameData;
    let result = GameData::new(\"Mc\").run();
    assert_eq!(result.weblink(), \"https://www.speedrun.com/mc\")", weblink, weblink, String);

    Generate_Function!("Returns the game discord \n ## Returns: \n The discord of a game as a string \n ## Example: \n
    use speedrunapi::GameData;
    let result = GameData::new(\"Mc\").run();
    assert_eq!(result.discord(), \"https://discord.gg/jmdFn3C\")", discord , discord, String);

    Generate_Function!("Returns the date the game was released \n ## Returns: \n The year the game was released as a i16 \n ## Example: \n
    use speedrunapi::GameData;
    let result = GameData::new(\"Mc\").run();
    assert_eq!(result.released(), 2011)", released, released, i16);

    Generate_Function!("Returns the date the game was released \n ## Returns: \n the date the game was released as a string \n Note:
    This function is similar to the released function it is just newer and more specific, so some games have yet to migrate or set it so it can be \"None\" \n ## Example: \n
    use speedrunapi::GameData;
    let result = GameData::new(\"Mc\").run();
    assert_eq!(result.release_date(), \"None\")", release_date, release_date, String, unwrap_or(String::from("None")));

    /*
    // Figure out how to be able to return multiple values
    pub fn ruleset(&self, rule: String) {
        if let GameResult::Game(game_data) = self {
            match rule.to_lowercase().as_str(){

            }
        }
        else{
            panic!("Cannot get ruleset from: {:?}", self);
        }
    }
    */

    Generate_Function!("Returns if the game has romhacks \n ## Returns: \n If the game has romhacks as a bool 
    \n Note: Use gametypes for more information \n ## Example: \n
    use speedrunapi::GameData;
    let result = GameData::new(\"Mc\").run();
    assert_eq!(result.romhack(), false)", romhack, romhack, bool);

    Generate_Function!("Returns the gametypes for a game \n ## Returns: \n The game types of a game in a vec of strings 
    \n If there is none listed defults to \\[\"None\"\\] or \\[\\] \n ## Example: \n
    use speedrunapi::GameData;
    let result: Vec<String> = GameData::new(\"Mc\").run().gametypes();
    assert_eq!(result, Vec::<String>::new())", gametypes, gametypes, Vec<String>, unwrap_or(vec!["None".to_string()]));

    Generate_Function!("Returns the platforms of a game \n ## Returns: \n the platforms of a game in a vec of strings
    \n If there is none listed defults to \\[\"None\"\\] or \\[\\] \n ## Example: \n
    use speedrunapi::GameData;
    let result: Vec<String>  = GameData::new(\"Mc\").run().platforms();
    assert_eq!(result, [\"8gej2n93\"])", platforms, platforms, Vec<String>, unwrap_or(vec!["None".to_string()]));

    Generate_Function!("Returns the regions of a game \n ## Returns: \n the regions of a game as a Vec<String>
    \n If there is none listed defults to \\[\"None\"\\] or \\[\\] \n ## Example: \n
    use speedrunapi::GameData;
    let result: Vec<String> = GameData::new(\"Mc\").run().regions();
    assert_eq!(result, Vec::<String>::new())", regions, regions, Vec<String>, unwrap_or(vec!["None".to_string()]));

    Generate_Function!("Returns the genres of a game \n ## Returns: \n the genres of a game as a Vec<String>
    \n If there is none listed defults to \\[\"None\"\\] or \\[\\] \n ## Example: \n
    use speedrunapi::GameData;
    let result: Vec<String> = GameData::new(\"Mc\").run().genres();
    assert_eq!(result, [\"q4n60ln9\", \"jp230326\"])", genres, genres, Vec<String>, unwrap_or(vec!["None".to_string()]));

    Generate_Function!("Returns the engines of a game \n ## Returns: \n the engines of a game as a Vec<String>
    \n If there is none listed defults to \\[\"None\"\\] or \\[\\] \n ## Example: \n
    use speedrunapi::GameData;
    let result: Vec<String> = GameData::new(\"Mc\").run().engines();
    assert_eq!(result, Vec::<String>::new())", engines, engines, Vec<String>, unwrap_or(vec!["None".to_string()]));

    Generate_Function!("Returns the developers of a game \n ## Returns: \n the developers of a game as a Vec<String>
    \n If there is none listed defults to \\[\"None\"\\] or \\[\\] \n ## Example: \n
    use speedrunapi::GameData;
    let result: Vec<String> = GameData::new(\"Mc\").run().developers();
    assert_eq!(result, [\"k62d97ex\"])", developers, developers, Vec<String>, unwrap_or(vec!["None".to_string()]));

    Generate_Function!("Returns the publishers of a game \n ## Returns: \n the publishers of a game as a Vec<String>
    \n If there is none listed defults to \\[\"None\"\\] or \\[\\] \n ## Example: \n
    use speedrunapi::GameData;
    let result: Vec<String> = GameData::new(\"Mc\").run().publishers();
    assert_eq!(result, Vec::<String>::new())", publishers, publishers, Vec<String>, unwrap_or(vec!["None".to_string()]));

    /// Returns the moderators of a game
    /// 
    /// ## Returns:
    /// 
    /// The moderators and their positions of a game as a Vec<(String, String)>
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::GameData;
    /// let result = GameData::new("Mc").run().moderators();
    /// println!("{:?}", result);
    /// ```
    
    pub fn moderators(&self) -> Vec<(String, String)> {
        //for translations
        //use crate::user_data::UserData;
        if let GameResult::Game(game_data) = self {
            let mods = game_data.data.moderators.clone();
            let ids: Vec<String> = mods.iter().map(|x| x.0.to_string()).collect();
            let positions: Vec<String> = mods.iter().map(|x| x.1.to_string()).collect();
            for pos in 1..positions.len(){
                positions[pos].trim_matches(|c| c == '\\' || c == '"').to_string();
            }
            let merged_vec: Vec<(String, String)> = ids.iter().zip(positions.iter()).map(|(x, y)| (x.to_string(), y.to_string())).collect();
            return merged_vec.to_vec();
        }
        else{
            panic!("Cannot get moderators from: {:?}", self);
        }
    }
    
    /// Returns when the game was created on speedrun.com
    /// 
    /// ## Returns:
    /// 
    /// The date the game was created as a string
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::GameData;
    /// let result: String = GameData::new("Mc").run().created();
    /// assert_eq!(result, "2015-01-29 23:41:21")
    /// ```
    
    pub fn created(&self) -> String{
        if let GameResult::Game(game_data) = self{
            tl_time!(game_data.data.created)
        }
        else {
            panic!("Cannot get creation date from: {:?}", self)
        }
    }
    
    /// Returns the games assets
    /// 
    /// ## Arguments:
    /// 
    /// `asset_type: &str`: The type of assets to return (NON CASE SENSITIVE)
    /// asset_type can be: 
    /// - logo
    /// - cover_tiny
    /// - cover_small
    /// - cover_medium
    /// - cover_large
    /// - icon
    /// - trophy_1st
    /// - trophy_2nd
    /// - trophy_3rd
    /// - trophy_4th
    /// - background
    /// - foreground
    /// 
    /// ## Returns:
    /// 
    /// The link to the asset as a string
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::GameData;
    /// let result: String = GameData::new("Mc").run().assets("logo");
    /// assert_eq!(result, "https://www.speedrun.com/themeasset/2wo6q4we/logo?v=413b0b3")
    /// ```
    /// 
    /// ## Panics!
    /// 
    /// The function will panic if it recives an invalid asset type or no asset type at all!
    
    pub fn assets(&self, asset_type: &str) -> String{
        if let GameResult::Game(game_data) = self {
            match asset_type.to_lowercase().as_str(){
                "logo" => game_data.data.assets.logo.uri.clone().unwrap_or(String::from("None")),
                "cover_tiny" => game_data.data.assets.cover_tiny.uri.clone().unwrap_or(String::from("None")),
                "cover_small" => game_data.data.assets.cover_small.uri.clone().unwrap_or(String::from("None")),
                "cover_medium" => game_data.data.assets.cover_medium.uri.clone().unwrap_or(String::from("None")),
                "cover_large" => game_data.data.assets.cover_large.uri.clone().unwrap_or(String::from("None")),
                "icon" => game_data.data.assets.icon.uri.clone().unwrap_or(String::from("None")),
                "trophy_1st" => game_data.data.assets.trophy_1st.uri.clone().unwrap_or(String::from("None")),
                "trophy_2nd" => game_data.data.assets.trophy_2nd.uri.clone().unwrap_or(String::from("None")),
                "trophy_3rd" => game_data.data.assets.trophy_3rd.uri.clone().unwrap_or(String::from("None")),
                "trophy_4th" => game_data.data.assets.trophy_4th.iter().map(|link| link.uri.clone().unwrap_or(String::from("None"))).collect(),
                "background" => game_data.data.assets.background.uri.clone().unwrap_or(String::from("None")),
                "foreground" => game_data.data.assets.foreground.iter().map(|link| link.uri.clone().unwrap_or(String::from("None"))).collect(),
                _ => panic!("Invalid asset type: {:?}", asset_type)
            }
       }
        else{
            panic!("Cannot get assets from: {:?}", self);
        }
    }

    /// Returns the links of a game
    /// 
    /// ## Returns:
    /// 
    /// The links of a game as a Vec<(String, String)>
    /// The format is (name, link)
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::GameData;
    /// let result: Vec<(String, String)> = GameData::new("Mc").run().links();
    /// println!("{:?}", result);
    /// ```

    pub fn links(&self) -> Vec<(String, String)> {
        if let GameResult::Game(game_data) = self {
            let game = &game_data.data.links;
            let names: Vec<&str> = game.iter().map(|link| link.rel.as_str()).collect();
            let urls: Vec<&str> = game.iter().map(|link| link.uri.as_str()).collect();
            let merged_vec: Vec<(String, String)> = names.iter().zip(urls.iter()).map(|(x, y)| (x.to_string(), y.to_string())).collect();
            return merged_vec.to_vec();
        }
        else{
            panic!("Cannot get links from: {:?}", self);
        }
    }

}

impl GameData{
    
    /// Creates a new GameData object
    /// 
    /// # Arguments:
    /// 
    /// `game: &str`: The name of the game
    /// 
    /// # Examples:
    /// ```rust
    /// use speedrunapi::GameData;
    /// let result = GameData::new("Mc");
    /// ```
    /// This will create a new GameData object with the name "Mc" but when printed will only print out the parameters passed to it.
    /// To show the date recieved you need to add .run()
    /// ```rust
    /// use speedrunapi::GameData;
    /// let result = GameData::new("Mc").run();
    /// println!("{:?}", result);
    /// ```

    pub fn new(game: &str) -> GameData{
        GameData{
            name: game.to_string(),
            categories: false,
            levels: false,
            variables: false,
            derived_games: false,
            records: false,
        }
    }

    /// Runs the request to the speedrun.com API
    /// 
    /// # Arguments:
    /// 
    /// This function requires that you have called the new function with the nessiasary parametrers first
    /// 
    /// # Returns:
    /// 
    /// The result of the request as a GameData object
    /// 
    /// If an error has occurred the program may return None or Error(ErrorType) as an OK
    /// 
    /// # Example:
    /// ```rust
    /// use speedrunapi::GameData;
    /// let result = GameData::new("Mc").run();
    /// println!("{:?}", result);
    /// ```
    
    #[tokio::main]
    pub async fn run(&self) -> GameResult{
        let client = reqwest::Client::new();
        let mut url = format!("https://www.speedrun.com/api/v1/games/{:1}", self.name);
        if self.categories{
            url.push_str("/categories");
        }
        if self.levels{
            url.push_str("/levels");
        }
        if self.variables{
            url.push_str("/variables");
        }
        if self.derived_games{
            url.push_str("/derived-games");
        }
        if self.records{
            url.push_str("/records");
        }
        else{
            url.to_string();
        }
        let response = match client.get(url).send().await{
            Ok(response) => response,
            Err(err) => return GameResult::Error(GameError::ReqwestError(err.into())),
        };
        if response.status() == reqwest::StatusCode::NOT_FOUND{
            return GameResult::Error(GameError::GameNotFound);
        }
        if self.categories{
            // for now print later on deseralize
            println!("test");
            GameResult::None
        }
        else{
            let response = match response.json::<Data>().await{
                Ok(response) => response,
                Err(err) => return GameResult::Error(GameError::ReqwestError(err.into())),
            };
            GameResult::Game(response)
        }
    }
}
