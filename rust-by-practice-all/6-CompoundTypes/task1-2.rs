fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(&s)  // Pass a reference to the Box<str>, which dereferences to &str
}

fn greetings(s: &str) {
    println!("{}", s)
}
