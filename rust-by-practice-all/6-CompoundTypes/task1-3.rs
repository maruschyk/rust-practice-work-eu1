fn main() {
    let mut s = String::new(); // Initialize s as an empty String
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}
