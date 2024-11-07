fn main() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s;  // Borrow `s` mutably
    
    p.push_str("world");

    println!("{}", s);  // To check if it worked, print `s`
}
