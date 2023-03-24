use speedrunapi::{tl_time};

#[test]
fn time(){
    let result = tl_time!("2014-10-02T12:34:23Z");
    assert_eq!(result, "2014-10-02 12:34:23");
    println!("{}", result);
}