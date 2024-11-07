fn main() {
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks with the correct variable names
    let (s1, s2) = t;

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
