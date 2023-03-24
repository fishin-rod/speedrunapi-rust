//! # Users
//!
//! Handles user requests and returing the user data.
//! 
//! What are users anyways?
//! Users are the people on speedrun.com that have an account.
//! This account then can submit runs and chat in fourms on the website.
//! 
//! Users have different data points assosiated with them
//! 
//! You can use the functions in this module to get differnt parts and peices of the data or the whole set!
//!

use crate::types::UserData as Data;
use crate::tl_time;

#[derive(Debug)]
pub struct UserData{
    pub name: String,
    personal_bests: bool,
}

#[derive(Debug)]
pub enum UserResult{
    User(Data),
    None,
    Error(UserError),
}

#[derive(Debug)]
pub enum UserError {
    UserNotFound,
    InvalidArguments,
    ReqwestError(reqwest::Error),
}

impl UserResult{

    /// Returns the name of the user
    /// 
    /// ## Returns:
    /// 
    /// The name of the user as an String
    /// 
    /// # Example:
    /// ```rust
    /// use speedrunapi::UserData;
    /// let result = UserData::new("fishin_rod").run();
    /// assert_eq!(result.name(), "fishin_rod");
    /// ```
    
    pub fn name(&self) -> String {
        if let UserResult::User(user_data) = self {
            user_data.data.names.international.to_string()
        }
        else{
            panic!("Cannot Get name from: {:?}", self);
        }
    }

    // Get japanese and inernational to be in the same function 
    // ## Arguments:
    // 
    // - `Japanese: bool` - If you want to return the Japanese name of the user set this to true, 
    // if not leave empty
    // 


    /// Return the jaapanese name of the user
    /// 
    /// ## Returns:
    /// 
    /// The Japanese name of the user as an String
    /// The Japanese name of the user can be equal to "None"
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::UserData;
    /// let result = UserData::new("fishin_rod").run();
    /// assert_eq!(result.japanese_name(), "None");
    /// ```
    
    pub fn japanese_name(&self) -> String {
        if let UserResult::User(user_data) = self {
            user_data.data.names.japanese.clone().unwrap_or(String::from("None"))
        }
        else{
            panic!("Cannot Get name from: {:?}", self);
        }
    }

    /// Returns the id of a user
    /// 
    /// ## Returns:
    /// 
    /// The id of the user as a String
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::UserData;
    /// let result = UserData::new("fishin_rod").run();
    /// assert_eq!(result.id(), "jonryvl8");
    /// ```
    
    pub fn id(&self) -> String {
        if let UserResult::User(user_data) = self {
            user_data.data.id.clone()
        }
        else{
            panic!("Cannot Get id from: {:?}", self);
        }
    }

    /// Rerturns if the user has a supporter animation or not
    /// 
    /// ## Returns:
    /// 
    /// Returns true if the user has a supporter animation, false if not in boolen format
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::UserData;
    /// let result = UserData::new("fishin_rod").run();
    /// assert_eq!(result.supporter_animation(), false);
    /// ```
    
    pub fn supporter_animation(&self) -> bool {
        if let UserResult::User(user_data) = self {
            user_data.data.supporter_animation
        }
        else{
            panic!("Cannot Get animation from: {:?}", self);
        }
    }

    /// Returns the pronouns the user has listed on their profile
    /// 
    /// ## Returns:
    /// 
    /// The pronouns the user has listed on their profile as a String
    /// The pronouns can be "None" if the user has not selected any pronouns
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::UserData;
    /// let result = UserData::new("fishin_rod").run();
    /// assert_eq!(result.pronouns(), "He/Him");
    /// ```
    
    pub fn pronouns(&self) -> String {
        if let UserResult::User(user_data) = self {
            user_data.data.pronouns.clone().unwrap_or(String::from("None"))
        }
        else{
            panic!("Cannot Get pronouns from: {:?}", self);
        }
    }

    /// Returns the weblink of the user
    /// 
    /// The weblink is the link used to view the persons profile on your browser
    /// ## Returns:
    /// 
    /// The weblink of the user as a String
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::UserData;
    /// let result = UserData::new("fishin_rod").run();
    /// assert_eq!(result.weblink(), "https://www.speedrun.com/user/fishin_rod");
    /// ```
    
    pub fn weblink(&self) -> String {
        if let UserResult::User(user_data) = self {
            user_data.data.weblink.clone()
        }
        else{
            panic!("Cannot Get weblink from: {:?}", self);
        }
    }

    /// Returns the role of the user
    /// 
    /// ## Returns:
    /// 
    /// The role of the user as a String
    /// 
    /// The role of the user can be: 
    /// - user
    /// - banned
    /// - trusted
    /// - moderator
    /// - admin
    /// - programer
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::UserData;
    /// let result = UserData::new("fishin_rod").run();
    /// assert_eq!(result.role(), "user");
    /// ```
    
    pub fn role(&self) -> String {
        if let UserResult::User(user_data) = self {
            user_data.data.role.clone()
        }
        else{
            panic!("Cannot Get role from: {:?}", self);
        }
    }

    /// Returns the date the user signed up for speedrun.com
    /// 
    /// ## Returns:
    /// 
    /// The date the user signed up for speedrun.com as a String
    /// 
    /// **Notes:** 
    /// - For older accounts the signup can be "None"
    /// - The time is translated using the crates macro tl_time into a more user friendly format
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::UserData;
    /// let result = UserData::new("fishin_rod").run();
    /// assert_eq!(result.signup(), "2022-12-20 01:16:42");
    /// ```
     
    pub fn signup(&self) -> String {
        if let UserResult::User(user_data) = self {
            let date = user_data.data.signup.clone().unwrap_or(String::from("None"));
            if date != "None"{
                let date = tl_time!(date);
                return date;
            } 
            else{
                return date;
            }
        }
        else{
            panic!("Cannot Get date from: {:?}", self);
        }
    }

    /// Returns the country the user is from
    /// 
    /// ## Arguments:
    /// 
    /// `name_type: &str` - The format that you want the countrys name to be
    /// 
    /// name_type can be:
    /// - "international" : The countrys name in english 
    /// - "japanese" : The countrys name in japanese can be "None"
    /// - "code" : The code of the country
    /// 
    /// ## Returns:
    /// 
    /// The country in the type specified that the user is from as a String
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::UserData;
    /// let result = UserData::new("fishin_rod").run();
    /// assert_eq!(result.country("code"), "us");
    /// ```

    pub fn country(&self, name_type: &str) -> String {
        if let UserResult::User(user_data) = self {
            match name_type.to_lowercase().as_str() {
                "code" => user_data.data.location.country.code.clone(),
                "international" => user_data.data.location.country.names.international.clone(),
                "japanese" => user_data.data.location.country.names.japanese.clone().unwrap_or(String::from("None")),
                _ => panic!("Invalid type: {:?}", name_type),
            } 
        }
        else{
            panic!("Cannot Get country from: {:?}", self);
        }
    }

    /// Returns the region the user is from
    /// 
    /// ## Arguments:
    /// 
    /// `name_type: &str` - The format that you want the regions name to be
    /// 
    /// name_type can be:
    /// - "international" : The regions name in english 
    /// - "japanese" : The regions name in japanese can be "None"
    /// - "code" : The code of the region
    /// 
    /// ## Returns:
    /// 
    /// The region in the type specified that the user is from as a String
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::UserData;
    /// let result = UserData::new("fishin_rod").run();
    /// assert_eq!(result.region("code"), "us/co");
    /// ```
    
    pub fn region(&self, name_type: &str) -> String {
        if let UserResult::User(user_data) = self {
            match name_type.to_lowercase().as_str() {
                "code" => user_data.data.location.region.as_ref().map(|code| code.code.clone())
                .unwrap_or(String::from("None")),
                "international" => user_data.data.location.region.as_ref().map(|name| name.names.international.clone())
                .unwrap_or(String::from("None")),
                "japanese" => user_data.data.location.region.as_ref().map(|name| name.names.japanese.clone()
                .unwrap_or(String::from("None"))).unwrap_or(String::from("None")),
                _ => panic!("Invalid type: {:?}", name_type),
            }   
        } else{
            panic!("Cannot Get region from: {:?}", self);
        }
    }

    /// Returns the links the user has listed on their profile
    /// 
    /// ## Arguments:
    /// 
    /// You must provide the type of link you want in the function arguments
    /// The link must be an &str
    /// 
    /// The type can be:
    /// - twitch
    /// - hitbox
    /// - youtube
    /// - twitter
    /// - speedrunslive
    /// 
    /// If none are provided you will get and error,
    /// if you provide a type that the user doenst have the function will return "None"
    /// 
    /// ## Returns:
    /// 
    /// The link chosen of the user as a String
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::UserData;
    /// let result = UserData::new("fishin_rod").run();
    /// assert_eq!(result.links("twitch"), "None");
    /// ```
    
    pub fn links(&self, service: &str) -> String {
        if let UserResult::User(user_data) = self {
            match service.to_lowercase().as_str() {
                "twitch" => user_data.data.twitch.clone().unwrap_or(String::from("None")),
                "hitbox" => user_data.data.hitbox.clone().unwrap_or(String::from("None")),
                "youtube" => user_data.data.youtube.clone().unwrap_or(String::from("None")),
                "twitter" => user_data.data.twitter.clone().unwrap_or(String::from("None")),
                "speedrunslive" => user_data.data.speedrunslive.clone().unwrap_or(String::from("None")),
                _ => panic!("Invalid Service: {}", service),
            }
        } else{
            panic!("Cannot Get links from: {:?}", self);
        }
    }
    
    /// Returns the assets the user has on their profile
    /// 
    /// ## Arguments:
    /// 
    /// You must provide the type of asset you want in the function arguments
    /// The asset must be an &str
    /// 
    /// The type can be:
    /// - icon
    /// - supporter_icon
    /// - image
    /// 
    /// If none are provided you will get and error,
    /// if you provide a type that the user doenst have the function will return "None"
    /// 
    /// ## Returns:
    /// 
    /// The asset chosen of the user as a String
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::UserData;
    /// let result = UserData::new("fishin_rod").run();
    /// assert_eq!(result.assets("icon"), "None");
    /// ```
    
    pub fn assets(&self, asset: &str) -> String{
        if let UserResult::User(user_data) = self {
            match asset.to_lowercase().as_str() {
                "icon" => user_data.data.assets.icon.uri.clone().unwrap_or(String::from("None")),
                "supporter_icon" => user_data.data.assets.supporter_icon.as_ref().map(|link| link.uri.clone()).flatten().unwrap_or(String::from("None")),
                "image" => user_data.data.assets.image.uri.clone().unwrap_or(String::from("None")),
                _ => panic!("Invalid Asset: {}", asset),
            }
        } else{
            panic!("Cannot Get assets from: {:?}", self);
        }
    }

    /// Returns the links to other apis about the user
    /// 
    /// ## Returns
    /// 
    /// The links to other apis about the user as a Vec<(String, String)>
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::UserData;
    /// let result = UserData::new("fishin_rod").run();
    /// println!("{:?}", result.user_links());
    /// ```
    
    pub fn user_links(&self) -> Vec<(String, String)> {
        if let UserResult::User(user_data) = self {
            let links = &user_data.data.links;
            let names: Vec<&str> = links.iter().map(|link| link.rel.as_str()).collect();
            let urls: Vec<&str> = links.iter().map(|link| link.uri.as_str()).collect();
            let merged_vec: Vec<(String, String)> = names.iter().zip(urls.iter()).map(|(x, y)| (x.to_string(), y.to_string())).collect();
            return merged_vec.to_vec();
        } else{
            panic!("Cannot Get links from: {:?}", self);
        }
    }
}

impl UserData{
    
    /// Creats a new UserData object
    /// 
    /// ## Arguments:
    /// 
    /// `name: &str` - The name of the user
    /// 
    /// ## Returns:
    /// 
    /// A new UserData object
    /// 
    /// ## Examples:
    /// ```rust
    /// use speedrunapi::UserData;
    /// let result = UserData::new("fishin_rod");
    /// println!("{:?}", result);
    /// ```
    /// This will print the parameters passed to the function, to print the data you must add .run() to the code:
    /// ```rust
    /// use speedrunapi::UserData;
    /// let result = UserData::new("fishin_rod").run();
    /// println!("{:?}", result);
    /// ```
    
    pub fn new(name: &str) -> UserData{
        UserData{
            name: name.to_string(),
            personal_bests: false,
        }
    }

    /// Does nothing for now but will swich embed to get categories later
    pub fn personal_bests(mut self) -> Self{
        self.personal_bests = true;
        self
    }

    /// Makes the request to the speedrun.com API
    /// 
    /// ## Arguments:
    /// 
    /// This function requires that you have called the new function with the nessiasary parametrers first
    /// 
    /// ## Returns:
    /// 
    /// The result of the request as a UserResult
    /// 
    /// ## Example:
    /// ```rust
    /// use speedrunapi::UserData;
    /// let result = UserData::new("fishin_rod").run();
    /// println!("{:?}", result);
    /// ```
    /// This will print the data of the user you are seraching for
    
    #[tokio::main]
    pub async fn run(&self) -> UserResult{
        let client = reqwest::Client::new();
        let url = if self.personal_bests{
            format!("https://www.speedrun.com/api/v1/users/{}/personal_bests", self.name)
        } else{
            format!("https://www.speedrun.com/api/v1/users/{}", self.name)
        };
        let response = match client.get(url).send().await{
            Ok(response) => response,
            Err(err) => return UserResult::Error(UserError::ReqwestError(err.into())),
        };
        if response.status() == reqwest::StatusCode::NOT_FOUND{
            return UserResult::Error(UserError::UserNotFound);
        }
        if self.personal_bests{
            println!("Test");
            UserResult::None
        }
        else{
            let response = match response.json::<Data>().await{
                Ok(response) => response,
                Err(err) => return UserResult::Error(UserError::ReqwestError(err.into())),
            };
            UserResult::User(response)
        }
    }
}
