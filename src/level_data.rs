//! Level data
//! 
//! This module provides data about levels on speedrun.com
//! 
//! Levels are subcategories of games.
//! 
//! # Arguments:
//! 
//! When Fetching a level you can use the levels ID to fetch the data, with planned support for the name!
//! 
//! When calling for leveldata first you must Inintialize the builder, by calling LevelData,
//! then to actutulay get it started you must provide ::new("level_name") suppling the builder with the level,
//! and then call .run() to get the data.
//! If you choose to use an extension of the Level Data, call that before the run statment. 
//! 
//! # Examples:
//! 
//! ```rust
//! use speedrunapi::LevelData;
//! let result = LevelData::new("495ggmwp").run();
//! println!("{:?}", result);
//! ```
//! This will fetch the entirenty of the data for the level as a json object and print it.
//! ```rust
//! use speedrunapi::LevelData;
//! let result = LevelData::new("495ggmwp").run();
//! println!("{:?}", result.name());
//! ```
//! This will fetch only the name of the level as a string and print it

use crate::types::LevelData as Data;

#[derive(Debug)]
pub struct LevelData{
    pub name: String,
    categories: bool,
}

#[derive(Debug)]
pub enum LevelError {
    LevelNotFound,
    InvalidArguments,
    ReqwestError(reqwest::Error),
}

#[derive(Debug)]
pub enum LevelResult {
    Level(Data),
    None,
    Error(LevelError),
}

impl LevelResult {

    /// Returns the name of the level
    /// 
    /// ## Returns:
    /// 
    /// The name of the level as an &str
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::LevelData;
    /// let result = LevelData::new("495ggmwp").run();
    /// assert_eq!(result.name(), "Shrub Forest");
    /// ```

    pub fn name(&self) -> &str {
        if let LevelResult::Level(level_data) = self {
            &level_data.data.name
        }
        else{
            panic!("Cannot get name from: {:?}", self);
        }
    }

    /// Returns the ID of a level
    /// 
    /// ## Returns:
    /// 
    /// The ID of the level as an &str
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::LevelData;
    /// let result = LevelData::new("495ggmwp").run();
    /// assert_eq!(result.id(), "495ggmwp");
    /// ```
    
    pub fn id(&self) -> &str {
        if let LevelResult::Level(level_data) = self {
            &level_data.data.id
        }
        else{
            panic!("Cannot get id from: {:?}", self);
        }
    }

    /// Returns the weblink of a level
    /// 
    /// ## Returns:
    /// 
    /// The weblink of the level as an &str
    /// 
    /// *Note:
    /// 
    /// The weblink is the url used to view the page on speedrun.com
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::LevelData;
    /// let result = LevelData::new("495ggmwp").run();
    /// assert_eq!(result.weblink(), "https://www.speedrun.com/pokemon_rumble_world/Shrub_Forest");
    /// ```
    
    pub fn weblink(&self) -> &str {
        if let LevelResult::Level(level_data) = self {
            &level_data.data.weblink
        }
        else{
            panic!("Cannot get weblink from: {:?}", self);
        }
    }

    /// Returns the rules of a level
    /// 
    /// ## Returns:
    /// 
    /// The rules of the level as a string
    /// 
    /// *Note: 
    /// 
    /// The character string "\r\n\r\n" does have to be removed from the stirng in some spots. Be prepared if you are looking for them
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::LevelData;
    /// let result = LevelData::new("495ggmwp").run();
    /// assert_eq!(result.rules(), "Normal Mode: Just go as fast as you can through the level. hard Mode: You must kill every Pokemon [Except the infinite re-spawning ones at the boss battle] Your time is the In game time.")
    /// ```

    pub fn rules(&self) -> String {
        if let LevelResult::Level(level_data) = self {
            let rules = &level_data.data.rules;
            rules.replace("\r\n\r\n", " ")
        }
        else{
            panic!("Cannot get rules from: {:?}", self);
        }
    }

    /// Returns the links associated with the level
    /// 
    /// ## Returns:
    /// 
    /// A Vector of tuples contating the link name and the url to the link
    /// 
    /// ## Examples:
    /// ```rust
    /// use speedrunapi::LevelData;
    /// let result = LevelData::new("495ggmwp").run();
    /// println!("{:?}", result.links());
    /// ```
    /// This example prints out the entire links vector
    /// ```rust
    /// use speedrunapi::LevelData;
    /// let result = LevelData::new("495ggmwp").run();
    /// println!("{:?}", result.links()[0].0);
    /// assert_eq!(result.links()[0].0, "self");
    /// ```
    /// This prints out the first name of a link in the links vector 
    
    pub fn links(&self) -> Vec<(String, String)> {
        if let LevelResult::Level(level_data) = self {
            let links = &level_data.data.links;
            let names: Vec<&str> = links.iter().map(|link| link.rel.as_str()).collect();
            let urls: Vec<&str> = links.iter().map(|link| link.uri.as_str()).collect();
            let merged_vec: Vec<(String, String)> = names.iter().zip(urls.iter()).map(|(x, y)| (x.to_string(), y.to_string())).collect();
            return merged_vec.to_vec();
        }      
        else{
            panic!("Cannot get links from: {:?}", self);
        }  
    }
}

impl LevelData{

    /// Creates a new LevelData Object
    /// 
    /// # Arguments:
    /// 
    /// `name: &str` - the name of the level you are seraching for
    /// 
    /// # Examples:
    /// ```rust
    /// use speedrunapi::LevelData;
    /// let result = LevelData::new("495ggmwp");
    /// println!("{:?}", result);
    /// ```
    /// This will return the parameters passed to the function, to return the data you will need to add run to the code.
    /// ```rust
    /// use speedrunapi::LevelData;
    /// let result = LevelData::new("495ggmwp").run();
    /// println!("{:?}", result);
    /// ```
    
    pub fn new(name: &str) -> LevelData{
        LevelData{
            name: name.to_string(),
            categories: false,
        }
    }

    /// Does nothing for now but will swich embed to get categories later
    pub fn categories(mut self) -> Self{
        self.categories = true;
        self
    }

    /// Runs the request to the speedrun.com API
    /// 
    /// # Arguments:
    /// 
    /// This function requires that you have called the new function with the nessiasary parametrers first
    /// 
    /// # Returns:
    /// 
    /// The result of the request as a RunResult object
    /// 
    /// If an error has occurred the program may return None or Error(ErrorType) as an OK
    /// 
    /// # Example:
    /// ```rust
    /// use speedrunapi::LevelData;
    /// let result = LevelData::new("495ggmwp").run();
    /// println!("{:?}", result);
    /// ```
    /// This will return the data from the level you are searching for
    
    #[tokio::main]
    pub async fn run(&self) -> LevelResult {
        let client = reqwest::Client::new();
        let url = if self.categories{
            // fix url later
            format!("https://www.speedrun.com/api/v1/levels/{}/categories", self.name)
        } else{
            format!("https://www.speedrun.com/api/v1/levels/{}", self.name)
        };
        let response = match client.get(url).send().await{
            Ok(response) => response,
            Err(err) => return LevelResult::Error(LevelError::ReqwestError(err.into())),
        };
        if response.status() == reqwest::StatusCode::NOT_FOUND{
            return LevelResult::Error(LevelError::LevelNotFound);
        }
        if self.categories{
            // for now print later on deseralize
            println!("test");
            LevelResult::None
        }
        else{
            let response = match response.json::<Data>().await{
                Ok(response) => response,
                Err(err) => return LevelResult::Error(LevelError::ReqwestError(err.into())),
            };
            LevelResult::Level(response)
        }
    }

}
