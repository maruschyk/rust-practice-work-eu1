fn main() {
    let (mut x, y) = (1, 2); // Make `x` mutable
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
