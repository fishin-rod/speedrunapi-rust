//! # Guests
//! 
//! Guests are users who havent yet logged into speedrun.com, so they have no account.
//! 
//! 
//! ## Arguments
//! Since guests can only have a name each of the function takes the argument name.
//! 
//! name: &str, the name of the guest you are searching for.
//! 
//! ## Returns
//! Any of the functions can panic if a 404 is returned.
//! 
//! If a 404 is not returned guest_data will return the users name and links inside of a vector as strings.

use crate::types::GuestData;

#[tokio::main]
async fn guest_reqwest(name: &str) -> Result<Vec<String>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("https://www.speedrun.com/api/v1/guests/{:1}", name);
    let response = client.get(url).send().await.unwrap();
    if response.status() == reqwest::StatusCode::NOT_FOUND{
        panic!("Guest Not Found!")
    }
    let response = response.json::<GuestData>().await.unwrap();
    
    let guest = response.data;
    let mut guest_data = Vec::<String>::new();

    guest_data.extend([guest.name, guest.links.rel, guest.links.uri]);
    Ok(guest_data)
}

/// Returns all of the data for a guest
/// 
/// The data iis returned inside a vector contaiting strings.
pub fn guest_data(name: &str) -> Vec<std::string::String>{
    let guest_data = guest_reqwest(name).expect("guest Not found");
    return guest_data;
}