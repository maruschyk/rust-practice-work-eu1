fn main() {
    let x = String::from("Hello world");
    let y = &x; // Borrow x

    let z = (x, y); // Store x and y together

    println!("{}, {}", z.0, z.1); // Access both values from the tuple
}
