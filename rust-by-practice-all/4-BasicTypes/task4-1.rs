fn main() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {  // Add type annotation for `x` and `y` and specify the return type `-> i32`
    x + y  // This is the last expression, and it will be returned implicitly
}
