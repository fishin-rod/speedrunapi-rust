use speedrunapi::GameData;

#[test]
fn print(){
    let result = GameData::new("Mc").run().assets("logo");
    //assert_eq!(result, Vec::<String>::new());
    println!("{:?}", result);
}
