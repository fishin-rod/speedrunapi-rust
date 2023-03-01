//! # Types
//! 
//! This section contains the JSON values for easy formating and extraction of the data
//! 
//! **NOTE:** This section is intedned for use inside the crate only,
//! if you want to venture and you this area in you own project go ahead, but you may encounter issues.

pub mod user;
pub use user::UserData;

pub mod guest;
pub use guest::GuestData;

pub mod game;
pub use game::GameData;

pub mod gametype;
pub use gametype::GameTypeData;
