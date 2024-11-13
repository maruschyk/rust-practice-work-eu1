fn main() {
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n += 1;
            continue;  // First blank: Use `continue` to move to the next iteration
        }
        
        break;  // Second blank: Use `break` to exit the loop when n == 66
    }

    assert_eq!(n, 66);  // Check that n reached 66

    println!("Success!");
}
