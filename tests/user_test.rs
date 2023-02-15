use speedrunapi::{user_data, user_id, user_link, user_pronouns, user_role, user_signup, user_region, user_country, user_links};

#[test]
fn data(){
    let result = user_data("fishin_rod");
    assert_eq!(result, ["jonryvl8", "fishin_rod", "https://www.speedrun.com/user/fishin_rod", "He/Him", "user", "2022-12-20 01:16:42", "us/co", "Colorado, USA", "us", "United States", "None", "None", "None", "None", "None"])

}

#[test]
fn id(){
    let result = user_id("fishin_rod");
    assert_eq!(result, "jonryvl8");
}

#[test]
fn link(){
    let result = user_link("fishin_rod");
    assert_eq!(result, "https://www.speedrun.com/user/fishin_rod");
}

#[test]
fn pronouns(){
    let result = user_pronouns("fishin_rod");
    assert_eq!(result, "He/Him");
}

#[test]
fn role(){
    let result = user_role("fishin_rod");
    assert_eq!(result, "user");
}

#[test]
fn signup(){
    let result = user_signup("fishin_rod");
    assert_eq!(result, "2022-12-20 01:16:42")
}

#[test]
fn region(){
    let result = user_region("fishin_rod");
    assert_eq!(result, "Colorado, USA");
}

#[test]
fn country(){
    let result = user_country("fishin_rod");
    assert_eq!(result, "United States");
}

#[test]
fn links(){
    let result = user_links("fishin_rod");
    assert_eq!(result, ["None", "None", "None", "None", "None"]);
}