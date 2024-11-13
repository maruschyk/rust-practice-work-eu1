fn main() {
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            break;  // Fill the blank with `break` to exit the loop when n reaches 66
        }
        n += 1;
    }

    assert_eq!(n, 66);  // Check that n reached 66

    println!("Success!");
}
