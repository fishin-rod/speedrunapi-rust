use speedrunapi::LevelData;

#[test]
fn print(){
    let result = LevelData::new("495ggmwp").run();
    println!("{:?}", result);
}

#[test]
fn name(){
    let result = LevelData::new("495ggmwp").run();
    assert_eq!(result.name(), "Shrub Forest");
}

#[test]
fn id(){
    let result = LevelData::new("495ggmwp").run();
    assert_eq!(result.id(), "495ggmwp");
}

#[test]
fn weblink(){
    let result = LevelData::new("495ggmwp").run();
    assert_eq!(result.weblink(), "https://www.speedrun.com/pokemon_rumble_world/Shrub_Forest");
}

#[test]
fn links(){
    let result = LevelData::new("495ggmwp").run();
    println!("{:1} {:2}", result.links()[0].0, result.links()[0].1);
    println!("{:?}", result.links());
}