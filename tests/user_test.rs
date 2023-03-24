use speedrunapi::UserData;

#[test]
fn print(){
    let result = UserData::new("fishin_rod").run();
    println!("{:?}", result);
}
