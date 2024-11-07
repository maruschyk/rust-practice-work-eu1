fn main() {
    // FILL in the blank
    let b = false; // Setting `b` to `false` triggers the panic

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!"); // This will be printed
            panic!("we have no value for `false`, but we can panic"); // This will cause a panic
        }
    };

    println!("Exercise Failed if printing out this line!"); // This line will never be printed
}
