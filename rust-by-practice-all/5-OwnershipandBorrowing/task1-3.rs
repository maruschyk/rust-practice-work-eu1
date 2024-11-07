fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    let _s_bytes = s.clone().into_bytes(); // Clone to retain ownership of `s`
    s // Return the original `String` after cloning for the byte conversion
}
