fn main() {
    let s = "hello, world".to_string();
    let s1: &str = &s;  // Borrow a &str directly from s

    println!("Success!");
}
