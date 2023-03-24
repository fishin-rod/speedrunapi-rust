use speedrunapi::GameTypeData;

#[test]
fn print(){
    let result = GameTypeData::new("Fangame").run();
    println!("{:?}", result)
}

#[test]
fn name(){
    let result = GameTypeData::new("d91jd1ex").run();
    assert_eq!(result.name(), "Fangame");
}

#[test]
fn id(){
    let result = GameTypeData::new("Fangame").run();
    assert_eq!(result.id(), "d91jd1ex");
}