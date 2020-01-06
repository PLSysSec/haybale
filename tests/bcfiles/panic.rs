pub fn may_panic(a: i32) -> i32 {
    if a > 2 {
        panic!("a > 2");
    } else {
        return 1;
    }
}
