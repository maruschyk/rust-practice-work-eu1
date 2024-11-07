fn main() {
    let mut s = String::from("hello, ");  // `s` must be mutable

    push_str(&mut s);  // Pass a mutable reference to `push_str`

    println!("{}", s);  // Print the modified string
}

fn push_str(s: &mut String) {
    s.push_str("world");  // Modify the string
}
