use speedrunapi::translate::{tl_time, username_to_id};

#[test]
fn time(){
    let result = tl_time("2014-10-02T12:34:23Z");
    assert_eq!(result, "2014-10-02 12:34:23");
}

#[test]
fn name(){
    let result = username_to_id("fishin_rod");
    assert_eq!(result, "jonryvl8");
}
