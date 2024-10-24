use std::io::{self, BufRead};

/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn staircase(n: i32) {
    for i in 0..n {
        // Calculate the number of spaces and hashes
        let spaces = " ".repeat((n - 1 - i) as usize);
        let hashes = "#".repeat((i + 1) as usize);
        
        // Print the current row
        println!("{}{}", spaces, hashes);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}
