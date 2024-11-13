fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break 20;  // Fill in the blank with `break 20` to assign 20 to `result`
        }
    };

    assert_eq!(result, 20);  // This checks that `result` is 20 after the loop

    println!("Success!");
}
