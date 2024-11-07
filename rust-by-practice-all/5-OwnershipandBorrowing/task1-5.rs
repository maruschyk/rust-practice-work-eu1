fn main() {
    let x = (1, 2, (), "hello");  // Use &'static str instead of String
    let y = x;  // Tuples that consist of types implementing Copy can be moved or copied

    println!("{:?}, {:?}", x, y);
}
