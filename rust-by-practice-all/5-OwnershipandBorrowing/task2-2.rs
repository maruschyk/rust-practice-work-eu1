fn main() {
    let x = 5;
    let y = &x;

    // Dereference `y` to compare the value it points to (i.e., `x`)
    assert_eq!(5, *y);

    println!("Success!");
}
