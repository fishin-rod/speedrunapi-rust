use speedrunapi::{game_data, game_id, game_boosts, game_abbreviation, game_weblink, game_discord, 
    game_release_date, game_ruleset, game_creation, game_assets};

#[test]
fn print(){
    let result = game_data("vertical_adventure");
    println!("{:#?}", result)
}

#[test]
fn id(){
    let result = game_id("Mc");
    assert_eq!(result, "j1npme6p")
}

#[test]
fn boosts(){
    let result = game_boosts("Mc");
    assert_eq!(result, 0)
}

#[test]
fn abbreviation(){
    let result = game_abbreviation("Mc");
    assert_eq!(result, "mc")
}

#[test]
fn weblink(){
    let result = game_weblink("Mc");
    assert_eq!(result, "https://www.speedrun.com/mc")
}

#[test]
fn discord(){
    let result = game_discord("Mc");
    assert_eq!(result, "https://discord.gg/jmdFn3C")
}

#[test]
fn release_date(){
    let result = game_release_date("Mc");
    assert_eq!(result, 2011)
}

#[test]
fn rules(){
    let result = game_ruleset("Mc");
    assert_eq!(result, ["true", "true", "true", "ingame", "false", "false"])
}

#[test]
fn creation(){
    let result = game_creation("Mc");
    assert_eq!(result, "2015-01-29 23:41:21")
}

#[test]
fn assets(){
    let result = game_assets("Mc");
    println!("{:?}", result)
}