pub fn add(a: i32, b: i32) -> i32{
    a + b
}

#[test]
fn it_works() {
    assert_eq!(add(1,1), 2);
}
