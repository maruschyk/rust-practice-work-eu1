fn main() {
    let mut s = String::from("hello");  // `s` must be mutable to modify it
    s.push(',');  // push a character
    s.push_str(" world");  // push a string slice
    s += "!";  // `+=` accepts a &str directly, no need to call to_string()

    println!("{}", s);
}
