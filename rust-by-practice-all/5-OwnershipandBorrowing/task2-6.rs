fn main() {
    let c = '中';

    let r1 = &c;
    // Fill the blank, don't change other code
    let r2 = &c;  // Borrow `c` as a reference

    assert_eq!(*r1, *r2);  // Dereference both to compare their values
    
    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));  // Both references point to the same address

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)  // Print memory address of the reference
}
