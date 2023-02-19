use speedrunapi::{gametype_data, gametype_name, gametype_id};

#[test]
fn print(){
    let result = gametype_data("Fangame");
    println!("{:?}", result)
}

#[test]
fn data(){
    let result = gametype_data("Fangame");
    assert_eq!(result, ["Fangame", "d91jd1ex", "self", "games", "https://www.speedrun.com/api/v1/gametypes/d91jd1ex", "https://www.speedrun.com/api/v1/games?gametype=d91jd1ex"])
}

#[test]
fn name(){
    let result = gametype_name("d91jd1ex");
    assert_eq!(result, "Fangame");
}

#[test]
fn id(){
    let result = gametype_id("Fangame");
    assert_eq!(result, "d91jd1ex");
}