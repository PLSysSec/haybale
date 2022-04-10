pub fn multipanic(a: i32) -> i32 {
    if a > 10 {
        panic!("a > 10");
    }else if a > 2 {
        panic!("a > 2");
    } else {
        return 1;
    }
}
