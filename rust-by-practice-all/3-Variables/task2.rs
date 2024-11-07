fn main() {
    let mut x = 1;  // `x` is mutable and initialized with 1
    x += 2;          // `x` is updated by adding 2
    
    assert_eq!(x, 3);  // Assert that `x` equals 3
    println!("Success!");
}
