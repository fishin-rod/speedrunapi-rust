//! Game Types
//! 
//! Game types are classifications for unofficial games, such as ROMS.
//! 
//! # Arguments:
//! 
//! When calling for Gametypes first asing a variable to GameTypeData after importing it. 
//! After that add ::new("gametype_name/id") and call .run() to get the data.
//! 
//! # Example:
//! 
//! ```rust
//! use speedrunapi::GameTypeData;
//! let result = GameTypeData::new("Fangame").run();
//! println!("{:?}", result);
//! ```
//! This will fetch the entirenty of the data for the gametype as a json object and print it.

use crate::types::GameTypeData as Data;

#[derive(Debug)]
pub struct GameTypeData{
    pub gametype: String,
}

#[derive(Debug)]
pub enum GameTypeError {
    GameTypeNotFound,
    InvalidArguments,
    ReqwestError(reqwest::Error),
}

#[derive(Debug)]
pub enum GameTypeResult {
    GameType(Data),
    None,
    Error(GameTypeError),
}

impl GameTypeResult{
    
    /// Returns the name of the gametype
    /// 
    /// ## Returns:
    /// 
    /// The name of the gametype as an &str
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::GameTypeData;
    /// let result = GameTypeData::new("Fangame").run();
    /// assert_eq!(result.name(), "Fangame");
    /// ```
    
    pub fn name(&self) -> &str{
        if let GameTypeResult::GameType(gametype_data) = self{
            &gametype_data.data.name
        }
        else{
            panic!("Cannot Get name from: {:?}", self);
        }
    }

    /// Returns the id of a gametype
    /// 
    /// ## Returns:
    /// 
    /// The id of the gametype as a &str
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::GameTypeData;
    /// let result = GameTypeData::new("Fangame").run();
    /// assert_eq!(result.id(), "d91jd1ex")
    /// ```
    
    pub fn id(&self) -> &str{
        if let GameTypeResult::GameType(gametype_data) = self{
            &gametype_data.data.id
        }
        else{
            panic!("Cannot Get id from: {:?}", self);
        }
    }
}

impl GameTypeData{
    
    /// Creates a new GameTypeData object
    /// 
    /// # Arguments:
    /// 
    /// `gametype: &str` - The name or id of the gametype you are seraching for
    /// 
    /// # Examples:
    /// ```rust
    /// use speedrunapi::GameTypeData;
    /// let result = GameTypeData::new("Fangame");
    /// println!("{:?}", result);
    /// ```
    /// This will return the parameters passed to the function
    /// ```rust
    /// use speedrunapi::GameTypeData;
    /// let result = GameTypeData::new("Fangame").run();
    /// println!("{:?}", result);
    /// ```
    /// This will print the JSON data of the gametype

    pub fn new(gametype: &str) -> GameTypeData{
        GameTypeData{
            gametype: gametype.to_string(),
        }
    }

    /// Runs the GameType Object
    /// 
    /// # Arguments:
    /// 
    /// This function requires that you have called the new function with the nessiasary parametrers first
    /// 
    /// # Returns:
    /// 
    /// The result of the request as a GameTypeResult object
    /// 
    /// If an error has occurred the program may return None or Error(ErrorType) as an OK
    /// 
    /// # Example:
    /// ```rust
    /// use speedrunapi::GameTypeData;
    /// let result = GameTypeData::new("Fangame").run();
    /// println!("{:?}", result);
    /// ```
    /// This will return the data from the gametype you are searching for

    #[tokio::main]
    pub async fn run(&self) -> GameTypeResult{
        let client = reqwest::Client::new();
        let url = format!("https://www.speedrun.com/api/v1/gametypes/{:1}", self.gametype);
        let response = match client.get(url).send().await{
            Ok(response) => response,
            Err(err) => return GameTypeResult::Error(GameTypeError::ReqwestError(err)),
        };
        if response.status() == reqwest::StatusCode::NOT_FOUND{
            return GameTypeResult::Error(GameTypeError::GameTypeNotFound);
        }
        let response = match response.json::<Data>().await{
            Ok(response) => response,
            Err(err) => return GameTypeResult::Error(GameTypeError::ReqwestError(err)),
        };
        GameTypeResult::GameType(response)
    }
}