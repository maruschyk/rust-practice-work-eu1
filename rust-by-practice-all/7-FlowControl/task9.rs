fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;  // First blank: use `continue` to skip this iteration
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;  // Second blank: use `break` to exit the loop
        }
    }

    assert_eq!(count, 5);

    println!("Success!");
}
