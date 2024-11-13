fn main() {
    let boolean = true;

    // Match on the boolean value and assign the corresponding binary value
    let binary = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);  // Check that binary is 1

    println!("Success!");
}
