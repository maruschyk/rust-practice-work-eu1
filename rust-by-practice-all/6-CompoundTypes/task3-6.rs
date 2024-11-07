fn main() {
    let mut s = String::from("hello world");

    let letter = first_letter(&s);

    // Clear the String after using the slice
    s.clear();

    println!("the first letter is: {}", letter);
}

fn first_letter(s: &str) -> &str {
    &s[..1]
}
