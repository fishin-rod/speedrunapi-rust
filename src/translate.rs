//! # Translations
//! 
//! This file is for the translations and conversions of certin items for working with the library

/// This function helps with translating time
/// 
/// ## Arguments
/// 
/// signup: String, a time in the format rfc3339 
/// 
/// ## Returns
/// 
/// A time that is more readable by a human as a string
/// 
/// ## Examples
/// ```rust
/// use speedrunapi::tl_time;
/// let result = tl_time!("2014-10-02T12:34:23Z");
/// assert_eq!(result, "2014-10-02 12:34:23");
/// ```

#[macro_export]
#[cfg(feature = "translations")]
macro_rules! tl_time {
    ($time:expr) => {
        chrono::DateTime::parse_from_rfc3339(&$time).unwrap().format("%Y-%m-%d %H:%M:%S").to_string()
    };
}
