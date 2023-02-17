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
//! - [User Data](##user-data)
//! - [Guest Data](##guest-data)
//! 
//! ## User Data
//! This module provides data about users on speedrun.com.
//! 
//! Users have lots of data points connected to them: 
//! id, weblink, pronouns(optional), role, signup-date, region(optional), country. and links. 
//! 
//! The user data module allows people using the crate to get these data points and use them!
//! 
//! **Example:**
//! 
//! I am using user: Bobertness as an example here, to show off roles. **This is a real user!**
//! ```rust
//! use speedrunapi::user_role;
//! let result = user_role("Bobertness");
//! assert_eq!(result, "user");
//! ```
//! 
//! ## Game Data
//! This module provides data about guests on speedrun.com
//! 
//! Guests are how speedrun.com deals with users who havent made an account / arnt logged on to their account.
//! 
//! Guests only have a name and a link connected to them.
//! 
//! ---
//! 
//! *This crate is licensed under the MIT license

pub mod user_data;
pub use user_data::{user_data, user_id, user_link, user_pronouns, user_role, user_signup, user_region, user_country, user_links};

pub mod guest_data;
pub use guest_data::{guest_data};

pub mod translate;

pub mod types;