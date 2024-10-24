use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */
fn plusMinus(arr: &[i32]) {
    let n = arr.len() as f64; // Total number of elements
    let (mut positive_count, mut negative_count, mut zero_count) = (0, 0, 0);

    for &num in arr {
        if num > 0 {
            positive_count += 1;
        } else if num < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    // Calculate fractions
    let positive_fraction = positive_count as f64 / n;
    let negative_fraction = negative_count as f64 / n;
    let zero_fraction = zero_count as f64 / n;

    // Print results formatted to six decimal places
    println!("{:.6}", positive_fraction);
    println!("{:.6}", negative_fraction);
    println!("{:.6}", zero_fraction);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
