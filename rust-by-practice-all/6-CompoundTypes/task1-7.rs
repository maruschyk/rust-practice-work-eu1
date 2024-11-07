fn main() {
    let s = "hello, world";
    greetings(s.to_string())  // Convert &str to String
}

fn greetings(s: String) {
    println!("{}", s)
}
