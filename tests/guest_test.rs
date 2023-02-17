use speedrunapi::guest_data;

#[test]
fn data(){
    let result = guest_data("Alex");
    println!("{:#?}", result);
}