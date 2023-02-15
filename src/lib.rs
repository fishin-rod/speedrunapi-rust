//! # Speedrunapi
//! A REST API wrapper for speedrun.com's API
//! 
//! This crate is licensed under the MIT license
//! 
//! Speedrunapi aims to make working with speedrun.com's api fast and easy
//! ### Speedrunapi provides:
//! 
//! - High and low level operations on the data
//! - Easy to use structure
//! - Pre formating of the data
//! - Asyncronys requests to speedrun.com saving time

#[doc = include_str!(r"C:\Users\conno\Downloads\speedrunapi(rust)\README.md" )]

pub mod user_data;
pub use user_data::{user_data, user_id, user_link, user_pronouns, user_role, user_signup, user_region, user_country, user_links};

pub mod translate;

pub mod types;