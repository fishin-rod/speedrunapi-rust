//! # Speedrunapi
//! A REST API wrapper for speedrun.com's API
//! 
//! Speedrunapi aims to make working with speedrun.com's api fast and easy
//! 
//! ## Speedrunapi provides:
//! 
//! - Pre-formatting of data 
//! - Easy to use structure
//! - Translations of ids/times (Not really used yet will be used more soon!)
//! - Error handeling
//! 
//! ## Current Modules:
//! 
//! - [User Data](#user-data)
//! - [Guest Data](#guest-data)
//! - [Game Data](#game-data)
//! - [Game Types](#game-types)
//! - [Level Data](#level-data)
//! 
//! # User Data
//! This module provides data about users on speedrun.com.
//! 
//! Users have lots of data points connected to them. 
//! 
//! The user data module allows people using the crate to get these data points and use them!
//! 
//! **Example:**
//! 
//! I am using user: Bobertness as an example here, to show off roles. **This is a real user!**
//! ```rust
//! use speedrunapi::UserData;
//! let result = UserData::new("Bobertness").run();
//! assert_eq!(result.role(), "user");
//! ```
//! 
//! # Guest Data
//! This module provides data about guests on speedrun.com
//! 
//! Guests are how speedrun.com deals with users who havent made an account / arn't logged on to their account.
//! 
//! Guests only have a name and a link connected to them.
//! 
//! # Game Data
//! This module provides data about games on speedrun.com
//! 
//! Games are places on speedrun.com where users can submit runs for.
//! 
//! Games have many data points connected to them.
//! There are a few more data points that go with a game, but require a new module to be used (in progress).
//! 
//! **Example:**
//! 
//! This examples shows the fetching of the weblink of a game.
//! ```rust
//! use speedrunapi::GameData;
//! let result = GameData::new("MC").run();
//! assert_eq!(result.weblink(), "https://www.speedrun.com/mc");
//! ```
//! 
//! # Game Types
//! This module provides data on game types.
//! 
//! What is a gametype?
//! 
//! According to speedrun.com: (Game types are classifications for unofficial games, for example ROM Hack, Fangame, Modification etc.)
//! 
//! Game types only have a name, an id, and links attached to them.
//! 
//! # Levels
//! 
//! ---
//! 
//! *This crate is licensed under the MIT license

#[cfg_attr(docsrs, doc(cfg(feature = "speedrunapi")))]
pub mod user_data;
pub use user_data::UserData;

pub mod guest_data;
pub use guest_data::{guest_data};

pub mod game_data;
pub use game_data::GameData;

pub mod gametype_data;
pub use gametype_data::GameTypeData;

pub mod level_data;
pub use level_data::LevelData;

#[cfg(feature = "translations")]
pub mod translate;

pub(crate) mod types;