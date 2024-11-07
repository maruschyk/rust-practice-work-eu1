fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);  // Borrow `s` as an immutable reference

    println!("Success!");
}

fn borrow_object(s: &String) {}  // Accept `s` as an immutable reference
