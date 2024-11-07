fn main() {
    let s = String::from("Hello World");

    print_str(&s);  // Borrow `s` as a reference

    println!("{}", s);  // `s` can still be used because ownership was not moved
}

fn print_str(s: &String) {  // Accept a reference to `s`
    println!("{}", s);
}
