fn main() {
    let a = [4, 3, 2, 1];

    // Iterate over the index and value in 'a'
    for (i, v) in a.iter().enumerate() { // Use .iter().enumerate()
        println!("The {}th element is {}", i + 1, v);
    }
}
